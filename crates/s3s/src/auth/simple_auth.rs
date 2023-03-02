use super::S3Auth;

use crate::auth::SecretKey;
use crate::error::S3Result;

use std::collections::HashMap;

/// A simple authentication provider
#[derive(Debug, Default)]
pub struct SimpleAuth {
    /// key map
    map: HashMap<String, SecretKey>,
}

impl SimpleAuth {
    /// Constructs a new `SimpleAuth`
    #[must_use]
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    #[must_use]
    pub fn from_single(access_key: impl Into<String>, secret_key: impl Into<SecretKey>) -> Self {
        let access_key = access_key.into();
        let secret_key = secret_key.into();
        let map = [(access_key, secret_key)].into_iter().collect();
        Self { map }
    }

    /// register a pair of keys
    pub fn register(&mut self, access_key: String, secret_key: SecretKey) -> Option<SecretKey> {
        self.map.insert(access_key, secret_key)
    }

    /// lookup a secret key
    #[must_use]
    pub fn lookup(&self, access_key: &str) -> Option<&SecretKey> {
        self.map.get(access_key)
    }
}

#[async_trait::async_trait]
impl S3Auth for SimpleAuth {
    async fn get_secret_key(&self, access_key: &str) -> S3Result<SecretKey> {
        match self.lookup(access_key) {
            None => Err(s3_error!(NotSignedUp, "Your account is not signed up")),
            Some(s) => Ok(s.clone()),
        }
    }
}
