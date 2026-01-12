use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn aggregate_from_latencies_counts_len() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;

    let in_path = td.path().join("hits.json");
    let out_path = td.path().join("agg.json");

    // bench-harness-style JSON with a latencies array
    fs::write(&in_path, r#"{"latencies": [1, 2, 3, 4]}"#)?;

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("rlinks-bh2"));
    cmd.arg("aggregate")
        .arg("--in")
        .arg(in_path.as_os_str())
        .arg("--out")
        .arg(out_path.as_os_str())
        .arg("--code")
        .arg("ok-code");

    cmd.assert().success();

    let out = fs::read_to_string(&out_path)?;
    let v: serde_json::Value = serde_json::from_str(&out)?;
    assert_eq!(v["count"], 4);
    assert_eq!(v["agg"]["ok-code"], 4);

    Ok(())
}

#[test]
fn aggregate_from_hits_array_counts_per_code() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;

    let in_path = td.path().join("hits2.json");
    let out_path = td.path().join("agg2.json");

    // array of hits
    fs::write(&in_path, r#"[{"code":"a"},{"code":"b"},{"code":"a"}]"#)?;

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("rlinks-bh2"));
    cmd.arg("aggregate")
        .arg("--in")
        .arg(in_path.as_os_str())
        .arg("--out")
        .arg(out_path.as_os_str());

    cmd.assert().success();

    let out = fs::read_to_string(&out_path)?;
    let v: serde_json::Value = serde_json::from_str(&out)?;
    assert_eq!(v["count"], 3);
    assert_eq!(v["agg"]["a"], 2);
    assert_eq!(v["agg"]["b"], 1);

    Ok(())
}
