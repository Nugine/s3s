use super::Request;

use crate::error::*;

use bytes::BufMut;
use futures::pin_mut;
use hyper::body::{Buf, Bytes, HttpBody};
use hyper::Body;

pub struct LengthLimit(pub u64);

pub struct FullBody(pub Bytes);

impl FullBody {
    pub async fn extract(req: &mut Request) -> S3Result<Self> {
        let body = std::mem::take(req.body_mut());
        Self::extract_with_body(req, body).await
    }

    pub async fn extract_with_body(req: &Request, body: Body) -> S3Result<Self> {
        let limit = match req.extensions().get::<LengthLimit>() {
            Some(lim) => lim.0,
            None => {
                // FIXME: unbounded memory allocation
                let bytes = hyper::body::to_bytes(body).await.map_err(S3Error::internal_error)?;
                return Ok(Self(bytes));
            }
        };

        let content_length = req
            .headers()
            .get(hyper::header::CONTENT_LENGTH)
            .and_then(|val| atoi::atoi::<u64>(val.as_bytes()));

        if let Some(content_length) = content_length {
            if content_length > 0 {
                check_len(content_length, limit)?;
            }
        }

        let bytes = concat(body, limit).await?;
        if bytes.is_empty() {
            return Ok(Self(bytes));
        }

        let content_length = content_length.ok_or(S3ErrorCode::MissingContentLength)?;
        if bytes.len() as u64 != content_length {
            return Err(s3_error!(IncompleteBody));
        }
        Ok(Self(bytes))
    }
}

async fn concat(body: Body, limit: u64) -> S3Result<Bytes> {
    check_len(body.size_hint().lower(), limit)?;

    pin_mut!(body);

    let mut first = match body.data().await {
        Some(ret) => ret.map_err(S3Error::internal_error)?,
        None => return Ok(Bytes::new()),
    };

    check_len(first.len() as u64, limit)?;

    let second = match body.data().await {
        Some(ret) => ret.map_err(S3Error::internal_error)?,
        None => return Ok(first.copy_to_bytes(first.remaining())),
    };

    let cap = first.remaining() + second.remaining() + body.size_hint().lower() as usize;
    check_len(cap as u64, limit)?;

    let mut vec = Vec::with_capacity(cap);
    vec.put(first);
    vec.put(second);

    while let Some(ret) = body.data().await {
        let buf = ret.map_err(S3Error::internal_error)?;
        check_len((vec.len() + buf.len()) as u64, limit)?;
        vec.put(buf);
    }

    Ok(vec.into())
}

fn check_len(actual: u64, limit: u64) -> S3Result<()> {
    if actual > limit {
        return Err(S3Error::with_message(
            S3ErrorCode::InvalidRequest,
            format!("body too large: actual={actual}, limit={limit}"),
        ));
    }
    Ok(())
}
