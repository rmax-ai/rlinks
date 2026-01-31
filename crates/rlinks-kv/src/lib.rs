use async_trait::async_trait;
use rlinks_core::Redirect;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Key not found: {0}")]
    NotFound(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Other: {0}")]
    Other(String),
}

#[async_trait]
pub trait KvStore {
    async fn get(&self, key: &str) -> Result<Option<Redirect>, KvError>;
    async fn put(&self, key: &str, value: &Redirect) -> Result<(), KvError>;
    async fn delete(&self, key: &str) -> Result<(), KvError>;
}

pub struct CloudflareKv {
    account_id: String,
    namespace_id: String,
    api_token: String,
    client: reqwest::Client,
}

impl CloudflareKv {
    pub fn new(account_id: String, namespace_id: String, api_token: String) -> Self {
        Self {
            account_id,
            namespace_id,
            api_token,
            client: reqwest::Client::new(),
        }
    }

    fn base_url(&self) -> String {
        format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/storage/kv/namespaces/{}/values",
            self.account_id, self.namespace_id
        )
    }
}

#[async_trait]
impl KvStore for CloudflareKv {
    async fn get(&self, key: &str) -> Result<Option<Redirect>, KvError> {
        let url = format!("{}/{}", self.base_url(), key);
        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await
            .map_err(|e| KvError::Network(e.to_string()))?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        if !resp.status().is_success() {
            return Err(KvError::Other(format!("KV Error: {}", resp.status())));
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| KvError::Network(e.to_string()))?;
        let redirect: Redirect =
            serde_json::from_slice(&bytes).map_err(|e| KvError::Serialization(e.to_string()))?;

        Ok(Some(redirect))
    }

    async fn put(&self, key: &str, value: &Redirect) -> Result<(), KvError> {
        let url = format!("{}/{}", self.base_url(), key);
        let body =
            serde_json::to_string(value).map_err(|e| KvError::Serialization(e.to_string()))?;

        let resp = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .body(body)
            .send()
            .await
            .map_err(|e| KvError::Network(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(KvError::Other(format!("KV Error: {}", resp.status())));
        }

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), KvError> {
        let url = format!("{}/{}", self.base_url(), key);
        let resp = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await
            .map_err(|e| KvError::Network(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(KvError::Other(format!("KV Error: {}", resp.status())));
        }

        Ok(())
    }
}
