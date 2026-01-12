use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde_json::Value;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Aggregate hits JSON into stats JSON
    Aggregate {
        /// Input JSON file (bench-harness output or hits array)
        #[arg(long = "in", value_name = "FILE")]
        in_file: PathBuf,

        /// Output file (defaults to stdout if omitted)
        #[arg(long = "out", value_name = "FILE")]
        out_file: Option<PathBuf>,

        /// Code to attribute (when input is bench-harness `latencies` array)
        #[arg(long = "code")]
        code: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Aggregate {
            in_file,
            out_file,
            code,
        } => {
            let mut s = String::new();
            File::open(&in_file)
                .with_context(|| format!("opening input file {}", in_file.display()))?
                .read_to_string(&mut s)
                .with_context(|| "reading input file contents")?;

            let v: Value = serde_json::from_str(&s).with_context(|| "parsing input JSON")?;
            let AggResult::Map(agg) = parse_and_aggregate(&v, code)?;

            let total: u64 = agg.values().copied().sum();
            let out_value = serde_json::json!({"agg": agg, "count": total});

            match out_file {
                Some(p) => {
                    let mut f = File::create(&p)
                        .with_context(|| format!("creating output file {}", p.display()))?;
                    f.write_all(serde_json::to_string_pretty(&out_value)?.as_bytes())?;
                    println!("Saved {}", p.display());
                }
                None => {
                    println!("{}", serde_json::to_string_pretty(&out_value)?);
                }
            }
        }
    }
    Ok(())
}

enum AggResult {
    Map(HashMap<String, u64>),
}

fn parse_and_aggregate(v: &Value, code_hint: Option<String>) -> Result<AggResult> {
    // If it's an object
    if let Value::Object(map) = v {
        // If it has 'hits' array of objects {code, ts}
        if let Some(Value::Array(hits)) = map.get("hits") {
            let mut agg: HashMap<String, u64> = HashMap::new();
            for h in hits {
                if let Some(Value::String(code)) = h.get("code") {
                    *agg.entry(code.clone()).or_insert(0) += 1;
                }
            }
            return Ok(AggResult::Map(agg));
        }
        // If it has 'latencies' or 'latencies_ms' and a code provided, count them
        if let Some(Value::Array(latencies)) =
            map.get("latencies").or_else(|| map.get("latencies_ms"))
        {
            let code = code_hint
                .ok_or_else(|| anyhow::anyhow!("input has 'latencies' but no --code provided"))?;
            let mut agg = HashMap::new();
            agg.insert(code, latencies.len() as u64);
            return Ok(AggResult::Map(agg));
        }
        // If it has 'count' at top-level, use that as total and map to provided code
        if let Some(cnt) = map.get("count")
            && let Some(n) = cnt.as_u64()
        {
            let code = code_hint
                .ok_or_else(|| anyhow::anyhow!("input has 'count' but no --code provided"))?;
            let mut agg = HashMap::new();
            agg.insert(code, n);
            return Ok(AggResult::Map(agg));
        }
    }

    // If it's an array of objects
    if let Value::Array(arr) = v {
        let mut agg: HashMap<String, u64> = HashMap::new();
        for item in arr {
            if let Some(Value::String(code)) = item.get("code") {
                *agg.entry(code.clone()).or_insert(0) += 1;
            }
        }
        return Ok(AggResult::Map(agg));
    }

    bail!("unrecognized input JSON format for aggregation");
}
