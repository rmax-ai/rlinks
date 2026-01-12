use std::io::Write;
use std::path::Path;

fn run_with_hits_path(args: &[String], hits_path: &Path) -> i32 {
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
            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
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

fn run(args: &[String]) -> i32 {
    let hits_path = std::path::Path::new("hits.log");
    run_with_hits_path(args, hits_path)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    std::process::exit(run(&args));
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
        assert_eq!(run_with_hits_path(&args, &hits_path), 0);
        let content = read_to_string(&hits_path).unwrap();
        assert!(content.contains("\"code\":\"ok-code\""));
    }

    #[test]
    fn test_run_not_found() {
        let dir = tempdir().unwrap();
        let hits_path = dir.path().join("hits.log");
        let args = vec!["rlinks-worker".to_string(), "missing".to_string()];
        assert_eq!(run_with_hits_path(&args, &hits_path), 1);
        assert!(!hits_path.exists());
    }
}
