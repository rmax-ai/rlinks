use clap::Parser;
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;
use std::time::Instant;

#[derive(Parser, Debug)]
struct Opts {
    #[arg(long)]
    url: String,

    #[arg(long, default_value_t = 10)]
    concurrency: usize,

    #[arg(long, default_value_t = 1000usize)]
    requests: usize,

    #[arg(long)]
    out: Option<String>,
}

#[derive(Serialize)]
struct ResultJson {
    p50_ms: f64,
    p95_ms: f64,
    p99_ms: f64,
    count: usize,
    errors: usize,
    rps: f64,
    latencies_ms: Vec<f64>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let client = Client::new();

    let results = run_harness(&client, &opts.url, opts.concurrency, opts.requests).await?;
    if let Some(path) = opts.out {
        let s = serde_json::to_string_pretty(&results)?;
        std::fs::write(path, s)?;
    } else {
        println!("{:?}", serde_json::to_string_pretty(&results)?);
    }
    Ok(())
}

async fn run_harness(
    client: &Client,
    url: &str,
    concurrency: usize,
    requests: usize,
) -> anyhow::Result<ResultJson> {
    let start = Instant::now();
    let latencies = Arc::new(tokio::sync::Mutex::new(Vec::with_capacity(requests)));
    let errors = Arc::new(tokio::sync::Mutex::new(0usize));

    let requests_per_task = requests / concurrency;
    let mut handles = Vec::new();
    for _ in 0..concurrency {
        let client = client.clone();
        let url = url.to_string();
        let latencies = latencies.clone();
        let errors = errors.clone();
        let handle = tokio::spawn(async move {
            for _ in 0..requests_per_task {
                let t = Instant::now();
                let resp = client.get(&url).send().await;
                let dur = t.elapsed();
                match resp {
                    Ok(r) => {
                        if !r.status().is_success() && r.status() != reqwest::StatusCode::FOUND {
                            let mut e = errors.lock().await;
                            *e += 1;
                        }
                    }
                    Err(_) => {
                        let mut e = errors.lock().await;
                        *e += 1;
                    }
                }
                let mut l = latencies.lock().await;
                l.push(dur.as_secs_f64() * 1_000.0f64);
            }
        });
        handles.push(handle);
    }

    for h in handles {
        let _ = h.await;
    }

    let elapsed = start.elapsed();
    let latencies = Arc::try_unwrap(latencies).unwrap().into_inner();
    let errors = Arc::try_unwrap(errors).unwrap().into_inner();
    let mut lat = latencies;
    lat.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let count = lat.len();
    let p50 = percentile(&lat, 50);
    let p95 = percentile(&lat, 95);
    let p99 = percentile(&lat, 99);
    let rps = (requests as f64) / elapsed.as_secs_f64();

    Ok(ResultJson {
        p50_ms: p50,
        p95_ms: p95,
        p99_ms: p99,
        count,
        errors,
        rps,
        latencies_ms: lat,
    })
}

fn percentile(sorted: &[f64], p: usize) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((p as f64) / 100.0 * (sorted.len() as f64 - 1.0)).round() as usize;
    sorted[idx]
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::service::{make_service_fn, service_fn};
    use hyper::{Body, Request, Response, Server, StatusCode};
    use std::convert::Infallible;
    use std::net::SocketAddr;

    #[tokio::test]
    async fn test_harness_runs_against_local_server() {
        // start a local server
        let make_svc = make_service_fn(|_| async {
            Ok::<_, Infallible>(service_fn(|_req: Request<Body>| async move {
                Ok::<_, Infallible>(
                    Response::builder()
                        .status(StatusCode::FOUND)
                        .header("Location", "https://example.com")
                        .body(Body::empty())
                        .unwrap(),
                )
            }))
        });
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let server = Server::bind(&addr).serve(make_svc);
        let actual_addr = server.local_addr();
        let (tx, rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            let _ = server
                .with_graceful_shutdown(async {
                    rx.await.ok();
                })
                .await;
        });

        let client = Client::new();
        let url = format!("http://{}/ok-code", actual_addr);
        let res = run_harness(&client, &url, 2, 20).await.unwrap();
        assert_eq!(res.count, 20);
        assert_eq!(res.errors, 0);
        let _ = tx.send(());
    }
}
