//! HTTP Range header

use crate::http;
use crate::utils::from_ascii;

/// HTTP Range header
///
/// See <https://www.rfc-editor.org/rfc/rfc9110.html#section-14.1.2>
#[allow(clippy::exhaustive_enums, missing_copy_implementations)]
#[derive(Debug, Clone)]
pub enum Range {
    /// Normal byte range
    Normal {
        /// first
        first: u64,
        /// last
        last: Option<u64>,
    },
    /// Suffix byte range
    Suffix {
        /// last
        last: u64,
    },
}

/// [`Range`]
#[derive(Debug, thiserror::Error)]
#[error("ParseRangeError")]
pub struct ParseRangeError {
    /// private place holder
    _priv: (),
}

impl Range {
    /// Parses `Range` from header
    /// # Errors
    /// Returns an error if the header is invalid
    pub fn parse(header: &str) -> Result<Self, ParseRangeError> {
        /// nom parser
        fn nom_parse(input: &str) -> nom::IResult<&str, Range> {
            use nom::{
                branch::alt,
                bytes::complete::tag,
                character::complete::digit1,
                combinator::{all_consuming, map, map_res, opt},
                sequence::tuple,
            };

            let normal_parser = map_res(
                tuple((map_res(digit1, str::parse::<u64>), tag("-"), opt(map_res(digit1, str::parse::<u64>)))),
                |ss: (u64, &str, Option<u64>)| {
                    if let (first, Some(last)) = (ss.0, ss.2) {
                        if first > last {
                            return Err(ParseRangeError { _priv: () });
                        }
                    }
                    Ok(Range::Normal { first: ss.0, last: ss.2 })
                },
            );

            let suffix_parser = map(tuple((tag("-"), map_res(digit1, str::parse::<u64>))), |ss: (&str, u64)| Range::Suffix {
                last: ss.1,
            });

            let mut parser = all_consuming(tuple((tag("bytes="), alt((normal_parser, suffix_parser)))));

            let (input, (_, ans)) = parser(input)?;

            Ok((input, ans))
        }

        match nom_parse(header) {
            Err(_) => Err(ParseRangeError { _priv: () }),
            Ok((_, ans)) => Ok(ans),
        }
    }

    #[must_use]
    pub fn format_to_string(&self) -> String {
        match self {
            Range::Normal { first, last } => match last {
                Some(last) => format!("bytes={first}-{last}"),
                None => format!("bytes={first}-"),
            },
            Range::Suffix { last } => format!("bytes=-{last}"),
        }
    }
}

impl http::TryFromHeaderValue for Range {
    type Error = ParseRangeError;

    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let header = from_ascii(val.as_bytes()).ok_or(ParseRangeError { _priv: () })?;
        Self::parse(header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_range() {
        {
            let src = "bytes=0-499";
            let result = Range::parse(src);
            assert!(matches!(
                result.unwrap(),
                Range::Normal {
                    first: 0,
                    last: Some(499)
                }
            ));
        }
        {
            let src = "bytes=0-499;";
            let result = Range::parse(src);
            let _ = result.unwrap_err();
        }
        {
            let src = "bytes=9500-";
            let result = Range::parse(src);
            assert!(matches!(result.unwrap(), Range::Normal { first: 9500, last: None }));
        }
        {
            let src = "bytes=9500-0-";
            let result = Range::parse(src);
            let _ = result.unwrap_err();
        }
        {
            let src = "bytes=-500";
            let result = Range::parse(src);
            assert!(matches!(result.unwrap(), Range::Suffix { last: 500 }));
        }
        {
            let src = "bytes=-500 ";
            let result = Range::parse(src);
            let _ = result.unwrap_err();
        }
        {
            let src = "bytes=-1000000000000000000000000";
            let result = Range::parse(src);
            let _ = result.unwrap_err();
        }
    }
}
