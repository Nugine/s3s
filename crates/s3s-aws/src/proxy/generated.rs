use super::*;

use crate::conv::{try_from_aws, try_into_aws};

use s3s::S3Result;
use s3s::S3;

use tracing::debug;

#[async_trait::async_trait]
impl S3 for Proxy {
    #[tracing::instrument(skip(self, input))]
    async fn abort_multipart_upload(
        &self,
        input: s3s::dto::AbortMultipartUploadInput,
    ) -> S3Result<s3s::dto::AbortMultipartUploadOutput> {
        debug!(?input);
        let mut b = self.0.abort_multipart_upload();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_upload_id(Some(try_into_aws(input.upload_id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn complete_multipart_upload(
        &self,
        input: s3s::dto::CompleteMultipartUploadInput,
    ) -> S3Result<s3s::dto::CompleteMultipartUploadOutput> {
        debug!(?input);
        let mut b = self.0.complete_multipart_upload();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_crc32(try_into_aws(input.checksum_crc32)?);
        b = b.set_checksum_crc32_c(try_into_aws(input.checksum_crc32c)?);
        b = b.set_checksum_sha1(try_into_aws(input.checksum_sha1)?);
        b = b.set_checksum_sha256(try_into_aws(input.checksum_sha256)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_multipart_upload(try_into_aws(input.multipart_upload)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_sse_customer_algorithm(try_into_aws(input.sse_customer_algorithm)?);
        b = b.set_sse_customer_key(try_into_aws(input.sse_customer_key)?);
        b = b.set_sse_customer_key_md5(try_into_aws(input.sse_customer_key_md5)?);
        b = b.set_upload_id(Some(try_into_aws(input.upload_id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn copy_object(&self, input: s3s::dto::CopyObjectInput) -> S3Result<s3s::dto::CopyObjectOutput> {
        debug!(?input);
        let mut b = self.0.copy_object();
        b = b.set_acl(try_into_aws(input.acl)?);
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_bucket_key_enabled(Some(try_into_aws(input.bucket_key_enabled)?));
        b = b.set_cache_control(try_into_aws(input.cache_control)?);
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_disposition(try_into_aws(input.content_disposition)?);
        b = b.set_content_encoding(try_into_aws(input.content_encoding)?);
        b = b.set_content_language(try_into_aws(input.content_language)?);
        b = b.set_content_type(try_into_aws(input.content_type)?);
        b = b.set_copy_source(Some(try_into_aws(input.copy_source)?));
        b = b.set_copy_source_if_match(try_into_aws(input.copy_source_if_match)?);
        b = b.set_copy_source_if_modified_since(try_into_aws(input.copy_source_if_modified_since)?);
        b = b.set_copy_source_if_none_match(try_into_aws(input.copy_source_if_none_match)?);
        b = b.set_copy_source_if_unmodified_since(try_into_aws(input.copy_source_if_unmodified_since)?);
        b = b.set_copy_source_sse_customer_algorithm(try_into_aws(input.copy_source_sse_customer_algorithm)?);
        b = b.set_copy_source_sse_customer_key(try_into_aws(input.copy_source_sse_customer_key)?);
        b = b.set_copy_source_sse_customer_key_md5(try_into_aws(input.copy_source_sse_customer_key_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_expected_source_bucket_owner(try_into_aws(input.expected_source_bucket_owner)?);
        b = b.set_expires(try_into_aws(input.expires)?);
        b = b.set_grant_full_control(try_into_aws(input.grant_full_control)?);
        b = b.set_grant_read(try_into_aws(input.grant_read)?);
        b = b.set_grant_read_acp(try_into_aws(input.grant_read_acp)?);
        b = b.set_grant_write_acp(try_into_aws(input.grant_write_acp)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_metadata(try_into_aws(input.metadata)?);
        b = b.set_metadata_directive(try_into_aws(input.metadata_directive)?);
        b = b.set_object_lock_legal_hold_status(try_into_aws(input.object_lock_legal_hold_status)?);
        b = b.set_object_lock_mode(try_into_aws(input.object_lock_mode)?);
        b = b.set_object_lock_retain_until_date(try_into_aws(input.object_lock_retain_until_date)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_sse_customer_algorithm(try_into_aws(input.sse_customer_algorithm)?);
        b = b.set_sse_customer_key(try_into_aws(input.sse_customer_key)?);
        b = b.set_sse_customer_key_md5(try_into_aws(input.sse_customer_key_md5)?);
        b = b.set_ssekms_encryption_context(try_into_aws(input.ssekms_encryption_context)?);
        b = b.set_ssekms_key_id(try_into_aws(input.ssekms_key_id)?);
        b = b.set_server_side_encryption(try_into_aws(input.server_side_encryption)?);
        b = b.set_storage_class(try_into_aws(input.storage_class)?);
        b = b.set_tagging(try_into_aws(input.tagging)?);
        b = b.set_tagging_directive(try_into_aws(input.tagging_directive)?);
        b = b.set_website_redirect_location(try_into_aws(input.website_redirect_location)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn create_bucket(&self, input: s3s::dto::CreateBucketInput) -> S3Result<s3s::dto::CreateBucketOutput> {
        debug!(?input);
        let mut b = self.0.create_bucket();
        b = b.set_acl(try_into_aws(input.acl)?);
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_create_bucket_configuration(try_into_aws(input.create_bucket_configuration)?);
        b = b.set_grant_full_control(try_into_aws(input.grant_full_control)?);
        b = b.set_grant_read(try_into_aws(input.grant_read)?);
        b = b.set_grant_read_acp(try_into_aws(input.grant_read_acp)?);
        b = b.set_grant_write(try_into_aws(input.grant_write)?);
        b = b.set_grant_write_acp(try_into_aws(input.grant_write_acp)?);
        b = b.set_object_lock_enabled_for_bucket(Some(try_into_aws(input.object_lock_enabled_for_bucket)?));
        b = b.set_object_ownership(try_into_aws(input.object_ownership)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn create_multipart_upload(
        &self,
        input: s3s::dto::CreateMultipartUploadInput,
    ) -> S3Result<s3s::dto::CreateMultipartUploadOutput> {
        debug!(?input);
        let mut b = self.0.create_multipart_upload();
        b = b.set_acl(try_into_aws(input.acl)?);
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_bucket_key_enabled(Some(try_into_aws(input.bucket_key_enabled)?));
        b = b.set_cache_control(try_into_aws(input.cache_control)?);
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_disposition(try_into_aws(input.content_disposition)?);
        b = b.set_content_encoding(try_into_aws(input.content_encoding)?);
        b = b.set_content_language(try_into_aws(input.content_language)?);
        b = b.set_content_type(try_into_aws(input.content_type)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_expires(try_into_aws(input.expires)?);
        b = b.set_grant_full_control(try_into_aws(input.grant_full_control)?);
        b = b.set_grant_read(try_into_aws(input.grant_read)?);
        b = b.set_grant_read_acp(try_into_aws(input.grant_read_acp)?);
        b = b.set_grant_write_acp(try_into_aws(input.grant_write_acp)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_metadata(try_into_aws(input.metadata)?);
        b = b.set_object_lock_legal_hold_status(try_into_aws(input.object_lock_legal_hold_status)?);
        b = b.set_object_lock_mode(try_into_aws(input.object_lock_mode)?);
        b = b.set_object_lock_retain_until_date(try_into_aws(input.object_lock_retain_until_date)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_sse_customer_algorithm(try_into_aws(input.sse_customer_algorithm)?);
        b = b.set_sse_customer_key(try_into_aws(input.sse_customer_key)?);
        b = b.set_sse_customer_key_md5(try_into_aws(input.sse_customer_key_md5)?);
        b = b.set_ssekms_encryption_context(try_into_aws(input.ssekms_encryption_context)?);
        b = b.set_ssekms_key_id(try_into_aws(input.ssekms_key_id)?);
        b = b.set_server_side_encryption(try_into_aws(input.server_side_encryption)?);
        b = b.set_storage_class(try_into_aws(input.storage_class)?);
        b = b.set_tagging(try_into_aws(input.tagging)?);
        b = b.set_website_redirect_location(try_into_aws(input.website_redirect_location)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket(&self, input: s3s::dto::DeleteBucketInput) -> S3Result<s3s::dto::DeleteBucketOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_analytics_configuration(
        &self,
        input: s3s::dto::DeleteBucketAnalyticsConfigurationInput,
    ) -> S3Result<s3s::dto::DeleteBucketAnalyticsConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_analytics_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_id(Some(try_into_aws(input.id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_cors(&self, input: s3s::dto::DeleteBucketCorsInput) -> S3Result<s3s::dto::DeleteBucketCorsOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_cors();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_encryption(
        &self,
        input: s3s::dto::DeleteBucketEncryptionInput,
    ) -> S3Result<s3s::dto::DeleteBucketEncryptionOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_encryption();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_intelligent_tiering_configuration(
        &self,
        input: s3s::dto::DeleteBucketIntelligentTieringConfigurationInput,
    ) -> S3Result<s3s::dto::DeleteBucketIntelligentTieringConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_intelligent_tiering_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_id(Some(try_into_aws(input.id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_inventory_configuration(
        &self,
        input: s3s::dto::DeleteBucketInventoryConfigurationInput,
    ) -> S3Result<s3s::dto::DeleteBucketInventoryConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_inventory_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_id(Some(try_into_aws(input.id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_lifecycle(
        &self,
        input: s3s::dto::DeleteBucketLifecycleInput,
    ) -> S3Result<s3s::dto::DeleteBucketLifecycleOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_lifecycle();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_metrics_configuration(
        &self,
        input: s3s::dto::DeleteBucketMetricsConfigurationInput,
    ) -> S3Result<s3s::dto::DeleteBucketMetricsConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_metrics_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_id(Some(try_into_aws(input.id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_ownership_controls(
        &self,
        input: s3s::dto::DeleteBucketOwnershipControlsInput,
    ) -> S3Result<s3s::dto::DeleteBucketOwnershipControlsOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_ownership_controls();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_policy(
        &self,
        input: s3s::dto::DeleteBucketPolicyInput,
    ) -> S3Result<s3s::dto::DeleteBucketPolicyOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_policy();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_replication(
        &self,
        input: s3s::dto::DeleteBucketReplicationInput,
    ) -> S3Result<s3s::dto::DeleteBucketReplicationOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_replication();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_tagging(
        &self,
        input: s3s::dto::DeleteBucketTaggingInput,
    ) -> S3Result<s3s::dto::DeleteBucketTaggingOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_tagging();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_bucket_website(
        &self,
        input: s3s::dto::DeleteBucketWebsiteInput,
    ) -> S3Result<s3s::dto::DeleteBucketWebsiteOutput> {
        debug!(?input);
        let mut b = self.0.delete_bucket_website();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_object(&self, input: s3s::dto::DeleteObjectInput) -> S3Result<s3s::dto::DeleteObjectOutput> {
        debug!(?input);
        let mut b = self.0.delete_object();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_bypass_governance_retention(Some(try_into_aws(input.bypass_governance_retention)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_mfa(try_into_aws(input.mfa)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_object_tagging(
        &self,
        input: s3s::dto::DeleteObjectTaggingInput,
    ) -> S3Result<s3s::dto::DeleteObjectTaggingOutput> {
        debug!(?input);
        let mut b = self.0.delete_object_tagging();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_objects(&self, input: s3s::dto::DeleteObjectsInput) -> S3Result<s3s::dto::DeleteObjectsOutput> {
        debug!(?input);
        let mut b = self.0.delete_objects();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_bypass_governance_retention(Some(try_into_aws(input.bypass_governance_retention)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_delete(Some(try_into_aws(input.delete)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_mfa(try_into_aws(input.mfa)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn delete_public_access_block(
        &self,
        input: s3s::dto::DeletePublicAccessBlockInput,
    ) -> S3Result<s3s::dto::DeletePublicAccessBlockOutput> {
        debug!(?input);
        let mut b = self.0.delete_public_access_block();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_accelerate_configuration(
        &self,
        input: s3s::dto::GetBucketAccelerateConfigurationInput,
    ) -> S3Result<s3s::dto::GetBucketAccelerateConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_accelerate_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_acl(&self, input: s3s::dto::GetBucketAclInput) -> S3Result<s3s::dto::GetBucketAclOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_acl();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_analytics_configuration(
        &self,
        input: s3s::dto::GetBucketAnalyticsConfigurationInput,
    ) -> S3Result<s3s::dto::GetBucketAnalyticsConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_analytics_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_id(Some(try_into_aws(input.id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_cors(&self, input: s3s::dto::GetBucketCorsInput) -> S3Result<s3s::dto::GetBucketCorsOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_cors();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_encryption(
        &self,
        input: s3s::dto::GetBucketEncryptionInput,
    ) -> S3Result<s3s::dto::GetBucketEncryptionOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_encryption();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_intelligent_tiering_configuration(
        &self,
        input: s3s::dto::GetBucketIntelligentTieringConfigurationInput,
    ) -> S3Result<s3s::dto::GetBucketIntelligentTieringConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_intelligent_tiering_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_id(Some(try_into_aws(input.id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_inventory_configuration(
        &self,
        input: s3s::dto::GetBucketInventoryConfigurationInput,
    ) -> S3Result<s3s::dto::GetBucketInventoryConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_inventory_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_id(Some(try_into_aws(input.id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_lifecycle_configuration(
        &self,
        input: s3s::dto::GetBucketLifecycleConfigurationInput,
    ) -> S3Result<s3s::dto::GetBucketLifecycleConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_lifecycle_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_location(&self, input: s3s::dto::GetBucketLocationInput) -> S3Result<s3s::dto::GetBucketLocationOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_location();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_logging(&self, input: s3s::dto::GetBucketLoggingInput) -> S3Result<s3s::dto::GetBucketLoggingOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_logging();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_metrics_configuration(
        &self,
        input: s3s::dto::GetBucketMetricsConfigurationInput,
    ) -> S3Result<s3s::dto::GetBucketMetricsConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_metrics_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_id(Some(try_into_aws(input.id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_notification_configuration(
        &self,
        input: s3s::dto::GetBucketNotificationConfigurationInput,
    ) -> S3Result<s3s::dto::GetBucketNotificationConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_notification_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_ownership_controls(
        &self,
        input: s3s::dto::GetBucketOwnershipControlsInput,
    ) -> S3Result<s3s::dto::GetBucketOwnershipControlsOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_ownership_controls();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_policy(&self, input: s3s::dto::GetBucketPolicyInput) -> S3Result<s3s::dto::GetBucketPolicyOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_policy();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_policy_status(
        &self,
        input: s3s::dto::GetBucketPolicyStatusInput,
    ) -> S3Result<s3s::dto::GetBucketPolicyStatusOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_policy_status();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_replication(
        &self,
        input: s3s::dto::GetBucketReplicationInput,
    ) -> S3Result<s3s::dto::GetBucketReplicationOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_replication();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_request_payment(
        &self,
        input: s3s::dto::GetBucketRequestPaymentInput,
    ) -> S3Result<s3s::dto::GetBucketRequestPaymentOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_request_payment();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_tagging(&self, input: s3s::dto::GetBucketTaggingInput) -> S3Result<s3s::dto::GetBucketTaggingOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_tagging();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_versioning(
        &self,
        input: s3s::dto::GetBucketVersioningInput,
    ) -> S3Result<s3s::dto::GetBucketVersioningOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_versioning();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_bucket_website(&self, input: s3s::dto::GetBucketWebsiteInput) -> S3Result<s3s::dto::GetBucketWebsiteOutput> {
        debug!(?input);
        let mut b = self.0.get_bucket_website();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_object(&self, input: s3s::dto::GetObjectInput) -> S3Result<s3s::dto::GetObjectOutput> {
        debug!(?input);
        let mut b = self.0.get_object();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_mode(try_into_aws(input.checksum_mode)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_if_match(try_into_aws(input.if_match)?);
        b = b.set_if_modified_since(try_into_aws(input.if_modified_since)?);
        b = b.set_if_none_match(try_into_aws(input.if_none_match)?);
        b = b.set_if_unmodified_since(try_into_aws(input.if_unmodified_since)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_part_number(Some(try_into_aws(input.part_number)?));
        b = b.set_range(try_into_aws(input.range)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_response_cache_control(try_into_aws(input.response_cache_control)?);
        b = b.set_response_content_disposition(try_into_aws(input.response_content_disposition)?);
        b = b.set_response_content_encoding(try_into_aws(input.response_content_encoding)?);
        b = b.set_response_content_language(try_into_aws(input.response_content_language)?);
        b = b.set_response_content_type(try_into_aws(input.response_content_type)?);
        b = b.set_response_expires(try_into_aws(input.response_expires)?);
        b = b.set_sse_customer_algorithm(try_into_aws(input.sse_customer_algorithm)?);
        b = b.set_sse_customer_key(try_into_aws(input.sse_customer_key)?);
        b = b.set_sse_customer_key_md5(try_into_aws(input.sse_customer_key_md5)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_object_acl(&self, input: s3s::dto::GetObjectAclInput) -> S3Result<s3s::dto::GetObjectAclOutput> {
        debug!(?input);
        let mut b = self.0.get_object_acl();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_object_attributes(
        &self,
        input: s3s::dto::GetObjectAttributesInput,
    ) -> S3Result<s3s::dto::GetObjectAttributesOutput> {
        debug!(?input);
        let mut b = self.0.get_object_attributes();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_max_parts(Some(try_into_aws(input.max_parts)?));
        b = b.set_object_attributes(Some(try_into_aws(input.object_attributes)?));
        b = b.set_part_number_marker(try_into_aws(input.part_number_marker)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_sse_customer_algorithm(try_into_aws(input.sse_customer_algorithm)?);
        b = b.set_sse_customer_key(try_into_aws(input.sse_customer_key)?);
        b = b.set_sse_customer_key_md5(try_into_aws(input.sse_customer_key_md5)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_object_legal_hold(
        &self,
        input: s3s::dto::GetObjectLegalHoldInput,
    ) -> S3Result<s3s::dto::GetObjectLegalHoldOutput> {
        debug!(?input);
        let mut b = self.0.get_object_legal_hold();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_object_lock_configuration(
        &self,
        input: s3s::dto::GetObjectLockConfigurationInput,
    ) -> S3Result<s3s::dto::GetObjectLockConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.get_object_lock_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_object_retention(
        &self,
        input: s3s::dto::GetObjectRetentionInput,
    ) -> S3Result<s3s::dto::GetObjectRetentionOutput> {
        debug!(?input);
        let mut b = self.0.get_object_retention();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_object_tagging(&self, input: s3s::dto::GetObjectTaggingInput) -> S3Result<s3s::dto::GetObjectTaggingOutput> {
        debug!(?input);
        let mut b = self.0.get_object_tagging();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_object_torrent(&self, input: s3s::dto::GetObjectTorrentInput) -> S3Result<s3s::dto::GetObjectTorrentOutput> {
        debug!(?input);
        let mut b = self.0.get_object_torrent();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn get_public_access_block(
        &self,
        input: s3s::dto::GetPublicAccessBlockInput,
    ) -> S3Result<s3s::dto::GetPublicAccessBlockOutput> {
        debug!(?input);
        let mut b = self.0.get_public_access_block();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn head_bucket(&self, input: s3s::dto::HeadBucketInput) -> S3Result<s3s::dto::HeadBucketOutput> {
        debug!(?input);
        let mut b = self.0.head_bucket();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn head_object(&self, input: s3s::dto::HeadObjectInput) -> S3Result<s3s::dto::HeadObjectOutput> {
        debug!(?input);
        let mut b = self.0.head_object();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_mode(try_into_aws(input.checksum_mode)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_if_match(try_into_aws(input.if_match)?);
        b = b.set_if_modified_since(try_into_aws(input.if_modified_since)?);
        b = b.set_if_none_match(try_into_aws(input.if_none_match)?);
        b = b.set_if_unmodified_since(try_into_aws(input.if_unmodified_since)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_part_number(Some(try_into_aws(input.part_number)?));
        b = b.set_range(try_into_aws(input.range)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_sse_customer_algorithm(try_into_aws(input.sse_customer_algorithm)?);
        b = b.set_sse_customer_key(try_into_aws(input.sse_customer_key)?);
        b = b.set_sse_customer_key_md5(try_into_aws(input.sse_customer_key_md5)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn list_bucket_analytics_configurations(
        &self,
        input: s3s::dto::ListBucketAnalyticsConfigurationsInput,
    ) -> S3Result<s3s::dto::ListBucketAnalyticsConfigurationsOutput> {
        debug!(?input);
        let mut b = self.0.list_bucket_analytics_configurations();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_continuation_token(try_into_aws(input.continuation_token)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn list_bucket_intelligent_tiering_configurations(
        &self,
        input: s3s::dto::ListBucketIntelligentTieringConfigurationsInput,
    ) -> S3Result<s3s::dto::ListBucketIntelligentTieringConfigurationsOutput> {
        debug!(?input);
        let mut b = self.0.list_bucket_intelligent_tiering_configurations();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_continuation_token(try_into_aws(input.continuation_token)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn list_bucket_inventory_configurations(
        &self,
        input: s3s::dto::ListBucketInventoryConfigurationsInput,
    ) -> S3Result<s3s::dto::ListBucketInventoryConfigurationsOutput> {
        debug!(?input);
        let mut b = self.0.list_bucket_inventory_configurations();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_continuation_token(try_into_aws(input.continuation_token)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn list_bucket_metrics_configurations(
        &self,
        input: s3s::dto::ListBucketMetricsConfigurationsInput,
    ) -> S3Result<s3s::dto::ListBucketMetricsConfigurationsOutput> {
        debug!(?input);
        let mut b = self.0.list_bucket_metrics_configurations();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_continuation_token(try_into_aws(input.continuation_token)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn list_buckets(&self, input: s3s::dto::ListBucketsInput) -> S3Result<s3s::dto::ListBucketsOutput> {
        debug!(?input);
        let result = self.0.list_buckets().send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn list_multipart_uploads(
        &self,
        input: s3s::dto::ListMultipartUploadsInput,
    ) -> S3Result<s3s::dto::ListMultipartUploadsOutput> {
        debug!(?input);
        let mut b = self.0.list_multipart_uploads();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_delimiter(try_into_aws(input.delimiter)?);
        b = b.set_encoding_type(try_into_aws(input.encoding_type)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key_marker(try_into_aws(input.key_marker)?);
        b = b.set_max_uploads(Some(try_into_aws(input.max_uploads)?));
        b = b.set_prefix(try_into_aws(input.prefix)?);
        b = b.set_upload_id_marker(try_into_aws(input.upload_id_marker)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn list_object_versions(
        &self,
        input: s3s::dto::ListObjectVersionsInput,
    ) -> S3Result<s3s::dto::ListObjectVersionsOutput> {
        debug!(?input);
        let mut b = self.0.list_object_versions();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_delimiter(try_into_aws(input.delimiter)?);
        b = b.set_encoding_type(try_into_aws(input.encoding_type)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key_marker(try_into_aws(input.key_marker)?);
        b = b.set_max_keys(Some(try_into_aws(input.max_keys)?));
        b = b.set_prefix(try_into_aws(input.prefix)?);
        b = b.set_version_id_marker(try_into_aws(input.version_id_marker)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn list_objects(&self, input: s3s::dto::ListObjectsInput) -> S3Result<s3s::dto::ListObjectsOutput> {
        debug!(?input);
        let mut b = self.0.list_objects();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_delimiter(try_into_aws(input.delimiter)?);
        b = b.set_encoding_type(try_into_aws(input.encoding_type)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_marker(try_into_aws(input.marker)?);
        b = b.set_max_keys(Some(try_into_aws(input.max_keys)?));
        b = b.set_prefix(try_into_aws(input.prefix)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn list_objects_v2(&self, input: s3s::dto::ListObjectsV2Input) -> S3Result<s3s::dto::ListObjectsV2Output> {
        debug!(?input);
        let mut b = self.0.list_objects_v2();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_continuation_token(try_into_aws(input.continuation_token)?);
        b = b.set_delimiter(try_into_aws(input.delimiter)?);
        b = b.set_encoding_type(try_into_aws(input.encoding_type)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_fetch_owner(Some(try_into_aws(input.fetch_owner)?));
        b = b.set_max_keys(Some(try_into_aws(input.max_keys)?));
        b = b.set_prefix(try_into_aws(input.prefix)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_start_after(try_into_aws(input.start_after)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn list_parts(&self, input: s3s::dto::ListPartsInput) -> S3Result<s3s::dto::ListPartsOutput> {
        debug!(?input);
        let mut b = self.0.list_parts();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_max_parts(Some(try_into_aws(input.max_parts)?));
        b = b.set_part_number_marker(try_into_aws(input.part_number_marker)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_sse_customer_algorithm(try_into_aws(input.sse_customer_algorithm)?);
        b = b.set_sse_customer_key(try_into_aws(input.sse_customer_key)?);
        b = b.set_sse_customer_key_md5(try_into_aws(input.sse_customer_key_md5)?);
        b = b.set_upload_id(Some(try_into_aws(input.upload_id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_accelerate_configuration(
        &self,
        input: s3s::dto::PutBucketAccelerateConfigurationInput,
    ) -> S3Result<s3s::dto::PutBucketAccelerateConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_accelerate_configuration();
        b = b.set_accelerate_configuration(Some(try_into_aws(input.accelerate_configuration)?));
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_acl(&self, input: s3s::dto::PutBucketAclInput) -> S3Result<s3s::dto::PutBucketAclOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_acl();
        b = b.set_acl(try_into_aws(input.acl)?);
        b = b.set_access_control_policy(try_into_aws(input.access_control_policy)?);
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_grant_full_control(try_into_aws(input.grant_full_control)?);
        b = b.set_grant_read(try_into_aws(input.grant_read)?);
        b = b.set_grant_read_acp(try_into_aws(input.grant_read_acp)?);
        b = b.set_grant_write(try_into_aws(input.grant_write)?);
        b = b.set_grant_write_acp(try_into_aws(input.grant_write_acp)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_analytics_configuration(
        &self,
        input: s3s::dto::PutBucketAnalyticsConfigurationInput,
    ) -> S3Result<s3s::dto::PutBucketAnalyticsConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_analytics_configuration();
        b = b.set_analytics_configuration(Some(try_into_aws(input.analytics_configuration)?));
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_id(Some(try_into_aws(input.id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_cors(&self, input: s3s::dto::PutBucketCorsInput) -> S3Result<s3s::dto::PutBucketCorsOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_cors();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_cors_configuration(Some(try_into_aws(input.cors_configuration)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_encryption(
        &self,
        input: s3s::dto::PutBucketEncryptionInput,
    ) -> S3Result<s3s::dto::PutBucketEncryptionOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_encryption();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_server_side_encryption_configuration(Some(try_into_aws(input.server_side_encryption_configuration)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_intelligent_tiering_configuration(
        &self,
        input: s3s::dto::PutBucketIntelligentTieringConfigurationInput,
    ) -> S3Result<s3s::dto::PutBucketIntelligentTieringConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_intelligent_tiering_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_id(Some(try_into_aws(input.id)?));
        b = b.set_intelligent_tiering_configuration(Some(try_into_aws(input.intelligent_tiering_configuration)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_inventory_configuration(
        &self,
        input: s3s::dto::PutBucketInventoryConfigurationInput,
    ) -> S3Result<s3s::dto::PutBucketInventoryConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_inventory_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_id(Some(try_into_aws(input.id)?));
        b = b.set_inventory_configuration(Some(try_into_aws(input.inventory_configuration)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_lifecycle_configuration(
        &self,
        input: s3s::dto::PutBucketLifecycleConfigurationInput,
    ) -> S3Result<s3s::dto::PutBucketLifecycleConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_lifecycle_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_lifecycle_configuration(try_into_aws(input.lifecycle_configuration)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_logging(&self, input: s3s::dto::PutBucketLoggingInput) -> S3Result<s3s::dto::PutBucketLoggingOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_logging();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_bucket_logging_status(Some(try_into_aws(input.bucket_logging_status)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_metrics_configuration(
        &self,
        input: s3s::dto::PutBucketMetricsConfigurationInput,
    ) -> S3Result<s3s::dto::PutBucketMetricsConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_metrics_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_id(Some(try_into_aws(input.id)?));
        b = b.set_metrics_configuration(Some(try_into_aws(input.metrics_configuration)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_notification_configuration(
        &self,
        input: s3s::dto::PutBucketNotificationConfigurationInput,
    ) -> S3Result<s3s::dto::PutBucketNotificationConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_notification_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_notification_configuration(Some(try_into_aws(input.notification_configuration)?));
        b = b.set_skip_destination_validation(Some(try_into_aws(input.skip_destination_validation)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_ownership_controls(
        &self,
        input: s3s::dto::PutBucketOwnershipControlsInput,
    ) -> S3Result<s3s::dto::PutBucketOwnershipControlsOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_ownership_controls();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_ownership_controls(Some(try_into_aws(input.ownership_controls)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_policy(&self, input: s3s::dto::PutBucketPolicyInput) -> S3Result<s3s::dto::PutBucketPolicyOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_policy();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_confirm_remove_self_bucket_access(Some(try_into_aws(input.confirm_remove_self_bucket_access)?));
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_policy(Some(try_into_aws(input.policy)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_replication(
        &self,
        input: s3s::dto::PutBucketReplicationInput,
    ) -> S3Result<s3s::dto::PutBucketReplicationOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_replication();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_replication_configuration(Some(try_into_aws(input.replication_configuration)?));
        b = b.set_token(try_into_aws(input.token)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_request_payment(
        &self,
        input: s3s::dto::PutBucketRequestPaymentInput,
    ) -> S3Result<s3s::dto::PutBucketRequestPaymentOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_request_payment();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_request_payment_configuration(Some(try_into_aws(input.request_payment_configuration)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_tagging(&self, input: s3s::dto::PutBucketTaggingInput) -> S3Result<s3s::dto::PutBucketTaggingOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_tagging();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_tagging(Some(try_into_aws(input.tagging)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_versioning(
        &self,
        input: s3s::dto::PutBucketVersioningInput,
    ) -> S3Result<s3s::dto::PutBucketVersioningOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_versioning();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_mfa(try_into_aws(input.mfa)?);
        b = b.set_versioning_configuration(Some(try_into_aws(input.versioning_configuration)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_bucket_website(&self, input: s3s::dto::PutBucketWebsiteInput) -> S3Result<s3s::dto::PutBucketWebsiteOutput> {
        debug!(?input);
        let mut b = self.0.put_bucket_website();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_website_configuration(Some(try_into_aws(input.website_configuration)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_object(&self, input: s3s::dto::PutObjectInput) -> S3Result<s3s::dto::PutObjectOutput> {
        debug!(?input);
        let mut b = self.0.put_object();
        b = b.set_acl(try_into_aws(input.acl)?);
        b = b.set_body(Some(transform_body(input.body).await));
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_bucket_key_enabled(Some(try_into_aws(input.bucket_key_enabled)?));
        b = b.set_cache_control(try_into_aws(input.cache_control)?);
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_checksum_crc32(try_into_aws(input.checksum_crc32)?);
        b = b.set_checksum_crc32_c(try_into_aws(input.checksum_crc32c)?);
        b = b.set_checksum_sha1(try_into_aws(input.checksum_sha1)?);
        b = b.set_checksum_sha256(try_into_aws(input.checksum_sha256)?);
        b = b.set_content_disposition(try_into_aws(input.content_disposition)?);
        b = b.set_content_encoding(try_into_aws(input.content_encoding)?);
        b = b.set_content_language(try_into_aws(input.content_language)?);
        b = b.set_content_length(Some(try_into_aws(input.content_length)?));
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_content_type(try_into_aws(input.content_type)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_expires(try_into_aws(input.expires)?);
        b = b.set_grant_full_control(try_into_aws(input.grant_full_control)?);
        b = b.set_grant_read(try_into_aws(input.grant_read)?);
        b = b.set_grant_read_acp(try_into_aws(input.grant_read_acp)?);
        b = b.set_grant_write_acp(try_into_aws(input.grant_write_acp)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_metadata(try_into_aws(input.metadata)?);
        b = b.set_object_lock_legal_hold_status(try_into_aws(input.object_lock_legal_hold_status)?);
        b = b.set_object_lock_mode(try_into_aws(input.object_lock_mode)?);
        b = b.set_object_lock_retain_until_date(try_into_aws(input.object_lock_retain_until_date)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_sse_customer_algorithm(try_into_aws(input.sse_customer_algorithm)?);
        b = b.set_sse_customer_key(try_into_aws(input.sse_customer_key)?);
        b = b.set_sse_customer_key_md5(try_into_aws(input.sse_customer_key_md5)?);
        b = b.set_ssekms_encryption_context(try_into_aws(input.ssekms_encryption_context)?);
        b = b.set_ssekms_key_id(try_into_aws(input.ssekms_key_id)?);
        b = b.set_server_side_encryption(try_into_aws(input.server_side_encryption)?);
        b = b.set_storage_class(try_into_aws(input.storage_class)?);
        b = b.set_tagging(try_into_aws(input.tagging)?);
        b = b.set_website_redirect_location(try_into_aws(input.website_redirect_location)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_object_acl(&self, input: s3s::dto::PutObjectAclInput) -> S3Result<s3s::dto::PutObjectAclOutput> {
        debug!(?input);
        let mut b = self.0.put_object_acl();
        b = b.set_acl(try_into_aws(input.acl)?);
        b = b.set_access_control_policy(try_into_aws(input.access_control_policy)?);
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_grant_full_control(try_into_aws(input.grant_full_control)?);
        b = b.set_grant_read(try_into_aws(input.grant_read)?);
        b = b.set_grant_read_acp(try_into_aws(input.grant_read_acp)?);
        b = b.set_grant_write(try_into_aws(input.grant_write)?);
        b = b.set_grant_write_acp(try_into_aws(input.grant_write_acp)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_object_legal_hold(
        &self,
        input: s3s::dto::PutObjectLegalHoldInput,
    ) -> S3Result<s3s::dto::PutObjectLegalHoldOutput> {
        debug!(?input);
        let mut b = self.0.put_object_legal_hold();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_legal_hold(try_into_aws(input.legal_hold)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_object_lock_configuration(
        &self,
        input: s3s::dto::PutObjectLockConfigurationInput,
    ) -> S3Result<s3s::dto::PutObjectLockConfigurationOutput> {
        debug!(?input);
        let mut b = self.0.put_object_lock_configuration();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_object_lock_configuration(try_into_aws(input.object_lock_configuration)?);
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_token(try_into_aws(input.token)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_object_retention(
        &self,
        input: s3s::dto::PutObjectRetentionInput,
    ) -> S3Result<s3s::dto::PutObjectRetentionOutput> {
        debug!(?input);
        let mut b = self.0.put_object_retention();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_bypass_governance_retention(Some(try_into_aws(input.bypass_governance_retention)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_retention(try_into_aws(input.retention)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_object_tagging(&self, input: s3s::dto::PutObjectTaggingInput) -> S3Result<s3s::dto::PutObjectTaggingOutput> {
        debug!(?input);
        let mut b = self.0.put_object_tagging();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_tagging(Some(try_into_aws(input.tagging)?));
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn put_public_access_block(
        &self,
        input: s3s::dto::PutPublicAccessBlockInput,
    ) -> S3Result<s3s::dto::PutPublicAccessBlockOutput> {
        debug!(?input);
        let mut b = self.0.put_public_access_block();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_public_access_block_configuration(Some(try_into_aws(input.public_access_block_configuration)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn restore_object(&self, input: s3s::dto::RestoreObjectInput) -> S3Result<s3s::dto::RestoreObjectOutput> {
        debug!(?input);
        let mut b = self.0.restore_object();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_restore_request(try_into_aws(input.restore_request)?);
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn upload_part(&self, input: s3s::dto::UploadPartInput) -> S3Result<s3s::dto::UploadPartOutput> {
        debug!(?input);
        let mut b = self.0.upload_part();
        b = b.set_body(Some(transform_body(input.body).await));
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_checksum_algorithm(try_into_aws(input.checksum_algorithm)?);
        b = b.set_checksum_crc32(try_into_aws(input.checksum_crc32)?);
        b = b.set_checksum_crc32_c(try_into_aws(input.checksum_crc32c)?);
        b = b.set_checksum_sha1(try_into_aws(input.checksum_sha1)?);
        b = b.set_checksum_sha256(try_into_aws(input.checksum_sha256)?);
        b = b.set_content_length(Some(try_into_aws(input.content_length)?));
        b = b.set_content_md5(try_into_aws(input.content_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_part_number(Some(try_into_aws(input.part_number)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_sse_customer_algorithm(try_into_aws(input.sse_customer_algorithm)?);
        b = b.set_sse_customer_key(try_into_aws(input.sse_customer_key)?);
        b = b.set_sse_customer_key_md5(try_into_aws(input.sse_customer_key_md5)?);
        b = b.set_upload_id(Some(try_into_aws(input.upload_id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn upload_part_copy(&self, input: s3s::dto::UploadPartCopyInput) -> S3Result<s3s::dto::UploadPartCopyOutput> {
        debug!(?input);
        let mut b = self.0.upload_part_copy();
        b = b.set_bucket(Some(try_into_aws(input.bucket)?));
        b = b.set_copy_source(Some(try_into_aws(input.copy_source)?));
        b = b.set_copy_source_if_match(try_into_aws(input.copy_source_if_match)?);
        b = b.set_copy_source_if_modified_since(try_into_aws(input.copy_source_if_modified_since)?);
        b = b.set_copy_source_if_none_match(try_into_aws(input.copy_source_if_none_match)?);
        b = b.set_copy_source_if_unmodified_since(try_into_aws(input.copy_source_if_unmodified_since)?);
        b = b.set_copy_source_range(try_into_aws(input.copy_source_range)?);
        b = b.set_copy_source_sse_customer_algorithm(try_into_aws(input.copy_source_sse_customer_algorithm)?);
        b = b.set_copy_source_sse_customer_key(try_into_aws(input.copy_source_sse_customer_key)?);
        b = b.set_copy_source_sse_customer_key_md5(try_into_aws(input.copy_source_sse_customer_key_md5)?);
        b = b.set_expected_bucket_owner(try_into_aws(input.expected_bucket_owner)?);
        b = b.set_expected_source_bucket_owner(try_into_aws(input.expected_source_bucket_owner)?);
        b = b.set_key(Some(try_into_aws(input.key)?));
        b = b.set_part_number(Some(try_into_aws(input.part_number)?));
        b = b.set_request_payer(try_into_aws(input.request_payer)?);
        b = b.set_sse_customer_algorithm(try_into_aws(input.sse_customer_algorithm)?);
        b = b.set_sse_customer_key(try_into_aws(input.sse_customer_key)?);
        b = b.set_sse_customer_key_md5(try_into_aws(input.sse_customer_key_md5)?);
        b = b.set_upload_id(Some(try_into_aws(input.upload_id)?));
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }

    #[tracing::instrument(skip(self, input))]
    async fn write_get_object_response(
        &self,
        input: s3s::dto::WriteGetObjectResponseInput,
    ) -> S3Result<s3s::dto::WriteGetObjectResponseOutput> {
        debug!(?input);
        let mut b = self.0.write_get_object_response();
        b = b.set_accept_ranges(try_into_aws(input.accept_ranges)?);
        b = b.set_body(Some(transform_body(input.body).await));
        b = b.set_bucket_key_enabled(Some(try_into_aws(input.bucket_key_enabled)?));
        b = b.set_cache_control(try_into_aws(input.cache_control)?);
        b = b.set_checksum_crc32(try_into_aws(input.checksum_crc32)?);
        b = b.set_checksum_crc32_c(try_into_aws(input.checksum_crc32c)?);
        b = b.set_checksum_sha1(try_into_aws(input.checksum_sha1)?);
        b = b.set_checksum_sha256(try_into_aws(input.checksum_sha256)?);
        b = b.set_content_disposition(try_into_aws(input.content_disposition)?);
        b = b.set_content_encoding(try_into_aws(input.content_encoding)?);
        b = b.set_content_language(try_into_aws(input.content_language)?);
        b = b.set_content_length(Some(try_into_aws(input.content_length)?));
        b = b.set_content_range(try_into_aws(input.content_range)?);
        b = b.set_content_type(try_into_aws(input.content_type)?);
        b = b.set_delete_marker(Some(try_into_aws(input.delete_marker)?));
        b = b.set_e_tag(try_into_aws(input.e_tag)?);
        b = b.set_error_code(try_into_aws(input.error_code)?);
        b = b.set_error_message(try_into_aws(input.error_message)?);
        b = b.set_expiration(try_into_aws(input.expiration)?);
        b = b.set_expires(try_into_aws(input.expires)?);
        b = b.set_last_modified(try_into_aws(input.last_modified)?);
        b = b.set_metadata(try_into_aws(input.metadata)?);
        b = b.set_missing_meta(Some(try_into_aws(input.missing_meta)?));
        b = b.set_object_lock_legal_hold_status(try_into_aws(input.object_lock_legal_hold_status)?);
        b = b.set_object_lock_mode(try_into_aws(input.object_lock_mode)?);
        b = b.set_object_lock_retain_until_date(try_into_aws(input.object_lock_retain_until_date)?);
        b = b.set_parts_count(Some(try_into_aws(input.parts_count)?));
        b = b.set_replication_status(try_into_aws(input.replication_status)?);
        b = b.set_request_charged(try_into_aws(input.request_charged)?);
        b = b.set_request_route(Some(try_into_aws(input.request_route)?));
        b = b.set_request_token(Some(try_into_aws(input.request_token)?));
        b = b.set_restore(try_into_aws(input.restore)?);
        b = b.set_sse_customer_algorithm(try_into_aws(input.sse_customer_algorithm)?);
        b = b.set_sse_customer_key_md5(try_into_aws(input.sse_customer_key_md5)?);
        b = b.set_ssekms_key_id(try_into_aws(input.ssekms_key_id)?);
        b = b.set_server_side_encryption(try_into_aws(input.server_side_encryption)?);
        b = b.set_status_code(Some(try_into_aws(input.status_code)?));
        b = b.set_storage_class(try_into_aws(input.storage_class)?);
        b = b.set_tag_count(Some(try_into_aws(input.tag_count)?));
        b = b.set_version_id(try_into_aws(input.version_id)?);
        let result = b.send().await;
        match result {
            Ok(output) => {
                let output = try_from_aws(output)?;
                debug!(?output);
                Ok(output)
            }
            Err(e) => Err(wrap_sdk_error!(e)),
        }
    }
}
