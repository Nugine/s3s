//! Authorization V2
//!
//! <https://docs.aws.amazon.com/AmazonS3/latest/userguide/RESTAuthentication.html#ConstructingTheAuthenticationHeader>
//!

use crate::utils::parser;

use nom::bytes::complete::{tag, take, take_till};
use nom::combinator::{all_consuming, rest};
use nom::sequence::terminated;

pub struct AuthorizationV2<'a> {
    pub access_key: &'a str,
    pub signature: &'a str,
}

/// [`AuthorizationV2`]
#[derive(Debug, thiserror::Error)]
#[error("ParseAuthorizationError")]
pub struct ParseAuthorizationV2Error {
    /// priv place holder
    _priv: (),
}

impl<'a> AuthorizationV2<'a> {
    pub fn parse(input: &'a str) -> Result<Self, ParseAuthorizationV2Error> {
        let error = |_| ParseAuthorizationV2Error { _priv: () };
        let colon_tail0 = terminated(take_till(|c| c == ':'), take(1_usize));

        parser::parse(input, |p| {
            p.nom(tag("AWS "))?;

            let access_key = p.nom(colon_tail0)?;
            let signature = p.nom(all_consuming(rest))?;

            Ok(Self { access_key, signature })
        })
        .map_err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "AWS AKIAIOSFODNN7EXAMPLE:qgk2+6Sv9/oM7G3qLEjTH1a1l1g=";
        let ans = AuthorizationV2::parse(input).unwrap();
        assert_eq!(ans.access_key, "AKIAIOSFODNN7EXAMPLE");
        assert_eq!(ans.signature, "qgk2+6Sv9/oM7G3qLEjTH1a1l1g=");
    }
}
