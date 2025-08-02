use std::fmt;

use serde::Deserialize;
use serde::Serialize;
use subtle::ConstantTimeEq;
use zeroize::Zeroize;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub access_key: String,
    pub secret_key: SecretKey,
}

#[derive(Clone)]
pub struct SecretKey(Box<str>);

impl SecretKey {
    fn new(s: impl Into<Box<str>>) -> Self {
        Self(s.into())
    }

    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl Zeroize for SecretKey {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl ConstantTimeEq for SecretKey {
    fn ct_eq(&self, other: &Self) -> subtle::Choice {
        self.0.as_bytes().ct_eq(other.0.as_bytes())
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl From<String> for SecretKey {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<Box<str>> for SecretKey {
    fn from(value: Box<str>) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SecretKey {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

const PLACEHOLDER: &str = "[SENSITIVE-SECRET-KEY]";

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SecretKey").field(&PLACEHOLDER).finish()
    }
}

impl<'de> Deserialize<'de> for SecretKey {
    fn deserialize<D>(deserializer: D) -> Result<SecretKey, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <String as Deserialize>::deserialize(deserializer).map(SecretKey::from)
    }
}

impl Serialize for SecretKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        <str as Serialize>::serialize(PLACEHOLDER, serializer)
    }
}
