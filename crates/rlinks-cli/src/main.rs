use rlinks_core::{validate_redirect, Redirect};

fn run(args: &[String]) -> i32 {
    if args.len() < 3 {
        eprintln!("Usage: rlinks-cli <code> <target>");
        return 2;
    }
    let r = Redirect {
        code: args[1].clone(),
        target: args[2].clone(),
    };
    match validate_redirect(&r) {
        Ok(()) => {
            println!("{}", serde_json::to_string_pretty(&r).unwrap());
            0
        }
        Err(e) => {
            eprintln!("Validation error: {}", e);
            1
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    std::process::exit(run(&args));
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_run_valid() {
        let args = vec![
            "rlinks-cli".to_string(),
            "ok-code".to_string(),
            "https://example.com".to_string(),
        ];
        assert_eq!(run(&args), 0);
    }

    #[test]
    fn test_run_invalid() {
        let args = vec![
            "rlinks-cli".to_string(),
            "api".to_string(),
            "http://example.com".to_string(),
        ];
        assert_eq!(run(&args), 1);
    }
}
