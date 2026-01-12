use std::convert::Infallible;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use clap::{Parser, ValueEnum};
use hyper::header::LOCATION;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

#[derive(Parser, Debug)]
#[command(name = "rlinks-worker")]
struct Opts {
    /// Start HTTP server on port
    #[arg(long)]
    http: Option<u16>,

    /// Mode for hit logging: append or kv
    #[arg(long, value_enum, default_value_t = Mode::Append)]
    mode: Mode,

    /// Artificial KV latency in ms (only for kv mode)
    #[arg(long, default_value_t = 0u64)]
    kv_latency_ms: u64,

    /// Positional code (legacy CLI mode)
    code: Option<String>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Mode {
    Append,
    Kv,
}

fn append_hit(hits_path: &PathBuf, code: &str) -> Result<()> {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let entry = serde_json::json!({"ts": ts, "code": code});
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(hits_path)?;
    writeln!(file, "{}", entry)?;
    Ok(())
}

fn kv_write_hit(stats_path: &PathBuf, code: &str, latency_ms: u64) -> Result<()> {
    // Simulate per-hit KV write with a synchronous write and optional latency
    if latency_ms > 0 {
        std::thread::sleep(Duration::from_millis(latency_ms));
    }
    let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let entry = serde_json::json!({"ts": ts, "code": code});
    // For simplicity, write each hit as its own line to stats.db (simulate KV write)
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(stats_path)?;
    writeln!(file, "{}", entry)?;
    Ok(())
}

async fn handle_request(
    req: Request<Body>,
    mode: Mode,
    kv_latency_ms: u64,
) -> Result<Response<Body>, Infallible> {
    // Minimal in-memory map
    let map = std::collections::HashMap::from([(
        "ok-code".to_string(),
        "https://example.com".to_string(),
    )]);

    let path = req.uri().path();
    if req.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("only GET allowed"))
            .unwrap());
    }

    // strip leading '/'
    let code = path.trim_start_matches('/');
    if code.is_empty() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("missing code"))
            .unwrap());
    }

    match map.get(code) {
        Some(target) => {
            // record hit depending on mode
            let code_owned = code.to_string();
            match mode {
                Mode::Append => {
                    let hits_path = PathBuf::from("hits.log");
                    // spawn blocking write
                    let _ =
                        tokio::task::spawn_blocking(move || append_hit(&hits_path, &code_owned))
                            .await;
                }
                Mode::Kv => {
                    let stats_path = PathBuf::from("stats.db");
                    let _ = tokio::task::spawn_blocking(move || {
                        kv_write_hit(&stats_path, &code_owned, kv_latency_ms)
                    })
                    .await;
                }
            }

            let resp = Response::builder()
                .status(StatusCode::FOUND)
                .header(LOCATION, target.as_str())
                .body(Body::empty())
                .unwrap();
            Ok(resp)
        }
        None => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not found"))
            .unwrap()),
    }
}

async fn run_server(port: u16, mode: Mode, kv_latency_ms: u64) -> Result<()> {
    let addr = ([127, 0, 0, 1], port).into();
    let make_svc = make_service_fn(move |_| {
        let mode = mode;
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, mode, kv_latency_ms)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    eprintln!(
        "rlinks-worker: listening on http://{} (mode={:?})",
        addr, mode
    );
    server.await?;
    Ok(())
}

fn run_legacy_with_hits_path(args: &[String], hits_path: &PathBuf) -> i32 {
    if args.len() < 2 {
        eprintln!("Usage: rlinks-worker <code>");
        return 2;
    }
    let code = &args[1];

    // Minimal in-memory store for PoC
    let map = std::collections::HashMap::from([(
        "ok-code".to_string(),
        "https://example.com".to_string(),
    )]);

    match map.get(code) {
        Some(target) => {
            // append a minimal hit record (ts, code)
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let entry = serde_json::json!({"ts": ts, "code": code});
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(hits_path)
                .unwrap();
            writeln!(file, "{}", entry).unwrap();
            println!("{}", target);
            0
        }
        None => {
            eprintln!("Not found");
            1
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    if let Some(port) = opts.http {
        // run HTTP server
        run_server(port, opts.mode, opts.kv_latency_ms).await?;
        Ok(())
    } else if let Some(code) = opts.code {
        // legacy CLI mode, call existing function
        let args = vec!["rlinks-worker".to_string(), code];
        let hits_path = PathBuf::from("hits.log");
        let exit = run_legacy_with_hits_path(&args, &hits_path);
        std::process::exit(exit);
    } else {
        eprintln!("Either provide --http PORT to run server or a <code> positional argument.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use tempfile::tempdir;

    #[test]
    fn test_run_appends_hit_and_prints_target() {
        let dir = tempdir().unwrap();
        let hits_path = dir.path().join("hits.log");
        let args = vec!["rlinks-worker".to_string(), "ok-code".to_string()];
        assert_eq!(run_legacy_with_hits_path(&args, &hits_path), 0);
        let content = read_to_string(&hits_path).unwrap();
        assert!(content.contains("\"code\":\"ok-code\""));
    }

    #[test]
    fn test_run_not_found() {
        let dir = tempdir().unwrap();
        let hits_path = dir.path().join("hits.log");
        let args = vec!["rlinks-worker".to_string(), "missing".to_string()];
        assert_eq!(run_legacy_with_hits_path(&args, &hits_path), 1);
        assert!(!hits_path.exists());
    }

    #[tokio::test]
    async fn test_kv_and_append_write() {
        let dir = tempdir().unwrap();
        let hits_path = dir.path().join("hits.log");
        let stats_path = dir.path().join("stats.db");

        append_hit(&hits_path, "ok-code").unwrap();
        assert!(read_to_string(&hits_path).unwrap().contains("ok-code"));

        kv_write_hit(&stats_path, "ok-code", 1).unwrap();
        assert!(read_to_string(&stats_path).unwrap().contains("ok-code"));
    }
}
