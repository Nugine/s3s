use super::*;

use aws_sdk_s3::error::*;
use aws_sdk_s3::input::*;
use aws_sdk_s3::model::*;
use aws_sdk_s3::output::*;

impl AwsConversion for s3s::dto::AbortIncompleteMultipartUpload {
    type Target = AbortIncompleteMultipartUpload;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            days_after_initiation: try_from_aws(x.days_after_initiation)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_days_after_initiation(Some(try_into_aws(x.days_after_initiation)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::AbortMultipartUploadInput {
    type Target = AbortMultipartUploadInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            request_payer: try_from_aws(x.request_payer)?,
            upload_id: unwrap_from_aws(x.upload_id, "upload_id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_upload_id(Some(try_into_aws(x.upload_id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::AbortMultipartUploadOutput {
    type Target = AbortMultipartUploadOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            request_charged: try_from_aws(x.request_charged)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::AccelerateConfiguration {
    type Target = AccelerateConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            status: try_from_aws(x.status)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_status(try_into_aws(x.status)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::AccessControlPolicy {
    type Target = AccessControlPolicy;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            grants: try_from_aws(x.grants)?,
            owner: try_from_aws(x.owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_grants(try_into_aws(x.grants)?);
        y = y.set_owner(try_into_aws(x.owner)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::AccessControlTranslation {
    type Target = AccessControlTranslation;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            owner: unwrap_from_aws(x.owner, "owner")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_owner(Some(try_into_aws(x.owner)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::AnalyticsAndOperator {
    type Target = AnalyticsAndOperator;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            prefix: try_from_aws(x.prefix)?,
            tags: try_from_aws(x.tags)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_tags(try_into_aws(x.tags)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::AnalyticsConfiguration {
    type Target = AnalyticsConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            filter: try_from_aws(x.filter)?,
            id: unwrap_from_aws(x.id, "id")?,
            storage_class_analysis: unwrap_from_aws(x.storage_class_analysis, "storage_class_analysis")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_filter(try_into_aws(x.filter)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y = y.set_storage_class_analysis(Some(try_into_aws(x.storage_class_analysis)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::AnalyticsExportDestination {
    type Target = AnalyticsExportDestination;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            s3_bucket_destination: unwrap_from_aws(x.s3_bucket_destination, "s3_bucket_destination")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_s3_bucket_destination(Some(try_into_aws(x.s3_bucket_destination)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::AnalyticsFilter {
    type Target = AnalyticsFilter;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            AnalyticsFilter::And(v) => Self::And(try_from_aws(v)?),
            AnalyticsFilter::Prefix(v) => Self::Prefix(try_from_aws(v)?),
            AnalyticsFilter::Tag(v) => Self::Tag(try_from_aws(v)?),
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::And(v) => AnalyticsFilter::And(try_into_aws(v)?),
            Self::Prefix(v) => AnalyticsFilter::Prefix(try_into_aws(v)?),
            Self::Tag(v) => AnalyticsFilter::Tag(try_into_aws(v)?),
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::AnalyticsS3BucketDestination {
    type Target = AnalyticsS3BucketDestination;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            bucket_account_id: try_from_aws(x.bucket_account_id)?,
            format: unwrap_from_aws(x.format, "format")?,
            prefix: try_from_aws(x.prefix)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_bucket_account_id(try_into_aws(x.bucket_account_id)?);
        y = y.set_format(Some(try_into_aws(x.format)?));
        y = y.set_prefix(try_into_aws(x.prefix)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::AnalyticsS3ExportFileFormat {
    type Target = AnalyticsS3ExportFileFormat;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            AnalyticsS3ExportFileFormat::Csv => Self::Csv,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Csv => AnalyticsS3ExportFileFormat::Csv,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ArchiveStatus {
    type Target = ArchiveStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ArchiveStatus::ArchiveAccess => Self::ArchiveAccess,
            ArchiveStatus::DeepArchiveAccess => Self::DeepArchiveAccess,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::ArchiveAccess => ArchiveStatus::ArchiveAccess,
            Self::DeepArchiveAccess => ArchiveStatus::DeepArchiveAccess,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::Bucket {
    type Target = Bucket;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            creation_date: try_from_aws(x.creation_date)?,
            name: try_from_aws(x.name)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_creation_date(try_into_aws(x.creation_date)?);
        y = y.set_name(try_into_aws(x.name)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::BucketAccelerateStatus {
    type Target = BucketAccelerateStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            BucketAccelerateStatus::Enabled => Self::Enabled,
            BucketAccelerateStatus::Suspended => Self::Suspended,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Enabled => BucketAccelerateStatus::Enabled,
            Self::Suspended => BucketAccelerateStatus::Suspended,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::BucketAlreadyExists {
    type Target = BucketAlreadyExists;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::BucketAlreadyOwnedByYou {
    type Target = BucketAlreadyOwnedByYou;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::BucketCannedACL {
    type Target = BucketCannedAcl;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            BucketCannedAcl::AuthenticatedRead => Self::AuthenticatedRead,
            BucketCannedAcl::Private => Self::Private,
            BucketCannedAcl::PublicRead => Self::PublicRead,
            BucketCannedAcl::PublicReadWrite => Self::PublicReadWrite,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::AuthenticatedRead => BucketCannedAcl::AuthenticatedRead,
            Self::Private => BucketCannedAcl::Private,
            Self::PublicRead => BucketCannedAcl::PublicRead,
            Self::PublicReadWrite => BucketCannedAcl::PublicReadWrite,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::BucketLifecycleConfiguration {
    type Target = BucketLifecycleConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            rules: unwrap_from_aws(x.rules, "rules")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_rules(Some(try_into_aws(x.rules)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::BucketLocationConstraint {
    type Target = BucketLocationConstraint;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            BucketLocationConstraint::Eu => Self::Eu,
            BucketLocationConstraint::AfSouth1 => Self::AfSouth1,
            BucketLocationConstraint::ApEast1 => Self::ApEast1,
            BucketLocationConstraint::ApNortheast1 => Self::ApNortheast1,
            BucketLocationConstraint::ApNortheast2 => Self::ApNortheast2,
            BucketLocationConstraint::ApNortheast3 => Self::ApNortheast3,
            BucketLocationConstraint::ApSouth1 => Self::ApSouth1,
            BucketLocationConstraint::ApSoutheast1 => Self::ApSoutheast1,
            BucketLocationConstraint::ApSoutheast2 => Self::ApSoutheast2,
            BucketLocationConstraint::ApSoutheast3 => Self::ApSoutheast3,
            BucketLocationConstraint::CaCentral1 => Self::CaCentral1,
            BucketLocationConstraint::CnNorth1 => Self::CnNorth1,
            BucketLocationConstraint::CnNorthwest1 => Self::CnNorthwest1,
            BucketLocationConstraint::EuCentral1 => Self::EuCentral1,
            BucketLocationConstraint::EuNorth1 => Self::EuNorth1,
            BucketLocationConstraint::EuSouth1 => Self::EuSouth1,
            BucketLocationConstraint::EuWest1 => Self::EuWest1,
            BucketLocationConstraint::EuWest2 => Self::EuWest2,
            BucketLocationConstraint::EuWest3 => Self::EuWest3,
            BucketLocationConstraint::MeSouth1 => Self::MeSouth1,
            BucketLocationConstraint::SaEast1 => Self::SaEast1,
            BucketLocationConstraint::UsEast2 => Self::UsEast2,
            BucketLocationConstraint::UsGovEast1 => Self::UsGovEast1,
            BucketLocationConstraint::UsGovWest1 => Self::UsGovWest1,
            BucketLocationConstraint::UsWest1 => Self::UsWest1,
            BucketLocationConstraint::UsWest2 => Self::UsWest2,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Eu => BucketLocationConstraint::Eu,
            Self::AfSouth1 => BucketLocationConstraint::AfSouth1,
            Self::ApEast1 => BucketLocationConstraint::ApEast1,
            Self::ApNortheast1 => BucketLocationConstraint::ApNortheast1,
            Self::ApNortheast2 => BucketLocationConstraint::ApNortheast2,
            Self::ApNortheast3 => BucketLocationConstraint::ApNortheast3,
            Self::ApSouth1 => BucketLocationConstraint::ApSouth1,
            Self::ApSoutheast1 => BucketLocationConstraint::ApSoutheast1,
            Self::ApSoutheast2 => BucketLocationConstraint::ApSoutheast2,
            Self::ApSoutheast3 => BucketLocationConstraint::ApSoutheast3,
            Self::CaCentral1 => BucketLocationConstraint::CaCentral1,
            Self::CnNorth1 => BucketLocationConstraint::CnNorth1,
            Self::CnNorthwest1 => BucketLocationConstraint::CnNorthwest1,
            Self::EuCentral1 => BucketLocationConstraint::EuCentral1,
            Self::EuNorth1 => BucketLocationConstraint::EuNorth1,
            Self::EuSouth1 => BucketLocationConstraint::EuSouth1,
            Self::EuWest1 => BucketLocationConstraint::EuWest1,
            Self::EuWest2 => BucketLocationConstraint::EuWest2,
            Self::EuWest3 => BucketLocationConstraint::EuWest3,
            Self::MeSouth1 => BucketLocationConstraint::MeSouth1,
            Self::SaEast1 => BucketLocationConstraint::SaEast1,
            Self::UsEast2 => BucketLocationConstraint::UsEast2,
            Self::UsGovEast1 => BucketLocationConstraint::UsGovEast1,
            Self::UsGovWest1 => BucketLocationConstraint::UsGovWest1,
            Self::UsWest1 => BucketLocationConstraint::UsWest1,
            Self::UsWest2 => BucketLocationConstraint::UsWest2,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::BucketLoggingStatus {
    type Target = BucketLoggingStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            logging_enabled: try_from_aws(x.logging_enabled)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_logging_enabled(try_into_aws(x.logging_enabled)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::BucketLogsPermission {
    type Target = BucketLogsPermission;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            BucketLogsPermission::FullControl => Self::FullControl,
            BucketLogsPermission::Read => Self::Read,
            BucketLogsPermission::Write => Self::Write,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::FullControl => BucketLogsPermission::FullControl,
            Self::Read => BucketLogsPermission::Read,
            Self::Write => BucketLogsPermission::Write,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::BucketVersioningStatus {
    type Target = BucketVersioningStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            BucketVersioningStatus::Enabled => Self::Enabled,
            BucketVersioningStatus::Suspended => Self::Suspended,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Enabled => BucketVersioningStatus::Enabled,
            Self::Suspended => BucketVersioningStatus::Suspended,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::CORSConfiguration {
    type Target = CorsConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            cors_rules: unwrap_from_aws(x.cors_rules, "cors_rules")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_cors_rules(Some(try_into_aws(x.cors_rules)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CORSRule {
    type Target = CorsRule;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            allowed_headers: try_from_aws(x.allowed_headers)?,
            allowed_methods: unwrap_from_aws(x.allowed_methods, "allowed_methods")?,
            allowed_origins: unwrap_from_aws(x.allowed_origins, "allowed_origins")?,
            expose_headers: try_from_aws(x.expose_headers)?,
            id: try_from_aws(x.id)?,
            max_age_seconds: try_from_aws(x.max_age_seconds)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_allowed_headers(try_into_aws(x.allowed_headers)?);
        y = y.set_allowed_methods(Some(try_into_aws(x.allowed_methods)?));
        y = y.set_allowed_origins(Some(try_into_aws(x.allowed_origins)?));
        y = y.set_expose_headers(try_into_aws(x.expose_headers)?);
        y = y.set_id(try_into_aws(x.id)?);
        y = y.set_max_age_seconds(Some(try_into_aws(x.max_age_seconds)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CSVInput {
    type Target = CsvInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            allow_quoted_record_delimiter: try_from_aws(x.allow_quoted_record_delimiter)?,
            comments: try_from_aws(x.comments)?,
            field_delimiter: try_from_aws(x.field_delimiter)?,
            file_header_info: try_from_aws(x.file_header_info)?,
            quote_character: try_from_aws(x.quote_character)?,
            quote_escape_character: try_from_aws(x.quote_escape_character)?,
            record_delimiter: try_from_aws(x.record_delimiter)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_allow_quoted_record_delimiter(Some(try_into_aws(x.allow_quoted_record_delimiter)?));
        y = y.set_comments(try_into_aws(x.comments)?);
        y = y.set_field_delimiter(try_into_aws(x.field_delimiter)?);
        y = y.set_file_header_info(try_into_aws(x.file_header_info)?);
        y = y.set_quote_character(try_into_aws(x.quote_character)?);
        y = y.set_quote_escape_character(try_into_aws(x.quote_escape_character)?);
        y = y.set_record_delimiter(try_into_aws(x.record_delimiter)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CSVOutput {
    type Target = CsvOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            field_delimiter: try_from_aws(x.field_delimiter)?,
            quote_character: try_from_aws(x.quote_character)?,
            quote_escape_character: try_from_aws(x.quote_escape_character)?,
            quote_fields: try_from_aws(x.quote_fields)?,
            record_delimiter: try_from_aws(x.record_delimiter)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_field_delimiter(try_into_aws(x.field_delimiter)?);
        y = y.set_quote_character(try_into_aws(x.quote_character)?);
        y = y.set_quote_escape_character(try_into_aws(x.quote_escape_character)?);
        y = y.set_quote_fields(try_into_aws(x.quote_fields)?);
        y = y.set_record_delimiter(try_into_aws(x.record_delimiter)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Checksum {
    type Target = Checksum;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ChecksumAlgorithm {
    type Target = ChecksumAlgorithm;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ChecksumAlgorithm::Crc32 => Self::Crc32,
            ChecksumAlgorithm::Crc32C => Self::Crc32C,
            ChecksumAlgorithm::Sha1 => Self::Sha1,
            ChecksumAlgorithm::Sha256 => Self::Sha256,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Crc32 => ChecksumAlgorithm::Crc32,
            Self::Crc32C => ChecksumAlgorithm::Crc32C,
            Self::Sha1 => ChecksumAlgorithm::Sha1,
            Self::Sha256 => ChecksumAlgorithm::Sha256,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ChecksumMode {
    type Target = ChecksumMode;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ChecksumMode::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Enabled => ChecksumMode::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::CommonPrefix {
    type Target = CommonPrefix;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            prefix: try_from_aws(x.prefix)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_prefix(try_into_aws(x.prefix)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CompleteMultipartUploadInput {
    type Target = CompleteMultipartUploadInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            multipart_upload: try_from_aws(x.multipart_upload)?,
            request_payer: try_from_aws(x.request_payer)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key: try_from_aws(x.sse_customer_key)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            upload_id: unwrap_from_aws(x.upload_id, "upload_id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_multipart_upload(try_into_aws(x.multipart_upload)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key(try_into_aws(x.sse_customer_key)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_upload_id(Some(try_into_aws(x.upload_id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::CompleteMultipartUploadOutput {
    type Target = CompleteMultipartUploadOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: try_from_aws(x.bucket)?,
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            e_tag: try_from_aws(x.e_tag)?,
            expiration: try_from_aws(x.expiration)?,
            key: try_from_aws(x.key)?,
            location: try_from_aws(x.location)?,
            request_charged: try_from_aws(x.request_charged)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(try_into_aws(x.bucket)?);
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_expiration(try_into_aws(x.expiration)?);
        y = y.set_key(try_into_aws(x.key)?);
        y = y.set_location(try_into_aws(x.location)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CompletedMultipartUpload {
    type Target = CompletedMultipartUpload;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            parts: try_from_aws(x.parts)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_parts(try_into_aws(x.parts)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CompletedPart {
    type Target = CompletedPart;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            e_tag: try_from_aws(x.e_tag)?,
            part_number: try_from_aws(x.part_number)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_part_number(Some(try_into_aws(x.part_number)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CompressionType {
    type Target = CompressionType;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            CompressionType::Bzip2 => Self::Bzip2,
            CompressionType::Gzip => Self::Gzip,
            CompressionType::None => Self::None,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Bzip2 => CompressionType::Bzip2,
            Self::Gzip => CompressionType::Gzip,
            Self::None => CompressionType::None,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::Condition {
    type Target = Condition;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            http_error_code_returned_equals: try_from_aws(x.http_error_code_returned_equals)?,
            key_prefix_equals: try_from_aws(x.key_prefix_equals)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_http_error_code_returned_equals(try_into_aws(x.http_error_code_returned_equals)?);
        y = y.set_key_prefix_equals(try_into_aws(x.key_prefix_equals)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ContinuationEvent {
    type Target = ContinuationEvent;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CopyObjectInput {
    type Target = CopyObjectInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            acl: try_from_aws(x.acl)?,
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            cache_control: try_from_aws(x.cache_control)?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_disposition: try_from_aws(x.content_disposition)?,
            content_encoding: try_from_aws(x.content_encoding)?,
            content_language: try_from_aws(x.content_language)?,
            content_type: try_from_aws(x.content_type)?,
            copy_source: unwrap_from_aws(x.copy_source, "copy_source")?,
            copy_source_if_match: try_from_aws(x.copy_source_if_match)?,
            copy_source_if_modified_since: try_from_aws(x.copy_source_if_modified_since)?,
            copy_source_if_none_match: try_from_aws(x.copy_source_if_none_match)?,
            copy_source_if_unmodified_since: try_from_aws(x.copy_source_if_unmodified_since)?,
            copy_source_sse_customer_algorithm: try_from_aws(x.copy_source_sse_customer_algorithm)?,
            copy_source_sse_customer_key: try_from_aws(x.copy_source_sse_customer_key)?,
            copy_source_sse_customer_key_md5: try_from_aws(x.copy_source_sse_customer_key_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            expected_source_bucket_owner: try_from_aws(x.expected_source_bucket_owner)?,
            expires: try_from_aws(x.expires)?,
            grant_full_control: try_from_aws(x.grant_full_control)?,
            grant_read: try_from_aws(x.grant_read)?,
            grant_read_acp: try_from_aws(x.grant_read_acp)?,
            grant_write_acp: try_from_aws(x.grant_write_acp)?,
            key: unwrap_from_aws(x.key, "key")?,
            metadata: try_from_aws(x.metadata)?,
            metadata_directive: try_from_aws(x.metadata_directive)?,
            object_lock_legal_hold_status: try_from_aws(x.object_lock_legal_hold_status)?,
            object_lock_mode: try_from_aws(x.object_lock_mode)?,
            object_lock_retain_until_date: try_from_aws(x.object_lock_retain_until_date)?,
            request_payer: try_from_aws(x.request_payer)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key: try_from_aws(x.sse_customer_key)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            ssekms_encryption_context: try_from_aws(x.ssekms_encryption_context)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
            storage_class: try_from_aws(x.storage_class)?,
            tagging: try_from_aws(x.tagging)?,
            tagging_directive: try_from_aws(x.tagging_directive)?,
            website_redirect_location: try_from_aws(x.website_redirect_location)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_acl(try_into_aws(x.acl)?);
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_cache_control(try_into_aws(x.cache_control)?);
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_disposition(try_into_aws(x.content_disposition)?);
        y = y.set_content_encoding(try_into_aws(x.content_encoding)?);
        y = y.set_content_language(try_into_aws(x.content_language)?);
        y = y.set_content_type(try_into_aws(x.content_type)?);
        y = y.set_copy_source(Some(try_into_aws(x.copy_source)?));
        y = y.set_copy_source_if_match(try_into_aws(x.copy_source_if_match)?);
        y = y.set_copy_source_if_modified_since(try_into_aws(x.copy_source_if_modified_since)?);
        y = y.set_copy_source_if_none_match(try_into_aws(x.copy_source_if_none_match)?);
        y = y.set_copy_source_if_unmodified_since(try_into_aws(x.copy_source_if_unmodified_since)?);
        y = y.set_copy_source_sse_customer_algorithm(try_into_aws(x.copy_source_sse_customer_algorithm)?);
        y = y.set_copy_source_sse_customer_key(try_into_aws(x.copy_source_sse_customer_key)?);
        y = y.set_copy_source_sse_customer_key_md5(try_into_aws(x.copy_source_sse_customer_key_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_expected_source_bucket_owner(try_into_aws(x.expected_source_bucket_owner)?);
        y = y.set_expires(try_into_aws(x.expires)?);
        y = y.set_grant_full_control(try_into_aws(x.grant_full_control)?);
        y = y.set_grant_read(try_into_aws(x.grant_read)?);
        y = y.set_grant_read_acp(try_into_aws(x.grant_read_acp)?);
        y = y.set_grant_write_acp(try_into_aws(x.grant_write_acp)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_metadata(try_into_aws(x.metadata)?);
        y = y.set_metadata_directive(try_into_aws(x.metadata_directive)?);
        y = y.set_object_lock_legal_hold_status(try_into_aws(x.object_lock_legal_hold_status)?);
        y = y.set_object_lock_mode(try_into_aws(x.object_lock_mode)?);
        y = y.set_object_lock_retain_until_date(try_into_aws(x.object_lock_retain_until_date)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key(try_into_aws(x.sse_customer_key)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_ssekms_encryption_context(try_into_aws(x.ssekms_encryption_context)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        y = y.set_tagging(try_into_aws(x.tagging)?);
        y = y.set_tagging_directive(try_into_aws(x.tagging_directive)?);
        y = y.set_website_redirect_location(try_into_aws(x.website_redirect_location)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::CopyObjectOutput {
    type Target = CopyObjectOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            copy_object_result: try_from_aws(x.copy_object_result)?,
            copy_source_version_id: try_from_aws(x.copy_source_version_id)?,
            expiration: try_from_aws(x.expiration)?,
            request_charged: try_from_aws(x.request_charged)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            ssekms_encryption_context: try_from_aws(x.ssekms_encryption_context)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_copy_object_result(try_into_aws(x.copy_object_result)?);
        y = y.set_copy_source_version_id(try_into_aws(x.copy_source_version_id)?);
        y = y.set_expiration(try_into_aws(x.expiration)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_ssekms_encryption_context(try_into_aws(x.ssekms_encryption_context)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CopyObjectResult {
    type Target = CopyObjectResult;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            e_tag: try_from_aws(x.e_tag)?,
            last_modified: try_from_aws(x.last_modified)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_last_modified(try_into_aws(x.last_modified)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CopyPartResult {
    type Target = CopyPartResult;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            e_tag: try_from_aws(x.e_tag)?,
            last_modified: try_from_aws(x.last_modified)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_last_modified(try_into_aws(x.last_modified)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CreateBucketConfiguration {
    type Target = CreateBucketConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            location_constraint: try_from_aws(x.location_constraint)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_location_constraint(try_into_aws(x.location_constraint)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CreateBucketInput {
    type Target = CreateBucketInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            acl: try_from_aws(x.acl)?,
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            create_bucket_configuration: try_from_aws(x.create_bucket_configuration)?,
            grant_full_control: try_from_aws(x.grant_full_control)?,
            grant_read: try_from_aws(x.grant_read)?,
            grant_read_acp: try_from_aws(x.grant_read_acp)?,
            grant_write: try_from_aws(x.grant_write)?,
            grant_write_acp: try_from_aws(x.grant_write_acp)?,
            object_lock_enabled_for_bucket: try_from_aws(x.object_lock_enabled_for_bucket)?,
            object_ownership: try_from_aws(x.object_ownership)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_acl(try_into_aws(x.acl)?);
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_create_bucket_configuration(try_into_aws(x.create_bucket_configuration)?);
        y = y.set_grant_full_control(try_into_aws(x.grant_full_control)?);
        y = y.set_grant_read(try_into_aws(x.grant_read)?);
        y = y.set_grant_read_acp(try_into_aws(x.grant_read_acp)?);
        y = y.set_grant_write(try_into_aws(x.grant_write)?);
        y = y.set_grant_write_acp(try_into_aws(x.grant_write_acp)?);
        y = y.set_object_lock_enabled_for_bucket(Some(try_into_aws(x.object_lock_enabled_for_bucket)?));
        y = y.set_object_ownership(try_into_aws(x.object_ownership)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::CreateBucketOutput {
    type Target = CreateBucketOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            location: try_from_aws(x.location)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_location(try_into_aws(x.location)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::CreateMultipartUploadInput {
    type Target = CreateMultipartUploadInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            acl: try_from_aws(x.acl)?,
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            cache_control: try_from_aws(x.cache_control)?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_disposition: try_from_aws(x.content_disposition)?,
            content_encoding: try_from_aws(x.content_encoding)?,
            content_language: try_from_aws(x.content_language)?,
            content_type: try_from_aws(x.content_type)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            expires: try_from_aws(x.expires)?,
            grant_full_control: try_from_aws(x.grant_full_control)?,
            grant_read: try_from_aws(x.grant_read)?,
            grant_read_acp: try_from_aws(x.grant_read_acp)?,
            grant_write_acp: try_from_aws(x.grant_write_acp)?,
            key: unwrap_from_aws(x.key, "key")?,
            metadata: try_from_aws(x.metadata)?,
            object_lock_legal_hold_status: try_from_aws(x.object_lock_legal_hold_status)?,
            object_lock_mode: try_from_aws(x.object_lock_mode)?,
            object_lock_retain_until_date: try_from_aws(x.object_lock_retain_until_date)?,
            request_payer: try_from_aws(x.request_payer)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key: try_from_aws(x.sse_customer_key)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            ssekms_encryption_context: try_from_aws(x.ssekms_encryption_context)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
            storage_class: try_from_aws(x.storage_class)?,
            tagging: try_from_aws(x.tagging)?,
            website_redirect_location: try_from_aws(x.website_redirect_location)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_acl(try_into_aws(x.acl)?);
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_cache_control(try_into_aws(x.cache_control)?);
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_disposition(try_into_aws(x.content_disposition)?);
        y = y.set_content_encoding(try_into_aws(x.content_encoding)?);
        y = y.set_content_language(try_into_aws(x.content_language)?);
        y = y.set_content_type(try_into_aws(x.content_type)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_expires(try_into_aws(x.expires)?);
        y = y.set_grant_full_control(try_into_aws(x.grant_full_control)?);
        y = y.set_grant_read(try_into_aws(x.grant_read)?);
        y = y.set_grant_read_acp(try_into_aws(x.grant_read_acp)?);
        y = y.set_grant_write_acp(try_into_aws(x.grant_write_acp)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_metadata(try_into_aws(x.metadata)?);
        y = y.set_object_lock_legal_hold_status(try_into_aws(x.object_lock_legal_hold_status)?);
        y = y.set_object_lock_mode(try_into_aws(x.object_lock_mode)?);
        y = y.set_object_lock_retain_until_date(try_into_aws(x.object_lock_retain_until_date)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key(try_into_aws(x.sse_customer_key)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_ssekms_encryption_context(try_into_aws(x.ssekms_encryption_context)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        y = y.set_tagging(try_into_aws(x.tagging)?);
        y = y.set_website_redirect_location(try_into_aws(x.website_redirect_location)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::CreateMultipartUploadOutput {
    type Target = CreateMultipartUploadOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            abort_date: try_from_aws(x.abort_date)?,
            abort_rule_id: try_from_aws(x.abort_rule_id)?,
            bucket: try_from_aws(x.bucket)?,
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            key: try_from_aws(x.key)?,
            request_charged: try_from_aws(x.request_charged)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            ssekms_encryption_context: try_from_aws(x.ssekms_encryption_context)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
            upload_id: try_from_aws(x.upload_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_abort_date(try_into_aws(x.abort_date)?);
        y = y.set_abort_rule_id(try_into_aws(x.abort_rule_id)?);
        y = y.set_bucket(try_into_aws(x.bucket)?);
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_key(try_into_aws(x.key)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_ssekms_encryption_context(try_into_aws(x.ssekms_encryption_context)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        y = y.set_upload_id(try_into_aws(x.upload_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DefaultRetention {
    type Target = DefaultRetention;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            days: try_from_aws(x.days)?,
            mode: try_from_aws(x.mode)?,
            years: try_from_aws(x.years)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_days(Some(try_into_aws(x.days)?));
        y = y.set_mode(try_into_aws(x.mode)?);
        y = y.set_years(Some(try_into_aws(x.years)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Delete {
    type Target = Delete;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            objects: unwrap_from_aws(x.objects, "objects")?,
            quiet: try_from_aws(x.quiet)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_objects(Some(try_into_aws(x.objects)?));
        y = y.set_quiet(Some(try_into_aws(x.quiet)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketAnalyticsConfigurationInput {
    type Target = DeleteBucketAnalyticsConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            id: unwrap_from_aws(x.id, "id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketAnalyticsConfigurationOutput {
    type Target = DeleteBucketAnalyticsConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketCorsInput {
    type Target = DeleteBucketCorsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketCorsOutput {
    type Target = DeleteBucketCorsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketEncryptionInput {
    type Target = DeleteBucketEncryptionInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketEncryptionOutput {
    type Target = DeleteBucketEncryptionOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketInput {
    type Target = DeleteBucketInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketIntelligentTieringConfigurationInput {
    type Target = DeleteBucketIntelligentTieringConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            id: unwrap_from_aws(x.id, "id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_id(Some(try_into_aws(x.id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketIntelligentTieringConfigurationOutput {
    type Target = DeleteBucketIntelligentTieringConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketInventoryConfigurationInput {
    type Target = DeleteBucketInventoryConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            id: unwrap_from_aws(x.id, "id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketInventoryConfigurationOutput {
    type Target = DeleteBucketInventoryConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketLifecycleInput {
    type Target = DeleteBucketLifecycleInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketLifecycleOutput {
    type Target = DeleteBucketLifecycleOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketMetricsConfigurationInput {
    type Target = DeleteBucketMetricsConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            id: unwrap_from_aws(x.id, "id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketMetricsConfigurationOutput {
    type Target = DeleteBucketMetricsConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketOutput {
    type Target = DeleteBucketOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketOwnershipControlsInput {
    type Target = DeleteBucketOwnershipControlsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketOwnershipControlsOutput {
    type Target = DeleteBucketOwnershipControlsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketPolicyInput {
    type Target = DeleteBucketPolicyInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketPolicyOutput {
    type Target = DeleteBucketPolicyOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketReplicationInput {
    type Target = DeleteBucketReplicationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketReplicationOutput {
    type Target = DeleteBucketReplicationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketTaggingInput {
    type Target = DeleteBucketTaggingInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketTaggingOutput {
    type Target = DeleteBucketTaggingOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteBucketWebsiteInput {
    type Target = DeleteBucketWebsiteInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteBucketWebsiteOutput {
    type Target = DeleteBucketWebsiteOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteMarkerEntry {
    type Target = DeleteMarkerEntry;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            is_latest: try_from_aws(x.is_latest)?,
            key: try_from_aws(x.key)?,
            last_modified: try_from_aws(x.last_modified)?,
            owner: try_from_aws(x.owner)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_is_latest(Some(try_into_aws(x.is_latest)?));
        y = y.set_key(try_into_aws(x.key)?);
        y = y.set_last_modified(try_into_aws(x.last_modified)?);
        y = y.set_owner(try_into_aws(x.owner)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteMarkerReplication {
    type Target = DeleteMarkerReplication;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            status: try_from_aws(x.status)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_status(try_into_aws(x.status)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteMarkerReplicationStatus {
    type Target = DeleteMarkerReplicationStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            DeleteMarkerReplicationStatus::Disabled => Self::Disabled,
            DeleteMarkerReplicationStatus::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Disabled => DeleteMarkerReplicationStatus::Disabled,
            Self::Enabled => DeleteMarkerReplicationStatus::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::DeleteObjectInput {
    type Target = DeleteObjectInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            bypass_governance_retention: try_from_aws(x.bypass_governance_retention)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            mfa: try_from_aws(x.mfa)?,
            request_payer: try_from_aws(x.request_payer)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_bypass_governance_retention(Some(try_into_aws(x.bypass_governance_retention)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_mfa(try_into_aws(x.mfa)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteObjectOutput {
    type Target = DeleteObjectOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            delete_marker: try_from_aws(x.delete_marker)?,
            request_charged: try_from_aws(x.request_charged)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_delete_marker(Some(try_into_aws(x.delete_marker)?));
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteObjectTaggingInput {
    type Target = DeleteObjectTaggingInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteObjectTaggingOutput {
    type Target = DeleteObjectTaggingOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeleteObjectsInput {
    type Target = DeleteObjectsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            bypass_governance_retention: try_from_aws(x.bypass_governance_retention)?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            delete: unwrap_from_aws(x.delete, "delete")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            mfa: try_from_aws(x.mfa)?,
            request_payer: try_from_aws(x.request_payer)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_bypass_governance_retention(Some(try_into_aws(x.bypass_governance_retention)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_delete(Some(try_into_aws(x.delete)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_mfa(try_into_aws(x.mfa)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeleteObjectsOutput {
    type Target = DeleteObjectsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            deleted: try_from_aws(x.deleted)?,
            errors: try_from_aws(x.errors)?,
            request_charged: try_from_aws(x.request_charged)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_deleted(try_into_aws(x.deleted)?);
        y = y.set_errors(try_into_aws(x.errors)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeletePublicAccessBlockInput {
    type Target = DeletePublicAccessBlockInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::DeletePublicAccessBlockOutput {
    type Target = DeletePublicAccessBlockOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::DeletedObject {
    type Target = DeletedObject;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            delete_marker: try_from_aws(x.delete_marker)?,
            delete_marker_version_id: try_from_aws(x.delete_marker_version_id)?,
            key: try_from_aws(x.key)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_delete_marker(Some(try_into_aws(x.delete_marker)?));
        y = y.set_delete_marker_version_id(try_into_aws(x.delete_marker_version_id)?);
        y = y.set_key(try_into_aws(x.key)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Destination {
    type Target = Destination;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            access_control_translation: try_from_aws(x.access_control_translation)?,
            account: try_from_aws(x.account)?,
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            encryption_configuration: try_from_aws(x.encryption_configuration)?,
            metrics: try_from_aws(x.metrics)?,
            replication_time: try_from_aws(x.replication_time)?,
            storage_class: try_from_aws(x.storage_class)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_access_control_translation(try_into_aws(x.access_control_translation)?);
        y = y.set_account(try_into_aws(x.account)?);
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_encryption_configuration(try_into_aws(x.encryption_configuration)?);
        y = y.set_metrics(try_into_aws(x.metrics)?);
        y = y.set_replication_time(try_into_aws(x.replication_time)?);
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::EncodingType {
    type Target = EncodingType;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            EncodingType::Url => Self::Url,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Url => EncodingType::Url,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::Encryption {
    type Target = Encryption;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            encryption_type: unwrap_from_aws(x.encryption_type, "encryption_type")?,
            kms_context: try_from_aws(x.kms_context)?,
            kms_key_id: try_from_aws(x.kms_key_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_encryption_type(Some(try_into_aws(x.encryption_type)?));
        y = y.set_kms_context(try_into_aws(x.kms_context)?);
        y = y.set_kms_key_id(try_into_aws(x.kms_key_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::EncryptionConfiguration {
    type Target = EncryptionConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            replica_kms_key_id: try_from_aws(x.replica_kms_key_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_replica_kms_key_id(try_into_aws(x.replica_kms_key_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::EndEvent {
    type Target = EndEvent;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Error {
    type Target = Error;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            code: try_from_aws(x.code)?,
            key: try_from_aws(x.key)?,
            message: try_from_aws(x.message)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_code(try_into_aws(x.code)?);
        y = y.set_key(try_into_aws(x.key)?);
        y = y.set_message(try_into_aws(x.message)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ErrorDocument {
    type Target = ErrorDocument;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            key: unwrap_from_aws(x.key, "key")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_key(Some(try_into_aws(x.key)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::EventBridgeConfiguration {
    type Target = EventBridgeConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ExistingObjectReplication {
    type Target = ExistingObjectReplication;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            status: unwrap_from_aws(x.status, "status")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_status(Some(try_into_aws(x.status)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ExistingObjectReplicationStatus {
    type Target = ExistingObjectReplicationStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ExistingObjectReplicationStatus::Disabled => Self::Disabled,
            ExistingObjectReplicationStatus::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Disabled => ExistingObjectReplicationStatus::Disabled,
            Self::Enabled => ExistingObjectReplicationStatus::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ExpirationStatus {
    type Target = ExpirationStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ExpirationStatus::Disabled => Self::Disabled,
            ExpirationStatus::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Disabled => ExpirationStatus::Disabled,
            Self::Enabled => ExpirationStatus::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ExpressionType {
    type Target = ExpressionType;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ExpressionType::Sql => Self::Sql,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Sql => ExpressionType::Sql,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::FileHeaderInfo {
    type Target = FileHeaderInfo;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            FileHeaderInfo::Ignore => Self::Ignore,
            FileHeaderInfo::None => Self::None,
            FileHeaderInfo::Use => Self::Use,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Ignore => FileHeaderInfo::Ignore,
            Self::None => FileHeaderInfo::None,
            Self::Use => FileHeaderInfo::Use,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::FilterRule {
    type Target = FilterRule;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            name: try_from_aws(x.name)?,
            value: try_from_aws(x.value)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_name(try_into_aws(x.name)?);
        y = y.set_value(try_into_aws(x.value)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::FilterRuleName {
    type Target = FilterRuleName;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            FilterRuleName::Prefix => Self::Prefix,
            FilterRuleName::Suffix => Self::Suffix,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Prefix => FilterRuleName::Prefix,
            Self::Suffix => FilterRuleName::Suffix,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::GetBucketAccelerateConfigurationInput {
    type Target = GetBucketAccelerateConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketAccelerateConfigurationOutput {
    type Target = GetBucketAccelerateConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            status: try_from_aws(x.status)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_status(try_into_aws(x.status)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketAclInput {
    type Target = GetBucketAclInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketAclOutput {
    type Target = GetBucketAclOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            grants: try_from_aws(x.grants)?,
            owner: try_from_aws(x.owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_grants(try_into_aws(x.grants)?);
        y = y.set_owner(try_into_aws(x.owner)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketAnalyticsConfigurationInput {
    type Target = GetBucketAnalyticsConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            id: unwrap_from_aws(x.id, "id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketAnalyticsConfigurationOutput {
    type Target = GetBucketAnalyticsConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            analytics_configuration: try_from_aws(x.analytics_configuration)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_analytics_configuration(try_into_aws(x.analytics_configuration)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketCorsInput {
    type Target = GetBucketCorsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketCorsOutput {
    type Target = GetBucketCorsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            cors_rules: try_from_aws(x.cors_rules)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_cors_rules(try_into_aws(x.cors_rules)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketEncryptionInput {
    type Target = GetBucketEncryptionInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketEncryptionOutput {
    type Target = GetBucketEncryptionOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            server_side_encryption_configuration: try_from_aws(x.server_side_encryption_configuration)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_server_side_encryption_configuration(try_into_aws(x.server_side_encryption_configuration)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketIntelligentTieringConfigurationInput {
    type Target = GetBucketIntelligentTieringConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            id: unwrap_from_aws(x.id, "id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_id(Some(try_into_aws(x.id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketIntelligentTieringConfigurationOutput {
    type Target = GetBucketIntelligentTieringConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            intelligent_tiering_configuration: try_from_aws(x.intelligent_tiering_configuration)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_intelligent_tiering_configuration(try_into_aws(x.intelligent_tiering_configuration)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketInventoryConfigurationInput {
    type Target = GetBucketInventoryConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            id: unwrap_from_aws(x.id, "id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketInventoryConfigurationOutput {
    type Target = GetBucketInventoryConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            inventory_configuration: try_from_aws(x.inventory_configuration)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_inventory_configuration(try_into_aws(x.inventory_configuration)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketLifecycleConfigurationInput {
    type Target = GetBucketLifecycleConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketLifecycleConfigurationOutput {
    type Target = GetBucketLifecycleConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            rules: try_from_aws(x.rules)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_rules(try_into_aws(x.rules)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketLocationInput {
    type Target = GetBucketLocationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketLocationOutput {
    type Target = GetBucketLocationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            location_constraint: try_from_aws(x.location_constraint)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_location_constraint(try_into_aws(x.location_constraint)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketLoggingInput {
    type Target = GetBucketLoggingInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketLoggingOutput {
    type Target = GetBucketLoggingOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            logging_enabled: try_from_aws(x.logging_enabled)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_logging_enabled(try_into_aws(x.logging_enabled)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketMetricsConfigurationInput {
    type Target = GetBucketMetricsConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            id: unwrap_from_aws(x.id, "id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketMetricsConfigurationOutput {
    type Target = GetBucketMetricsConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            metrics_configuration: try_from_aws(x.metrics_configuration)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_metrics_configuration(try_into_aws(x.metrics_configuration)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketNotificationConfigurationInput {
    type Target = GetBucketNotificationConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketNotificationConfigurationOutput {
    type Target = GetBucketNotificationConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            event_bridge_configuration: try_from_aws(x.event_bridge_configuration)?,
            lambda_function_configurations: try_from_aws(x.lambda_function_configurations)?,
            queue_configurations: try_from_aws(x.queue_configurations)?,
            topic_configurations: try_from_aws(x.topic_configurations)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_event_bridge_configuration(try_into_aws(x.event_bridge_configuration)?);
        y = y.set_lambda_function_configurations(try_into_aws(x.lambda_function_configurations)?);
        y = y.set_queue_configurations(try_into_aws(x.queue_configurations)?);
        y = y.set_topic_configurations(try_into_aws(x.topic_configurations)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketOwnershipControlsInput {
    type Target = GetBucketOwnershipControlsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketOwnershipControlsOutput {
    type Target = GetBucketOwnershipControlsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            ownership_controls: try_from_aws(x.ownership_controls)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_ownership_controls(try_into_aws(x.ownership_controls)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketPolicyInput {
    type Target = GetBucketPolicyInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketPolicyOutput {
    type Target = GetBucketPolicyOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            policy: try_from_aws(x.policy)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_policy(try_into_aws(x.policy)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketPolicyStatusInput {
    type Target = GetBucketPolicyStatusInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketPolicyStatusOutput {
    type Target = GetBucketPolicyStatusOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            policy_status: try_from_aws(x.policy_status)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_policy_status(try_into_aws(x.policy_status)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketReplicationInput {
    type Target = GetBucketReplicationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketReplicationOutput {
    type Target = GetBucketReplicationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            replication_configuration: try_from_aws(x.replication_configuration)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_replication_configuration(try_into_aws(x.replication_configuration)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketRequestPaymentInput {
    type Target = GetBucketRequestPaymentInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketRequestPaymentOutput {
    type Target = GetBucketRequestPaymentOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            payer: try_from_aws(x.payer)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_payer(try_into_aws(x.payer)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketTaggingInput {
    type Target = GetBucketTaggingInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketTaggingOutput {
    type Target = GetBucketTaggingOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            tag_set: unwrap_from_aws(x.tag_set, "tag_set")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_tag_set(Some(try_into_aws(x.tag_set)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketVersioningInput {
    type Target = GetBucketVersioningInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketVersioningOutput {
    type Target = GetBucketVersioningOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            mfa_delete: try_from_aws(x.mfa_delete)?,
            status: try_from_aws(x.status)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_mfa_delete(try_into_aws(x.mfa_delete)?);
        y = y.set_status(try_into_aws(x.status)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetBucketWebsiteInput {
    type Target = GetBucketWebsiteInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetBucketWebsiteOutput {
    type Target = GetBucketWebsiteOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            error_document: try_from_aws(x.error_document)?,
            index_document: try_from_aws(x.index_document)?,
            redirect_all_requests_to: try_from_aws(x.redirect_all_requests_to)?,
            routing_rules: try_from_aws(x.routing_rules)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_error_document(try_into_aws(x.error_document)?);
        y = y.set_index_document(try_into_aws(x.index_document)?);
        y = y.set_redirect_all_requests_to(try_into_aws(x.redirect_all_requests_to)?);
        y = y.set_routing_rules(try_into_aws(x.routing_rules)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetObjectAclInput {
    type Target = GetObjectAclInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            request_payer: try_from_aws(x.request_payer)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetObjectAclOutput {
    type Target = GetObjectAclOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            grants: try_from_aws(x.grants)?,
            owner: try_from_aws(x.owner)?,
            request_charged: try_from_aws(x.request_charged)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_grants(try_into_aws(x.grants)?);
        y = y.set_owner(try_into_aws(x.owner)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetObjectAttributesInput {
    type Target = GetObjectAttributesInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            max_parts: try_from_aws(x.max_parts)?,
            object_attributes: unwrap_from_aws(x.object_attributes, "object_attributes")?,
            part_number_marker: try_from_aws(x.part_number_marker)?,
            request_payer: try_from_aws(x.request_payer)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key: try_from_aws(x.sse_customer_key)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_max_parts(Some(try_into_aws(x.max_parts)?));
        y = y.set_object_attributes(Some(try_into_aws(x.object_attributes)?));
        y = y.set_part_number_marker(try_into_aws(x.part_number_marker)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key(try_into_aws(x.sse_customer_key)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetObjectAttributesOutput {
    type Target = GetObjectAttributesOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            checksum: try_from_aws(x.checksum)?,
            delete_marker: try_from_aws(x.delete_marker)?,
            e_tag: try_from_aws(x.e_tag)?,
            last_modified: try_from_aws(x.last_modified)?,
            object_parts: try_from_aws(x.object_parts)?,
            object_size: try_from_aws(x.object_size)?,
            request_charged: try_from_aws(x.request_charged)?,
            storage_class: try_from_aws(x.storage_class)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_checksum(try_into_aws(x.checksum)?);
        y = y.set_delete_marker(Some(try_into_aws(x.delete_marker)?));
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_last_modified(try_into_aws(x.last_modified)?);
        y = y.set_object_parts(try_into_aws(x.object_parts)?);
        y = y.set_object_size(Some(try_into_aws(x.object_size)?));
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetObjectAttributesParts {
    type Target = GetObjectAttributesParts;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            is_truncated: try_from_aws(x.is_truncated)?,
            max_parts: try_from_aws(x.max_parts)?,
            next_part_number_marker: try_from_aws(x.next_part_number_marker)?,
            part_number_marker: try_from_aws(x.part_number_marker)?,
            parts: try_from_aws(x.parts)?,
            total_parts_count: try_from_aws(x.total_parts_count)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_is_truncated(Some(try_into_aws(x.is_truncated)?));
        y = y.set_max_parts(Some(try_into_aws(x.max_parts)?));
        y = y.set_next_part_number_marker(try_into_aws(x.next_part_number_marker)?);
        y = y.set_part_number_marker(try_into_aws(x.part_number_marker)?);
        y = y.set_parts(try_into_aws(x.parts)?);
        y = y.set_total_parts_count(Some(try_into_aws(x.total_parts_count)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetObjectInput {
    type Target = GetObjectInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_mode: try_from_aws(x.checksum_mode)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            if_match: try_from_aws(x.if_match)?,
            if_modified_since: try_from_aws(x.if_modified_since)?,
            if_none_match: try_from_aws(x.if_none_match)?,
            if_unmodified_since: try_from_aws(x.if_unmodified_since)?,
            key: unwrap_from_aws(x.key, "key")?,
            part_number: try_from_aws(x.part_number)?,
            range: try_from_aws(x.range)?,
            request_payer: try_from_aws(x.request_payer)?,
            response_cache_control: try_from_aws(x.response_cache_control)?,
            response_content_disposition: try_from_aws(x.response_content_disposition)?,
            response_content_encoding: try_from_aws(x.response_content_encoding)?,
            response_content_language: try_from_aws(x.response_content_language)?,
            response_content_type: try_from_aws(x.response_content_type)?,
            response_expires: try_from_aws(x.response_expires)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key: try_from_aws(x.sse_customer_key)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_mode(try_into_aws(x.checksum_mode)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_if_match(try_into_aws(x.if_match)?);
        y = y.set_if_modified_since(try_into_aws(x.if_modified_since)?);
        y = y.set_if_none_match(try_into_aws(x.if_none_match)?);
        y = y.set_if_unmodified_since(try_into_aws(x.if_unmodified_since)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_part_number(Some(try_into_aws(x.part_number)?));
        y = y.set_range(try_into_aws(x.range)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_response_cache_control(try_into_aws(x.response_cache_control)?);
        y = y.set_response_content_disposition(try_into_aws(x.response_content_disposition)?);
        y = y.set_response_content_encoding(try_into_aws(x.response_content_encoding)?);
        y = y.set_response_content_language(try_into_aws(x.response_content_language)?);
        y = y.set_response_content_type(try_into_aws(x.response_content_type)?);
        y = y.set_response_expires(try_into_aws(x.response_expires)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key(try_into_aws(x.sse_customer_key)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetObjectLegalHoldInput {
    type Target = GetObjectLegalHoldInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            request_payer: try_from_aws(x.request_payer)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetObjectLegalHoldOutput {
    type Target = GetObjectLegalHoldOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            legal_hold: try_from_aws(x.legal_hold)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_legal_hold(try_into_aws(x.legal_hold)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetObjectLockConfigurationInput {
    type Target = GetObjectLockConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetObjectLockConfigurationOutput {
    type Target = GetObjectLockConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            object_lock_configuration: try_from_aws(x.object_lock_configuration)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_object_lock_configuration(try_into_aws(x.object_lock_configuration)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetObjectOutput {
    type Target = GetObjectOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            accept_ranges: try_from_aws(x.accept_ranges)?,
            body: stream_from_aws(x.body),
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            cache_control: try_from_aws(x.cache_control)?,
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            content_disposition: try_from_aws(x.content_disposition)?,
            content_encoding: try_from_aws(x.content_encoding)?,
            content_language: try_from_aws(x.content_language)?,
            content_length: try_from_aws(x.content_length)?,
            content_range: try_from_aws(x.content_range)?,
            content_type: try_from_aws(x.content_type)?,
            delete_marker: try_from_aws(x.delete_marker)?,
            e_tag: try_from_aws(x.e_tag)?,
            expiration: try_from_aws(x.expiration)?,
            expires: try_from_aws(x.expires)?,
            last_modified: try_from_aws(x.last_modified)?,
            metadata: try_from_aws(x.metadata)?,
            missing_meta: try_from_aws(x.missing_meta)?,
            object_lock_legal_hold_status: try_from_aws(x.object_lock_legal_hold_status)?,
            object_lock_mode: try_from_aws(x.object_lock_mode)?,
            object_lock_retain_until_date: try_from_aws(x.object_lock_retain_until_date)?,
            parts_count: try_from_aws(x.parts_count)?,
            replication_status: try_from_aws(x.replication_status)?,
            request_charged: try_from_aws(x.request_charged)?,
            restore: try_from_aws(x.restore)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
            storage_class: try_from_aws(x.storage_class)?,
            tag_count: try_from_aws(x.tag_count)?,
            version_id: try_from_aws(x.version_id)?,
            website_redirect_location: try_from_aws(x.website_redirect_location)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_accept_ranges(try_into_aws(x.accept_ranges)?);
        y = y.set_body(try_into_aws(x.body)?);
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_cache_control(try_into_aws(x.cache_control)?);
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_content_disposition(try_into_aws(x.content_disposition)?);
        y = y.set_content_encoding(try_into_aws(x.content_encoding)?);
        y = y.set_content_language(try_into_aws(x.content_language)?);
        y = y.set_content_length(Some(try_into_aws(x.content_length)?));
        y = y.set_content_range(try_into_aws(x.content_range)?);
        y = y.set_content_type(try_into_aws(x.content_type)?);
        y = y.set_delete_marker(Some(try_into_aws(x.delete_marker)?));
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_expiration(try_into_aws(x.expiration)?);
        y = y.set_expires(try_into_aws(x.expires)?);
        y = y.set_last_modified(try_into_aws(x.last_modified)?);
        y = y.set_metadata(try_into_aws(x.metadata)?);
        y = y.set_missing_meta(Some(try_into_aws(x.missing_meta)?));
        y = y.set_object_lock_legal_hold_status(try_into_aws(x.object_lock_legal_hold_status)?);
        y = y.set_object_lock_mode(try_into_aws(x.object_lock_mode)?);
        y = y.set_object_lock_retain_until_date(try_into_aws(x.object_lock_retain_until_date)?);
        y = y.set_parts_count(Some(try_into_aws(x.parts_count)?));
        y = y.set_replication_status(try_into_aws(x.replication_status)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_restore(try_into_aws(x.restore)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        y = y.set_tag_count(Some(try_into_aws(x.tag_count)?));
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y = y.set_website_redirect_location(try_into_aws(x.website_redirect_location)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetObjectRetentionInput {
    type Target = GetObjectRetentionInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            request_payer: try_from_aws(x.request_payer)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetObjectRetentionOutput {
    type Target = GetObjectRetentionOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            retention: try_from_aws(x.retention)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_retention(try_into_aws(x.retention)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetObjectTaggingInput {
    type Target = GetObjectTaggingInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            request_payer: try_from_aws(x.request_payer)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetObjectTaggingOutput {
    type Target = GetObjectTaggingOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            tag_set: unwrap_from_aws(x.tag_set, "tag_set")?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_tag_set(Some(try_into_aws(x.tag_set)?));
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetObjectTorrentInput {
    type Target = GetObjectTorrentInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            request_payer: try_from_aws(x.request_payer)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetObjectTorrentOutput {
    type Target = GetObjectTorrentOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            body: stream_from_aws(x.body),
            request_charged: try_from_aws(x.request_charged)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_body(try_into_aws(x.body)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GetPublicAccessBlockInput {
    type Target = GetPublicAccessBlockInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::GetPublicAccessBlockOutput {
    type Target = GetPublicAccessBlockOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            public_access_block_configuration: try_from_aws(x.public_access_block_configuration)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_public_access_block_configuration(try_into_aws(x.public_access_block_configuration)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::GlacierJobParameters {
    type Target = GlacierJobParameters;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            tier: unwrap_from_aws(x.tier, "tier")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_tier(Some(try_into_aws(x.tier)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Grant {
    type Target = Grant;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            grantee: try_from_aws(x.grantee)?,
            permission: try_from_aws(x.permission)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_grantee(try_into_aws(x.grantee)?);
        y = y.set_permission(try_into_aws(x.permission)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Grantee {
    type Target = Grantee;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            display_name: try_from_aws(x.display_name)?,
            email_address: try_from_aws(x.email_address)?,
            id: try_from_aws(x.id)?,
            type_: unwrap_from_aws(x.r#type, "type_")?,
            uri: try_from_aws(x.uri)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_display_name(try_into_aws(x.display_name)?);
        y = y.set_email_address(try_into_aws(x.email_address)?);
        y = y.set_id(try_into_aws(x.id)?);
        y = y.set_type(Some(try_into_aws(x.type_)?));
        y = y.set_uri(try_into_aws(x.uri)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::HeadBucketInput {
    type Target = HeadBucketInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::HeadBucketOutput {
    type Target = HeadBucketOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::HeadObjectInput {
    type Target = HeadObjectInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_mode: try_from_aws(x.checksum_mode)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            if_match: try_from_aws(x.if_match)?,
            if_modified_since: try_from_aws(x.if_modified_since)?,
            if_none_match: try_from_aws(x.if_none_match)?,
            if_unmodified_since: try_from_aws(x.if_unmodified_since)?,
            key: unwrap_from_aws(x.key, "key")?,
            part_number: try_from_aws(x.part_number)?,
            range: try_from_aws(x.range)?,
            request_payer: try_from_aws(x.request_payer)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key: try_from_aws(x.sse_customer_key)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_mode(try_into_aws(x.checksum_mode)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_if_match(try_into_aws(x.if_match)?);
        y = y.set_if_modified_since(try_into_aws(x.if_modified_since)?);
        y = y.set_if_none_match(try_into_aws(x.if_none_match)?);
        y = y.set_if_unmodified_since(try_into_aws(x.if_unmodified_since)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_part_number(Some(try_into_aws(x.part_number)?));
        y = y.set_range(try_into_aws(x.range)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key(try_into_aws(x.sse_customer_key)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::HeadObjectOutput {
    type Target = HeadObjectOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            accept_ranges: try_from_aws(x.accept_ranges)?,
            archive_status: try_from_aws(x.archive_status)?,
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            cache_control: try_from_aws(x.cache_control)?,
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            content_disposition: try_from_aws(x.content_disposition)?,
            content_encoding: try_from_aws(x.content_encoding)?,
            content_language: try_from_aws(x.content_language)?,
            content_length: try_from_aws(x.content_length)?,
            content_type: try_from_aws(x.content_type)?,
            delete_marker: try_from_aws(x.delete_marker)?,
            e_tag: try_from_aws(x.e_tag)?,
            expiration: try_from_aws(x.expiration)?,
            expires: try_from_aws(x.expires)?,
            last_modified: try_from_aws(x.last_modified)?,
            metadata: try_from_aws(x.metadata)?,
            missing_meta: try_from_aws(x.missing_meta)?,
            object_lock_legal_hold_status: try_from_aws(x.object_lock_legal_hold_status)?,
            object_lock_mode: try_from_aws(x.object_lock_mode)?,
            object_lock_retain_until_date: try_from_aws(x.object_lock_retain_until_date)?,
            parts_count: try_from_aws(x.parts_count)?,
            replication_status: try_from_aws(x.replication_status)?,
            request_charged: try_from_aws(x.request_charged)?,
            restore: try_from_aws(x.restore)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
            storage_class: try_from_aws(x.storage_class)?,
            version_id: try_from_aws(x.version_id)?,
            website_redirect_location: try_from_aws(x.website_redirect_location)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_accept_ranges(try_into_aws(x.accept_ranges)?);
        y = y.set_archive_status(try_into_aws(x.archive_status)?);
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_cache_control(try_into_aws(x.cache_control)?);
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_content_disposition(try_into_aws(x.content_disposition)?);
        y = y.set_content_encoding(try_into_aws(x.content_encoding)?);
        y = y.set_content_language(try_into_aws(x.content_language)?);
        y = y.set_content_length(Some(try_into_aws(x.content_length)?));
        y = y.set_content_type(try_into_aws(x.content_type)?);
        y = y.set_delete_marker(Some(try_into_aws(x.delete_marker)?));
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_expiration(try_into_aws(x.expiration)?);
        y = y.set_expires(try_into_aws(x.expires)?);
        y = y.set_last_modified(try_into_aws(x.last_modified)?);
        y = y.set_metadata(try_into_aws(x.metadata)?);
        y = y.set_missing_meta(Some(try_into_aws(x.missing_meta)?));
        y = y.set_object_lock_legal_hold_status(try_into_aws(x.object_lock_legal_hold_status)?);
        y = y.set_object_lock_mode(try_into_aws(x.object_lock_mode)?);
        y = y.set_object_lock_retain_until_date(try_into_aws(x.object_lock_retain_until_date)?);
        y = y.set_parts_count(Some(try_into_aws(x.parts_count)?));
        y = y.set_replication_status(try_into_aws(x.replication_status)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_restore(try_into_aws(x.restore)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y = y.set_website_redirect_location(try_into_aws(x.website_redirect_location)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::IndexDocument {
    type Target = IndexDocument;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            suffix: unwrap_from_aws(x.suffix, "suffix")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_suffix(Some(try_into_aws(x.suffix)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Initiator {
    type Target = Initiator;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            display_name: try_from_aws(x.display_name)?,
            id: try_from_aws(x.id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_display_name(try_into_aws(x.display_name)?);
        y = y.set_id(try_into_aws(x.id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::InputSerialization {
    type Target = InputSerialization;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            csv: try_from_aws(x.csv)?,
            compression_type: try_from_aws(x.compression_type)?,
            json: try_from_aws(x.json)?,
            parquet: try_from_aws(x.parquet)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_csv(try_into_aws(x.csv)?);
        y = y.set_compression_type(try_into_aws(x.compression_type)?);
        y = y.set_json(try_into_aws(x.json)?);
        y = y.set_parquet(try_into_aws(x.parquet)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::IntelligentTieringAccessTier {
    type Target = IntelligentTieringAccessTier;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            IntelligentTieringAccessTier::ArchiveAccess => Self::ArchiveAccess,
            IntelligentTieringAccessTier::DeepArchiveAccess => Self::DeepArchiveAccess,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::ArchiveAccess => IntelligentTieringAccessTier::ArchiveAccess,
            Self::DeepArchiveAccess => IntelligentTieringAccessTier::DeepArchiveAccess,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::IntelligentTieringAndOperator {
    type Target = IntelligentTieringAndOperator;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            prefix: try_from_aws(x.prefix)?,
            tags: try_from_aws(x.tags)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_tags(try_into_aws(x.tags)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::IntelligentTieringConfiguration {
    type Target = IntelligentTieringConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            filter: try_from_aws(x.filter)?,
            id: unwrap_from_aws(x.id, "id")?,
            status: unwrap_from_aws(x.status, "status")?,
            tierings: unwrap_from_aws(x.tierings, "tierings")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_filter(try_into_aws(x.filter)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y = y.set_status(Some(try_into_aws(x.status)?));
        y = y.set_tierings(Some(try_into_aws(x.tierings)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::IntelligentTieringFilter {
    type Target = IntelligentTieringFilter;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            and: try_from_aws(x.and)?,
            prefix: try_from_aws(x.prefix)?,
            tag: try_from_aws(x.tag)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_and(try_into_aws(x.and)?);
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_tag(try_into_aws(x.tag)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::IntelligentTieringStatus {
    type Target = IntelligentTieringStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            IntelligentTieringStatus::Disabled => Self::Disabled,
            IntelligentTieringStatus::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Disabled => IntelligentTieringStatus::Disabled,
            Self::Enabled => IntelligentTieringStatus::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::InvalidObjectState {
    type Target = InvalidObjectState;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            access_tier: try_from_aws(x.access_tier)?,
            storage_class: try_from_aws(x.storage_class)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_access_tier(try_into_aws(x.access_tier)?);
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::InventoryConfiguration {
    type Target = InventoryConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            destination: unwrap_from_aws(x.destination, "destination")?,
            filter: try_from_aws(x.filter)?,
            id: unwrap_from_aws(x.id, "id")?,
            included_object_versions: unwrap_from_aws(x.included_object_versions, "included_object_versions")?,
            is_enabled: try_from_aws(x.is_enabled)?,
            optional_fields: try_from_aws(x.optional_fields)?,
            schedule: unwrap_from_aws(x.schedule, "schedule")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_destination(Some(try_into_aws(x.destination)?));
        y = y.set_filter(try_into_aws(x.filter)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y = y.set_included_object_versions(Some(try_into_aws(x.included_object_versions)?));
        y = y.set_is_enabled(Some(try_into_aws(x.is_enabled)?));
        y = y.set_optional_fields(try_into_aws(x.optional_fields)?);
        y = y.set_schedule(Some(try_into_aws(x.schedule)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::InventoryDestination {
    type Target = InventoryDestination;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            s3_bucket_destination: unwrap_from_aws(x.s3_bucket_destination, "s3_bucket_destination")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_s3_bucket_destination(Some(try_into_aws(x.s3_bucket_destination)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::InventoryEncryption {
    type Target = InventoryEncryption;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            ssekms: try_from_aws(x.ssekms)?,
            sses3: try_from_aws(x.sses3)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_ssekms(try_into_aws(x.ssekms)?);
        y = y.set_sses3(try_into_aws(x.sses3)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::InventoryFilter {
    type Target = InventoryFilter;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            prefix: unwrap_from_aws(x.prefix, "prefix")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_prefix(Some(try_into_aws(x.prefix)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::InventoryFormat {
    type Target = InventoryFormat;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            InventoryFormat::Csv => Self::Csv,
            InventoryFormat::Orc => Self::Orc,
            InventoryFormat::Parquet => Self::Parquet,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Csv => InventoryFormat::Csv,
            Self::Orc => InventoryFormat::Orc,
            Self::Parquet => InventoryFormat::Parquet,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::InventoryFrequency {
    type Target = InventoryFrequency;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            InventoryFrequency::Daily => Self::Daily,
            InventoryFrequency::Weekly => Self::Weekly,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Daily => InventoryFrequency::Daily,
            Self::Weekly => InventoryFrequency::Weekly,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::InventoryIncludedObjectVersions {
    type Target = InventoryIncludedObjectVersions;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            InventoryIncludedObjectVersions::All => Self::All,
            InventoryIncludedObjectVersions::Current => Self::Current,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::All => InventoryIncludedObjectVersions::All,
            Self::Current => InventoryIncludedObjectVersions::Current,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::InventoryOptionalField {
    type Target = InventoryOptionalField;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            InventoryOptionalField::BucketKeyStatus => Self::BucketKeyStatus,
            InventoryOptionalField::ChecksumAlgorithm => Self::ChecksumAlgorithm,
            InventoryOptionalField::ETag => Self::ETag,
            InventoryOptionalField::EncryptionStatus => Self::EncryptionStatus,
            InventoryOptionalField::IntelligentTieringAccessTier => Self::IntelligentTieringAccessTier,
            InventoryOptionalField::IsMultipartUploaded => Self::IsMultipartUploaded,
            InventoryOptionalField::LastModifiedDate => Self::LastModifiedDate,
            InventoryOptionalField::ObjectLockLegalHoldStatus => Self::ObjectLockLegalHoldStatus,
            InventoryOptionalField::ObjectLockMode => Self::ObjectLockMode,
            InventoryOptionalField::ObjectLockRetainUntilDate => Self::ObjectLockRetainUntilDate,
            InventoryOptionalField::ReplicationStatus => Self::ReplicationStatus,
            InventoryOptionalField::Size => Self::Size,
            InventoryOptionalField::StorageClass => Self::StorageClass,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::BucketKeyStatus => InventoryOptionalField::BucketKeyStatus,
            Self::ChecksumAlgorithm => InventoryOptionalField::ChecksumAlgorithm,
            Self::ETag => InventoryOptionalField::ETag,
            Self::EncryptionStatus => InventoryOptionalField::EncryptionStatus,
            Self::IntelligentTieringAccessTier => InventoryOptionalField::IntelligentTieringAccessTier,
            Self::IsMultipartUploaded => InventoryOptionalField::IsMultipartUploaded,
            Self::LastModifiedDate => InventoryOptionalField::LastModifiedDate,
            Self::ObjectLockLegalHoldStatus => InventoryOptionalField::ObjectLockLegalHoldStatus,
            Self::ObjectLockMode => InventoryOptionalField::ObjectLockMode,
            Self::ObjectLockRetainUntilDate => InventoryOptionalField::ObjectLockRetainUntilDate,
            Self::ReplicationStatus => InventoryOptionalField::ReplicationStatus,
            Self::Size => InventoryOptionalField::Size,
            Self::StorageClass => InventoryOptionalField::StorageClass,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::InventoryS3BucketDestination {
    type Target = InventoryS3BucketDestination;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            account_id: try_from_aws(x.account_id)?,
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            encryption: try_from_aws(x.encryption)?,
            format: unwrap_from_aws(x.format, "format")?,
            prefix: try_from_aws(x.prefix)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_account_id(try_into_aws(x.account_id)?);
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_encryption(try_into_aws(x.encryption)?);
        y = y.set_format(Some(try_into_aws(x.format)?));
        y = y.set_prefix(try_into_aws(x.prefix)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::InventorySchedule {
    type Target = InventorySchedule;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            frequency: unwrap_from_aws(x.frequency, "frequency")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_frequency(Some(try_into_aws(x.frequency)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::JSONInput {
    type Target = JsonInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            type_: try_from_aws(x.r#type)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_type(try_into_aws(x.type_)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::JSONOutput {
    type Target = JsonOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            record_delimiter: try_from_aws(x.record_delimiter)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_record_delimiter(try_into_aws(x.record_delimiter)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::JSONType {
    type Target = JsonType;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            JsonType::Document => Self::Document,
            JsonType::Lines => Self::Lines,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Document => JsonType::Document,
            Self::Lines => JsonType::Lines,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::LambdaFunctionConfiguration {
    type Target = LambdaFunctionConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            events: unwrap_from_aws(x.events, "events")?,
            filter: try_from_aws(x.filter)?,
            id: try_from_aws(x.id)?,
            lambda_function_arn: unwrap_from_aws(x.lambda_function_arn, "lambda_function_arn")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_events(Some(try_into_aws(x.events)?));
        y = y.set_filter(try_into_aws(x.filter)?);
        y = y.set_id(try_into_aws(x.id)?);
        y = y.set_lambda_function_arn(Some(try_into_aws(x.lambda_function_arn)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::LifecycleExpiration {
    type Target = LifecycleExpiration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            date: try_from_aws(x.date)?,
            days: try_from_aws(x.days)?,
            expired_object_delete_marker: try_from_aws(x.expired_object_delete_marker)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_date(try_into_aws(x.date)?);
        y = y.set_days(Some(try_into_aws(x.days)?));
        y = y.set_expired_object_delete_marker(Some(try_into_aws(x.expired_object_delete_marker)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::LifecycleRule {
    type Target = LifecycleRule;

    #[allow(deprecated)]
    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            abort_incomplete_multipart_upload: try_from_aws(x.abort_incomplete_multipart_upload)?,
            expiration: try_from_aws(x.expiration)?,
            filter: try_from_aws(x.filter)?,
            id: try_from_aws(x.id)?,
            noncurrent_version_expiration: try_from_aws(x.noncurrent_version_expiration)?,
            noncurrent_version_transitions: try_from_aws(x.noncurrent_version_transitions)?,
            prefix: try_from_aws(x.prefix)?,
            status: unwrap_from_aws(x.status, "status")?,
            transitions: try_from_aws(x.transitions)?,
        })
    }

    #[allow(deprecated)]
    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_abort_incomplete_multipart_upload(try_into_aws(x.abort_incomplete_multipart_upload)?);
        y = y.set_expiration(try_into_aws(x.expiration)?);
        y = y.set_filter(try_into_aws(x.filter)?);
        y = y.set_id(try_into_aws(x.id)?);
        y = y.set_noncurrent_version_expiration(try_into_aws(x.noncurrent_version_expiration)?);
        y = y.set_noncurrent_version_transitions(try_into_aws(x.noncurrent_version_transitions)?);
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_status(Some(try_into_aws(x.status)?));
        y = y.set_transitions(try_into_aws(x.transitions)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::LifecycleRuleAndOperator {
    type Target = LifecycleRuleAndOperator;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            object_size_greater_than: try_from_aws(x.object_size_greater_than)?,
            object_size_less_than: try_from_aws(x.object_size_less_than)?,
            prefix: try_from_aws(x.prefix)?,
            tags: try_from_aws(x.tags)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_object_size_greater_than(Some(try_into_aws(x.object_size_greater_than)?));
        y = y.set_object_size_less_than(Some(try_into_aws(x.object_size_less_than)?));
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_tags(try_into_aws(x.tags)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::LifecycleRuleFilter {
    type Target = LifecycleRuleFilter;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            LifecycleRuleFilter::And(v) => Self::And(try_from_aws(v)?),
            LifecycleRuleFilter::ObjectSizeGreaterThan(v) => Self::ObjectSizeGreaterThan(try_from_aws(v)?),
            LifecycleRuleFilter::ObjectSizeLessThan(v) => Self::ObjectSizeLessThan(try_from_aws(v)?),
            LifecycleRuleFilter::Prefix(v) => Self::Prefix(try_from_aws(v)?),
            LifecycleRuleFilter::Tag(v) => Self::Tag(try_from_aws(v)?),
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::And(v) => LifecycleRuleFilter::And(try_into_aws(v)?),
            Self::ObjectSizeGreaterThan(v) => LifecycleRuleFilter::ObjectSizeGreaterThan(try_into_aws(v)?),
            Self::ObjectSizeLessThan(v) => LifecycleRuleFilter::ObjectSizeLessThan(try_into_aws(v)?),
            Self::Prefix(v) => LifecycleRuleFilter::Prefix(try_into_aws(v)?),
            Self::Tag(v) => LifecycleRuleFilter::Tag(try_into_aws(v)?),
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ListBucketAnalyticsConfigurationsInput {
    type Target = ListBucketAnalyticsConfigurationsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            continuation_token: try_from_aws(x.continuation_token)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_continuation_token(try_into_aws(x.continuation_token)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::ListBucketAnalyticsConfigurationsOutput {
    type Target = ListBucketAnalyticsConfigurationsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            analytics_configuration_list: try_from_aws(x.analytics_configuration_list)?,
            continuation_token: try_from_aws(x.continuation_token)?,
            is_truncated: try_from_aws(x.is_truncated)?,
            next_continuation_token: try_from_aws(x.next_continuation_token)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_analytics_configuration_list(try_into_aws(x.analytics_configuration_list)?);
        y = y.set_continuation_token(try_into_aws(x.continuation_token)?);
        y = y.set_is_truncated(Some(try_into_aws(x.is_truncated)?));
        y = y.set_next_continuation_token(try_into_aws(x.next_continuation_token)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ListBucketIntelligentTieringConfigurationsInput {
    type Target = ListBucketIntelligentTieringConfigurationsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            continuation_token: try_from_aws(x.continuation_token)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_continuation_token(try_into_aws(x.continuation_token)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::ListBucketIntelligentTieringConfigurationsOutput {
    type Target = ListBucketIntelligentTieringConfigurationsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            continuation_token: try_from_aws(x.continuation_token)?,
            intelligent_tiering_configuration_list: try_from_aws(x.intelligent_tiering_configuration_list)?,
            is_truncated: try_from_aws(x.is_truncated)?,
            next_continuation_token: try_from_aws(x.next_continuation_token)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_continuation_token(try_into_aws(x.continuation_token)?);
        y = y.set_intelligent_tiering_configuration_list(try_into_aws(x.intelligent_tiering_configuration_list)?);
        y = y.set_is_truncated(Some(try_into_aws(x.is_truncated)?));
        y = y.set_next_continuation_token(try_into_aws(x.next_continuation_token)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ListBucketInventoryConfigurationsInput {
    type Target = ListBucketInventoryConfigurationsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            continuation_token: try_from_aws(x.continuation_token)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_continuation_token(try_into_aws(x.continuation_token)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::ListBucketInventoryConfigurationsOutput {
    type Target = ListBucketInventoryConfigurationsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            continuation_token: try_from_aws(x.continuation_token)?,
            inventory_configuration_list: try_from_aws(x.inventory_configuration_list)?,
            is_truncated: try_from_aws(x.is_truncated)?,
            next_continuation_token: try_from_aws(x.next_continuation_token)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_continuation_token(try_into_aws(x.continuation_token)?);
        y = y.set_inventory_configuration_list(try_into_aws(x.inventory_configuration_list)?);
        y = y.set_is_truncated(Some(try_into_aws(x.is_truncated)?));
        y = y.set_next_continuation_token(try_into_aws(x.next_continuation_token)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ListBucketMetricsConfigurationsInput {
    type Target = ListBucketMetricsConfigurationsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            continuation_token: try_from_aws(x.continuation_token)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_continuation_token(try_into_aws(x.continuation_token)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::ListBucketMetricsConfigurationsOutput {
    type Target = ListBucketMetricsConfigurationsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            continuation_token: try_from_aws(x.continuation_token)?,
            is_truncated: try_from_aws(x.is_truncated)?,
            metrics_configuration_list: try_from_aws(x.metrics_configuration_list)?,
            next_continuation_token: try_from_aws(x.next_continuation_token)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_continuation_token(try_into_aws(x.continuation_token)?);
        y = y.set_is_truncated(Some(try_into_aws(x.is_truncated)?));
        y = y.set_metrics_configuration_list(try_into_aws(x.metrics_configuration_list)?);
        y = y.set_next_continuation_token(try_into_aws(x.next_continuation_token)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ListBucketsInput {
    type Target = ListBucketsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::ListBucketsOutput {
    type Target = ListBucketsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            buckets: try_from_aws(x.buckets)?,
            owner: try_from_aws(x.owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_buckets(try_into_aws(x.buckets)?);
        y = y.set_owner(try_into_aws(x.owner)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ListMultipartUploadsInput {
    type Target = ListMultipartUploadsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            delimiter: try_from_aws(x.delimiter)?,
            encoding_type: try_from_aws(x.encoding_type)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key_marker: try_from_aws(x.key_marker)?,
            max_uploads: try_from_aws(x.max_uploads)?,
            prefix: try_from_aws(x.prefix)?,
            upload_id_marker: try_from_aws(x.upload_id_marker)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_delimiter(try_into_aws(x.delimiter)?);
        y = y.set_encoding_type(try_into_aws(x.encoding_type)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key_marker(try_into_aws(x.key_marker)?);
        y = y.set_max_uploads(Some(try_into_aws(x.max_uploads)?));
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_upload_id_marker(try_into_aws(x.upload_id_marker)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::ListMultipartUploadsOutput {
    type Target = ListMultipartUploadsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: try_from_aws(x.bucket)?,
            common_prefixes: try_from_aws(x.common_prefixes)?,
            delimiter: try_from_aws(x.delimiter)?,
            encoding_type: try_from_aws(x.encoding_type)?,
            is_truncated: try_from_aws(x.is_truncated)?,
            key_marker: try_from_aws(x.key_marker)?,
            max_uploads: try_from_aws(x.max_uploads)?,
            next_key_marker: try_from_aws(x.next_key_marker)?,
            next_upload_id_marker: try_from_aws(x.next_upload_id_marker)?,
            prefix: try_from_aws(x.prefix)?,
            upload_id_marker: try_from_aws(x.upload_id_marker)?,
            uploads: try_from_aws(x.uploads)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(try_into_aws(x.bucket)?);
        y = y.set_common_prefixes(try_into_aws(x.common_prefixes)?);
        y = y.set_delimiter(try_into_aws(x.delimiter)?);
        y = y.set_encoding_type(try_into_aws(x.encoding_type)?);
        y = y.set_is_truncated(Some(try_into_aws(x.is_truncated)?));
        y = y.set_key_marker(try_into_aws(x.key_marker)?);
        y = y.set_max_uploads(Some(try_into_aws(x.max_uploads)?));
        y = y.set_next_key_marker(try_into_aws(x.next_key_marker)?);
        y = y.set_next_upload_id_marker(try_into_aws(x.next_upload_id_marker)?);
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_upload_id_marker(try_into_aws(x.upload_id_marker)?);
        y = y.set_uploads(try_into_aws(x.uploads)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ListObjectVersionsInput {
    type Target = ListObjectVersionsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            delimiter: try_from_aws(x.delimiter)?,
            encoding_type: try_from_aws(x.encoding_type)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key_marker: try_from_aws(x.key_marker)?,
            max_keys: try_from_aws(x.max_keys)?,
            prefix: try_from_aws(x.prefix)?,
            version_id_marker: try_from_aws(x.version_id_marker)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_delimiter(try_into_aws(x.delimiter)?);
        y = y.set_encoding_type(try_into_aws(x.encoding_type)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key_marker(try_into_aws(x.key_marker)?);
        y = y.set_max_keys(Some(try_into_aws(x.max_keys)?));
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_version_id_marker(try_into_aws(x.version_id_marker)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::ListObjectVersionsOutput {
    type Target = ListObjectVersionsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            common_prefixes: try_from_aws(x.common_prefixes)?,
            delete_markers: try_from_aws(x.delete_markers)?,
            delimiter: try_from_aws(x.delimiter)?,
            encoding_type: try_from_aws(x.encoding_type)?,
            is_truncated: try_from_aws(x.is_truncated)?,
            key_marker: try_from_aws(x.key_marker)?,
            max_keys: try_from_aws(x.max_keys)?,
            name: try_from_aws(x.name)?,
            next_key_marker: try_from_aws(x.next_key_marker)?,
            next_version_id_marker: try_from_aws(x.next_version_id_marker)?,
            prefix: try_from_aws(x.prefix)?,
            version_id_marker: try_from_aws(x.version_id_marker)?,
            versions: try_from_aws(x.versions)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_common_prefixes(try_into_aws(x.common_prefixes)?);
        y = y.set_delete_markers(try_into_aws(x.delete_markers)?);
        y = y.set_delimiter(try_into_aws(x.delimiter)?);
        y = y.set_encoding_type(try_into_aws(x.encoding_type)?);
        y = y.set_is_truncated(Some(try_into_aws(x.is_truncated)?));
        y = y.set_key_marker(try_into_aws(x.key_marker)?);
        y = y.set_max_keys(Some(try_into_aws(x.max_keys)?));
        y = y.set_name(try_into_aws(x.name)?);
        y = y.set_next_key_marker(try_into_aws(x.next_key_marker)?);
        y = y.set_next_version_id_marker(try_into_aws(x.next_version_id_marker)?);
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_version_id_marker(try_into_aws(x.version_id_marker)?);
        y = y.set_versions(try_into_aws(x.versions)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ListObjectsInput {
    type Target = ListObjectsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            delimiter: try_from_aws(x.delimiter)?,
            encoding_type: try_from_aws(x.encoding_type)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            marker: try_from_aws(x.marker)?,
            max_keys: try_from_aws(x.max_keys)?,
            prefix: try_from_aws(x.prefix)?,
            request_payer: try_from_aws(x.request_payer)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_delimiter(try_into_aws(x.delimiter)?);
        y = y.set_encoding_type(try_into_aws(x.encoding_type)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_marker(try_into_aws(x.marker)?);
        y = y.set_max_keys(Some(try_into_aws(x.max_keys)?));
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::ListObjectsOutput {
    type Target = ListObjectsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            common_prefixes: try_from_aws(x.common_prefixes)?,
            contents: try_from_aws(x.contents)?,
            delimiter: try_from_aws(x.delimiter)?,
            encoding_type: try_from_aws(x.encoding_type)?,
            is_truncated: try_from_aws(x.is_truncated)?,
            marker: try_from_aws(x.marker)?,
            max_keys: try_from_aws(x.max_keys)?,
            name: try_from_aws(x.name)?,
            next_marker: try_from_aws(x.next_marker)?,
            prefix: try_from_aws(x.prefix)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_common_prefixes(try_into_aws(x.common_prefixes)?);
        y = y.set_contents(try_into_aws(x.contents)?);
        y = y.set_delimiter(try_into_aws(x.delimiter)?);
        y = y.set_encoding_type(try_into_aws(x.encoding_type)?);
        y = y.set_is_truncated(Some(try_into_aws(x.is_truncated)?));
        y = y.set_marker(try_into_aws(x.marker)?);
        y = y.set_max_keys(Some(try_into_aws(x.max_keys)?));
        y = y.set_name(try_into_aws(x.name)?);
        y = y.set_next_marker(try_into_aws(x.next_marker)?);
        y = y.set_prefix(try_into_aws(x.prefix)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ListObjectsV2Input {
    type Target = ListObjectsV2Input;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            continuation_token: try_from_aws(x.continuation_token)?,
            delimiter: try_from_aws(x.delimiter)?,
            encoding_type: try_from_aws(x.encoding_type)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            fetch_owner: try_from_aws(x.fetch_owner)?,
            max_keys: try_from_aws(x.max_keys)?,
            prefix: try_from_aws(x.prefix)?,
            request_payer: try_from_aws(x.request_payer)?,
            start_after: try_from_aws(x.start_after)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_continuation_token(try_into_aws(x.continuation_token)?);
        y = y.set_delimiter(try_into_aws(x.delimiter)?);
        y = y.set_encoding_type(try_into_aws(x.encoding_type)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_fetch_owner(Some(try_into_aws(x.fetch_owner)?));
        y = y.set_max_keys(Some(try_into_aws(x.max_keys)?));
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_start_after(try_into_aws(x.start_after)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::ListObjectsV2Output {
    type Target = ListObjectsV2Output;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            common_prefixes: try_from_aws(x.common_prefixes)?,
            contents: try_from_aws(x.contents)?,
            continuation_token: try_from_aws(x.continuation_token)?,
            delimiter: try_from_aws(x.delimiter)?,
            encoding_type: try_from_aws(x.encoding_type)?,
            is_truncated: try_from_aws(x.is_truncated)?,
            key_count: try_from_aws(x.key_count)?,
            max_keys: try_from_aws(x.max_keys)?,
            name: try_from_aws(x.name)?,
            next_continuation_token: try_from_aws(x.next_continuation_token)?,
            prefix: try_from_aws(x.prefix)?,
            start_after: try_from_aws(x.start_after)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_common_prefixes(try_into_aws(x.common_prefixes)?);
        y = y.set_contents(try_into_aws(x.contents)?);
        y = y.set_continuation_token(try_into_aws(x.continuation_token)?);
        y = y.set_delimiter(try_into_aws(x.delimiter)?);
        y = y.set_encoding_type(try_into_aws(x.encoding_type)?);
        y = y.set_is_truncated(Some(try_into_aws(x.is_truncated)?));
        y = y.set_key_count(Some(try_into_aws(x.key_count)?));
        y = y.set_max_keys(Some(try_into_aws(x.max_keys)?));
        y = y.set_name(try_into_aws(x.name)?);
        y = y.set_next_continuation_token(try_into_aws(x.next_continuation_token)?);
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_start_after(try_into_aws(x.start_after)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ListPartsInput {
    type Target = ListPartsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            max_parts: try_from_aws(x.max_parts)?,
            part_number_marker: try_from_aws(x.part_number_marker)?,
            request_payer: try_from_aws(x.request_payer)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key: try_from_aws(x.sse_customer_key)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            upload_id: unwrap_from_aws(x.upload_id, "upload_id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_max_parts(Some(try_into_aws(x.max_parts)?));
        y = y.set_part_number_marker(try_into_aws(x.part_number_marker)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key(try_into_aws(x.sse_customer_key)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_upload_id(Some(try_into_aws(x.upload_id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::ListPartsOutput {
    type Target = ListPartsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            abort_date: try_from_aws(x.abort_date)?,
            abort_rule_id: try_from_aws(x.abort_rule_id)?,
            bucket: try_from_aws(x.bucket)?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            initiator: try_from_aws(x.initiator)?,
            is_truncated: try_from_aws(x.is_truncated)?,
            key: try_from_aws(x.key)?,
            max_parts: try_from_aws(x.max_parts)?,
            next_part_number_marker: try_from_aws(x.next_part_number_marker)?,
            owner: try_from_aws(x.owner)?,
            part_number_marker: try_from_aws(x.part_number_marker)?,
            parts: try_from_aws(x.parts)?,
            request_charged: try_from_aws(x.request_charged)?,
            storage_class: try_from_aws(x.storage_class)?,
            upload_id: try_from_aws(x.upload_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_abort_date(try_into_aws(x.abort_date)?);
        y = y.set_abort_rule_id(try_into_aws(x.abort_rule_id)?);
        y = y.set_bucket(try_into_aws(x.bucket)?);
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_initiator(try_into_aws(x.initiator)?);
        y = y.set_is_truncated(Some(try_into_aws(x.is_truncated)?));
        y = y.set_key(try_into_aws(x.key)?);
        y = y.set_max_parts(Some(try_into_aws(x.max_parts)?));
        y = y.set_next_part_number_marker(try_into_aws(x.next_part_number_marker)?);
        y = y.set_owner(try_into_aws(x.owner)?);
        y = y.set_part_number_marker(try_into_aws(x.part_number_marker)?);
        y = y.set_parts(try_into_aws(x.parts)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        y = y.set_upload_id(try_into_aws(x.upload_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::LoggingEnabled {
    type Target = LoggingEnabled;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            target_bucket: unwrap_from_aws(x.target_bucket, "target_bucket")?,
            target_grants: try_from_aws(x.target_grants)?,
            target_prefix: unwrap_from_aws(x.target_prefix, "target_prefix")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_target_bucket(Some(try_into_aws(x.target_bucket)?));
        y = y.set_target_grants(try_into_aws(x.target_grants)?);
        y = y.set_target_prefix(Some(try_into_aws(x.target_prefix)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::MFADelete {
    type Target = MfaDelete;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            MfaDelete::Disabled => Self::Disabled,
            MfaDelete::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Disabled => MfaDelete::Disabled,
            Self::Enabled => MfaDelete::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::MFADeleteStatus {
    type Target = MfaDeleteStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            MfaDeleteStatus::Disabled => Self::Disabled,
            MfaDeleteStatus::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Disabled => MfaDeleteStatus::Disabled,
            Self::Enabled => MfaDeleteStatus::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::MetadataDirective {
    type Target = MetadataDirective;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            MetadataDirective::Copy => Self::Copy,
            MetadataDirective::Replace => Self::Replace,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Copy => MetadataDirective::Copy,
            Self::Replace => MetadataDirective::Replace,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::MetadataEntry {
    type Target = MetadataEntry;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            name: try_from_aws(x.name)?,
            value: try_from_aws(x.value)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_name(try_into_aws(x.name)?);
        y = y.set_value(try_into_aws(x.value)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Metrics {
    type Target = Metrics;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            event_threshold: try_from_aws(x.event_threshold)?,
            status: unwrap_from_aws(x.status, "status")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_event_threshold(try_into_aws(x.event_threshold)?);
        y = y.set_status(Some(try_into_aws(x.status)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::MetricsAndOperator {
    type Target = MetricsAndOperator;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            access_point_arn: try_from_aws(x.access_point_arn)?,
            prefix: try_from_aws(x.prefix)?,
            tags: try_from_aws(x.tags)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_access_point_arn(try_into_aws(x.access_point_arn)?);
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_tags(try_into_aws(x.tags)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::MetricsConfiguration {
    type Target = MetricsConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            filter: try_from_aws(x.filter)?,
            id: unwrap_from_aws(x.id, "id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_filter(try_into_aws(x.filter)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::MetricsFilter {
    type Target = MetricsFilter;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            MetricsFilter::AccessPointArn(v) => Self::AccessPointArn(try_from_aws(v)?),
            MetricsFilter::And(v) => Self::And(try_from_aws(v)?),
            MetricsFilter::Prefix(v) => Self::Prefix(try_from_aws(v)?),
            MetricsFilter::Tag(v) => Self::Tag(try_from_aws(v)?),
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::AccessPointArn(v) => MetricsFilter::AccessPointArn(try_into_aws(v)?),
            Self::And(v) => MetricsFilter::And(try_into_aws(v)?),
            Self::Prefix(v) => MetricsFilter::Prefix(try_into_aws(v)?),
            Self::Tag(v) => MetricsFilter::Tag(try_into_aws(v)?),
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::MetricsStatus {
    type Target = MetricsStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            MetricsStatus::Disabled => Self::Disabled,
            MetricsStatus::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Disabled => MetricsStatus::Disabled,
            Self::Enabled => MetricsStatus::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::MultipartUpload {
    type Target = MultipartUpload;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            initiated: try_from_aws(x.initiated)?,
            initiator: try_from_aws(x.initiator)?,
            key: try_from_aws(x.key)?,
            owner: try_from_aws(x.owner)?,
            storage_class: try_from_aws(x.storage_class)?,
            upload_id: try_from_aws(x.upload_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_initiated(try_into_aws(x.initiated)?);
        y = y.set_initiator(try_into_aws(x.initiator)?);
        y = y.set_key(try_into_aws(x.key)?);
        y = y.set_owner(try_into_aws(x.owner)?);
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        y = y.set_upload_id(try_into_aws(x.upload_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::NoSuchBucket {
    type Target = NoSuchBucket;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::NoSuchKey {
    type Target = NoSuchKey;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::NoSuchUpload {
    type Target = NoSuchUpload;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::NoncurrentVersionExpiration {
    type Target = NoncurrentVersionExpiration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            newer_noncurrent_versions: try_from_aws(x.newer_noncurrent_versions)?,
            noncurrent_days: try_from_aws(x.noncurrent_days)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_newer_noncurrent_versions(Some(try_into_aws(x.newer_noncurrent_versions)?));
        y = y.set_noncurrent_days(Some(try_into_aws(x.noncurrent_days)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::NoncurrentVersionTransition {
    type Target = NoncurrentVersionTransition;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            newer_noncurrent_versions: try_from_aws(x.newer_noncurrent_versions)?,
            noncurrent_days: try_from_aws(x.noncurrent_days)?,
            storage_class: try_from_aws(x.storage_class)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_newer_noncurrent_versions(Some(try_into_aws(x.newer_noncurrent_versions)?));
        y = y.set_noncurrent_days(Some(try_into_aws(x.noncurrent_days)?));
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::NotFound {
    type Target = NotFound;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::NotificationConfiguration {
    type Target = NotificationConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            event_bridge_configuration: try_from_aws(x.event_bridge_configuration)?,
            lambda_function_configurations: try_from_aws(x.lambda_function_configurations)?,
            queue_configurations: try_from_aws(x.queue_configurations)?,
            topic_configurations: try_from_aws(x.topic_configurations)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_event_bridge_configuration(try_into_aws(x.event_bridge_configuration)?);
        y = y.set_lambda_function_configurations(try_into_aws(x.lambda_function_configurations)?);
        y = y.set_queue_configurations(try_into_aws(x.queue_configurations)?);
        y = y.set_topic_configurations(try_into_aws(x.topic_configurations)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::NotificationConfigurationFilter {
    type Target = NotificationConfigurationFilter;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            key: try_from_aws(x.key)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_key(try_into_aws(x.key)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Object {
    type Target = Object;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            e_tag: try_from_aws(x.e_tag)?,
            key: try_from_aws(x.key)?,
            last_modified: try_from_aws(x.last_modified)?,
            owner: try_from_aws(x.owner)?,
            size: try_from_aws(x.size)?,
            storage_class: try_from_aws(x.storage_class)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_key(try_into_aws(x.key)?);
        y = y.set_last_modified(try_into_aws(x.last_modified)?);
        y = y.set_owner(try_into_aws(x.owner)?);
        y = y.set_size(Some(try_into_aws(x.size)?));
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ObjectAlreadyInActiveTierError {
    type Target = ObjectAlreadyInActiveTierError;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ObjectAttributes {
    type Target = ObjectAttributes;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ObjectAttributes::Checksum => Self::Checksum,
            ObjectAttributes::Etag => Self::Etag,
            ObjectAttributes::ObjectParts => Self::ObjectParts,
            ObjectAttributes::ObjectSize => Self::ObjectSize,
            ObjectAttributes::StorageClass => Self::StorageClass,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Checksum => ObjectAttributes::Checksum,
            Self::Etag => ObjectAttributes::Etag,
            Self::ObjectParts => ObjectAttributes::ObjectParts,
            Self::ObjectSize => ObjectAttributes::ObjectSize,
            Self::StorageClass => ObjectAttributes::StorageClass,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ObjectCannedACL {
    type Target = ObjectCannedAcl;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ObjectCannedAcl::AuthenticatedRead => Self::AuthenticatedRead,
            ObjectCannedAcl::AwsExecRead => Self::AwsExecRead,
            ObjectCannedAcl::BucketOwnerFullControl => Self::BucketOwnerFullControl,
            ObjectCannedAcl::BucketOwnerRead => Self::BucketOwnerRead,
            ObjectCannedAcl::Private => Self::Private,
            ObjectCannedAcl::PublicRead => Self::PublicRead,
            ObjectCannedAcl::PublicReadWrite => Self::PublicReadWrite,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::AuthenticatedRead => ObjectCannedAcl::AuthenticatedRead,
            Self::AwsExecRead => ObjectCannedAcl::AwsExecRead,
            Self::BucketOwnerFullControl => ObjectCannedAcl::BucketOwnerFullControl,
            Self::BucketOwnerRead => ObjectCannedAcl::BucketOwnerRead,
            Self::Private => ObjectCannedAcl::Private,
            Self::PublicRead => ObjectCannedAcl::PublicRead,
            Self::PublicReadWrite => ObjectCannedAcl::PublicReadWrite,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ObjectIdentifier {
    type Target = ObjectIdentifier;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            key: unwrap_from_aws(x.key, "key")?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ObjectLockConfiguration {
    type Target = ObjectLockConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            object_lock_enabled: try_from_aws(x.object_lock_enabled)?,
            rule: try_from_aws(x.rule)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_object_lock_enabled(try_into_aws(x.object_lock_enabled)?);
        y = y.set_rule(try_into_aws(x.rule)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ObjectLockEnabled {
    type Target = ObjectLockEnabled;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ObjectLockEnabled::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Enabled => ObjectLockEnabled::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ObjectLockLegalHold {
    type Target = ObjectLockLegalHold;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            status: try_from_aws(x.status)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_status(try_into_aws(x.status)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ObjectLockLegalHoldStatus {
    type Target = ObjectLockLegalHoldStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ObjectLockLegalHoldStatus::Off => Self::Off,
            ObjectLockLegalHoldStatus::On => Self::On,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Off => ObjectLockLegalHoldStatus::Off,
            Self::On => ObjectLockLegalHoldStatus::On,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ObjectLockMode {
    type Target = ObjectLockMode;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ObjectLockMode::Compliance => Self::Compliance,
            ObjectLockMode::Governance => Self::Governance,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Compliance => ObjectLockMode::Compliance,
            Self::Governance => ObjectLockMode::Governance,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ObjectLockRetention {
    type Target = ObjectLockRetention;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            mode: try_from_aws(x.mode)?,
            retain_until_date: try_from_aws(x.retain_until_date)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_mode(try_into_aws(x.mode)?);
        y = y.set_retain_until_date(try_into_aws(x.retain_until_date)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ObjectLockRetentionMode {
    type Target = ObjectLockRetentionMode;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ObjectLockRetentionMode::Compliance => Self::Compliance,
            ObjectLockRetentionMode::Governance => Self::Governance,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Compliance => ObjectLockRetentionMode::Compliance,
            Self::Governance => ObjectLockRetentionMode::Governance,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ObjectLockRule {
    type Target = ObjectLockRule;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            default_retention: try_from_aws(x.default_retention)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_default_retention(try_into_aws(x.default_retention)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ObjectNotInActiveTierError {
    type Target = ObjectNotInActiveTierError;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ObjectOwnership {
    type Target = ObjectOwnership;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ObjectOwnership::BucketOwnerEnforced => Self::BucketOwnerEnforced,
            ObjectOwnership::BucketOwnerPreferred => Self::BucketOwnerPreferred,
            ObjectOwnership::ObjectWriter => Self::ObjectWriter,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::BucketOwnerEnforced => ObjectOwnership::BucketOwnerEnforced,
            Self::BucketOwnerPreferred => ObjectOwnership::BucketOwnerPreferred,
            Self::ObjectWriter => ObjectOwnership::ObjectWriter,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ObjectPart {
    type Target = ObjectPart;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            part_number: try_from_aws(x.part_number)?,
            size: try_from_aws(x.size)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_part_number(Some(try_into_aws(x.part_number)?));
        y = y.set_size(Some(try_into_aws(x.size)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ObjectStorageClass {
    type Target = ObjectStorageClass;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ObjectStorageClass::DeepArchive => Self::DeepArchive,
            ObjectStorageClass::Glacier => Self::Glacier,
            ObjectStorageClass::GlacierIr => Self::GlacierIr,
            ObjectStorageClass::IntelligentTiering => Self::IntelligentTiering,
            ObjectStorageClass::OnezoneIa => Self::OnezoneIa,
            ObjectStorageClass::Outposts => Self::Outposts,
            ObjectStorageClass::ReducedRedundancy => Self::ReducedRedundancy,
            ObjectStorageClass::Standard => Self::Standard,
            ObjectStorageClass::StandardIa => Self::StandardIa,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::DeepArchive => ObjectStorageClass::DeepArchive,
            Self::Glacier => ObjectStorageClass::Glacier,
            Self::GlacierIr => ObjectStorageClass::GlacierIr,
            Self::IntelligentTiering => ObjectStorageClass::IntelligentTiering,
            Self::OnezoneIa => ObjectStorageClass::OnezoneIa,
            Self::Outposts => ObjectStorageClass::Outposts,
            Self::ReducedRedundancy => ObjectStorageClass::ReducedRedundancy,
            Self::Standard => ObjectStorageClass::Standard,
            Self::StandardIa => ObjectStorageClass::StandardIa,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ObjectVersion {
    type Target = ObjectVersion;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            e_tag: try_from_aws(x.e_tag)?,
            is_latest: try_from_aws(x.is_latest)?,
            key: try_from_aws(x.key)?,
            last_modified: try_from_aws(x.last_modified)?,
            owner: try_from_aws(x.owner)?,
            size: try_from_aws(x.size)?,
            storage_class: try_from_aws(x.storage_class)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_is_latest(Some(try_into_aws(x.is_latest)?));
        y = y.set_key(try_into_aws(x.key)?);
        y = y.set_last_modified(try_into_aws(x.last_modified)?);
        y = y.set_owner(try_into_aws(x.owner)?);
        y = y.set_size(Some(try_into_aws(x.size)?));
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ObjectVersionStorageClass {
    type Target = ObjectVersionStorageClass;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ObjectVersionStorageClass::Standard => Self::Standard,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Standard => ObjectVersionStorageClass::Standard,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::OutputLocation {
    type Target = OutputLocation;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self { s3: try_from_aws(x.s3)? })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_s3(try_into_aws(x.s3)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::OutputSerialization {
    type Target = OutputSerialization;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            csv: try_from_aws(x.csv)?,
            json: try_from_aws(x.json)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_csv(try_into_aws(x.csv)?);
        y = y.set_json(try_into_aws(x.json)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Owner {
    type Target = Owner;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            display_name: try_from_aws(x.display_name)?,
            id: try_from_aws(x.id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_display_name(try_into_aws(x.display_name)?);
        y = y.set_id(try_into_aws(x.id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::OwnerOverride {
    type Target = OwnerOverride;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            OwnerOverride::Destination => Self::Destination,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Destination => OwnerOverride::Destination,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::OwnershipControls {
    type Target = OwnershipControls;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            rules: unwrap_from_aws(x.rules, "rules")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_rules(Some(try_into_aws(x.rules)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::OwnershipControlsRule {
    type Target = OwnershipControlsRule;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            object_ownership: unwrap_from_aws(x.object_ownership, "object_ownership")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_object_ownership(Some(try_into_aws(x.object_ownership)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ParquetInput {
    type Target = ParquetInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Part {
    type Target = Part;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            e_tag: try_from_aws(x.e_tag)?,
            last_modified: try_from_aws(x.last_modified)?,
            part_number: try_from_aws(x.part_number)?,
            size: try_from_aws(x.size)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_last_modified(try_into_aws(x.last_modified)?);
        y = y.set_part_number(Some(try_into_aws(x.part_number)?));
        y = y.set_size(Some(try_into_aws(x.size)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Payer {
    type Target = Payer;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            Payer::BucketOwner => Self::BucketOwner,
            Payer::Requester => Self::Requester,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::BucketOwner => Payer::BucketOwner,
            Self::Requester => Payer::Requester,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::Permission {
    type Target = Permission;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            Permission::FullControl => Self::FullControl,
            Permission::Read => Self::Read,
            Permission::ReadAcp => Self::ReadAcp,
            Permission::Write => Self::Write,
            Permission::WriteAcp => Self::WriteAcp,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::FullControl => Permission::FullControl,
            Self::Read => Permission::Read,
            Self::ReadAcp => Permission::ReadAcp,
            Self::Write => Permission::Write,
            Self::WriteAcp => Permission::WriteAcp,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::PolicyStatus {
    type Target = PolicyStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            is_public: try_from_aws(x.is_public)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_is_public(Some(try_into_aws(x.is_public)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Progress {
    type Target = Progress;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bytes_processed: try_from_aws(x.bytes_processed)?,
            bytes_returned: try_from_aws(x.bytes_returned)?,
            bytes_scanned: try_from_aws(x.bytes_scanned)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bytes_processed(Some(try_into_aws(x.bytes_processed)?));
        y = y.set_bytes_returned(Some(try_into_aws(x.bytes_returned)?));
        y = y.set_bytes_scanned(Some(try_into_aws(x.bytes_scanned)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ProgressEvent {
    type Target = ProgressEvent;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            details: try_from_aws(x.details)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_details(try_into_aws(x.details)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Protocol {
    type Target = Protocol;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            Protocol::Http => Self::Http,
            Protocol::Https => Self::Https,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Http => Protocol::Http,
            Self::Https => Protocol::Https,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::PublicAccessBlockConfiguration {
    type Target = PublicAccessBlockConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            block_public_acls: try_from_aws(x.block_public_acls)?,
            block_public_policy: try_from_aws(x.block_public_policy)?,
            ignore_public_acls: try_from_aws(x.ignore_public_acls)?,
            restrict_public_buckets: try_from_aws(x.restrict_public_buckets)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_block_public_acls(Some(try_into_aws(x.block_public_acls)?));
        y = y.set_block_public_policy(Some(try_into_aws(x.block_public_policy)?));
        y = y.set_ignore_public_acls(Some(try_into_aws(x.ignore_public_acls)?));
        y = y.set_restrict_public_buckets(Some(try_into_aws(x.restrict_public_buckets)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketAccelerateConfigurationInput {
    type Target = PutBucketAccelerateConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            accelerate_configuration: unwrap_from_aws(x.accelerate_configuration, "accelerate_configuration")?,
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_accelerate_configuration(Some(try_into_aws(x.accelerate_configuration)?));
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketAccelerateConfigurationOutput {
    type Target = PutBucketAccelerateConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketAclInput {
    type Target = PutBucketAclInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            acl: try_from_aws(x.acl)?,
            access_control_policy: try_from_aws(x.access_control_policy)?,
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            grant_full_control: try_from_aws(x.grant_full_control)?,
            grant_read: try_from_aws(x.grant_read)?,
            grant_read_acp: try_from_aws(x.grant_read_acp)?,
            grant_write: try_from_aws(x.grant_write)?,
            grant_write_acp: try_from_aws(x.grant_write_acp)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_acl(try_into_aws(x.acl)?);
        y = y.set_access_control_policy(try_into_aws(x.access_control_policy)?);
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_grant_full_control(try_into_aws(x.grant_full_control)?);
        y = y.set_grant_read(try_into_aws(x.grant_read)?);
        y = y.set_grant_read_acp(try_into_aws(x.grant_read_acp)?);
        y = y.set_grant_write(try_into_aws(x.grant_write)?);
        y = y.set_grant_write_acp(try_into_aws(x.grant_write_acp)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketAclOutput {
    type Target = PutBucketAclOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketAnalyticsConfigurationInput {
    type Target = PutBucketAnalyticsConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            analytics_configuration: unwrap_from_aws(x.analytics_configuration, "analytics_configuration")?,
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            id: unwrap_from_aws(x.id, "id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_analytics_configuration(Some(try_into_aws(x.analytics_configuration)?));
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketAnalyticsConfigurationOutput {
    type Target = PutBucketAnalyticsConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketCorsInput {
    type Target = PutBucketCorsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            cors_configuration: unwrap_from_aws(x.cors_configuration, "cors_configuration")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_cors_configuration(Some(try_into_aws(x.cors_configuration)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketCorsOutput {
    type Target = PutBucketCorsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketEncryptionInput {
    type Target = PutBucketEncryptionInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            server_side_encryption_configuration: unwrap_from_aws(
                x.server_side_encryption_configuration,
                "server_side_encryption_configuration",
            )?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_server_side_encryption_configuration(Some(try_into_aws(x.server_side_encryption_configuration)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketEncryptionOutput {
    type Target = PutBucketEncryptionOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketIntelligentTieringConfigurationInput {
    type Target = PutBucketIntelligentTieringConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            id: unwrap_from_aws(x.id, "id")?,
            intelligent_tiering_configuration: unwrap_from_aws(
                x.intelligent_tiering_configuration,
                "intelligent_tiering_configuration",
            )?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_id(Some(try_into_aws(x.id)?));
        y = y.set_intelligent_tiering_configuration(Some(try_into_aws(x.intelligent_tiering_configuration)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketIntelligentTieringConfigurationOutput {
    type Target = PutBucketIntelligentTieringConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketInventoryConfigurationInput {
    type Target = PutBucketInventoryConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            id: unwrap_from_aws(x.id, "id")?,
            inventory_configuration: unwrap_from_aws(x.inventory_configuration, "inventory_configuration")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y = y.set_inventory_configuration(Some(try_into_aws(x.inventory_configuration)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketInventoryConfigurationOutput {
    type Target = PutBucketInventoryConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketLifecycleConfigurationInput {
    type Target = PutBucketLifecycleConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            lifecycle_configuration: try_from_aws(x.lifecycle_configuration)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_lifecycle_configuration(try_into_aws(x.lifecycle_configuration)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketLifecycleConfigurationOutput {
    type Target = PutBucketLifecycleConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketLoggingInput {
    type Target = PutBucketLoggingInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            bucket_logging_status: unwrap_from_aws(x.bucket_logging_status, "bucket_logging_status")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_bucket_logging_status(Some(try_into_aws(x.bucket_logging_status)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketLoggingOutput {
    type Target = PutBucketLoggingOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketMetricsConfigurationInput {
    type Target = PutBucketMetricsConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            id: unwrap_from_aws(x.id, "id")?,
            metrics_configuration: unwrap_from_aws(x.metrics_configuration, "metrics_configuration")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_id(Some(try_into_aws(x.id)?));
        y = y.set_metrics_configuration(Some(try_into_aws(x.metrics_configuration)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketMetricsConfigurationOutput {
    type Target = PutBucketMetricsConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketNotificationConfigurationInput {
    type Target = PutBucketNotificationConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            notification_configuration: unwrap_from_aws(x.notification_configuration, "notification_configuration")?,
            skip_destination_validation: try_from_aws(x.skip_destination_validation)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_notification_configuration(Some(try_into_aws(x.notification_configuration)?));
        y = y.set_skip_destination_validation(Some(try_into_aws(x.skip_destination_validation)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketNotificationConfigurationOutput {
    type Target = PutBucketNotificationConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketOwnershipControlsInput {
    type Target = PutBucketOwnershipControlsInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            ownership_controls: unwrap_from_aws(x.ownership_controls, "ownership_controls")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_ownership_controls(Some(try_into_aws(x.ownership_controls)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketOwnershipControlsOutput {
    type Target = PutBucketOwnershipControlsOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketPolicyInput {
    type Target = PutBucketPolicyInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            confirm_remove_self_bucket_access: try_from_aws(x.confirm_remove_self_bucket_access)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            policy: unwrap_from_aws(x.policy, "policy")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_confirm_remove_self_bucket_access(Some(try_into_aws(x.confirm_remove_self_bucket_access)?));
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_policy(Some(try_into_aws(x.policy)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketPolicyOutput {
    type Target = PutBucketPolicyOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketReplicationInput {
    type Target = PutBucketReplicationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            replication_configuration: unwrap_from_aws(x.replication_configuration, "replication_configuration")?,
            token: try_from_aws(x.token)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_replication_configuration(Some(try_into_aws(x.replication_configuration)?));
        y = y.set_token(try_into_aws(x.token)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketReplicationOutput {
    type Target = PutBucketReplicationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketRequestPaymentInput {
    type Target = PutBucketRequestPaymentInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            request_payment_configuration: unwrap_from_aws(x.request_payment_configuration, "request_payment_configuration")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_request_payment_configuration(Some(try_into_aws(x.request_payment_configuration)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketRequestPaymentOutput {
    type Target = PutBucketRequestPaymentOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketTaggingInput {
    type Target = PutBucketTaggingInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            tagging: unwrap_from_aws(x.tagging, "tagging")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_tagging(Some(try_into_aws(x.tagging)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketTaggingOutput {
    type Target = PutBucketTaggingOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketVersioningInput {
    type Target = PutBucketVersioningInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            mfa: try_from_aws(x.mfa)?,
            versioning_configuration: unwrap_from_aws(x.versioning_configuration, "versioning_configuration")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_mfa(try_into_aws(x.mfa)?);
        y = y.set_versioning_configuration(Some(try_into_aws(x.versioning_configuration)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketVersioningOutput {
    type Target = PutBucketVersioningOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutBucketWebsiteInput {
    type Target = PutBucketWebsiteInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            website_configuration: unwrap_from_aws(x.website_configuration, "website_configuration")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_website_configuration(Some(try_into_aws(x.website_configuration)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutBucketWebsiteOutput {
    type Target = PutBucketWebsiteOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutObjectAclInput {
    type Target = PutObjectAclInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            acl: try_from_aws(x.acl)?,
            access_control_policy: try_from_aws(x.access_control_policy)?,
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            grant_full_control: try_from_aws(x.grant_full_control)?,
            grant_read: try_from_aws(x.grant_read)?,
            grant_read_acp: try_from_aws(x.grant_read_acp)?,
            grant_write: try_from_aws(x.grant_write)?,
            grant_write_acp: try_from_aws(x.grant_write_acp)?,
            key: unwrap_from_aws(x.key, "key")?,
            request_payer: try_from_aws(x.request_payer)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_acl(try_into_aws(x.acl)?);
        y = y.set_access_control_policy(try_into_aws(x.access_control_policy)?);
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_grant_full_control(try_into_aws(x.grant_full_control)?);
        y = y.set_grant_read(try_into_aws(x.grant_read)?);
        y = y.set_grant_read_acp(try_into_aws(x.grant_read_acp)?);
        y = y.set_grant_write(try_into_aws(x.grant_write)?);
        y = y.set_grant_write_acp(try_into_aws(x.grant_write_acp)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutObjectAclOutput {
    type Target = PutObjectAclOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            request_charged: try_from_aws(x.request_charged)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutObjectInput {
    type Target = PutObjectInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            acl: try_from_aws(x.acl)?,
            body: stream_from_aws(x.body),
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            cache_control: try_from_aws(x.cache_control)?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            content_disposition: try_from_aws(x.content_disposition)?,
            content_encoding: try_from_aws(x.content_encoding)?,
            content_language: try_from_aws(x.content_language)?,
            content_length: try_from_aws(x.content_length)?,
            content_md5: try_from_aws(x.content_md5)?,
            content_type: try_from_aws(x.content_type)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            expires: try_from_aws(x.expires)?,
            grant_full_control: try_from_aws(x.grant_full_control)?,
            grant_read: try_from_aws(x.grant_read)?,
            grant_read_acp: try_from_aws(x.grant_read_acp)?,
            grant_write_acp: try_from_aws(x.grant_write_acp)?,
            key: unwrap_from_aws(x.key, "key")?,
            metadata: try_from_aws(x.metadata)?,
            object_lock_legal_hold_status: try_from_aws(x.object_lock_legal_hold_status)?,
            object_lock_mode: try_from_aws(x.object_lock_mode)?,
            object_lock_retain_until_date: try_from_aws(x.object_lock_retain_until_date)?,
            request_payer: try_from_aws(x.request_payer)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key: try_from_aws(x.sse_customer_key)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            ssekms_encryption_context: try_from_aws(x.ssekms_encryption_context)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
            storage_class: try_from_aws(x.storage_class)?,
            tagging: try_from_aws(x.tagging)?,
            website_redirect_location: try_from_aws(x.website_redirect_location)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_acl(try_into_aws(x.acl)?);
        y = y.set_body(try_into_aws(x.body)?);
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_cache_control(try_into_aws(x.cache_control)?);
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_content_disposition(try_into_aws(x.content_disposition)?);
        y = y.set_content_encoding(try_into_aws(x.content_encoding)?);
        y = y.set_content_language(try_into_aws(x.content_language)?);
        y = y.set_content_length(Some(try_into_aws(x.content_length)?));
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_content_type(try_into_aws(x.content_type)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_expires(try_into_aws(x.expires)?);
        y = y.set_grant_full_control(try_into_aws(x.grant_full_control)?);
        y = y.set_grant_read(try_into_aws(x.grant_read)?);
        y = y.set_grant_read_acp(try_into_aws(x.grant_read_acp)?);
        y = y.set_grant_write_acp(try_into_aws(x.grant_write_acp)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_metadata(try_into_aws(x.metadata)?);
        y = y.set_object_lock_legal_hold_status(try_into_aws(x.object_lock_legal_hold_status)?);
        y = y.set_object_lock_mode(try_into_aws(x.object_lock_mode)?);
        y = y.set_object_lock_retain_until_date(try_into_aws(x.object_lock_retain_until_date)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key(try_into_aws(x.sse_customer_key)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_ssekms_encryption_context(try_into_aws(x.ssekms_encryption_context)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        y = y.set_tagging(try_into_aws(x.tagging)?);
        y = y.set_website_redirect_location(try_into_aws(x.website_redirect_location)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutObjectLegalHoldInput {
    type Target = PutObjectLegalHoldInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            legal_hold: try_from_aws(x.legal_hold)?,
            request_payer: try_from_aws(x.request_payer)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_legal_hold(try_into_aws(x.legal_hold)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutObjectLegalHoldOutput {
    type Target = PutObjectLegalHoldOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            request_charged: try_from_aws(x.request_charged)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutObjectLockConfigurationInput {
    type Target = PutObjectLockConfigurationInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            object_lock_configuration: try_from_aws(x.object_lock_configuration)?,
            request_payer: try_from_aws(x.request_payer)?,
            token: try_from_aws(x.token)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_object_lock_configuration(try_into_aws(x.object_lock_configuration)?);
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_token(try_into_aws(x.token)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutObjectLockConfigurationOutput {
    type Target = PutObjectLockConfigurationOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            request_charged: try_from_aws(x.request_charged)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutObjectOutput {
    type Target = PutObjectOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            e_tag: try_from_aws(x.e_tag)?,
            expiration: try_from_aws(x.expiration)?,
            request_charged: try_from_aws(x.request_charged)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            ssekms_encryption_context: try_from_aws(x.ssekms_encryption_context)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_expiration(try_into_aws(x.expiration)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_ssekms_encryption_context(try_into_aws(x.ssekms_encryption_context)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutObjectRetentionInput {
    type Target = PutObjectRetentionInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            bypass_governance_retention: try_from_aws(x.bypass_governance_retention)?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            request_payer: try_from_aws(x.request_payer)?,
            retention: try_from_aws(x.retention)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_bypass_governance_retention(Some(try_into_aws(x.bypass_governance_retention)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_retention(try_into_aws(x.retention)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutObjectRetentionOutput {
    type Target = PutObjectRetentionOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            request_charged: try_from_aws(x.request_charged)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutObjectTaggingInput {
    type Target = PutObjectTaggingInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            request_payer: try_from_aws(x.request_payer)?,
            tagging: unwrap_from_aws(x.tagging, "tagging")?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_tagging(Some(try_into_aws(x.tagging)?));
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutObjectTaggingOutput {
    type Target = PutObjectTaggingOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_version_id(try_into_aws(x.version_id)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::PutPublicAccessBlockInput {
    type Target = PutPublicAccessBlockInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            public_access_block_configuration: unwrap_from_aws(
                x.public_access_block_configuration,
                "public_access_block_configuration",
            )?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_public_access_block_configuration(Some(try_into_aws(x.public_access_block_configuration)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::PutPublicAccessBlockOutput {
    type Target = PutPublicAccessBlockOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::QueueConfiguration {
    type Target = QueueConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            events: unwrap_from_aws(x.events, "events")?,
            filter: try_from_aws(x.filter)?,
            id: try_from_aws(x.id)?,
            queue_arn: unwrap_from_aws(x.queue_arn, "queue_arn")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_events(Some(try_into_aws(x.events)?));
        y = y.set_filter(try_into_aws(x.filter)?);
        y = y.set_id(try_into_aws(x.id)?);
        y = y.set_queue_arn(Some(try_into_aws(x.queue_arn)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::QuoteFields {
    type Target = QuoteFields;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            QuoteFields::Always => Self::Always,
            QuoteFields::Asneeded => Self::Asneeded,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Always => QuoteFields::Always,
            Self::Asneeded => QuoteFields::Asneeded,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::RecordsEvent {
    type Target = RecordsEvent;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            payload: try_from_aws(x.payload)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_payload(try_into_aws(x.payload)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Redirect {
    type Target = Redirect;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            host_name: try_from_aws(x.host_name)?,
            http_redirect_code: try_from_aws(x.http_redirect_code)?,
            protocol: try_from_aws(x.protocol)?,
            replace_key_prefix_with: try_from_aws(x.replace_key_prefix_with)?,
            replace_key_with: try_from_aws(x.replace_key_with)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_host_name(try_into_aws(x.host_name)?);
        y = y.set_http_redirect_code(try_into_aws(x.http_redirect_code)?);
        y = y.set_protocol(try_into_aws(x.protocol)?);
        y = y.set_replace_key_prefix_with(try_into_aws(x.replace_key_prefix_with)?);
        y = y.set_replace_key_with(try_into_aws(x.replace_key_with)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::RedirectAllRequestsTo {
    type Target = RedirectAllRequestsTo;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            host_name: unwrap_from_aws(x.host_name, "host_name")?,
            protocol: try_from_aws(x.protocol)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_host_name(Some(try_into_aws(x.host_name)?));
        y = y.set_protocol(try_into_aws(x.protocol)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ReplicaModifications {
    type Target = ReplicaModifications;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            status: unwrap_from_aws(x.status, "status")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_status(Some(try_into_aws(x.status)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ReplicaModificationsStatus {
    type Target = ReplicaModificationsStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ReplicaModificationsStatus::Disabled => Self::Disabled,
            ReplicaModificationsStatus::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Disabled => ReplicaModificationsStatus::Disabled,
            Self::Enabled => ReplicaModificationsStatus::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ReplicationConfiguration {
    type Target = ReplicationConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            role: unwrap_from_aws(x.role, "role")?,
            rules: unwrap_from_aws(x.rules, "rules")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_role(Some(try_into_aws(x.role)?));
        y = y.set_rules(Some(try_into_aws(x.rules)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ReplicationRule {
    type Target = ReplicationRule;

    #[allow(deprecated)]
    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            delete_marker_replication: try_from_aws(x.delete_marker_replication)?,
            destination: unwrap_from_aws(x.destination, "destination")?,
            existing_object_replication: try_from_aws(x.existing_object_replication)?,
            filter: try_from_aws(x.filter)?,
            id: try_from_aws(x.id)?,
            prefix: try_from_aws(x.prefix)?,
            priority: try_from_aws(x.priority)?,
            source_selection_criteria: try_from_aws(x.source_selection_criteria)?,
            status: unwrap_from_aws(x.status, "status")?,
        })
    }

    #[allow(deprecated)]
    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_delete_marker_replication(try_into_aws(x.delete_marker_replication)?);
        y = y.set_destination(Some(try_into_aws(x.destination)?));
        y = y.set_existing_object_replication(try_into_aws(x.existing_object_replication)?);
        y = y.set_filter(try_into_aws(x.filter)?);
        y = y.set_id(try_into_aws(x.id)?);
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_priority(Some(try_into_aws(x.priority)?));
        y = y.set_source_selection_criteria(try_into_aws(x.source_selection_criteria)?);
        y = y.set_status(Some(try_into_aws(x.status)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ReplicationRuleAndOperator {
    type Target = ReplicationRuleAndOperator;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            prefix: try_from_aws(x.prefix)?,
            tags: try_from_aws(x.tags)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_prefix(try_into_aws(x.prefix)?);
        y = y.set_tags(try_into_aws(x.tags)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ReplicationRuleFilter {
    type Target = ReplicationRuleFilter;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ReplicationRuleFilter::And(v) => Self::And(try_from_aws(v)?),
            ReplicationRuleFilter::Prefix(v) => Self::Prefix(try_from_aws(v)?),
            ReplicationRuleFilter::Tag(v) => Self::Tag(try_from_aws(v)?),
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::And(v) => ReplicationRuleFilter::And(try_into_aws(v)?),
            Self::Prefix(v) => ReplicationRuleFilter::Prefix(try_into_aws(v)?),
            Self::Tag(v) => ReplicationRuleFilter::Tag(try_into_aws(v)?),
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ReplicationRuleStatus {
    type Target = ReplicationRuleStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ReplicationRuleStatus::Disabled => Self::Disabled,
            ReplicationRuleStatus::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Disabled => ReplicationRuleStatus::Disabled,
            Self::Enabled => ReplicationRuleStatus::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ReplicationStatus {
    type Target = ReplicationStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ReplicationStatus::Complete => Self::Complete,
            ReplicationStatus::Failed => Self::Failed,
            ReplicationStatus::Pending => Self::Pending,
            ReplicationStatus::Replica => Self::Replica,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Complete => ReplicationStatus::Complete,
            Self::Failed => ReplicationStatus::Failed,
            Self::Pending => ReplicationStatus::Pending,
            Self::Replica => ReplicationStatus::Replica,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ReplicationTime {
    type Target = ReplicationTime;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            status: unwrap_from_aws(x.status, "status")?,
            time: unwrap_from_aws(x.time, "time")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_status(Some(try_into_aws(x.status)?));
        y = y.set_time(Some(try_into_aws(x.time)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ReplicationTimeStatus {
    type Target = ReplicationTimeStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ReplicationTimeStatus::Disabled => Self::Disabled,
            ReplicationTimeStatus::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Disabled => ReplicationTimeStatus::Disabled,
            Self::Enabled => ReplicationTimeStatus::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ReplicationTimeValue {
    type Target = ReplicationTimeValue;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            minutes: try_from_aws(x.minutes)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_minutes(Some(try_into_aws(x.minutes)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::RequestCharged {
    type Target = RequestCharged;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            RequestCharged::Requester => Self::Requester,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Requester => RequestCharged::Requester,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::RequestPayer {
    type Target = RequestPayer;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            RequestPayer::Requester => Self::Requester,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Requester => RequestPayer::Requester,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::RequestPaymentConfiguration {
    type Target = RequestPaymentConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            payer: unwrap_from_aws(x.payer, "payer")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_payer(Some(try_into_aws(x.payer)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::RequestProgress {
    type Target = RequestProgress;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            enabled: try_from_aws(x.enabled)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_enabled(Some(try_into_aws(x.enabled)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::RestoreObjectInput {
    type Target = RestoreObjectInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            request_payer: try_from_aws(x.request_payer)?,
            restore_request: try_from_aws(x.restore_request)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_restore_request(try_into_aws(x.restore_request)?);
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::RestoreObjectOutput {
    type Target = RestoreObjectOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            request_charged: try_from_aws(x.request_charged)?,
            restore_output_path: try_from_aws(x.restore_output_path)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_restore_output_path(try_into_aws(x.restore_output_path)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::RestoreRequest {
    type Target = RestoreRequest;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            days: try_from_aws(x.days)?,
            description: try_from_aws(x.description)?,
            glacier_job_parameters: try_from_aws(x.glacier_job_parameters)?,
            output_location: try_from_aws(x.output_location)?,
            select_parameters: try_from_aws(x.select_parameters)?,
            tier: try_from_aws(x.tier)?,
            type_: try_from_aws(x.r#type)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_days(Some(try_into_aws(x.days)?));
        y = y.set_description(try_into_aws(x.description)?);
        y = y.set_glacier_job_parameters(try_into_aws(x.glacier_job_parameters)?);
        y = y.set_output_location(try_into_aws(x.output_location)?);
        y = y.set_select_parameters(try_into_aws(x.select_parameters)?);
        y = y.set_tier(try_into_aws(x.tier)?);
        y = y.set_type(try_into_aws(x.type_)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::RestoreRequestType {
    type Target = RestoreRequestType;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            RestoreRequestType::Select => Self::Select,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Select => RestoreRequestType::Select,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::RoutingRule {
    type Target = RoutingRule;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            condition: try_from_aws(x.condition)?,
            redirect: unwrap_from_aws(x.redirect, "redirect")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_condition(try_into_aws(x.condition)?);
        y = y.set_redirect(Some(try_into_aws(x.redirect)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::S3KeyFilter {
    type Target = S3KeyFilter;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            filter_rules: try_from_aws(x.filter_rules)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_filter_rules(try_into_aws(x.filter_rules)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::S3Location {
    type Target = S3Location;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            access_control_list: try_from_aws(x.access_control_list)?,
            bucket_name: unwrap_from_aws(x.bucket_name, "bucket_name")?,
            canned_acl: try_from_aws(x.canned_acl)?,
            encryption: try_from_aws(x.encryption)?,
            prefix: unwrap_from_aws(x.prefix, "prefix")?,
            storage_class: try_from_aws(x.storage_class)?,
            tagging: try_from_aws(x.tagging)?,
            user_metadata: try_from_aws(x.user_metadata)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_access_control_list(try_into_aws(x.access_control_list)?);
        y = y.set_bucket_name(Some(try_into_aws(x.bucket_name)?));
        y = y.set_canned_acl(try_into_aws(x.canned_acl)?);
        y = y.set_encryption(try_into_aws(x.encryption)?);
        y = y.set_prefix(Some(try_into_aws(x.prefix)?));
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        y = y.set_tagging(try_into_aws(x.tagging)?);
        y = y.set_user_metadata(try_into_aws(x.user_metadata)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::SSEKMS {
    type Target = Ssekms;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            key_id: unwrap_from_aws(x.key_id, "key_id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_key_id(Some(try_into_aws(x.key_id)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::SSES3 {
    type Target = Sses3;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ScanRange {
    type Target = ScanRange;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            end: try_from_aws(x.end)?,
            start: try_from_aws(x.start)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_end(Some(try_into_aws(x.end)?));
        y = y.set_start(Some(try_into_aws(x.start)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::SelectParameters {
    type Target = SelectParameters;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            expression: unwrap_from_aws(x.expression, "expression")?,
            expression_type: unwrap_from_aws(x.expression_type, "expression_type")?,
            input_serialization: unwrap_from_aws(x.input_serialization, "input_serialization")?,
            output_serialization: unwrap_from_aws(x.output_serialization, "output_serialization")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_expression(Some(try_into_aws(x.expression)?));
        y = y.set_expression_type(Some(try_into_aws(x.expression_type)?));
        y = y.set_input_serialization(Some(try_into_aws(x.input_serialization)?));
        y = y.set_output_serialization(Some(try_into_aws(x.output_serialization)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ServerSideEncryption {
    type Target = ServerSideEncryption;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            ServerSideEncryption::Aes256 => Self::Aes256,
            ServerSideEncryption::AwsKms => Self::AwsKms,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Aes256 => ServerSideEncryption::Aes256,
            Self::AwsKms => ServerSideEncryption::AwsKms,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::ServerSideEncryptionByDefault {
    type Target = ServerSideEncryptionByDefault;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            kms_master_key_id: try_from_aws(x.kms_master_key_id)?,
            sse_algorithm: unwrap_from_aws(x.sse_algorithm, "sse_algorithm")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_kms_master_key_id(try_into_aws(x.kms_master_key_id)?);
        y = y.set_sse_algorithm(Some(try_into_aws(x.sse_algorithm)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ServerSideEncryptionConfiguration {
    type Target = ServerSideEncryptionConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            rules: unwrap_from_aws(x.rules, "rules")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_rules(Some(try_into_aws(x.rules)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::ServerSideEncryptionRule {
    type Target = ServerSideEncryptionRule;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            apply_server_side_encryption_by_default: try_from_aws(x.apply_server_side_encryption_by_default)?,
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_apply_server_side_encryption_by_default(try_into_aws(x.apply_server_side_encryption_by_default)?);
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::SourceSelectionCriteria {
    type Target = SourceSelectionCriteria;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            replica_modifications: try_from_aws(x.replica_modifications)?,
            sse_kms_encrypted_objects: try_from_aws(x.sse_kms_encrypted_objects)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_replica_modifications(try_into_aws(x.replica_modifications)?);
        y = y.set_sse_kms_encrypted_objects(try_into_aws(x.sse_kms_encrypted_objects)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::SseKmsEncryptedObjects {
    type Target = SseKmsEncryptedObjects;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            status: unwrap_from_aws(x.status, "status")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_status(Some(try_into_aws(x.status)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::SseKmsEncryptedObjectsStatus {
    type Target = SseKmsEncryptedObjectsStatus;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            SseKmsEncryptedObjectsStatus::Disabled => Self::Disabled,
            SseKmsEncryptedObjectsStatus::Enabled => Self::Enabled,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Disabled => SseKmsEncryptedObjectsStatus::Disabled,
            Self::Enabled => SseKmsEncryptedObjectsStatus::Enabled,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::Stats {
    type Target = Stats;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bytes_processed: try_from_aws(x.bytes_processed)?,
            bytes_returned: try_from_aws(x.bytes_returned)?,
            bytes_scanned: try_from_aws(x.bytes_scanned)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bytes_processed(Some(try_into_aws(x.bytes_processed)?));
        y = y.set_bytes_returned(Some(try_into_aws(x.bytes_returned)?));
        y = y.set_bytes_scanned(Some(try_into_aws(x.bytes_scanned)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::StatsEvent {
    type Target = StatsEvent;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            details: try_from_aws(x.details)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_details(try_into_aws(x.details)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::StorageClass {
    type Target = StorageClass;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            StorageClass::DeepArchive => Self::DeepArchive,
            StorageClass::Glacier => Self::Glacier,
            StorageClass::GlacierIr => Self::GlacierIr,
            StorageClass::IntelligentTiering => Self::IntelligentTiering,
            StorageClass::OnezoneIa => Self::OnezoneIa,
            StorageClass::Outposts => Self::Outposts,
            StorageClass::ReducedRedundancy => Self::ReducedRedundancy,
            StorageClass::Standard => Self::Standard,
            StorageClass::StandardIa => Self::StandardIa,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::DeepArchive => StorageClass::DeepArchive,
            Self::Glacier => StorageClass::Glacier,
            Self::GlacierIr => StorageClass::GlacierIr,
            Self::IntelligentTiering => StorageClass::IntelligentTiering,
            Self::OnezoneIa => StorageClass::OnezoneIa,
            Self::Outposts => StorageClass::Outposts,
            Self::ReducedRedundancy => StorageClass::ReducedRedundancy,
            Self::Standard => StorageClass::Standard,
            Self::StandardIa => StorageClass::StandardIa,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::StorageClassAnalysis {
    type Target = StorageClassAnalysis;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            data_export: try_from_aws(x.data_export)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_data_export(try_into_aws(x.data_export)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::StorageClassAnalysisDataExport {
    type Target = StorageClassAnalysisDataExport;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            destination: unwrap_from_aws(x.destination, "destination")?,
            output_schema_version: unwrap_from_aws(x.output_schema_version, "output_schema_version")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_destination(Some(try_into_aws(x.destination)?));
        y = y.set_output_schema_version(Some(try_into_aws(x.output_schema_version)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::StorageClassAnalysisSchemaVersion {
    type Target = StorageClassAnalysisSchemaVersion;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            StorageClassAnalysisSchemaVersion::V1 => Self::V1,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::V1 => StorageClassAnalysisSchemaVersion::V1,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::Tag {
    type Target = Tag;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            key: unwrap_from_aws(x.key, "key")?,
            value: unwrap_from_aws(x.value, "value")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_value(Some(try_into_aws(x.value)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Tagging {
    type Target = Tagging;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            tag_set: unwrap_from_aws(x.tag_set, "tag_set")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_tag_set(Some(try_into_aws(x.tag_set)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::TaggingDirective {
    type Target = TaggingDirective;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            TaggingDirective::Copy => Self::Copy,
            TaggingDirective::Replace => Self::Replace,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Copy => TaggingDirective::Copy,
            Self::Replace => TaggingDirective::Replace,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::TargetGrant {
    type Target = TargetGrant;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            grantee: try_from_aws(x.grantee)?,
            permission: try_from_aws(x.permission)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_grantee(try_into_aws(x.grantee)?);
        y = y.set_permission(try_into_aws(x.permission)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Tier {
    type Target = Tier;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            Tier::Bulk => Self::Bulk,
            Tier::Expedited => Self::Expedited,
            Tier::Standard => Self::Standard,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::Bulk => Tier::Bulk,
            Self::Expedited => Tier::Expedited,
            Self::Standard => Tier::Standard,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::Tiering {
    type Target = Tiering;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            access_tier: unwrap_from_aws(x.access_tier, "access_tier")?,
            days: try_from_aws(x.days)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_access_tier(Some(try_into_aws(x.access_tier)?));
        y = y.set_days(Some(try_into_aws(x.days)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::TopicConfiguration {
    type Target = TopicConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            events: unwrap_from_aws(x.events, "events")?,
            filter: try_from_aws(x.filter)?,
            id: try_from_aws(x.id)?,
            topic_arn: unwrap_from_aws(x.topic_arn, "topic_arn")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_events(Some(try_into_aws(x.events)?));
        y = y.set_filter(try_into_aws(x.filter)?);
        y = y.set_id(try_into_aws(x.id)?);
        y = y.set_topic_arn(Some(try_into_aws(x.topic_arn)?));
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::Transition {
    type Target = Transition;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            date: try_from_aws(x.date)?,
            days: try_from_aws(x.days)?,
            storage_class: try_from_aws(x.storage_class)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_date(try_into_aws(x.date)?);
        y = y.set_days(Some(try_into_aws(x.days)?));
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::TransitionStorageClass {
    type Target = TransitionStorageClass;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            TransitionStorageClass::DeepArchive => Self::DeepArchive,
            TransitionStorageClass::Glacier => Self::Glacier,
            TransitionStorageClass::GlacierIr => Self::GlacierIr,
            TransitionStorageClass::IntelligentTiering => Self::IntelligentTiering,
            TransitionStorageClass::OnezoneIa => Self::OnezoneIa,
            TransitionStorageClass::StandardIa => Self::StandardIa,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::DeepArchive => TransitionStorageClass::DeepArchive,
            Self::Glacier => TransitionStorageClass::Glacier,
            Self::GlacierIr => TransitionStorageClass::GlacierIr,
            Self::IntelligentTiering => TransitionStorageClass::IntelligentTiering,
            Self::OnezoneIa => TransitionStorageClass::OnezoneIa,
            Self::StandardIa => TransitionStorageClass::StandardIa,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::Type {
    type Target = Type;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(match x {
            Type::AmazonCustomerByEmail => Self::AmazonCustomerByEmail,
            Type::CanonicalUser => Self::CanonicalUser,
            Type::Group => Self::Group,
            _ => unreachable!(),
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        Ok(match x {
            Self::AmazonCustomerByEmail => Type::AmazonCustomerByEmail,
            Self::CanonicalUser => Type::CanonicalUser,
            Self::Group => Type::Group,
            _ => unreachable!(),
        })
    }
}

impl AwsConversion for s3s::dto::UploadPartCopyInput {
    type Target = UploadPartCopyInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            copy_source: unwrap_from_aws(x.copy_source, "copy_source")?,
            copy_source_if_match: try_from_aws(x.copy_source_if_match)?,
            copy_source_if_modified_since: try_from_aws(x.copy_source_if_modified_since)?,
            copy_source_if_none_match: try_from_aws(x.copy_source_if_none_match)?,
            copy_source_if_unmodified_since: try_from_aws(x.copy_source_if_unmodified_since)?,
            copy_source_range: try_from_aws(x.copy_source_range)?,
            copy_source_sse_customer_algorithm: try_from_aws(x.copy_source_sse_customer_algorithm)?,
            copy_source_sse_customer_key: try_from_aws(x.copy_source_sse_customer_key)?,
            copy_source_sse_customer_key_md5: try_from_aws(x.copy_source_sse_customer_key_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            expected_source_bucket_owner: try_from_aws(x.expected_source_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            part_number: try_from_aws(x.part_number)?,
            request_payer: try_from_aws(x.request_payer)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key: try_from_aws(x.sse_customer_key)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            upload_id: unwrap_from_aws(x.upload_id, "upload_id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_copy_source(Some(try_into_aws(x.copy_source)?));
        y = y.set_copy_source_if_match(try_into_aws(x.copy_source_if_match)?);
        y = y.set_copy_source_if_modified_since(try_into_aws(x.copy_source_if_modified_since)?);
        y = y.set_copy_source_if_none_match(try_into_aws(x.copy_source_if_none_match)?);
        y = y.set_copy_source_if_unmodified_since(try_into_aws(x.copy_source_if_unmodified_since)?);
        y = y.set_copy_source_range(try_into_aws(x.copy_source_range)?);
        y = y.set_copy_source_sse_customer_algorithm(try_into_aws(x.copy_source_sse_customer_algorithm)?);
        y = y.set_copy_source_sse_customer_key(try_into_aws(x.copy_source_sse_customer_key)?);
        y = y.set_copy_source_sse_customer_key_md5(try_into_aws(x.copy_source_sse_customer_key_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_expected_source_bucket_owner(try_into_aws(x.expected_source_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_part_number(Some(try_into_aws(x.part_number)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key(try_into_aws(x.sse_customer_key)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_upload_id(Some(try_into_aws(x.upload_id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::UploadPartCopyOutput {
    type Target = UploadPartCopyOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            copy_part_result: try_from_aws(x.copy_part_result)?,
            copy_source_version_id: try_from_aws(x.copy_source_version_id)?,
            request_charged: try_from_aws(x.request_charged)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_copy_part_result(try_into_aws(x.copy_part_result)?);
        y = y.set_copy_source_version_id(try_into_aws(x.copy_source_version_id)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::UploadPartInput {
    type Target = UploadPartInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            body: stream_from_aws(x.body),
            bucket: unwrap_from_aws(x.bucket, "bucket")?,
            checksum_algorithm: try_from_aws(x.checksum_algorithm)?,
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            content_length: try_from_aws(x.content_length)?,
            content_md5: try_from_aws(x.content_md5)?,
            expected_bucket_owner: try_from_aws(x.expected_bucket_owner)?,
            key: unwrap_from_aws(x.key, "key")?,
            part_number: try_from_aws(x.part_number)?,
            request_payer: try_from_aws(x.request_payer)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key: try_from_aws(x.sse_customer_key)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            upload_id: unwrap_from_aws(x.upload_id, "upload_id")?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_body(try_into_aws(x.body)?);
        y = y.set_bucket(Some(try_into_aws(x.bucket)?));
        y = y.set_checksum_algorithm(try_into_aws(x.checksum_algorithm)?);
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_content_length(Some(try_into_aws(x.content_length)?));
        y = y.set_content_md5(try_into_aws(x.content_md5)?);
        y = y.set_expected_bucket_owner(try_into_aws(x.expected_bucket_owner)?);
        y = y.set_key(Some(try_into_aws(x.key)?));
        y = y.set_part_number(Some(try_into_aws(x.part_number)?));
        y = y.set_request_payer(try_into_aws(x.request_payer)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key(try_into_aws(x.sse_customer_key)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_upload_id(Some(try_into_aws(x.upload_id)?));
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::UploadPartOutput {
    type Target = UploadPartOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            e_tag: try_from_aws(x.e_tag)?,
            request_charged: try_from_aws(x.request_charged)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::VersioningConfiguration {
    type Target = VersioningConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            mfa_delete: try_from_aws(x.mfa_delete)?,
            status: try_from_aws(x.status)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_mfa_delete(try_into_aws(x.mfa_delete)?);
        y = y.set_status(try_into_aws(x.status)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::WebsiteConfiguration {
    type Target = WebsiteConfiguration;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            error_document: try_from_aws(x.error_document)?,
            index_document: try_from_aws(x.index_document)?,
            redirect_all_requests_to: try_from_aws(x.redirect_all_requests_to)?,
            routing_rules: try_from_aws(x.routing_rules)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_error_document(try_into_aws(x.error_document)?);
        y = y.set_index_document(try_into_aws(x.index_document)?);
        y = y.set_redirect_all_requests_to(try_into_aws(x.redirect_all_requests_to)?);
        y = y.set_routing_rules(try_into_aws(x.routing_rules)?);
        Ok(y.build())
    }
}

impl AwsConversion for s3s::dto::WriteGetObjectResponseInput {
    type Target = WriteGetObjectResponseInput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        Ok(Self {
            accept_ranges: try_from_aws(x.accept_ranges)?,
            body: stream_from_aws(x.body),
            bucket_key_enabled: try_from_aws(x.bucket_key_enabled)?,
            cache_control: try_from_aws(x.cache_control)?,
            checksum_crc32: try_from_aws(x.checksum_crc32)?,
            checksum_crc32c: try_from_aws(x.checksum_crc32_c)?,
            checksum_sha1: try_from_aws(x.checksum_sha1)?,
            checksum_sha256: try_from_aws(x.checksum_sha256)?,
            content_disposition: try_from_aws(x.content_disposition)?,
            content_encoding: try_from_aws(x.content_encoding)?,
            content_language: try_from_aws(x.content_language)?,
            content_length: try_from_aws(x.content_length)?,
            content_range: try_from_aws(x.content_range)?,
            content_type: try_from_aws(x.content_type)?,
            delete_marker: try_from_aws(x.delete_marker)?,
            e_tag: try_from_aws(x.e_tag)?,
            error_code: try_from_aws(x.error_code)?,
            error_message: try_from_aws(x.error_message)?,
            expiration: try_from_aws(x.expiration)?,
            expires: try_from_aws(x.expires)?,
            last_modified: try_from_aws(x.last_modified)?,
            metadata: try_from_aws(x.metadata)?,
            missing_meta: try_from_aws(x.missing_meta)?,
            object_lock_legal_hold_status: try_from_aws(x.object_lock_legal_hold_status)?,
            object_lock_mode: try_from_aws(x.object_lock_mode)?,
            object_lock_retain_until_date: try_from_aws(x.object_lock_retain_until_date)?,
            parts_count: try_from_aws(x.parts_count)?,
            replication_status: try_from_aws(x.replication_status)?,
            request_charged: try_from_aws(x.request_charged)?,
            request_route: unwrap_from_aws(x.request_route, "request_route")?,
            request_token: unwrap_from_aws(x.request_token, "request_token")?,
            restore: try_from_aws(x.restore)?,
            sse_customer_algorithm: try_from_aws(x.sse_customer_algorithm)?,
            sse_customer_key_md5: try_from_aws(x.sse_customer_key_md5)?,
            ssekms_key_id: try_from_aws(x.ssekms_key_id)?,
            server_side_encryption: try_from_aws(x.server_side_encryption)?,
            status_code: try_from_aws(x.status_code)?,
            storage_class: try_from_aws(x.storage_class)?,
            tag_count: try_from_aws(x.tag_count)?,
            version_id: try_from_aws(x.version_id)?,
        })
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let mut y = Self::Target::builder();
        y = y.set_accept_ranges(try_into_aws(x.accept_ranges)?);
        y = y.set_body(try_into_aws(x.body)?);
        y = y.set_bucket_key_enabled(Some(try_into_aws(x.bucket_key_enabled)?));
        y = y.set_cache_control(try_into_aws(x.cache_control)?);
        y = y.set_checksum_crc32(try_into_aws(x.checksum_crc32)?);
        y = y.set_checksum_crc32_c(try_into_aws(x.checksum_crc32c)?);
        y = y.set_checksum_sha1(try_into_aws(x.checksum_sha1)?);
        y = y.set_checksum_sha256(try_into_aws(x.checksum_sha256)?);
        y = y.set_content_disposition(try_into_aws(x.content_disposition)?);
        y = y.set_content_encoding(try_into_aws(x.content_encoding)?);
        y = y.set_content_language(try_into_aws(x.content_language)?);
        y = y.set_content_length(Some(try_into_aws(x.content_length)?));
        y = y.set_content_range(try_into_aws(x.content_range)?);
        y = y.set_content_type(try_into_aws(x.content_type)?);
        y = y.set_delete_marker(Some(try_into_aws(x.delete_marker)?));
        y = y.set_e_tag(try_into_aws(x.e_tag)?);
        y = y.set_error_code(try_into_aws(x.error_code)?);
        y = y.set_error_message(try_into_aws(x.error_message)?);
        y = y.set_expiration(try_into_aws(x.expiration)?);
        y = y.set_expires(try_into_aws(x.expires)?);
        y = y.set_last_modified(try_into_aws(x.last_modified)?);
        y = y.set_metadata(try_into_aws(x.metadata)?);
        y = y.set_missing_meta(Some(try_into_aws(x.missing_meta)?));
        y = y.set_object_lock_legal_hold_status(try_into_aws(x.object_lock_legal_hold_status)?);
        y = y.set_object_lock_mode(try_into_aws(x.object_lock_mode)?);
        y = y.set_object_lock_retain_until_date(try_into_aws(x.object_lock_retain_until_date)?);
        y = y.set_parts_count(Some(try_into_aws(x.parts_count)?));
        y = y.set_replication_status(try_into_aws(x.replication_status)?);
        y = y.set_request_charged(try_into_aws(x.request_charged)?);
        y = y.set_request_route(Some(try_into_aws(x.request_route)?));
        y = y.set_request_token(Some(try_into_aws(x.request_token)?));
        y = y.set_restore(try_into_aws(x.restore)?);
        y = y.set_sse_customer_algorithm(try_into_aws(x.sse_customer_algorithm)?);
        y = y.set_sse_customer_key_md5(try_into_aws(x.sse_customer_key_md5)?);
        y = y.set_ssekms_key_id(try_into_aws(x.ssekms_key_id)?);
        y = y.set_server_side_encryption(try_into_aws(x.server_side_encryption)?);
        y = y.set_status_code(Some(try_into_aws(x.status_code)?));
        y = y.set_storage_class(try_into_aws(x.storage_class)?);
        y = y.set_tag_count(Some(try_into_aws(x.tag_count)?));
        y = y.set_version_id(try_into_aws(x.version_id)?);
        y.build().map_err(S3Error::internal_error)
    }
}

impl AwsConversion for s3s::dto::WriteGetObjectResponseOutput {
    type Target = WriteGetObjectResponseOutput;

    fn try_from_aws(x: Self::Target) -> S3Result<Self> {
        let _ = x;
        Ok(Self {})
    }

    fn try_into_aws(x: Self) -> S3Result<Self::Target> {
        let _ = x;
        let y = Self::Target::builder();
        Ok(y.build())
    }
}
