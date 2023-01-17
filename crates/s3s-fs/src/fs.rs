use crate::error::*;
use crate::utils::hex;

use s3s::dto;

use std::env;
use std::ops::Not;
use std::path::{Path, PathBuf};

use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use md5::{Digest, Md5};
use path_absolutize::Absolutize;

#[derive(Debug)]
pub struct FileSystem {
    pub(crate) root: PathBuf,
}

impl FileSystem {
    pub fn new(root: impl AsRef<Path>) -> Result<Self> {
        let root = env::current_dir()?.join(root).canonicalize()?;
        Ok(Self { root })
    }

    /// resolve object path under the virtual root
    pub(crate) fn get_object_path(&self, bucket: &str, key: &str) -> Result<PathBuf> {
        let dir = Path::new(&bucket);
        let file_path = Path::new(&key);
        let ans = dir.join(file_path).absolutize_virtually(&self.root)?.into();
        Ok(ans)
    }

    /// resolve bucket path under the virtual root
    pub(crate) fn get_bucket_path(&self, bucket: &str) -> Result<PathBuf> {
        let dir = Path::new(&bucket);
        let ans = dir.absolutize_virtually(&self.root)?.into();
        Ok(ans)
    }

    /// resolve metadata path under the virtual root (custom format)
    pub(crate) fn get_metadata_path(&self, bucket: &str, key: &str) -> Result<PathBuf> {
        let encode = |s: &str| base64_simd::URL_SAFE_NO_PAD.encode_to_string(s);
        let file_path = format!(".bucket-{}.object-{}.metadata.json", encode(bucket), encode(key));
        let ans = Path::new(&file_path).absolutize_virtually(&self.root)?.into();
        Ok(ans)
    }

    /// load metadata from fs
    pub(crate) async fn load_metadata(&self, bucket: &str, key: &str) -> Result<Option<dto::Metadata>> {
        let path = self.get_metadata_path(bucket, key)?;
        if path.exists().not() {
            return Ok(None);
        }
        let content = fs::read(&path).await?;
        let map = serde_json::from_slice(&content)?;
        Ok(Some(map))
    }

    /// save metadata to fs
    pub(crate) async fn save_metadata(&self, bucket: &str, key: &str, metadata: &dto::Metadata) -> Result<()> {
        let path = self.get_metadata_path(bucket, key)?;
        let content = serde_json::to_vec(metadata)?;
        fs::write(&path, &content).await?;
        Ok(())
    }

    /// get md5 sum
    pub(crate) async fn get_md5_sum(&self, bucket: &str, key: &str) -> Result<String> {
        let object_path = self.get_object_path(bucket, key)?;
        let mut file = File::open(&object_path).await?;
        let mut buf = vec![0; 65536];
        let mut md5_hash = Md5::new();
        loop {
            let nread = file.read(&mut buf).await?;
            if nread == 0 {
                break;
            }
            md5_hash.update(&buf[..nread]);
        }
        Ok(hex(md5_hash.finalize()))
    }
}
