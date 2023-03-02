use crate::auth::SecretKey;
use crate::http::OrderedHeaders;
use crate::http::OrderedQs;
use crate::path::S3Path;
use crate::utils::hmac_sha1;

use std::ops::Not;

use hyper::Method;

fn base64(data: impl AsRef<[u8]>) -> String {
    base64_simd::STANDARD.encode_to_string(data)
}

pub fn calculate_signature(secret_key: &SecretKey, string_to_sign: &str) -> String {
    base64(hmac_sha1(secret_key.expose(), string_to_sign))
}

const INCLUDED_QUERY: &[&str] = &[
    "acl",
    "delete",
    "lifecycle",
    "location",
    "logging",
    "notification",
    "partNumber",
    "policy",
    "requestPayment",
    "response-cache-control",
    "response-content-disposition",
    "response-content-encoding",
    "response-content-language",
    "response-content-type",
    "response-expires",
    "uploadId",
    "uploads",
    "versionId",
    "versioning",
    "versions",
    "website",
];

pub fn get_date<'a>(headers: &'_ OrderedHeaders<'a>) -> Option<&'a str> {
    headers.get_unique("x-amz-date").or_else(|| headers.get_unique("date"))
}

pub fn create_string_to_sign(
    method: &Method,
    date_or_expires: &str,
    headers: &OrderedHeaders<'_>,
    uri_s3_path: &S3Path,
    qs: Option<&OrderedQs>,
) -> String {
    let mut ans = String::with_capacity(256);

    {
        // {HTTP-Verb}\n
        ans.push_str(method.as_str());
        ans.push('\n');
    }

    {
        // {Content-MD5}\n
        if let Some(v) = headers.get_unique("content-md5") {
            ans.push_str(v);
        }
        ans.push('\n');
    }

    {
        // {Content-Type}\n
        if let Some(v) = headers.get_unique("content-type") {
            ans.push_str(v);
        }
        ans.push('\n');
    }

    {
        // {Date}\n or {Expires}\n
        ans.push_str(date_or_expires);
        ans.push('\n');
    }

    {
        // {CanonicalizedAmzHeaders}
        let mut last = "";
        for &(name, _) in headers.as_ref() {
            if name.starts_with("x-amz-").not() {
                continue;
            }
            if name == "x-amz-date" {
                continue;
            }
            if name == last {
                continue;
            }
            last = name;

            ans.push_str(name);
            ans.push(':');

            let mut iter = headers.get_all(name);
            let first = iter.next().unwrap();
            ans.push_str(first.trim());

            for value in iter {
                ans.push(',');
                ans.push_str(value.trim());
            }

            ans.push('\n');
        }
    }

    {
        // {CanonicalizedResource}

        match uri_s3_path {
            S3Path::Root => {
                ans.push('/');
            }
            S3Path::Bucket { bucket } => {
                ans.push('/');
                ans.push_str(bucket);
                ans.push('/');
            }
            S3Path::Object { bucket, key } => {
                ans.push('/');
                ans.push_str(bucket);
                ans.push('/');
                ans.push_str(key);
            }
        }

        if let Some(qs) = qs {
            let mut is_first = true;
            for q in INCLUDED_QUERY {
                if let Some(v) = qs.get_unique(q) {
                    if is_first {
                        ans.push('?');
                        is_first = false;
                    } else {
                        ans.push('&');
                    }
                    ans.push_str(q);
                    if v.is_empty().not() {
                        ans.push('=');
                        ans.push_str(v);
                    }
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted() {
        for w in INCLUDED_QUERY.windows(2) {
            assert!(w[0] < w[1], "{w:?}")
        }
    }

    #[test]
    fn examples() {
        let access_key = "AKIAIOSFODNN7EXAMPLE";
        let secret_key = SecretKey::from("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");

        {
            // Object GET
            let method = &Method::GET;
            let s3_path = S3Path::object("awsexamplebucket1", "photos/puppy.jpg");
            let date = "Tue, 27 Mar 2007 19:36:42 +0000";
            let headers = OrderedHeaders::default();
            let qs = None;

            let string_to_sign = create_string_to_sign(method, date, &headers, &s3_path, qs);
            let signature = calculate_signature(&secret_key, &string_to_sign);

            assert_eq!(
                string_to_sign,
                concat!(
                    "GET\n",
                    "\n",
                    "\n",
                    "Tue, 27 Mar 2007 19:36:42 +0000\n",
                    "/awsexamplebucket1/photos/puppy.jpg",
                )
            );

            assert_eq!(signature, "qgk2+6Sv9/oM7G3qLEjTH1a1l1g=");
        }

        {
            // Object PUT
            let method = &Method::PUT;
            let s3_path = S3Path::object("awsexamplebucket1", "photos/puppy.jpg");
            let date = "Tue, 27 Mar 2007 21:15:45 +0000";
            let headers = OrderedHeaders::from_slice_unchecked(&[("content-type", "image/jpeg")]);
            let qs = None;

            let string_to_sign = create_string_to_sign(method, date, &headers, &s3_path, qs);
            let signature = calculate_signature(&secret_key, &string_to_sign);

            assert_eq!(
                string_to_sign,
                concat!(
                    "PUT\n",
                    "\n",
                    "image/jpeg\n",
                    "Tue, 27 Mar 2007 21:15:45 +0000\n",
                    "/awsexamplebucket1/photos/puppy.jpg",
                )
            );

            assert_eq!(signature, "iqRzw+ileNPu1fhspnRs8nOjjIA=");
        }

        {
            // List
            let method = &Method::GET;
            let s3_path = S3Path::bucket("awsexamplebucket1");
            let date = "Tue, 27 Mar 2007 19:42:41 +0000";
            let headers = OrderedHeaders::default();
            let qs = None;

            let string_to_sign = create_string_to_sign(method, date, &headers, &s3_path, qs);
            let signature = calculate_signature(&secret_key, &string_to_sign);

            assert_eq!(
                string_to_sign,
                concat!(
                    "GET\n",
                    "\n",
                    "\n",
                    "Tue, 27 Mar 2007 19:42:41 +0000\n", //
                    "/awsexamplebucket1/",
                )
            );

            assert_eq!(signature, "m0WP8eCtspQl5Ahe6L1SozdX9YA=");
        }

        {
            // Fetch
            let method = &Method::GET;
            let s3_path = S3Path::bucket("awsexamplebucket1");
            let date = "Tue, 27 Mar 2007 19:44:46 +0000";
            let qs = OrderedQs::from_vec_unchecked(vec![("acl".into(), "".into())]);
            let headers = OrderedHeaders::default();

            let string_to_sign = create_string_to_sign(method, date, &headers, &s3_path, Some(&qs));
            let signature = calculate_signature(&secret_key, &string_to_sign);

            assert_eq!(
                string_to_sign,
                concat!(
                    "GET\n",
                    "\n",
                    "\n",
                    "Tue, 27 Mar 2007 19:44:46 +0000\n", //
                    "/awsexamplebucket1/?acl",
                )
            );

            assert_eq!(signature, "82ZHiFIjc+WbcwFKGUVEQspPn+0=");
        }

        {
            // Delete
            let method = &Method::DELETE;
            let s3_path = S3Path::object("awsexamplebucket1", "photos/puppy.jpg");
            let headers = OrderedHeaders::from_slice_unchecked(&[
                ("date", "Tue, 27 Mar 2007 21:20:27 +0000"),
                ("x-amz-date", "Tue, 27 Mar 2007 21:20:26 +0000"),
            ]);
            let qs = None;
            let date = get_date(&headers).unwrap();

            let string_to_sign = create_string_to_sign(method, date, &headers, &s3_path, qs);
            let signature = calculate_signature(&secret_key, &string_to_sign);

            assert_eq!(
                string_to_sign,
                concat!(
                    "DELETE\n",
                    "\n",
                    "\n",
                    "Tue, 27 Mar 2007 21:20:26 +0000\n",
                    "/awsexamplebucket1/photos/puppy.jpg",
                )
            );

            assert_eq!(signature, "XbyTlbQdu9Xw5o8P4iMwPktxQd8=");
        }

        {
            // Upload
            let method = &Method::PUT;
            let s3_path = S3Path::object("static.example.com", "db-backup.dat.gz");
            let headers = OrderedHeaders::from_slice_unchecked(&[
                ("date", "Tue, 27 Mar 2007 21:06:08 +0000"),
                ("x-amz-acl", "public-read"),
                ("content-type", "application/x-download"),
                ("content-md5", "4gJE4saaMU4BqNR0kLY+lw=="),
                ("x-amz-meta-reviewedby", "joe@example.com"),
                ("x-amz-meta-reviewedby", "jane@example.com"),
                ("x-amz-meta-filechecksum", "0x02661779"),
                ("x-amz-meta-checksumalgorithm", "crc32"),
                ("content-disposition", "attachment; filename=database.dat"),
                ("content-encoding", "gzip"),
                ("content-length", "5913339"),
            ]);
            let qs = None;
            let date = get_date(&headers).unwrap();

            let string_to_sign = create_string_to_sign(method, date, &headers, &s3_path, qs);
            let signature = calculate_signature(&secret_key, &string_to_sign);

            assert_eq!(
                string_to_sign,
                concat!(
                    "PUT\n",
                    "4gJE4saaMU4BqNR0kLY+lw==\n",
                    "application/x-download\n",
                    "Tue, 27 Mar 2007 21:06:08 +0000\n",
                    "x-amz-acl:public-read\n",
                    "x-amz-meta-checksumalgorithm:crc32\n",
                    "x-amz-meta-filechecksum:0x02661779\n",
                    "x-amz-meta-reviewedby:joe@example.com,jane@example.com\n",
                    "/static.example.com/db-backup.dat.gz",
                )
            );

            // assert_eq!(signature, "dKZcB+bz2EPXgSdXZp9ozGeOM4I="); // The example is wrong?
            assert_eq!(signature, "jtBQa0Aq+DkULFI8qrpwIjGEx0E=");
        }

        {
            // List all my buckets
            let method = &Method::GET;
            let s3_path = S3Path::Root;
            let headers = OrderedHeaders::from_slice_unchecked(&[("date", "Wed, 28 Mar 2007 01:29:59 +0000")]);
            let qs = None;
            let date = get_date(&headers).unwrap();

            let string_to_sign = create_string_to_sign(method, date, &headers, &s3_path, qs);
            let signature = calculate_signature(&secret_key, &string_to_sign);

            assert_eq!(
                string_to_sign,
                concat!(
                    "GET\n",
                    "\n",
                    "\n",
                    "Wed, 28 Mar 2007 01:29:59 +0000\n", //
                    "/",
                )
            );

            assert_eq!(signature, "qGdzdERIC03wnaRNKh6OqZehG9s=");
        }

        {
            // Unicode keys
            let method = &Method::GET;
            let uri_s3_path = crate::path::parse_path_style("/dictionary/fran%C3%A7ais/pr%c3%a9f%c3%a8re").unwrap();
            let headers = OrderedHeaders::from_slice_unchecked(&[("date", "Wed, 28 Mar 2007 01:49:49 +0000")]);
            let qs = None;
            let date = get_date(&headers).unwrap();

            let string_to_sign = create_string_to_sign(method, date, &headers, &uri_s3_path, qs);
            let signature = calculate_signature(&secret_key, &string_to_sign);

            assert_eq!(
                string_to_sign,
                concat!(
                    "GET\n",
                    "\n",
                    "\n",
                    "Wed, 28 Mar 2007 01:49:49 +0000\n",
                    "/dictionary/fran%C3%A7ais/pr%c3%a9f%c3%a8re",
                )
            );

            assert_eq!(signature, "DNEZGsoieTZ92F3bUfSPQcbGmlM=");
        }

        {
            // Query string request authentication
            let method = &Method::GET;
            let s3_path = S3Path::object("awsexamplebucket1", "photos/puppy.jpg");
            let headers = OrderedHeaders::default();
            let qs = OrderedQs::parse(concat!(
                "AWSAccessKeyId=AKIAIOSFODNN7EXAMPLE",
                // "&Signature=NpgCjnDzrM%2BWFzoENXmpNDUsSn8%3D", // The example is wrong?
                "&Signature=1No4mq5ETf02z8aet9voy6gui6E%3D",
                "&Expires=1175139620",
            ))
            .unwrap();
            let presigned_url = super::super::PresignedUrlV2::parse(&qs).unwrap();
            assert_eq!(presigned_url.access_key, access_key);

            let string_to_sign = create_string_to_sign(method, presigned_url.expires_str, &headers, &s3_path, Some(&qs));
            let signature = calculate_signature(&secret_key, &string_to_sign);

            assert_eq!(
                string_to_sign,
                concat!(
                    "GET\n",
                    "\n",
                    "\n",
                    "1175139620\n", //
                    "/awsexamplebucket1/photos/puppy.jpg",
                )
            );

            assert_eq!(signature, presigned_url.signature);
        }
    }
}
