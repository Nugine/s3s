//! Authorization V2
//!
//! <https://docs.aws.amazon.com/AmazonS3/latest/userguide/RESTAuthentication.html#ConstructingTheAuthenticationHeader>
//!

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
        match parser::parse_authorization(input) {
            Ok(("", ans)) => Ok(ans),
            Ok(_) | Err(_) => Err(ParseAuthorizationV2Error { _priv: () }),
        }
    }
}

mod parser {
    use super::AuthorizationV2;

    use crate::utils::parser::consume;

    use nom::bytes::complete::{tag, take, take_till};
    use nom::combinator::rest;
    use nom::sequence::terminated;
    use nom::IResult;

    pub fn parse_authorization(mut input: &str) -> IResult<&str, AuthorizationV2<'_>> {
        let s = &mut input;

        consume(s, tag("AWS "))?;
        let access_key = consume(s, until_colon0)?;
        let signature = consume(s, rest)?;

        Ok((input, AuthorizationV2 { access_key, signature }))
    }

    fn until_colon0(input: &str) -> IResult<&str, &str> {
        terminated(take_till(|c| c == ':'), take(1_usize))(input)
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
