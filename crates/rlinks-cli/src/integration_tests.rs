use serde_json::Value;
use std::process::Command;

// Integration tests for the CLI
// These tests run the compiled CLI binary against a mock or local environment
// to ensure end-to-end functionality.

fn get_cli_path() -> String {
    // Assuming the binary is built and available in target/debug/rlinks-cli
    // or run via cargo run -p rlinks-cli --
    "cargo".to_string()
}

fn run_cli(args: &[&str]) -> (bool, String, String) {
    let mut command = Command::new(get_cli_path());
    command.arg("run").arg("-p").arg("rlinks-cli").arg("--");

    for arg in args {
        command.arg(arg);
    }

    let output = command.output().expect("Failed to execute command");

    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

#[test]
fn test_create_valid_redirect() {
    let (success, stdout, _) = run_cli(&["test-code", "https://example.com"]);
    assert!(success, "CLI should succeed for valid redirect");

    let json: Value = serde_json::from_str(&stdout).expect("Output should be valid JSON");
    assert_eq!(json["code"], "test-code");
    assert_eq!(json["target"], "https://example.com");
}

#[test]
fn test_create_invalid_redirect_reserved() {
    let (success, _, stderr) = run_cli(&["api", "https://example.com"]);
    assert!(!success, "CLI should fail for reserved code");
    assert!(
        stderr.contains("Validation error"),
        "Stderr should contain validation error"
    );
}

#[test]
fn test_create_invalid_redirect_url() {
    let (success, _, stderr) = run_cli(&["valid-code", "not-a-url"]);
    assert!(!success, "CLI should fail for invalid URL");
    assert!(
        stderr.contains("Validation error"),
        "Stderr should contain validation error"
    );
}

#[test]
fn test_create_invalid_redirect_http() {
    // By default HTTP is not allowed
    let (success, _, _stderr) = run_cli(&["valid-code", "http://example.com"]);
    assert!(!success, "CLI should fail for HTTP URL without flag");
    // Note: The current CLI implementation in main.rs doesn't seem to expose flags yet,
    // but the core validation might enforce it.
    // Based on rlinks-cli/src/main.rs, it just creates a Redirect struct and calls validate_redirect.
}
