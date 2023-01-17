//! Ordered headers

use hyper::header::ToStrError;
use hyper::http::HeaderValue;
use hyper::HeaderMap;
use smallvec::SmallVec;

/// Immutable http header container
#[derive(Debug)]
pub struct OrderedHeaders<'a> {
    /// Ascending headers (header names are lowercase)
    headers: SmallVec<[(&'a str, &'a str); 16]>,
}

impl<'a> OrderedHeaders<'a> {
    /// Constructs [`OrderedHeaders`] from slice
    ///
    /// + header names must be lowercase
    /// + header values must be valid
    #[cfg(test)]
    #[must_use]
    pub fn from_slice_unchecked(slice: &[(&'a str, &'a str)]) -> Self {
        let mut headers = SmallVec::new();
        headers.extend_from_slice(slice);
        headers.sort_unstable();
        Self { headers }
    }

    /// Constructs [`OrderedHeaders`] from a header map
    ///
    /// # Errors
    /// Returns [`ToStrError`] if header value cannot be converted to string slice
    pub fn from_headers(map: &'a HeaderMap<HeaderValue>) -> Result<Self, ToStrError> {
        let mut headers: SmallVec<[(&'a str, &'a str); 16]> = SmallVec::with_capacity(map.len());

        for (name, value) in map.iter() {
            headers.push((name.as_str(), value.to_str()?));
        }
        headers.sort_unstable();

        Ok(Self { headers })
    }

    fn get_pair(&self, name: &'_ str) -> Option<(&'a str, &'a str)> {
        let headers = self.headers.as_slice();
        let idx = headers.binary_search_by_key(&name, |&(n, _)| n).ok()?;
        headers.get(idx).copied()
    }

    /// Gets header value by name. Time `O(logn)`
    pub fn get(&self, name: impl AsRef<str>) -> Option<&'a str> {
        self.get_pair(name.as_ref()).map(|(_, v)| v)
    }

    /// Finds headers by names. Time `O(mlogn)`
    #[must_use]
    pub fn find_multiple(&self, names: &[impl AsRef<str>]) -> Self {
        let mut headers: SmallVec<[(&'a str, &'a str); 16]> = SmallVec::new();
        for name in names {
            if let Some(pair) = self.get_pair(name.as_ref()) {
                headers.push(pair);
            }
        }
        Self { headers }
    }
}

impl<'a> AsRef<[(&'a str, &'a str)]> for OrderedHeaders<'a> {
    fn as_ref(&self) -> &[(&'a str, &'a str)] {
        self.headers.as_ref()
    }
}
