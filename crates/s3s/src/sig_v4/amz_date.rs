//! x-amz-date

use std::fmt::Write as _;
use std::str::FromStr;

use arrayvec::ArrayString;

/// x-amz-date
#[derive(Debug, Clone)]
pub struct AmzDate {
    /// year
    year: u16,
    /// month
    month: u8,
    /// day
    day: u8,
    /// hour
    hour: u8,
    /// minute
    minute: u8,
    /// second
    second: u8,
}

/// [`AmzDate`]
#[derive(Debug, thiserror::Error)]
#[error("ParseAmzDateError")]
pub struct ParseAmzDateError {
    /// private place holder
    _priv: (),
}

impl AmzDate {
    /// Parses `AmzDate` from header
    /// # Errors
    /// Returns an error if the header is invalid
    pub fn parse(header: &str) -> Result<Self, ParseAmzDateError> {
        /// nom parser
        fn nom_parse(input: &str) -> nom::IResult<&str, [&str; 6]> {
            use nom::{
                bytes::complete::{tag, take},
                combinator::all_consuming,
                sequence::tuple,
            };

            let mut parser = all_consuming(tuple((
                take(4_usize),
                take(2_usize),
                take(2_usize),
                tag("T"),
                take(2_usize),
                take(2_usize),
                take(2_usize),
                tag("Z"),
            )));

            let (_, (year_str, month_str, day_str, _, hour_str, minute_str, second_str, _)) = parser(input)?;

            Ok((input, [year_str, month_str, day_str, hour_str, minute_str, second_str]))
        }

        /// parse number
        fn to_num<T: FromStr>(input: &str) -> Result<T, ParseAmzDateError> {
            match input.parse::<T>() {
                Ok(x) => Ok(x),
                Err(_) => Err(ParseAmzDateError { _priv: () }),
            }
        }

        match nom_parse(header) {
            Err(_) => Err(ParseAmzDateError { _priv: () }),
            Ok((_, [year_str, month_str, day_str, hour_str, minute_str, second_str])) => Ok(Self {
                year: to_num(year_str)?,
                month: to_num(month_str)?,
                day: to_num(day_str)?,
                hour: to_num(hour_str)?,
                minute: to_num(minute_str)?,
                second: to_num(second_str)?,
            }),
        }
    }

    /// `{YYYY}{MM}{DD}T{HH}{MM}{SS}Z`
    #[must_use]
    pub fn fmt_iso8601(&self) -> ArrayString<16> {
        let mut buf = <ArrayString<16>>::new();
        let (y, m, d, hh, mm, ss) = (self.year, self.month, self.day, self.hour, self.minute, self.second);
        write!(&mut buf, "{y:04}{m:02}{d:02}T{hh:02}{mm:02}{ss:02}Z").unwrap();
        buf
    }

    /// `{YYYY}{MM}{DD}`
    #[must_use]
    pub fn fmt_date(&self) -> ArrayString<8> {
        let mut buf = <ArrayString<8>>::new();
        write!(&mut buf, "{:04}{:02}{:02}", self.year, self.month, self.day).unwrap();
        buf
    }

    pub fn to_time(&self) -> Option<time::OffsetDateTime> {
        let y = i32::from(self.year);
        let m: time::Month = self.month.try_into().ok()?;
        let d = self.day;

        let t = time::Date::from_calendar_date(y, m, d).ok()?;
        let t = t.with_hms(self.hour, self.minute, self.second).ok()?;
        Some(t.assume_utc())
    }
}
