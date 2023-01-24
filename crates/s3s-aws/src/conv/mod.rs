mod generated;

use s3s::s3_error;
use s3s::{S3Error, S3Result};

use std::collections::HashMap;
use std::convert::Infallible;

pub trait AwsConversion: Sized {
    type Target;
    type Error;

    fn try_from_aws(x: Self::Target) -> Result<Self, Self::Error>;

    fn try_into_aws(x: Self) -> Result<Self::Target, Self::Error>;
}

pub fn try_from_aws<T: AwsConversion>(x: T::Target) -> Result<T, T::Error> {
    T::try_from_aws(x)
}

pub fn try_into_aws<T: AwsConversion>(x: T) -> S3Result<T::Target, T::Error> {
    T::try_into_aws(x)
}

fn unwrap_from_aws<T: AwsConversion>(opt: Option<T::Target>, field_name: &str) -> S3Result<T>
where
    S3Error: From<T::Error>,
{
    match opt {
        Some(x) => T::try_from_aws(x).map_err(Into::into),
        None => Err(s3_error!(InternalError, "missing field: {}", field_name)),
    }
}

macro_rules! identity_impl {
    ($($ty:ty),+) => {
        $(
            impl AwsConversion for $ty {
                type Target = $ty;
                type Error = Infallible;

                #[inline(always)]
                fn try_from_aws(x: Self::Target) -> Result<Self, Self::Error> {
                    Ok(x)
                }

                #[inline(always)]
                fn try_into_aws(x: Self) -> Result<Self::Target, Self::Error> {
                    Ok(x)
                }
            }
        )+
    };
}

identity_impl!(bool, i32, i64, String, HashMap<String, String>);

impl<T: AwsConversion> AwsConversion for Option<T> {
    type Target = Option<T::Target>;
    type Error = T::Error;

    fn try_from_aws(x: Self::Target) -> Result<Self, Self::Error> {
        x.map(try_from_aws).transpose()
    }

    fn try_into_aws(x: Self) -> Result<Self::Target, Self::Error> {
        x.map(try_into_aws).transpose()
    }
}

impl<T: AwsConversion> AwsConversion for Vec<T> {
    type Target = Vec<T::Target>;
    type Error = T::Error;

    fn try_from_aws(x: Self::Target) -> Result<Self, Self::Error> {
        x.into_iter().map(try_from_aws).collect()
    }

    fn try_into_aws(x: Self) -> Result<Self::Target, Self::Error> {
        x.into_iter().map(try_into_aws).collect()
    }
}

impl AwsConversion for s3s::dto::Timestamp {
    type Target = aws_sdk_s3::types::DateTime;
    type Error = S3Error;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        use aws_smithy_types_convert::date_time::DateTimeExt;
        Ok(Self::from(x.to_time().map_err(S3Error::internal_error)?))
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        use aws_smithy_types_convert::date_time::DateTimeExt;
        Ok(aws_sdk_s3::types::DateTime::from_time(x.into()))
    }
}

impl AwsConversion for s3s::dto::ContentType {
    type Target = String;
    type Error = S3Error;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        x.parse::<Self>().map_err(S3Error::internal_error)
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(x.to_string())
    }
}

impl AwsConversion for s3s::dto::CopySource {
    type Target = String;
    type Error = S3Error;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Self::parse(x.as_str()).map_err(S3Error::internal_error)
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(x.format_to_string())
    }
}

impl AwsConversion for s3s::dto::Range {
    type Target = String;
    type Error = S3Error;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Self::parse(x.as_str()).map_err(S3Error::internal_error)
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(x.format_to_string())
    }
}

impl AwsConversion for s3s::dto::Event {
    type Target = aws_sdk_s3::model::Event;
    type Error = Infallible;

    fn try_from_aws(x: Self::Target) -> Result<Self, Self::Error> {
        Ok(Self::from(x.as_str().to_owned()))
    }

    fn try_into_aws(x: Self) -> Result<Self::Target, Self::Error> {
        Ok(Self::Target::from(x))
    }
}

impl AwsConversion for s3s::dto::StreamingBlob {
    type Target = aws_sdk_s3::types::ByteStream;
    type Error = Infallible;

    fn try_from_aws(x: Self::Target) -> Result<Self, Self::Error> {
        Ok(Self::wrap(x))
    }

    fn try_into_aws(x: Self) -> Result<Self::Target, Self::Error> {
        Ok(hyper::Body::wrap_stream(x.0).into())
    }
}

impl AwsConversion for s3s::dto::Body {
    type Target = aws_sdk_s3::types::Blob;
    type Error = Infallible;

    fn try_from_aws(x: Self::Target) -> Result<Self, Self::Error> {
        Ok(x.into_inner().into())
    }

    fn try_into_aws(x: Self) -> Result<Self::Target, Self::Error> {
        Ok(Self::Target::new(x))
    }
}
