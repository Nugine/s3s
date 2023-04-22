use std::fmt;

use zeroize::Zeroize;

#[derive(Clone, PartialEq, Eq)]
pub struct SecretKey(Box<str>);

impl SecretKey {
    #[inline(always)]
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

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = "[SECRET-KEY]";
        f.debug_tuple("SecretKey").field(&inner).finish()
    }
}
