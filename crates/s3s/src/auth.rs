//! S3 Authentication

use crate::error::S3Result;

use std::collections::HashMap;

/// S3 Authentication Provider
#[async_trait::async_trait]
pub trait S3Auth: Send + Sync + 'static {
    /// lookup `secret_access` by `access_key`
    async fn get_secret_key(&self, access_key: &str) -> S3Result<String>;
}

/// A simple authentication provider
#[derive(Debug, Default)]
pub struct SimpleAuth {
    /// key map
    map: HashMap<String, String>,
}

impl SimpleAuth {
    /// Constructs a new `SimpleAuth`
    #[must_use]
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    /// register a credential
    pub fn register(&mut self, access_key: String, secret_key: String) -> Option<String> {
        self.map.insert(access_key, secret_key)
    }

    /// lookup a credential
    #[must_use]
    pub fn lookup(&self, access_key: &str) -> Option<&str> {
        Some(self.map.get(access_key)?.as_str())
    }
}

#[async_trait::async_trait]
impl S3Auth for SimpleAuth {
    async fn get_secret_key(&self, access_key: &str) -> S3Result<String> {
        match self.lookup(access_key) {
            None => Err(s3_error!(NotSignedUp, "Your account is not signed up")),
            Some(s) => Ok(s.to_owned()),
        }
    }
}
