use crate::fs::FileSystem;
use crate::utils::*;

use s3s::dto::*;
use s3s::s3_error;
use s3s::S3Request;
use s3s::S3Result;
use s3s::S3;

use std::collections::VecDeque;
use std::io;
use std::ops::Neg;
use std::ops::Not;
use std::path::Path;
use std::path::PathBuf;

use tokio::fs;
use tokio::io::AsyncSeekExt;
use tokio::io::BufWriter;
use tokio_util::io::ReaderStream;

use futures::TryStreamExt;
use md5::{Digest, Md5};
use numeric_cast::NumericCast;
use path_absolutize::Absolutize;
use tracing::debug;

#[async_trait::async_trait]
impl S3 for FileSystem {
    #[tracing::instrument]
    async fn create_bucket(&self, req: S3Request<CreateBucketInput>) -> S3Result<CreateBucketOutput> {
        let input = req.input;
        let path = self.get_bucket_path(&input.bucket)?;

        if path.exists() {
            return Err(s3_error!(BucketAlreadyExists));
        }

        try_!(fs::create_dir(&path).await);

        let output = CreateBucketOutput::default(); // TODO: handle other fields
        Ok(output)
    }

    #[tracing::instrument]
    async fn copy_object(&self, req: S3Request<CopyObjectInput>) -> S3Result<CopyObjectOutput> {
        let input = req.input;
        let (bucket, key) = match input.copy_source {
            CopySource::AccessPoint { .. } => return Err(s3_error!(NotImplemented)),
            CopySource::Bucket { ref bucket, ref key, .. } => (bucket, key),
        };

        let src_path = self.get_object_path(bucket, key)?;
        let dst_path = self.get_object_path(&input.bucket, &input.key)?;

        if src_path.exists().not() {
            return Err(s3_error!(NoSuchKey));
        }

        let file_metadata = try_!(fs::metadata(&src_path).await);
        let last_modified = Timestamp::from(try_!(file_metadata.modified()));

        let _ = try_!(fs::copy(&src_path, &dst_path).await);

        debug!(from = %src_path.display(), to = %dst_path.display(), "copy file");

        let src_metadata_path = self.get_metadata_path(bucket, key)?;
        if src_metadata_path.exists() {
            let dst_metadata_path = self.get_metadata_path(&input.bucket, &input.key)?;
            let _ = try_!(fs::copy(src_metadata_path, dst_metadata_path).await);
        }

        let md5_sum = self.get_md5_sum(bucket, key).await?;

        let copy_object_result = CopyObjectResult {
            e_tag: Some(format!("\"{md5_sum}\"")),
            last_modified: Some(last_modified),
            ..Default::default()
        };

        let output = CopyObjectOutput {
            copy_object_result: Some(copy_object_result),
            ..Default::default()
        };
        Ok(output)
    }

    #[tracing::instrument]
    async fn delete_bucket(&self, req: S3Request<DeleteBucketInput>) -> S3Result<DeleteBucketOutput> {
        let input = req.input;
        let path = self.get_bucket_path(&input.bucket)?;
        try_!(fs::remove_dir_all(path).await);
        Ok(DeleteBucketOutput {})
    }

    #[tracing::instrument]
    async fn delete_object(&self, req: S3Request<DeleteObjectInput>) -> S3Result<DeleteObjectOutput> {
        let input = req.input;
        let path = self.get_object_path(&input.bucket, &input.key)?;
        if input.key.ends_with('/') {
            let mut dir = try_!(fs::read_dir(&path).await);
            let is_empty = try_!(dir.next_entry().await).is_none();
            if is_empty {
                try_!(fs::remove_dir(&path).await);
            }
        } else {
            try_!(fs::remove_file(path).await);
        }
        let output = DeleteObjectOutput::default(); // TODO: handle other fields
        Ok(output)
    }

    #[tracing::instrument]
    async fn delete_objects(&self, req: S3Request<DeleteObjectsInput>) -> S3Result<DeleteObjectsOutput> {
        let input = req.input;
        let mut objects: Vec<(PathBuf, String)> = Vec::new();
        for object in input.delete.objects {
            let path = self.get_object_path(&input.bucket, &object.key)?;
            if path.exists() {
                objects.push((path, object.key));
            }
        }

        let mut deleted_objects: Vec<DeletedObject> = Vec::new();
        for (path, key) in objects {
            try_!(fs::remove_file(path).await);

            let deleted_object = DeletedObject {
                key: Some(key),
                ..Default::default()
            };

            deleted_objects.push(deleted_object);
        }

        let output = DeleteObjectsOutput {
            deleted: Some(deleted_objects),
            ..Default::default()
        };
        Ok(output)
    }

    #[tracing::instrument]
    async fn get_bucket_location(&self, req: S3Request<GetBucketLocationInput>) -> S3Result<GetBucketLocationOutput> {
        let input = req.input;
        let path = self.get_bucket_path(&input.bucket)?;

        if !path.exists() {
            return Err(s3_error!(NoSuchBucket));
        }

        let output = GetBucketLocationOutput::default();
        Ok(output)
    }

    #[tracing::instrument]
    async fn get_object(&self, req: S3Request<GetObjectInput>) -> S3Result<GetObjectOutput> {
        let input = req.input;
        let object_path = self.get_object_path(&input.bucket, &input.key)?;

        let mut file = fs::File::open(&object_path).await.map_err(|e| s3_error!(e, NoSuchKey))?;

        let file_metadata = try_!(file.metadata().await);
        let last_modified = Timestamp::from(try_!(file_metadata.modified()));
        let file_len = file_metadata.len();

        let content_length = match input.range {
            None => file_len,
            Some(range) => {
                let file_range = range.check(file_len)?;
                file_range.end - file_range.start
            }
        };
        let content_length_usize = try_!(usize::try_from(content_length));
        let content_length_i64 = try_!(i64::try_from(content_length));

        match input.range {
            Some(Range::Int { first, .. }) => {
                try_!(file.seek(io::SeekFrom::Start(first)).await);
            }
            Some(Range::Suffix { length }) => {
                let neg_offset = length.numeric_cast::<i64>().neg();
                try_!(file.seek(io::SeekFrom::End(neg_offset)).await);
            }
            None => {}
        }

        let body = bytes_stream(ReaderStream::with_capacity(file, 4096), content_length_usize);

        let object_metadata = self.load_metadata(&input.bucket, &input.key).await?;

        let md5_sum = self.get_md5_sum(&input.bucket, &input.key).await?;

        let output = GetObjectOutput {
            body: Some(StreamingBlob::wrap(body)),
            content_length: content_length_i64,
            last_modified: Some(last_modified),
            metadata: object_metadata,
            e_tag: Some(format!("\"{md5_sum}\"")),
            ..Default::default()
        };
        Ok(output)
    }

    #[tracing::instrument]
    async fn head_bucket(&self, req: S3Request<HeadBucketInput>) -> S3Result<HeadBucketOutput> {
        let input = req.input;
        let path = self.get_bucket_path(&input.bucket)?;

        if !path.exists() {
            return Err(s3_error!(NoSuchBucket));
        }

        Ok(HeadBucketOutput {})
    }

    #[tracing::instrument]
    async fn head_object(&self, req: S3Request<HeadObjectInput>) -> S3Result<HeadObjectOutput> {
        let input = req.input;
        let path = self.get_object_path(&input.bucket, &input.key)?;

        if !path.exists() {
            return Err(s3_error!(NoSuchBucket));
        }

        let file_metadata = try_!(fs::metadata(path).await);
        let last_modified = Timestamp::from(try_!(file_metadata.modified()));
        let file_len = file_metadata.len();

        let object_metadata = self.load_metadata(&input.bucket, &input.key).await?;

        // TODO: detect content type
        let content_type = mime::APPLICATION_OCTET_STREAM;

        let output = HeadObjectOutput {
            content_length: try_!(i64::try_from(file_len)),
            content_type: Some(content_type),
            last_modified: Some(last_modified),
            metadata: object_metadata,
            ..Default::default()
        };
        Ok(output)
    }

    #[tracing::instrument]
    async fn list_buckets(&self, _: S3Request<ListBucketsInput>) -> S3Result<ListBucketsOutput> {
        let mut buckets: Vec<Bucket> = Vec::new();
        let mut iter = try_!(fs::read_dir(&self.root).await);
        while let Some(entry) = try_!(iter.next_entry().await) {
            let file_type = try_!(entry.file_type().await);
            if file_type.is_dir().not() {
                continue;
            }

            let file_name = entry.file_name();
            let Some(name) = file_name.to_str() else { continue };
            if s3s::path::check_bucket_name(name).not() {
                continue;
            }

            let file_meta = try_!(entry.metadata().await);
            // Not all filesystems/mounts provide all file attributes like created timestamp,
            // therefore we try to fallback to modified if possible.
            // See https://github.com/Nugine/s3s/pull/22 for more details.
            let created_or_modified_date = Timestamp::from(try_!(file_meta.created().or(file_meta.modified())));

            let bucket = Bucket {
                creation_date: Some(created_or_modified_date),
                name: Some(name.to_owned()),
            };
            buckets.push(bucket);
        }

        let output = ListBucketsOutput {
            buckets: Some(buckets),
            owner: None,
        };
        Ok(output)
    }

    #[tracing::instrument]
    async fn list_objects(&self, req: S3Request<ListObjectsInput>) -> S3Result<ListObjectsOutput> {
        let v2 = self.list_objects_v2(req.map_input(Into::into)).await?;

        let output = ListObjectsOutput {
            contents: v2.contents,
            delimiter: v2.delimiter,
            encoding_type: v2.encoding_type,
            name: v2.name,
            prefix: v2.prefix,
            max_keys: v2.max_keys,
            ..Default::default()
        };
        Ok(output)
    }

    #[tracing::instrument]
    async fn list_objects_v2(&self, req: S3Request<ListObjectsV2Input>) -> S3Result<ListObjectsV2Output> {
        let input = req.input;
        let path = self.get_bucket_path(&input.bucket)?;

        if path.exists().not() {
            return Err(s3_error!(NoSuchBucket));
        }

        let mut objects: Vec<Object> = Default::default();
        let mut dir_queue: VecDeque<PathBuf> = Default::default();
        dir_queue.push_back(path.clone());

        while let Some(dir) = dir_queue.pop_front() {
            let mut iter = try_!(fs::read_dir(dir).await);
            while let Some(entry) = try_!(iter.next_entry().await) {
                let file_type = try_!(entry.file_type().await);
                if file_type.is_dir() {
                    dir_queue.push_back(entry.path());
                } else {
                    let file_path = entry.path();
                    let key = try_!(file_path.strip_prefix(&path));
                    let Some(key) = key.to_str() else { continue };

                    if let Some(ref prefix) = input.prefix {
                        if !key.starts_with(prefix) {
                            continue;
                        }
                    }

                    let metadata = try_!(entry.metadata().await);
                    let last_modified = Timestamp::from(try_!(metadata.modified()));
                    let size = metadata.len();

                    let object = Object {
                        key: Some(key.to_owned()),
                        last_modified: Some(last_modified),
                        size: try_!(i64::try_from(size)),
                        ..Default::default()
                    };
                    objects.push(object);
                }
            }
        }

        objects.sort_by(|lhs, rhs| {
            let lhs_key = lhs.key.as_deref().unwrap_or("");
            let rhs_key = rhs.key.as_deref().unwrap_or("");
            lhs_key.cmp(rhs_key)
        });

        let key_count = try_!(i32::try_from(objects.len()));

        let output = ListObjectsV2Output {
            key_count,
            max_keys: key_count,
            contents: Some(objects),
            delimiter: input.delimiter,
            encoding_type: input.encoding_type,
            name: Some(input.bucket),
            prefix: input.prefix,
            ..Default::default()
        };
        Ok(output)
    }

    #[tracing::instrument]
    async fn put_object(&self, req: S3Request<PutObjectInput>) -> S3Result<PutObjectOutput> {
        let input = req.input;
        if let Some(ref storage_class) = input.storage_class {
            let is_valid = ["STANDARD", "REDUCED_REDUNDANCY"].contains(&storage_class.as_str());
            if !is_valid {
                return Err(s3_error!(InvalidStorageClass));
            }
        }

        let PutObjectInput {
            body,
            bucket,
            key,
            metadata,
            content_length,
            ..
        } = input;

        let Some(body) = body else { return Err(s3_error!(IncompleteBody)) };

        if key.ends_with('/') {
            if let Some(len) = content_length {
                if len > 0 {
                    return Err(s3_error!(UnexpectedContent, "Unexpected request body when creating a directory object."));
                }
            }
            let object_path = self.get_object_path(&bucket, &key)?;
            try_!(fs::create_dir_all(&object_path).await);
            let output = PutObjectOutput::default();
            return Ok(output);
        }

        let object_path = self.get_object_path(&bucket, &key)?;
        if let Some(dir_path) = object_path.parent() {
            try_!(fs::create_dir_all(&dir_path).await);
        }

        let mut md5_hash = Md5::new();
        let stream = body.inspect_ok(|bytes| md5_hash.update(bytes.as_ref()));

        let file = try_!(fs::File::create(&object_path).await);
        let mut writer = BufWriter::new(file);

        let size = copy_bytes(stream, &mut writer).await?;
        let md5_sum = hex(md5_hash.finalize());

        debug!(path = %object_path.display(), ?size, %md5_sum, "write file");

        if let Some(ref metadata) = metadata {
            self.save_metadata(&bucket, &key, metadata).await?;
        }

        let output = PutObjectOutput {
            e_tag: Some(format!("\"{md5_sum}\"")),
            ..Default::default()
        };
        Ok(output)
    }

    #[tracing::instrument]
    async fn create_multipart_upload(&self, req: S3Request<CreateMultipartUploadInput>) -> S3Result<CreateMultipartUploadOutput> {
        let input = req.input;
        let upload_id = uuid::Uuid::new_v4().to_string();

        let output = CreateMultipartUploadOutput {
            bucket: Some(input.bucket),
            key: Some(input.key),
            upload_id: Some(upload_id),
            ..Default::default()
        };

        Ok(output)
    }

    #[tracing::instrument]
    async fn upload_part(&self, req: S3Request<UploadPartInput>) -> S3Result<UploadPartOutput> {
        let UploadPartInput {
            body,
            upload_id,
            part_number,
            ..
        } = req.input;

        let body = body.ok_or_else(|| s3_error!(IncompleteBody))?;

        if uuid::Uuid::parse_str(&upload_id).is_err() {
            return Err(s3_error!(InvalidRequest));
        }

        let file_path_str = format!(".upload_id-{upload_id}.part-{part_number}");
        let file_path = try_!(Path::new(&file_path_str).absolutize_virtually(&self.root));

        let mut md5_hash = Md5::new();
        let stream = body.inspect_ok(|bytes| md5_hash.update(bytes.as_ref()));

        let file = try_!(fs::File::create(&file_path).await);
        let mut writer = BufWriter::new(file);

        let size = copy_bytes(stream, &mut writer).await?;
        let md5_sum = hex(md5_hash.finalize());

        debug!(path = %file_path.display(), ?size, %md5_sum, "write file");

        let output = UploadPartOutput {
            e_tag: Some(format!("\"{md5_sum}\"")),
            ..Default::default()
        };
        Ok(output)
    }

    #[tracing::instrument]
    async fn complete_multipart_upload(
        &self,
        req: S3Request<CompleteMultipartUploadInput>,
    ) -> S3Result<CompleteMultipartUploadOutput> {
        let CompleteMultipartUploadInput {
            multipart_upload,
            bucket,
            key,
            upload_id,
            ..
        } = req.input;

        let Some(multipart_upload) = multipart_upload else { return Err(s3_error!(InvalidPart)) };

        let object_path = self.get_object_path(&bucket, &key)?;
        let file = try_!(fs::File::create(&object_path).await);
        let mut writer = BufWriter::new(file);

        let mut cnt: i32 = 0;
        for part in multipart_upload.parts.into_iter().flatten() {
            let part_number = part.part_number;
            cnt += 1;
            if part_number != cnt {
                return Err(s3_error!(InvalidRequest, "invalid part order"));
            }

            if uuid::Uuid::parse_str(&upload_id).is_err() {
                return Err(s3_error!(InvalidRequest));
            }

            let part_path_str = format!(".upload_id-{upload_id}.part-{part_number}");
            let part_path = try_!(Path::new(&part_path_str).absolutize_virtually(&self.root));

            let mut reader = try_!(fs::File::open(&part_path).await);
            let size = try_!(tokio::io::copy(&mut reader, &mut writer).await);

            debug!(from = %part_path.display(), to = %object_path.display(), ?size, "write file");
            try_!(fs::remove_file(&part_path).await);
        }
        drop(writer);

        let file_size = try_!(fs::metadata(&object_path).await).len();
        let md5_sum = self.get_md5_sum(&bucket, &key).await?;

        debug!(?md5_sum, path = %object_path.display(), size = ?file_size, "file md5 sum");

        let output = CompleteMultipartUploadOutput {
            bucket: Some(bucket),
            key: Some(key),
            e_tag: Some(format!("\"{md5_sum}\"")),
            ..Default::default()
        };
        Ok(output)
    }
}
