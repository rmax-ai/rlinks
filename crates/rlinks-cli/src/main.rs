use clap::{Parser, Subcommand};
use rlinks_core::{validate_redirect, Redirect};
use rlinks_kv::{CloudflareKv, KvStore};

#[cfg(test)]
mod integration_tests;

#[derive(Parser)]
#[command(name = "rlinks")]
#[command(about = "Manage rlinks redirects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Cloudflare Account ID (can also be set via CF_ACCOUNT_ID env var)
    #[clap(long)]
    account_id: Option<String>,

    /// Cloudflare KV Namespace ID (can also be set via CF_NAMESPACE_ID env var)
    #[clap(long)]
    namespace_id: Option<String>,

    /// Cloudflare API Token (can also be set via CF_API_TOKEN env var)
    #[clap(long)]
    api_token: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create or update a redirect
    Set {
        /// The short code for the redirect
        code: String,
        /// The target URL
        target: String,
    },
    /// Get a redirect by code
    Get {
        /// The short code to retrieve
        code: String,
    },
    /// Delete a redirect (disable)
    Delete {
        /// The short code to delete
        code: String,
    },
    /// Validate a redirect without saving
    Validate {
        /// The short code
        code: String,
        /// The target URL
        target: String,
    },
}

async fn get_store(cli: &Cli) -> anyhow::Result<CloudflareKv> {
    let account_id = cli
        .account_id
        .clone()
        .or_else(|| std::env::var("CF_ACCOUNT_ID").ok())
        .ok_or_else(|| anyhow::anyhow!("CF_ACCOUNT_ID is required (via flag or env var)"))?;
    let namespace_id = cli
        .namespace_id
        .clone()
        .or_else(|| std::env::var("CF_NAMESPACE_ID").ok())
        .ok_or_else(|| anyhow::anyhow!("CF_NAMESPACE_ID is required (via flag or env var)"))?;
    let api_token = cli
        .api_token
        .clone()
        .or_else(|| std::env::var("CF_API_TOKEN").ok())
        .ok_or_else(|| anyhow::anyhow!("CF_API_TOKEN is required (via flag or env var)"))?;

    Ok(CloudflareKv::new(account_id, namespace_id, api_token))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Validate { code, target } => {
            let r = Redirect {
                code: code.clone(),
                target: target.clone(),
            };
            match validate_redirect(&r) {
                Ok(()) => {
                    println!("{}", serde_json::to_string_pretty(&r).unwrap());
                    Ok(())
                }
                Err(e) => Err(anyhow::anyhow!("Validation error: {}", e)),
            }
        }
        Commands::Set { code, target } => {
            let store = get_store(&cli).await?;
            let r = Redirect {
                code: code.clone(),
                target: target.clone(),
            };
            validate_redirect(&r).map_err(|e| anyhow::anyhow!("Validation error: {}", e))?;
            store.put(code, &r).await?;
            println!("Redirect set: {} -> {}", code, target);
            Ok(())
        }
        Commands::Get { code } => {
            let store = get_store(&cli).await?;
            match store.get(code).await? {
                Some(r) => {
                    println!("{}", serde_json::to_string_pretty(&r).unwrap());
                    Ok(())
                }
                None => Err(anyhow::anyhow!("Redirect not found: {}", code)),
            }
        }
        Commands::Delete { code } => {
            let store = get_store(&cli).await?;
            store.delete(code).await?;
            println!("Redirect deleted: {}", code);
            Ok(())
        }
    }
}
