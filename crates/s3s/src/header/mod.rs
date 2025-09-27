#[cfg(feature = "minio")]
stdx::cfg_group! {
    mod generated_minio;
    use self::generated_minio as generated;
}

#[cfg(not(feature = "minio"))]
mod generated;

pub use self::generated::*;
