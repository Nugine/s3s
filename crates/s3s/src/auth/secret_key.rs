use std::fmt;

use zeroize::Zeroize;

#[derive(Clone, PartialEq, Eq)]
pub struct SecretKey(String);

impl Zeroize for SecretKey {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl SecretKey {
    #[inline(always)]
    fn new(s: String) -> Self {
        Self(s)
    }

    #[must_use]
    pub fn expose(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for SecretKey {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<Box<str>> for SecretKey {
    fn from(value: Box<str>) -> Self {
        Self::new(value.into())
    }
}

impl From<&str> for SecretKey {
    fn from(value: &str) -> Self {
        Self::new(value.into())
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = "[SECRET-KEY]";
        f.debug_tuple("SecretKey").field(&inner).finish()
    }
}
