//! Auto-generated definitions
#![allow(clippy::empty_structs_with_brackets)]

use super::*;

use std::str::FromStr;

pub type AbortDate = Timestamp;

/// <p>Specifies the days since the initiation of an incomplete multipart upload that Amazon S3 will
/// wait before permanently removing all parts of the upload. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config">
/// Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Policy</a> in the
/// <i>Amazon S3 User Guide</i>.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct AbortIncompleteMultipartUpload {
    /// <p>Specifies the number of days after which Amazon S3 aborts an incomplete multipart
    /// upload.</p>
    pub days_after_initiation: DaysAfterInitiation,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct AbortMultipartUploadInput {
    /// <p>The bucket name to which the upload was taking place. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Key of the object for which the multipart upload was initiated.</p>
    pub key: ObjectKey,
    pub request_payer: Option<RequestPayer>,
    /// <p>Upload ID that identifies the multipart upload.</p>
    pub upload_id: MultipartUploadId,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct AbortMultipartUploadOutput {
    pub request_charged: Option<RequestCharged>,
}

pub type AbortRuleId = String;

/// <p>Configures the transfer acceleration state for an Amazon S3 bucket. For more information, see
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/transfer-acceleration.html">Amazon S3
/// Transfer Acceleration</a> in the <i>Amazon S3 User Guide</i>.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct AccelerateConfiguration {
    /// <p>Specifies the transfer acceleration status of the bucket.</p>
    pub status: Option<BucketAccelerateStatus>,
}

pub type AcceptRanges = String;

/// <p>Contains the elements that set the ACL permissions for an object per grantee.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct AccessControlPolicy {
    /// <p>A list of grants.</p>
    pub grants: Option<Grants>,
    /// <p>Container for the bucket owner's display name and ID.</p>
    pub owner: Option<Owner>,
}

/// <p>A container for information about access control for replicas.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct AccessControlTranslation {
    /// <p>Specifies the replica ownership. For default and valid values, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTreplication.html">PUT bucket
    /// replication</a> in the <i>Amazon S3 API Reference</i>.</p>
    pub owner: OwnerOverride,
}

pub type AccessPointArn = String;

pub type AccountId = String;

pub type AllowQuotedRecordDelimiter = bool;

pub type AllowedHeader = String;

pub type AllowedHeaders = List<AllowedHeader>;

pub type AllowedMethod = String;

pub type AllowedMethods = List<AllowedMethod>;

pub type AllowedOrigin = String;

pub type AllowedOrigins = List<AllowedOrigin>;

/// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter.
/// The operator must have at least two predicates in any combination, and an object must match
/// all of the predicates for the filter to apply.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct AnalyticsAndOperator {
    /// <p>The prefix to use when evaluating an AND predicate: The prefix that an object must have
    /// to be included in the metrics results.</p>
    pub prefix: Option<Prefix>,
    /// <p>The list of tags to use when evaluating an AND predicate.</p>
    pub tags: Option<TagSet>,
}

/// <p>Specifies the configuration and any analyses for the analytics filter of an Amazon S3 bucket.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct AnalyticsConfiguration {
    /// <p>The filter used to describe a set of objects for analyses. A filter must have exactly
    /// one prefix, one tag, or one conjunction (AnalyticsAndOperator). If no filter is provided,
    /// all objects will be considered in any analysis.</p>
    pub filter: Option<AnalyticsFilter>,
    /// <p>The ID that identifies the analytics configuration.</p>
    pub id: AnalyticsId,
    /// <p> Contains data related to access patterns to be collected and made available to analyze
    /// the tradeoffs between different storage classes. </p>
    pub storage_class_analysis: StorageClassAnalysis,
}

pub type AnalyticsConfigurationList = List<AnalyticsConfiguration>;

/// <p>Where to publish the analytics results.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct AnalyticsExportDestination {
    /// <p>A destination signifying output to an S3 bucket.</p>
    pub s3_bucket_destination: AnalyticsS3BucketDestination,
}

/// <p>The filter used to describe a set of objects for analyses. A filter must have exactly
/// one prefix, one tag, or one conjunction (AnalyticsAndOperator). If no filter is provided,
/// all objects will be considered in any analysis.</p>
#[derive(Debug)]
#[non_exhaustive]
pub enum AnalyticsFilter {
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating an analytics
    /// filter. The operator must have at least two predicates.</p>
    And(AnalyticsAndOperator),
    /// <p>The prefix to use when evaluating an analytics filter.</p>
    Prefix(Prefix),
    /// <p>The tag to use when evaluating an analytics filter.</p>
    Tag(Tag),
}

pub type AnalyticsId = String;

/// <p>Contains information about where to publish the analytics results.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct AnalyticsS3BucketDestination {
    /// <p>The Amazon Resource Name (ARN) of the bucket to which data is exported.</p>
    pub bucket: BucketName,
    /// <p>The account ID that owns the destination S3 bucket. If no account ID is provided, the
    /// owner is not validated before exporting data.</p>
    /// <note>
    /// <p> Although this value is optional, we strongly recommend that you set it to help
    /// prevent problems if the destination bucket ownership changes. </p>
    /// </note>
    pub bucket_account_id: Option<AccountId>,
    /// <p>Specifies the file format used when exporting data to Amazon S3.</p>
    pub format: AnalyticsS3ExportFileFormat,
    /// <p>The prefix to use when exporting data. The prefix is prepended to all results.</p>
    pub prefix: Option<Prefix>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalyticsS3ExportFileFormat {
    Csv,
}

impl AnalyticsS3ExportFileFormat {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Csv => "CSV",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"CSV" => Some(Self::Csv),
            _ => None,
        }
    }
}

impl FromStr for AnalyticsS3ExportFileFormat {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveStatus {
    ArchiveAccess,
    DeepArchiveAccess,
}

impl ArchiveStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ArchiveAccess => "ARCHIVE_ACCESS",
            Self::DeepArchiveAccess => "DEEP_ARCHIVE_ACCESS",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"ARCHIVE_ACCESS" => Some(Self::ArchiveAccess),
            b"DEEP_ARCHIVE_ACCESS" => Some(Self::DeepArchiveAccess),
            _ => None,
        }
    }
}

impl FromStr for ArchiveStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p> In terms of implementation, a Bucket is a resource. An Amazon S3 bucket name is globally
/// unique, and the namespace is shared by all Amazon Web Services accounts. </p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Bucket {
    /// <p>Date the bucket was created. This date can change when making changes to your bucket, such as editing its bucket policy.</p>
    pub creation_date: Option<CreationDate>,
    /// <p>The name of the bucket.</p>
    pub name: Option<BucketName>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BucketAccelerateStatus {
    Enabled,
    Suspended,
}

impl BucketAccelerateStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Suspended => "Suspended",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Enabled" => Some(Self::Enabled),
            b"Suspended" => Some(Self::Suspended),
            _ => None,
        }
    }
}

impl FromStr for BucketAccelerateStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>The requested bucket name is not available. The bucket namespace is shared by all users
/// of the system. Select a different name and try again.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct BucketAlreadyExists {}

/// <p>The bucket you tried to create already exists, and you own it. Amazon S3 returns this error
/// in all Amazon Web Services Regions except in the North Virginia Region. For legacy compatibility, if you
/// re-create an existing bucket that you already own in the North Virginia Region, Amazon S3
/// returns 200 OK and resets the bucket access control lists (ACLs).</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct BucketAlreadyOwnedByYou {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BucketCannedACL {
    AuthenticatedRead,
    Private,
    PublicRead,
    PublicReadWrite,
}

impl BucketCannedACL {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AuthenticatedRead => "authenticated-read",
            Self::Private => "private",
            Self::PublicRead => "public-read",
            Self::PublicReadWrite => "public-read-write",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"authenticated-read" => Some(Self::AuthenticatedRead),
            b"private" => Some(Self::Private),
            b"public-read" => Some(Self::PublicRead),
            b"public-read-write" => Some(Self::PublicReadWrite),
            _ => None,
        }
    }
}

impl FromStr for BucketCannedACL {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type BucketKeyEnabled = bool;

/// <p>Specifies the lifecycle configuration for objects in an Amazon S3 bucket. For more
/// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html">Object Lifecycle Management</a>
/// in the <i>Amazon S3 User Guide</i>.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct BucketLifecycleConfiguration {
    /// <p>A lifecycle rule for individual objects in an Amazon S3 bucket.</p>
    pub rules: LifecycleRules,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BucketLocationConstraint {
    Eu,
    AfSouth1,
    ApEast1,
    ApNortheast1,
    ApNortheast2,
    ApNortheast3,
    ApSouth1,
    ApSoutheast1,
    ApSoutheast2,
    ApSoutheast3,
    CaCentral1,
    CnNorth1,
    CnNorthwest1,
    EuCentral1,
    EuNorth1,
    EuSouth1,
    EuWest1,
    EuWest2,
    EuWest3,
    MeSouth1,
    SaEast1,
    UsEast2,
    UsGovEast1,
    UsGovWest1,
    UsWest1,
    UsWest2,
}

impl BucketLocationConstraint {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Eu => "EU",
            Self::AfSouth1 => "af-south-1",
            Self::ApEast1 => "ap-east-1",
            Self::ApNortheast1 => "ap-northeast-1",
            Self::ApNortheast2 => "ap-northeast-2",
            Self::ApNortheast3 => "ap-northeast-3",
            Self::ApSouth1 => "ap-south-1",
            Self::ApSoutheast1 => "ap-southeast-1",
            Self::ApSoutheast2 => "ap-southeast-2",
            Self::ApSoutheast3 => "ap-southeast-3",
            Self::CaCentral1 => "ca-central-1",
            Self::CnNorth1 => "cn-north-1",
            Self::CnNorthwest1 => "cn-northwest-1",
            Self::EuCentral1 => "eu-central-1",
            Self::EuNorth1 => "eu-north-1",
            Self::EuSouth1 => "eu-south-1",
            Self::EuWest1 => "eu-west-1",
            Self::EuWest2 => "eu-west-2",
            Self::EuWest3 => "eu-west-3",
            Self::MeSouth1 => "me-south-1",
            Self::SaEast1 => "sa-east-1",
            Self::UsEast2 => "us-east-2",
            Self::UsGovEast1 => "us-gov-east-1",
            Self::UsGovWest1 => "us-gov-west-1",
            Self::UsWest1 => "us-west-1",
            Self::UsWest2 => "us-west-2",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"EU" => Some(Self::Eu),
            b"af-south-1" => Some(Self::AfSouth1),
            b"ap-east-1" => Some(Self::ApEast1),
            b"ap-northeast-1" => Some(Self::ApNortheast1),
            b"ap-northeast-2" => Some(Self::ApNortheast2),
            b"ap-northeast-3" => Some(Self::ApNortheast3),
            b"ap-south-1" => Some(Self::ApSouth1),
            b"ap-southeast-1" => Some(Self::ApSoutheast1),
            b"ap-southeast-2" => Some(Self::ApSoutheast2),
            b"ap-southeast-3" => Some(Self::ApSoutheast3),
            b"ca-central-1" => Some(Self::CaCentral1),
            b"cn-north-1" => Some(Self::CnNorth1),
            b"cn-northwest-1" => Some(Self::CnNorthwest1),
            b"eu-central-1" => Some(Self::EuCentral1),
            b"eu-north-1" => Some(Self::EuNorth1),
            b"eu-south-1" => Some(Self::EuSouth1),
            b"eu-west-1" => Some(Self::EuWest1),
            b"eu-west-2" => Some(Self::EuWest2),
            b"eu-west-3" => Some(Self::EuWest3),
            b"me-south-1" => Some(Self::MeSouth1),
            b"sa-east-1" => Some(Self::SaEast1),
            b"us-east-2" => Some(Self::UsEast2),
            b"us-gov-east-1" => Some(Self::UsGovEast1),
            b"us-gov-west-1" => Some(Self::UsGovWest1),
            b"us-west-1" => Some(Self::UsWest1),
            b"us-west-2" => Some(Self::UsWest2),
            _ => None,
        }
    }
}

impl FromStr for BucketLocationConstraint {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>Container for logging status information.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct BucketLoggingStatus {
    pub logging_enabled: Option<LoggingEnabled>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BucketLogsPermission {
    FullControl,
    Read,
    Write,
}

impl BucketLogsPermission {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::FullControl => "FULL_CONTROL",
            Self::Read => "READ",
            Self::Write => "WRITE",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"FULL_CONTROL" => Some(Self::FullControl),
            b"READ" => Some(Self::Read),
            b"WRITE" => Some(Self::Write),
            _ => None,
        }
    }
}

impl FromStr for BucketLogsPermission {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type BucketName = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BucketVersioningStatus {
    Enabled,
    Suspended,
}

impl BucketVersioningStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Suspended => "Suspended",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Enabled" => Some(Self::Enabled),
            b"Suspended" => Some(Self::Suspended),
            _ => None,
        }
    }
}

impl FromStr for BucketVersioningStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type Buckets = List<Bucket>;

pub type BypassGovernanceRetention = bool;

pub type BytesProcessed = i64;

pub type BytesReturned = i64;

pub type BytesScanned = i64;

/// <p>Describes the cross-origin access configuration for objects in an Amazon S3 bucket. For more
/// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cors.html">Enabling
/// Cross-Origin Resource Sharing</a> in the <i>Amazon S3 User Guide</i>.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct CORSConfiguration {
    /// <p>A set of origins and methods (cross-origin access that you want to allow). You can add
    /// up to 100 rules to the configuration.</p>
    pub cors_rules: CORSRules,
}

/// <p>Specifies a cross-origin access rule for an Amazon S3 bucket.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct CORSRule {
    /// <p>Headers that are specified in the <code>Access-Control-Request-Headers</code> header.
    /// These headers are allowed in a preflight OPTIONS request. In response to any preflight
    /// OPTIONS request, Amazon S3 returns any requested headers that are allowed.</p>
    pub allowed_headers: Option<AllowedHeaders>,
    /// <p>An HTTP method that you allow the origin to execute. Valid values are <code>GET</code>,
    /// <code>PUT</code>, <code>HEAD</code>, <code>POST</code>, and <code>DELETE</code>.</p>
    pub allowed_methods: AllowedMethods,
    /// <p>One or more origins you want customers to be able to access the bucket from.</p>
    pub allowed_origins: AllowedOrigins,
    /// <p>One or more headers in the response that you want customers to be able to access from
    /// their applications (for example, from a JavaScript <code>XMLHttpRequest</code>
    /// object).</p>
    pub expose_headers: Option<ExposeHeaders>,
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub id: Option<ID>,
    /// <p>The time in seconds that your browser is to cache the preflight response for the
    /// specified resource.</p>
    pub max_age_seconds: MaxAgeSeconds,
}

pub type CORSRules = List<CORSRule>;

/// <p>Describes how an uncompressed comma-separated values (CSV)-formatted input object is
/// formatted.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CSVInput {
    /// <p>Specifies that CSV field values may contain quoted record delimiters and such records
    /// should be allowed. Default value is FALSE. Setting this value to TRUE may lower
    /// performance.</p>
    pub allow_quoted_record_delimiter: AllowQuotedRecordDelimiter,
    /// <p>A single character used to indicate that a row should be ignored when the character is
    /// present at the start of that row. You can specify any character to indicate a comment
    /// line.</p>
    pub comments: Option<Comments>,
    /// <p>A single character used to separate individual fields in a record. You can specify an
    /// arbitrary delimiter.</p>
    pub field_delimiter: Option<FieldDelimiter>,
    /// <p>Describes the first line of input. Valid values are:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>NONE</code>: First line is not a header.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>IGNORE</code>: First line is a header, but you can't use the header values
    /// to indicate the column in an expression. You can use column position (such as _1, _2,
    /// …) to indicate the column (<code>SELECT s._1 FROM OBJECT s</code>).</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>Use</code>: First line is a header, and you can use the header value to
    /// identify a column in an expression (<code>SELECT "name" FROM OBJECT</code>). </p>
    /// </li>
    /// </ul>
    pub file_header_info: Option<FileHeaderInfo>,
    /// <p>A single character used for escaping when the field delimiter is part of the value. For
    /// example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks,
    /// as follows: <code>" a , b "</code>.</p>
    /// <p>Type: String</p>
    /// <p>Default: <code>"</code>
    /// </p>
    /// <p>Ancestors: <code>CSV</code>
    /// </p>
    pub quote_character: Option<QuoteCharacter>,
    /// <p>A single character used for escaping the quotation mark character inside an already
    /// escaped value. For example, the value <code>""" a , b """</code> is parsed as <code>" a , b
    /// "</code>.</p>
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
    /// <p>A single character used to separate individual records in the input. Instead of the
    /// default value, you can specify an arbitrary delimiter.</p>
    pub record_delimiter: Option<RecordDelimiter>,
}

/// <p>Describes how uncompressed comma-separated values (CSV)-formatted results are
/// formatted.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CSVOutput {
    /// <p>The value used to separate individual fields in a record. You can specify an arbitrary
    /// delimiter.</p>
    pub field_delimiter: Option<FieldDelimiter>,
    /// <p>A single character used for escaping when the field delimiter is part of the value. For
    /// example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks,
    /// as follows: <code>" a , b "</code>.</p>
    pub quote_character: Option<QuoteCharacter>,
    /// <p>The single character used for escaping the quote character inside an already escaped
    /// value.</p>
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
    /// <p>Indicates whether to use quotation marks around output fields. </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>ALWAYS</code>: Always use quotation marks for output fields.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>ASNEEDED</code>: Use quotation marks for output fields when needed.</p>
    /// </li>
    /// </ul>
    pub quote_fields: Option<QuoteFields>,
    /// <p>A single character used to separate individual records in the output. Instead of the
    /// default value, you can specify an arbitrary delimiter.</p>
    pub record_delimiter: Option<RecordDelimiter>,
}

pub type CacheControl = String;

/// <p>Contains all the possible checksum or digest values for an object.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Checksum {
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumAlgorithm {
    Crc32,
    Crc32C,
    Sha1,
    Sha256,
}

impl ChecksumAlgorithm {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Crc32 => "CRC32",
            Self::Crc32C => "CRC32C",
            Self::Sha1 => "SHA1",
            Self::Sha256 => "SHA256",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"CRC32" => Some(Self::Crc32),
            b"CRC32C" => Some(Self::Crc32C),
            b"SHA1" => Some(Self::Sha1),
            b"SHA256" => Some(Self::Sha256),
            _ => None,
        }
    }
}

impl FromStr for ChecksumAlgorithm {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type ChecksumAlgorithmList = List<ChecksumAlgorithm>;

pub type ChecksumCRC32 = String;

pub type ChecksumCRC32C = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumMode {
    Enabled,
}

impl ChecksumMode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "ENABLED",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"ENABLED" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for ChecksumMode {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type ChecksumSHA1 = String;

pub type ChecksumSHA256 = String;

pub type Code = String;

pub type Comments = String;

/// <p>Container for all (if there are any) keys between Prefix and the next occurrence of the
/// string specified by a delimiter. CommonPrefixes lists keys that act like subdirectories in
/// the directory specified by Prefix. For example, if the prefix is notes/ and the delimiter
/// is a slash (/) as in notes/summer/july, the common prefix is notes/summer/. </p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CommonPrefix {
    /// <p>Container for the specified common prefix.</p>
    pub prefix: Option<Prefix>,
}

pub type CommonPrefixList = List<CommonPrefix>;

#[derive(Debug)]
#[non_exhaustive]
pub struct CompleteMultipartUploadInput {
    /// <p>Name of the bucket to which the multipart upload was initiated.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 32-bit CRC32C checksum of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 160-bit SHA-1 digest of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: ObjectKey,
    /// <p>The container for the multipart upload request information.</p>
    pub multipart_upload: Option<CompletedMultipartUpload>,
    pub request_payer: Option<RequestPayer>,
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created
    /// using a checksum algorithm. For more information,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm.
    /// For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub sse_customer_key: Option<SSECustomerKey>,
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum
    /// algorithm. For more information,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>ID for the initiated multipart upload.</p>
    pub upload_id: MultipartUploadId,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CompleteMultipartUploadOutput {
    /// <p>The name of the bucket that contains the newly created object. Does not return the access point ARN or access point alias if used.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: Option<BucketName>,
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Entity tag that identifies the newly created object's data. Objects with different
    /// object data will have different entity tags. The entity tag is an opaque string. The entity
    /// tag may or may not be an MD5 digest of the object data. If the entity tag is not an MD5
    /// digest of the object data, it will contain one or more nonhexadecimal characters and/or
    /// will consist of less than 32 or more than 32 hexadecimal digits. For more information about
    /// how the entity tag is calculated, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking
    /// object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub e_tag: Option<ETag>,
    /// <p>If the object expiration is configured, this will contain the expiration date
    /// (<code>expiry-date</code>) and rule ID (<code>rule-id</code>). The value of
    /// <code>rule-id</code> is URL-encoded.</p>
    pub expiration: Option<Expiration>,
    /// <p>The object key of the newly created object.</p>
    pub key: Option<ObjectKey>,
    /// <p>The URI that identifies the newly created object.</p>
    pub location: Option<Location>,
    pub request_charged: Option<RequestCharged>,
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric
    /// customer managed key that was used for the object.</p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p>If you specified server-side encryption either with an Amazon S3-managed encryption key or an
    /// Amazon Web Services KMS key in your initiate multipart upload request, the response
    /// includes this header. It confirms the encryption algorithm that Amazon S3 used to encrypt the
    /// object.</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// <p>Version ID of the newly created object, in case the bucket has versioning turned
    /// on.</p>
    pub version_id: Option<ObjectVersionId>,
}

/// <p>The container for the completed multipart upload details.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CompletedMultipartUpload {
    /// <p>Array of CompletedPart data types.</p>
    /// <p>If you do not supply a valid <code>Part</code> with your request, the service sends back an HTTP
    /// 400 response.</p>
    pub parts: Option<CompletedPartList>,
}

/// <p>Details of the parts that were uploaded.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CompletedPart {
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub e_tag: Option<ETag>,
    /// <p>Part number that identifies the part. This is a positive integer between 1 and
    /// 10,000.</p>
    pub part_number: PartNumber,
}

pub type CompletedPartList = List<CompletedPart>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    Bzip2,
    Gzip,
    None,
}

impl CompressionType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Bzip2 => "BZIP2",
            Self::Gzip => "GZIP",
            Self::None => "NONE",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"BZIP2" => Some(Self::Bzip2),
            b"GZIP" => Some(Self::Gzip),
            b"NONE" => Some(Self::None),
            _ => None,
        }
    }
}

impl FromStr for CompressionType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>A container for describing a condition that must be met for the specified redirect to
/// apply. For example, 1. If request is for pages in the <code>/docs</code> folder, redirect
/// to the <code>/documents</code> folder. 2. If request results in HTTP error 4xx, redirect
/// request to another host where you might process the error.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Condition {
    /// <p>The HTTP error code when the redirect is applied. In the event of an error, if the error
    /// code equals this value, then the specified redirect is applied. Required when parent
    /// element <code>Condition</code> is specified and sibling <code>KeyPrefixEquals</code> is not
    /// specified. If both are specified, then both must be true for the redirect to be
    /// applied.</p>
    pub http_error_code_returned_equals: Option<HttpErrorCodeReturnedEquals>,
    /// <p>The object key name prefix when the redirect is applied. For example, to redirect
    /// requests for <code>ExamplePage.html</code>, the key prefix will be
    /// <code>ExamplePage.html</code>. To redirect request for all pages with the prefix
    /// <code>docs/</code>, the key prefix will be <code>/docs</code>, which identifies all
    /// objects in the <code>docs/</code> folder. Required when the parent element
    /// <code>Condition</code> is specified and sibling <code>HttpErrorCodeReturnedEquals</code>
    /// is not specified. If both conditions are specified, both must be true for the redirect to
    /// be applied.</p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    pub key_prefix_equals: Option<KeyPrefixEquals>,
}

pub type ConfirmRemoveSelfBucketAccess = bool;

pub type ContentDisposition = String;

pub type ContentEncoding = String;

pub type ContentLanguage = String;

pub type ContentLength = i64;

pub type ContentMD5 = String;

pub type ContentRange = String;

/// <p></p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ContinuationEvent {}

#[derive(Debug)]
#[non_exhaustive]
pub struct CopyObjectInput {
    /// <p>The canned ACL to apply to the object.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub acl: Option<ObjectCannedACL>,
    /// <p>The name of the destination bucket.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Specifies whether Amazon S3 should use an S3 Bucket Key for object encryption with server-side encryption using AWS KMS (SSE-KMS). Setting this header to <code>true</code> causes Amazon S3 to use an S3 Bucket Key for object encryption with SSE-KMS. </p>
    /// <p>Specifying this header with a COPY action doesn’t affect bucket-level settings for S3 Bucket Key.</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<CacheControl>,
    /// <p>Indicates the algorithm you want Amazon S3 to use to create the checksum for the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<ContentDisposition>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding
    /// mechanisms must be applied to obtain the media-type referenced by the Content-Type header
    /// field.</p>
    pub content_encoding: Option<ContentEncoding>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<ContentLanguage>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<ContentType>,
    /// <p>Specifies the source object for the copy operation. You specify the value in one of two
    /// formats, depending on whether you want to access the source object through an <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points.html">access point</a>:</p>
    /// <ul>
    /// <li>
    /// <p>For objects not accessed through an access point, specify the name of the source bucket
    /// and the key of the source object, separated by a slash (/). For example, to copy the
    /// object <code>reports/january.pdf</code> from the bucket
    /// <code>awsexamplebucket</code>, use <code>awsexamplebucket/reports/january.pdf</code>.
    /// The value must be URL-encoded.</p>
    /// </li>
    /// <li>
    /// <p>For objects accessed through access points, specify the Amazon Resource Name (ARN) of the object as accessed through the access point, in the format <code>arn:aws:s3:&lt;Region&gt;:&lt;account-id&gt;:accesspoint/&lt;access-point-name&gt;/object/&lt;key&gt;</code>. For example, to copy the object <code>reports/january.pdf</code> through access point <code>my-access-point</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3:us-west-2:123456789012:accesspoint/my-access-point/object/reports/january.pdf</code>. The value must be URL encoded.</p>
    /// <note>
    /// <p>Amazon S3 supports copy operations using access points only when the source and destination buckets are in the same Amazon Web Services Region.</p>
    /// </note>
    /// <p>Alternatively, for objects accessed through Amazon S3 on Outposts, specify the ARN of the object as accessed in the format <code>arn:aws:s3-outposts:&lt;Region&gt;:&lt;account-id&gt;:outpost/&lt;outpost-id&gt;/object/&lt;key&gt;</code>. For example, to copy the object <code>reports/january.pdf</code> through outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/object/reports/january.pdf</code>. The value must be URL-encoded.  </p>
    /// </li>
    /// </ul>
    /// <p>To copy a specific version of an object, append <code>?versionId=&lt;version-id&gt;</code>
    /// to the value (for example,
    /// <code>awsexamplebucket/reports/january.pdf?versionId=QUpfdndhfd8438MNFDN93jdnJFkdmqnh893</code>).
    /// If you don't specify a version ID, Amazon S3 copies the latest version of the source
    /// object.</p>
    pub copy_source: CopySource,
    /// <p>Copies the object if its entity tag (ETag) matches the specified tag.</p>
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    /// <p>Copies the object if it has been modified since the specified time.</p>
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    /// <p>Copies the object if its entity tag (ETag) is different than the specified ETag.</p>
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    /// <p>Copies the object if it hasn't been modified since the specified time.</p>
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    /// <p>Specifies the algorithm to use when decrypting the source object (for example,
    /// AES256).</p>
    pub copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source
    /// object. The encryption key provided in this header must be one that was used when the
    /// source object was created.</p>
    pub copy_source_sse_customer_key: Option<CopySourceSSECustomerKey>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses
    /// this header for a message integrity check to ensure that the encryption key was transmitted
    /// without error.</p>
    pub copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5>,
    /// <p>The account ID of the expected destination bucket owner. If the destination bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The account ID of the expected source bucket owner. If the source bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_source_bucket_owner: Option<AccountId>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<Expires>,
    /// <p>Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the
    /// object.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_full_control: Option<GrantFullControl>,
    /// <p>Allows grantee to read the object data and its
    /// metadata.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_read: Option<GrantRead>,
    /// <p>Allows grantee to read the object ACL.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_read_acp: Option<GrantReadACP>,
    /// <p>Allows grantee to write the ACL for the applicable
    /// object.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_write_acp: Option<GrantWriteACP>,
    /// <p>The key of the destination object.</p>
    pub key: ObjectKey,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<Metadata>,
    /// <p>Specifies whether the metadata is copied from the source object or replaced with
    /// metadata provided in the request.</p>
    pub metadata_directive: Option<MetadataDirective>,
    /// <p>Specifies whether you want to apply a legal hold to the copied object.</p>
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    /// <p>The Object Lock mode that you want to apply to the copied object.</p>
    pub object_lock_mode: Option<ObjectLockMode>,
    /// <p>The date and time when you want the copied object's Object Lock to expire.</p>
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub request_payer: Option<RequestPayer>,
    /// <p>Specifies the algorithm to use to when encrypting the object (for example,
    /// AES256).</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This
    /// value is used to store the object and then it is discarded; Amazon S3 does not store the
    /// encryption key. The key must be appropriate for use with the algorithm specified in the
    /// <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub sse_customer_key: Option<SSECustomerKey>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses
    /// this header for a message integrity check to ensure that the encryption key was transmitted
    /// without error.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>Specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this
    /// header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value
    /// pairs.</p>
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    /// <p>Specifies the Amazon Web Services KMS key ID to use for object encryption. All GET and PUT requests for
    /// an object protected by Amazon Web Services KMS will fail if not made via SSL or using SigV4. For
    /// information about configuring using any of the officially supported Amazon Web Services SDKs and Amazon Web Services CLI,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version">Specifying the
    /// Signature Version in Request Authentication</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example,
    /// AES256, aws:kms).</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// <p>By default, Amazon S3 uses the STANDARD Storage Class to store newly created objects. The
    /// STANDARD storage class provides high durability and high availability. Depending on
    /// performance needs, you can specify a different Storage Class. Amazon S3 on Outposts only uses
    /// the OUTPOSTS Storage Class. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html">Storage Classes</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub storage_class: Option<StorageClass>,
    /// <p>The tag-set for the object destination object this value must be used in conjunction
    /// with the <code>TaggingDirective</code>. The tag-set must be encoded as URL Query
    /// parameters.</p>
    pub tagging: Option<TaggingHeader>,
    /// <p>Specifies whether the object tag-set are copied from the source object or replaced with
    /// tag-set provided in the request.</p>
    pub tagging_directive: Option<TaggingDirective>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another
    /// object in the same bucket or to an external URL. Amazon S3 stores the value of this header in
    /// the object metadata.</p>
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CopyObjectOutput {
    /// <p>Indicates whether the copied object uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p>Container for all response elements.</p>
    pub copy_object_result: Option<CopyObjectResult>,
    /// <p>Version of the copied object in the destination bucket.</p>
    pub copy_source_version_id: Option<CopySourceVersionId>,
    /// <p>If the object expiration is configured, the response includes this header.</p>
    pub expiration: Option<Expiration>,
    pub request_charged: Option<RequestCharged>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header to provide round-trip message integrity verification of
    /// the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>If present, specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The
    /// value of this header is a base64-encoded UTF-8 string holding JSON with the encryption
    /// context key-value pairs.</p>
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric
    /// customer managed key that was used for the object.</p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example,
    /// AES256, aws:kms).</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// <p>Version ID of the newly created copy.</p>
    pub version_id: Option<ObjectVersionId>,
}

/// <p>Container for all response elements.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CopyObjectResult {
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Returns the ETag of the new object. The ETag reflects only changes to the contents of an object, not its metadata.</p>
    pub e_tag: Option<ETag>,
    /// <p>Creation date of the object.</p>
    pub last_modified: Option<LastModified>,
}

/// <p>Container for all response elements.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CopyPartResult {
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Entity tag of the object.</p>
    pub e_tag: Option<ETag>,
    /// <p>Date and time at which the object was uploaded.</p>
    pub last_modified: Option<LastModified>,
}

pub type CopySourceIfMatch = String;

pub type CopySourceIfModifiedSince = Timestamp;

pub type CopySourceIfNoneMatch = String;

pub type CopySourceIfUnmodifiedSince = Timestamp;

pub type CopySourceRange = String;

pub type CopySourceSSECustomerAlgorithm = String;

pub type CopySourceSSECustomerKey = String;

pub type CopySourceSSECustomerKeyMD5 = String;

pub type CopySourceVersionId = String;

/// <p>The configuration information for the bucket.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CreateBucketConfiguration {
    /// <p>Specifies the Region where the bucket will be created. If you don't specify a Region,
    /// the bucket is created in the US East (N. Virginia) Region (us-east-1).</p>
    pub location_constraint: Option<BucketLocationConstraint>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct CreateBucketInput {
    /// <p>The canned ACL to apply to the bucket.</p>
    pub acl: Option<BucketCannedACL>,
    /// <p>The name of the bucket to create.</p>
    pub bucket: BucketName,
    /// <p>The configuration information for the bucket.</p>
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the
    /// bucket.</p>
    pub grant_full_control: Option<GrantFullControl>,
    /// <p>Allows grantee to list the objects in the bucket.</p>
    pub grant_read: Option<GrantRead>,
    /// <p>Allows grantee to read the bucket ACL.</p>
    pub grant_read_acp: Option<GrantReadACP>,
    /// <p>Allows grantee to create new objects in the bucket.</p>
    /// <p>For the bucket and object owners of existing objects, also allows deletions and overwrites of those objects.</p>
    pub grant_write: Option<GrantWrite>,
    /// <p>Allows grantee to write the ACL for the applicable bucket.</p>
    pub grant_write_acp: Option<GrantWriteACP>,
    /// <p>Specifies whether you want S3 Object Lock to be enabled for the new bucket.</p>
    pub object_lock_enabled_for_bucket: ObjectLockEnabledForBucket,
    pub object_ownership: Option<ObjectOwnership>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CreateBucketOutput {
    /// <p>A forward slash followed by the name of the bucket.</p>
    pub location: Option<Location>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct CreateMultipartUploadInput {
    /// <p>The canned ACL to apply to the object.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub acl: Option<ObjectCannedACL>,
    /// <p>The name of the bucket to which to initiate the upload</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Specifies whether Amazon S3 should use an S3 Bucket Key for object encryption with server-side encryption using AWS KMS (SSE-KMS). Setting this header to <code>true</code> causes Amazon S3 to use an S3 Bucket Key for object encryption with SSE-KMS.</p>
    /// <p>Specifying this header with an object action doesn’t affect bucket-level settings for S3 Bucket Key.</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<CacheControl>,
    /// <p>Indicates the algorithm you want Amazon S3 to use to create the checksum for the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<ContentDisposition>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding
    /// mechanisms must be applied to obtain the media-type referenced by the Content-Type header
    /// field.</p>
    pub content_encoding: Option<ContentEncoding>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<ContentLanguage>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<ContentType>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<Expires>,
    /// <p>Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the
    /// object.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_full_control: Option<GrantFullControl>,
    /// <p>Allows grantee to read the object data and its
    /// metadata.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_read: Option<GrantRead>,
    /// <p>Allows grantee to read the object ACL.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_read_acp: Option<GrantReadACP>,
    /// <p>Allows grantee to write the ACL for the applicable
    /// object.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_write_acp: Option<GrantWriteACP>,
    /// <p>Object key for which the multipart upload is to be initiated.</p>
    pub key: ObjectKey,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<Metadata>,
    /// <p>Specifies whether you want to apply a legal hold to the uploaded object.</p>
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    /// <p>Specifies the Object Lock mode that you want to apply to the uploaded object.</p>
    pub object_lock_mode: Option<ObjectLockMode>,
    /// <p>Specifies the date and time when you want the Object Lock to expire.</p>
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub request_payer: Option<RequestPayer>,
    /// <p>Specifies the algorithm to use to when encrypting the object (for example,
    /// AES256).</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This
    /// value is used to store the object and then it is discarded; Amazon S3 does not store the
    /// encryption key. The key must be appropriate for use with the algorithm specified in the
    /// <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub sse_customer_key: Option<SSECustomerKey>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses
    /// this header for a message integrity check to ensure that the encryption key was transmitted
    /// without error.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>Specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this
    /// header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value
    /// pairs.</p>
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    /// <p>Specifies the ID of the symmetric customer managed key to use for object
    /// encryption. All GET and PUT requests for an object protected by Amazon Web Services KMS will fail if not
    /// made via SSL or using SigV4. For information about configuring using any of the officially
    /// supported Amazon Web Services SDKs and Amazon Web Services CLI, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version">Specifying the Signature Version in Request Authentication</a>
    /// in the <i>Amazon S3 User Guide</i>.</p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example,
    /// AES256, aws:kms).</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// <p>By default, Amazon S3 uses the STANDARD Storage Class to store newly created objects. The
    /// STANDARD storage class provides high durability and high availability. Depending on
    /// performance needs, you can specify a different Storage Class. Amazon S3 on Outposts only uses
    /// the OUTPOSTS Storage Class. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html">Storage Classes</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub storage_class: Option<StorageClass>,
    /// <p>The tag-set for the object. The tag-set must be encoded as URL Query parameters.</p>
    pub tagging: Option<TaggingHeader>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another
    /// object in the same bucket or to an external URL. Amazon S3 stores the value of this header in
    /// the object metadata.</p>
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CreateMultipartUploadOutput {
    /// <p>If the bucket has a lifecycle rule configured with an action to abort incomplete
    /// multipart uploads and the prefix in the lifecycle rule matches the object name in the
    /// request, the response includes this header. The header indicates when the initiated
    /// multipart upload becomes eligible for an abort operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config">
    /// Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Policy</a>.</p>
    ///
    /// <p>The response also includes the <code>x-amz-abort-rule-id</code> header that provides the
    /// ID of the lifecycle configuration rule that defines this action.</p>
    pub abort_date: Option<AbortDate>,
    /// <p>This header is returned along with the <code>x-amz-abort-date</code> header. It
    /// identifies the applicable lifecycle configuration rule that defines the action to abort
    /// incomplete multipart uploads.</p>
    pub abort_rule_id: Option<AbortRuleId>,
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: Option<BucketName>,
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: Option<ObjectKey>,
    pub request_charged: Option<RequestCharged>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header to provide round-trip message integrity verification of
    /// the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>If present, specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The
    /// value of this header is a base64-encoded UTF-8 string holding JSON with the encryption
    /// context key-value pairs.</p>
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric
    /// customer managed key that was used for the object.</p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example,
    /// AES256, aws:kms).</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// <p>ID for the initiated multipart upload.</p>
    pub upload_id: Option<MultipartUploadId>,
}

pub type CreationDate = Timestamp;

pub type Date = Timestamp;

pub type Days = i32;

pub type DaysAfterInitiation = i32;

/// <p>The container element for specifying the default Object Lock retention settings for new
/// objects placed in the specified bucket.</p>
/// <note>
/// <ul>
/// <li>
/// <p>The <code>DefaultRetention</code> settings require both a mode and a
/// period.</p>
/// </li>
/// <li>
/// <p>The <code>DefaultRetention</code> period can be either <code>Days</code>
/// or <code>Years</code> but you must select one. You cannot specify <code>Days</code>
/// and <code>Years</code> at the same time.</p>
/// </li>
/// </ul>
/// </note>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct DefaultRetention {
    /// <p>The number of days that you want to specify for the default retention period. Must be
    /// used with <code>Mode</code>.</p>
    pub days: Days,
    /// <p>The default Object Lock retention mode you want to apply to new objects placed in the
    /// specified bucket. Must be used with either <code>Days</code> or <code>Years</code>.</p>
    pub mode: Option<ObjectLockRetentionMode>,
    /// <p>The number of years that you want to specify for the default retention period. Must be
    /// used with <code>Mode</code>.</p>
    pub years: Years,
}

/// <p>Container for the objects to delete.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct Delete {
    /// <p>The objects to delete.</p>
    pub objects: ObjectIdentifierList,
    /// <p>Element to enable quiet mode for the request. When you add this element, you must set
    /// its value to true.</p>
    pub quiet: Quiet,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketAnalyticsConfigurationInput {
    /// <p>The name of the bucket from which an analytics configuration is deleted.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID that identifies the analytics configuration.</p>
    pub id: AnalyticsId,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketCorsInput {
    /// <p>Specifies the bucket whose <code>cors</code> configuration is being deleted.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketEncryptionInput {
    /// <p>The name of the bucket containing the server-side encryption configuration to
    /// delete.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketInput {
    /// <p>Specifies the bucket being deleted.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketIntelligentTieringConfigurationInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub bucket: BucketName,
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub id: IntelligentTieringId,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketInventoryConfigurationInput {
    /// <p>The name of the bucket containing the inventory configuration to delete.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: InventoryId,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketLifecycleInput {
    /// <p>The bucket name of the lifecycle to delete.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketMetricsConfigurationInput {
    /// <p>The name of the bucket containing the metrics configuration to delete.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: MetricsId,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketOwnershipControlsInput {
    /// <p>The Amazon S3 bucket whose <code>OwnershipControls</code> you want to delete. </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketPolicyInput {
    /// <p>The bucket name.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketReplicationInput {
    /// <p> The bucket name. </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketTaggingInput {
    /// <p>The bucket that has the tag set to be removed.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteBucketWebsiteInput {
    /// <p>The bucket name for which you want to remove the website configuration. </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

pub type DeleteMarker = bool;

/// <p>Information about the delete marker.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct DeleteMarkerEntry {
    /// <p>Specifies whether the object is (true) or is not (false) the latest version of an
    /// object.</p>
    pub is_latest: IsLatest,
    /// <p>The object key.</p>
    pub key: Option<ObjectKey>,
    /// <p>Date and time the object was last modified.</p>
    pub last_modified: Option<LastModified>,
    /// <p>The account that created the delete marker.></p>
    pub owner: Option<Owner>,
    /// <p>Version ID of an object.</p>
    pub version_id: Option<ObjectVersionId>,
}

/// <p>Specifies whether Amazon S3 replicates delete markers. If you specify a <code>Filter</code>
/// in your replication configuration, you must also include a
/// <code>DeleteMarkerReplication</code> element. If your <code>Filter</code> includes a
/// <code>Tag</code> element, the <code>DeleteMarkerReplication</code>
/// <code>Status</code> must be set to Disabled, because Amazon S3 does not support replicating
/// delete markers for tag-based rules. For an example configuration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-add-config.html#replication-config-min-rule-config">Basic Rule Configuration</a>. </p>
/// <p>For more information about delete marker replication, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/delete-marker-replication.html">Basic Rule
/// Configuration</a>. </p>
/// <note>
/// <p>If you are using an earlier version of the replication configuration, Amazon S3 handles
/// replication of delete markers differently. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-add-config.html#replication-backward-compat-considerations">Backward Compatibility</a>.</p>
/// </note>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct DeleteMarkerReplication {
    /// <p>Indicates whether to replicate delete markers.</p>
    /// <note>
    /// <p>Indicates whether to replicate delete markers.</p>
    /// </note>
    pub status: Option<DeleteMarkerReplicationStatus>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteMarkerReplicationStatus {
    Disabled,
    Enabled,
}

impl DeleteMarkerReplicationStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Disabled" => Some(Self::Disabled),
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for DeleteMarkerReplicationStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type DeleteMarkerVersionId = String;

pub type DeleteMarkers = List<DeleteMarkerEntry>;

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteObjectInput {
    /// <p>The bucket name of the bucket containing the object. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Indicates whether S3 Object Lock should bypass Governance-mode restrictions to process
    /// this operation. To use this header, you must have the <code>s3:BypassGovernanceRetention</code>
    /// permission.</p>
    pub bypass_governance_retention: BypassGovernanceRetention,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Key name of the object to delete.</p>
    pub key: ObjectKey,
    /// <p>The concatenation of the authentication device's serial number, a space, and the value
    /// that is displayed on your authentication device. Required to permanently delete a versioned
    /// object if versioning is configured with MFA delete enabled.</p>
    pub mfa: Option<MFA>,
    pub request_payer: Option<RequestPayer>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct DeleteObjectOutput {
    /// <p>Specifies whether the versioned object that was permanently deleted was (true) or was
    /// not (false) a delete marker.</p>
    pub delete_marker: DeleteMarker,
    pub request_charged: Option<RequestCharged>,
    /// <p>Returns the version ID of the delete marker created as a result of the DELETE
    /// operation.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteObjectTaggingInput {
    /// <p>The bucket name containing the objects from which to remove the tags. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The key that identifies the object in the bucket from which to remove all tags.</p>
    pub key: ObjectKey,
    /// <p>The versionId of the object that the tag-set will be removed from.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct DeleteObjectTaggingOutput {
    /// <p>The versionId of the object the tag-set was removed from.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeleteObjectsInput {
    /// <p>The bucket name containing the objects to delete. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Specifies whether you want to delete this object even if it has a Governance-type Object
    /// Lock in place. To use this header, you must have the <code>s3:BypassGovernanceRetention</code>
    /// permission.</p>
    pub bypass_governance_retention: BypassGovernanceRetention,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    /// <p>This checksum algorithm must be the same for all parts and it match the checksum
    /// value supplied in the <code>CreateMultipartUpload</code> request.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>Container for the request.</p>
    pub delete: Delete,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The concatenation of the authentication device's serial number, a space, and the value
    /// that is displayed on your authentication device. Required to permanently delete a versioned
    /// object if versioning is configured with MFA delete enabled.</p>
    pub mfa: Option<MFA>,
    pub request_payer: Option<RequestPayer>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct DeleteObjectsOutput {
    /// <p>Container element for a successful delete. It identifies the object that was
    /// successfully deleted.</p>
    pub deleted: Option<DeletedObjects>,
    /// <p>Container for a failed delete action that describes the object that Amazon S3 attempted to
    /// delete and the error it encountered.</p>
    pub errors: Option<Errors>,
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct DeletePublicAccessBlockInput {
    /// <p>The Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to delete.
    /// </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

/// <p>Information about the deleted object.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct DeletedObject {
    /// <p>Specifies whether the versioned object that was permanently deleted was (true) or was
    /// not (false) a delete marker. In a simple DELETE, this header indicates whether (true) or
    /// not (false) a delete marker was created.</p>
    pub delete_marker: DeleteMarker,
    /// <p>The version ID of the delete marker created as a result of the DELETE operation. If you
    /// delete a specific object version, the value returned by this header is the version ID of
    /// the object version deleted.</p>
    pub delete_marker_version_id: Option<DeleteMarkerVersionId>,
    /// <p>The name of the deleted object.</p>
    pub key: Option<ObjectKey>,
    /// <p>The version ID of the deleted object.</p>
    pub version_id: Option<ObjectVersionId>,
}

pub type DeletedObjects = List<DeletedObject>;

pub type Delimiter = String;

pub type Description = String;

/// <p>Specifies information about where to publish analysis or configuration results for an
/// Amazon S3 bucket and S3 Replication Time Control (S3 RTC).</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct Destination {
    /// <p>Specify this only in a cross-account scenario (where source and destination bucket
    /// owners are not the same), and you want to change replica ownership to the Amazon Web Services account that
    /// owns the destination bucket. If this is not specified in the replication configuration, the
    /// replicas are owned by same Amazon Web Services account that owns the source object.</p>
    pub access_control_translation: Option<AccessControlTranslation>,
    /// <p>Destination bucket owner account ID. In a cross-account scenario, if you direct Amazon S3 to
    /// change replica ownership to the Amazon Web Services account that owns the destination bucket by specifying
    /// the <code>AccessControlTranslation</code> property, this is the account ID of the
    /// destination bucket owner. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-change-owner.html">Replication Additional
    /// Configuration: Changing the Replica Owner</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub account: Option<AccountId>,
    /// <p> The Amazon Resource Name (ARN) of the bucket where you want Amazon S3 to store the results.</p>
    pub bucket: BucketName,
    /// <p>A container that provides information about encryption. If
    /// <code>SourceSelectionCriteria</code> is specified, you must specify this element.</p>
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p> A container specifying replication metrics-related settings enabling replication
    /// metrics and events. </p>
    pub metrics: Option<Metrics>,
    /// <p> A container specifying S3 Replication Time Control (S3 RTC), including whether S3 RTC is enabled and the time
    /// when all objects and operations on objects must be replicated. Must be specified together
    /// with a <code>Metrics</code> block. </p>
    pub replication_time: Option<ReplicationTime>,
    /// <p> The storage class to use when replicating objects, such as S3 Standard or reduced
    /// redundancy. By default, Amazon S3 uses the storage class of the source object to create the
    /// object replica. </p>
    /// <p>For valid values, see the <code>StorageClass</code> element of the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTreplication.html">PUT Bucket
    /// replication</a> action in the <i>Amazon S3 API Reference</i>.</p>
    pub storage_class: Option<StorageClass>,
}

pub type DisplayName = String;

pub type ETag = String;

pub type EmailAddress = String;

pub type EnableRequestProgress = bool;

/// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding
/// method to use. An object key may contain any Unicode character; however, XML 1.0 parser
/// cannot parse some characters, such as characters with an ASCII value from 0 to 10. For
/// characters that are not supported in XML 1.0, you can add this parameter to request that
/// Amazon S3 encode the keys in the response.</p>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingType {
    Url,
}

impl EncodingType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Url => "url",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"url" => Some(Self::Url),
            _ => None,
        }
    }
}

impl FromStr for EncodingType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>Contains the type of server-side encryption used.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct Encryption {
    /// <p>The server-side encryption algorithm used when storing job results in Amazon S3 (for example,
    /// AES256, aws:kms).</p>
    pub encryption_type: ServerSideEncryption,
    /// <p>If the encryption type is <code>aws:kms</code>, this optional value can be used to
    /// specify the encryption context for the restore results.</p>
    pub kms_context: Option<KMSContext>,
    /// <p>If the encryption type is <code>aws:kms</code>, this optional value specifies the ID of
    /// the symmetric customer managed key to use for encryption of job results. Amazon S3 only
    /// supports symmetric keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using symmetric and
    /// asymmetric keys</a> in the <i>Amazon Web Services Key Management Service Developer
    /// Guide</i>.</p>
    pub kms_key_id: Option<SSEKMSKeyId>,
}

/// <p>Specifies encryption-related information for an Amazon S3 bucket that is a destination for
/// replicated objects.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct EncryptionConfiguration {
    /// <p>Specifies the ID (Key ARN or Alias ARN) of the customer managed Amazon Web Services KMS key
    /// stored in Amazon Web Services Key Management Service (KMS) for the destination bucket. Amazon S3 uses
    /// this key to encrypt replica objects. Amazon S3 only supports symmetric, customer managed KMS keys.
    /// For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using symmetric and
    /// asymmetric keys</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub replica_kms_key_id: Option<ReplicaKmsKeyID>,
}

pub type End = i64;

/// <p>A message that indicates the request is complete and no more messages will be sent. You
/// should not assume that the request is complete until the client receives an
/// <code>EndEvent</code>.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct EndEvent {}

/// <p>Container for all error elements.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Error {
    /// <p>The error code is a string that uniquely identifies an error condition. It is meant to
    /// be read and understood by programs that detect and handle errors by type. </p>
    /// <p class="title">
    /// <b>Amazon S3 error codes</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> AccessDenied </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Access Denied</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 403 Forbidden</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> AccountProblem</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> There is a problem with your Amazon Web Services account
    /// that prevents the action from completing successfully. Contact Amazon Web Services Support
    /// for further assistance.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 403 Forbidden</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> AllAccessDisabled</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> All access to this Amazon S3 resource has been
    /// disabled. Contact Amazon Web Services Support for further assistance.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 403 Forbidden</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> AmbiguousGrantByEmailAddress</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The email address you provided is
    /// associated with more than one account.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> AuthorizationHeaderMalformed</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The authorization header you provided is
    /// invalid.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> N/A</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> BadDigest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The Content-MD5 you specified did not
    /// match what we received.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> BucketAlreadyExists</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The requested bucket name is not
    /// available. The bucket namespace is shared by all users of the system. Please
    /// select a different name and try again.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 409 Conflict</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> BucketAlreadyOwnedByYou</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The bucket you tried to create already
    /// exists, and you own it. Amazon S3 returns this error in all Amazon Web Services Regions except in
    /// the North Virginia Region. For legacy compatibility, if you re-create an
    /// existing bucket that you already own in the North Virginia Region, Amazon S3 returns
    /// 200 OK and resets the bucket access control lists (ACLs).</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code:</i> 409 Conflict (in all Regions except the North
    /// Virginia Region) </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> BucketNotEmpty</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The bucket you tried to delete is not
    /// empty.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 409 Conflict</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> CredentialsNotSupported</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> This request does not support
    /// credentials.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> CrossLocationLoggingProhibited</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Cross-location logging not allowed.
    /// Buckets in one geographic location cannot log information to a bucket in
    /// another location.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 403 Forbidden</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> EntityTooSmall</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Your proposed upload is smaller than the
    /// minimum allowed object size.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> EntityTooLarge</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Your proposed upload exceeds the maximum
    /// allowed object size.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> ExpiredToken</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The provided token has expired.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> IllegalVersioningConfigurationException </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Indicates that the versioning
    /// configuration specified in the request is invalid.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> IncompleteBody</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> You did not provide the number of bytes
    /// specified by the Content-Length HTTP header</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> IncorrectNumberOfFilesInPostRequest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> POST requires exactly one file upload per
    /// request.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InlineDataTooLarge</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Inline data exceeds the maximum allowed
    /// size.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InternalError</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> We encountered an internal error. Please
    /// try again.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 500 Internal Server Error</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Server</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidAccessKeyId</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The Amazon Web Services access key ID you provided does
    /// not exist in our records.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 403 Forbidden</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidAddressingHeader</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> You must specify the Anonymous
    /// role.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> N/A</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidArgument</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Invalid Argument</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidBucketName</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The specified bucket is not valid.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidBucketState</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The request is not valid with the current
    /// state of the bucket.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 409 Conflict</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidDigest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The Content-MD5 you specified is not
    /// valid.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidEncryptionAlgorithmError</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The encryption request you specified is
    /// not valid. The valid value is AES256.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidLocationConstraint</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The specified location constraint is not
    /// valid. For more information about Regions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingBucket.html#access-bucket-intro">How to Select a
    /// Region for Your Buckets</a>. </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidObjectState</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The action is not valid for the current
    /// state of the object.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 403 Forbidden</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidPart</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> One or more of the specified parts could
    /// not be found. The part might not have been uploaded, or the specified entity
    /// tag might not have matched the part's entity tag.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidPartOrder</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The list of parts was not in ascending
    /// order. Parts list must be specified in order by part number.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidPayer</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> All access to this object has been
    /// disabled. Please contact Amazon Web Services Support for further assistance.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 403 Forbidden</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidPolicyDocument</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The content of the form does not meet the
    /// conditions specified in the policy document.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidRange</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The requested range cannot be
    /// satisfied.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 416 Requested Range Not
    /// Satisfiable</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidRequest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Please use <code>AWS4-HMAC-SHA256</code>.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code:</i> N/A</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidRequest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> SOAP requests must be made over an HTTPS
    /// connection.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidRequest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Amazon S3 Transfer Acceleration is not
    /// supported for buckets with non-DNS compliant names.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code:</i> N/A</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidRequest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Amazon S3 Transfer Acceleration is not
    /// supported for buckets with periods (.) in their names.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code:</i> N/A</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidRequest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Amazon S3 Transfer Accelerate endpoint only
    /// supports virtual style requests.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code:</i> N/A</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidRequest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Amazon S3 Transfer Accelerate is not configured
    /// on this bucket.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code:</i> N/A</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidRequest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Amazon S3 Transfer Accelerate is disabled on
    /// this bucket.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code:</i> N/A</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidRequest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Amazon S3 Transfer Acceleration is not
    /// supported on this bucket. Contact Amazon Web Services Support for more information.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code:</i> N/A</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidRequest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Amazon S3 Transfer Acceleration cannot be
    /// enabled on this bucket. Contact Amazon Web Services Support for more information.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code:</i> N/A</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidSecurity</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The provided security credentials are not
    /// valid.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 403 Forbidden</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidSOAPRequest</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The SOAP request body is invalid.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidStorageClass</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The storage class you specified is not
    /// valid.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidTargetBucketForLogging</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The target bucket for logging does not
    /// exist, is not owned by you, or does not have the appropriate grants for the
    /// log-delivery group. </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidToken</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The provided token is malformed or
    /// otherwise invalid.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidURI</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Couldn't parse the specified URI.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> KeyTooLongError</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Your key is too long.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MalformedACLError</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The XML you provided was not well-formed
    /// or did not validate against our published schema.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MalformedPOSTRequest </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The body of your POST request is not
    /// well-formed multipart/form-data.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MalformedXML</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> This happens when the user sends malformed
    /// XML (XML that doesn't conform to the published XSD) for the configuration. The
    /// error message is, "The XML you provided was not well-formed or did not validate
    /// against our published schema." </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MaxMessageLengthExceeded</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Your request was too big.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MaxPostPreDataLengthExceededError</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Your POST request fields preceding the
    /// upload file were too large.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MetadataTooLarge</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Your metadata headers exceed the maximum
    /// allowed metadata size.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MethodNotAllowed</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The specified method is not allowed
    /// against this resource.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 405 Method Not Allowed</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MissingAttachment</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> A SOAP attachment was expected, but none
    /// were found.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> N/A</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MissingContentLength</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> You must provide the Content-Length HTTP
    /// header.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 411 Length Required</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MissingRequestBodyError</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> This happens when the user sends an empty
    /// XML document as a request. The error message is, "Request body is empty."
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MissingSecurityElement</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The SOAP 1.1 request is missing a security
    /// element.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> MissingSecurityHeader</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Your request is missing a required
    /// header.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> NoLoggingStatusForKey</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> There is no such thing as a logging status
    /// subresource for a key.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> NoSuchBucket</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The specified bucket does not
    /// exist.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 404 Not Found</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> NoSuchBucketPolicy</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The specified bucket does not have a
    /// bucket policy.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 404 Not Found</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> NoSuchKey</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The specified key does not exist.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 404 Not Found</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> NoSuchLifecycleConfiguration</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The lifecycle configuration does not
    /// exist. </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 404 Not Found</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> NoSuchUpload</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The specified multipart upload does not
    /// exist. The upload ID might be invalid, or the multipart upload might have been
    /// aborted or completed.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 404 Not Found</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> NoSuchVersion </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Indicates that the version ID specified in
    /// the request does not match an existing version.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 404 Not Found</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> NotImplemented</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> A header you provided implies
    /// functionality that is not implemented.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 501 Not Implemented</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Server</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> NotSignedUp</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Your account is not signed up for the Amazon S3
    /// service. You must sign up before you can use Amazon S3. You can sign up at the
    /// following URL: <a href="http://aws.amazon.com/s3">Amazon S3</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 403 Forbidden</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> OperationAborted</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> A conflicting conditional action is
    /// currently in progress against this resource. Try again.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 409 Conflict</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> PermanentRedirect</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The bucket you are attempting to access
    /// must be addressed using the specified endpoint. Send all future requests to
    /// this endpoint.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 301 Moved Permanently</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> PreconditionFailed</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> At least one of the preconditions you
    /// specified did not hold.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 412 Precondition Failed</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> Redirect</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Temporary redirect.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 307 Moved Temporarily</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> RestoreAlreadyInProgress</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Object restore is already in
    /// progress.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 409 Conflict</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> RequestIsNotMultiPartContent</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Bucket POST must be of the enclosure-type
    /// multipart/form-data.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> RequestTimeout</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Your socket connection to the server was
    /// not read from or written to within the timeout period.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> RequestTimeTooSkewed</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The difference between the request time
    /// and the server's time is too large.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 403 Forbidden</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> RequestTorrentOfBucketError</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Requesting the torrent file of a bucket is
    /// not permitted.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> SignatureDoesNotMatch</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The request signature we calculated does
    /// not match the signature you provided. Check your Amazon Web Services secret access key and
    /// signing method. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAuthentication.html">REST Authentication</a> and
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/SOAPAuthentication.html">SOAP Authentication</a>
    /// for details.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 403 Forbidden</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> ServiceUnavailable</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Reduce your request rate.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 503 Service Unavailable</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Server</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> SlowDown</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> Reduce your request rate.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 503 Slow Down</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Server</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> TemporaryRedirect</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> You are being redirected to the bucket
    /// while DNS updates.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 307 Moved Temporarily</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> TokenRefreshRequired</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The provided token must be
    /// refreshed.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> TooManyBuckets</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> You have attempted to create more buckets
    /// than allowed.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> UnexpectedContent</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> This request does not support
    /// content.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> UnresolvableGrantByEmailAddress</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The email address you provided does not
    /// match any account on record.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> UserKeyMustBeSpecified</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Description:</i> The bucket POST must contain the specified
    /// field name. If it is specified, check the order of the fields.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code:</i> 400 Bad Request</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix:</i> Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    /// <p></p>
    pub code: Option<Code>,
    /// <p>The error key.</p>
    pub key: Option<ObjectKey>,
    /// <p>The error message contains a generic description of the error condition in English. It
    /// is intended for a human audience. Simple programs display the message directly to the end
    /// user if they encounter an error condition they don't know how or don't care to handle.
    /// Sophisticated programs with more exhaustive error handling and proper internationalization
    /// are more likely to ignore the error message.</p>
    pub message: Option<Message>,
    /// <p>The version ID of the error.</p>
    pub version_id: Option<ObjectVersionId>,
}

pub type ErrorCode = String;

/// <p>The error information.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct ErrorDocument {
    /// <p>The object key name to use when a 4XX class error occurs.</p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    pub key: ObjectKey,
}

pub type ErrorMessage = String;

pub type Errors = List<Error>;

/// <p>The bucket event for which to send notifications.</p>
pub type Event = String;

/// <p>A container for specifying the configuration for Amazon EventBridge.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct EventBridgeConfiguration {}

pub type EventList = List<Event>;

/// <p>Optional configuration to replicate existing source bucket objects. For more
/// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-what-is-isnot-replicated.html#existing-object-replication">Replicating Existing Objects</a> in the <i>Amazon S3 User Guide</i>.
/// </p>
#[derive(Debug)]
#[non_exhaustive]
pub struct ExistingObjectReplication {
    /// <p></p>
    pub status: ExistingObjectReplicationStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExistingObjectReplicationStatus {
    Disabled,
    Enabled,
}

impl ExistingObjectReplicationStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Disabled" => Some(Self::Disabled),
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for ExistingObjectReplicationStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type Expiration = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpirationStatus {
    Disabled,
    Enabled,
}

impl ExpirationStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Disabled" => Some(Self::Disabled),
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for ExpirationStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type ExpiredObjectDeleteMarker = bool;

pub type Expires = Timestamp;

pub type ExposeHeader = String;

pub type ExposeHeaders = List<ExposeHeader>;

pub type Expression = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionType {
    Sql,
}

impl ExpressionType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Sql => "SQL",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"SQL" => Some(Self::Sql),
            _ => None,
        }
    }
}

impl FromStr for ExpressionType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type FetchOwner = bool;

pub type FieldDelimiter = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileHeaderInfo {
    Ignore,
    None,
    Use,
}

impl FileHeaderInfo {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Ignore => "IGNORE",
            Self::None => "NONE",
            Self::Use => "USE",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"IGNORE" => Some(Self::Ignore),
            b"NONE" => Some(Self::None),
            b"USE" => Some(Self::Use),
            _ => None,
        }
    }
}

impl FromStr for FileHeaderInfo {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>Specifies the Amazon S3 object key name to filter on and whether to filter on the suffix or
/// prefix of the key name.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct FilterRule {
    /// <p>The object key name prefix or suffix identifying one or more objects to which the
    /// filtering rule applies. The maximum length is 1,024 characters. Overlapping prefixes and
    /// suffixes are not supported. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring Event Notifications</a>
    /// in the <i>Amazon S3 User Guide</i>.</p>
    pub name: Option<FilterRuleName>,
    /// <p>The value that the filter searches for in object key names.</p>
    pub value: Option<FilterRuleValue>,
}

/// <p>A list of containers for the key-value pair that defines the criteria for the filter
/// rule.</p>
pub type FilterRuleList = List<FilterRule>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterRuleName {
    Prefix,
    Suffix,
}

impl FilterRuleName {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Prefix => "prefix",
            Self::Suffix => "suffix",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"prefix" => Some(Self::Prefix),
            b"suffix" => Some(Self::Suffix),
            _ => None,
        }
    }
}

impl FromStr for FilterRuleName {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type FilterRuleValue = String;

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketAccelerateConfigurationInput {
    /// <p>The name of the bucket for which the accelerate configuration is retrieved.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketAccelerateConfigurationOutput {
    /// <p>The accelerate configuration of the bucket.</p>
    pub status: Option<BucketAccelerateStatus>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketAclInput {
    /// <p>Specifies the S3 bucket whose ACL is being requested.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketAclOutput {
    /// <p>A list of grants.</p>
    pub grants: Option<Grants>,
    /// <p>Container for the bucket owner's display name and ID.</p>
    pub owner: Option<Owner>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketAnalyticsConfigurationInput {
    /// <p>The name of the bucket from which an analytics configuration is retrieved.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID that identifies the analytics configuration.</p>
    pub id: AnalyticsId,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketAnalyticsConfigurationOutput {
    /// <p>The configuration and any analyses for the analytics filter.</p>
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketCorsInput {
    /// <p>The bucket name for which to get the cors configuration.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketCorsOutput {
    /// <p>A set of origins and methods (cross-origin access that you want to allow). You can add
    /// up to 100 rules to the configuration.</p>
    pub cors_rules: Option<CORSRules>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketEncryptionInput {
    /// <p>The name of the bucket from which the server-side encryption configuration is
    /// retrieved.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketEncryptionOutput {
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketIntelligentTieringConfigurationInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub bucket: BucketName,
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub id: IntelligentTieringId,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketIntelligentTieringConfigurationOutput {
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketInventoryConfigurationInput {
    /// <p>The name of the bucket containing the inventory configuration to retrieve.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: InventoryId,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketInventoryConfigurationOutput {
    /// <p>Specifies the inventory configuration.</p>
    pub inventory_configuration: Option<InventoryConfiguration>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketLifecycleConfigurationInput {
    /// <p>The name of the bucket for which to get the lifecycle information.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketLifecycleConfigurationOutput {
    /// <p>Container for a lifecycle rule.</p>
    pub rules: Option<LifecycleRules>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketLocationInput {
    /// <p>The name of the bucket for which to get the location.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketLocationOutput {
    /// <p>Specifies the Region where the bucket resides. For a list of all the Amazon S3 supported
    /// location constraints by Region, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region">Regions and Endpoints</a>.
    /// Buckets in Region <code>us-east-1</code> have a LocationConstraint of
    /// <code>null</code>.</p>
    pub location_constraint: Option<BucketLocationConstraint>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketLoggingInput {
    /// <p>The bucket name for which to get the logging information.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketLoggingOutput {
    pub logging_enabled: Option<LoggingEnabled>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketMetricsConfigurationInput {
    /// <p>The name of the bucket containing the metrics configuration to retrieve.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: MetricsId,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketMetricsConfigurationOutput {
    /// <p>Specifies the metrics configuration.</p>
    pub metrics_configuration: Option<MetricsConfiguration>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketNotificationConfigurationInput {
    /// <p>The name of the bucket for which to get the notification configuration.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketOwnershipControlsInput {
    /// <p>The name of the Amazon S3 bucket whose <code>OwnershipControls</code> you want to retrieve.
    /// </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketOwnershipControlsOutput {
    /// <p>The <code>OwnershipControls</code> (BucketOwnerEnforced, BucketOwnerPreferred, or ObjectWriter) currently in
    /// effect for this Amazon S3 bucket.</p>
    pub ownership_controls: Option<OwnershipControls>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketPolicyInput {
    /// <p>The bucket name for which to get the bucket policy.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketPolicyOutput {
    /// <p>The bucket policy as a JSON document.</p>
    pub policy: Option<Policy>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketPolicyStatusInput {
    /// <p>The name of the Amazon S3 bucket whose policy status you want to retrieve.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketPolicyStatusOutput {
    /// <p>The policy status for the specified bucket.</p>
    pub policy_status: Option<PolicyStatus>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketReplicationInput {
    /// <p>The bucket name for which to get the replication information.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketReplicationOutput {
    pub replication_configuration: Option<ReplicationConfiguration>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketRequestPaymentInput {
    /// <p>The name of the bucket for which to get the payment request configuration</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketRequestPaymentOutput {
    /// <p>Specifies who pays for the download and request fees.</p>
    pub payer: Option<Payer>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketTaggingInput {
    /// <p>The name of the bucket for which to get the tagging information.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketTaggingOutput {
    /// <p>Contains the tag set.</p>
    pub tag_set: TagSet,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketVersioningInput {
    /// <p>The name of the bucket for which to get the versioning information.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketVersioningOutput {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This
    /// element is only returned if the bucket has been configured with MFA delete. If the bucket
    /// has never been so configured, this element is not returned.</p>
    pub mfa_delete: Option<MFADeleteStatus>,
    /// <p>The versioning state of the bucket.</p>
    pub status: Option<BucketVersioningStatus>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetBucketWebsiteInput {
    /// <p>The bucket name for which to get the website configuration.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetBucketWebsiteOutput {
    /// <p>The object key name of the website error document to use for 4XX class errors.</p>
    pub error_document: Option<ErrorDocument>,
    /// <p>The name of the index document for the website (for example
    /// <code>index.html</code>).</p>
    pub index_document: Option<IndexDocument>,
    /// <p>Specifies the redirect behavior of all requests to a website endpoint of an Amazon S3
    /// bucket.</p>
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    /// <p>Rules that define when a redirect is applied and the redirect behavior.</p>
    pub routing_rules: Option<RoutingRules>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetObjectAclInput {
    /// <p>The bucket name that contains the object for which to get the ACL information. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The key of the object for which to get the ACL information.</p>
    pub key: ObjectKey,
    pub request_payer: Option<RequestPayer>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetObjectAclOutput {
    /// <p>A list of grants.</p>
    pub grants: Option<Grants>,
    /// <p> Container for the bucket owner's display name and ID.</p>
    pub owner: Option<Owner>,
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetObjectAttributesInput {
    /// <p>The name of the bucket that contains the object.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The object key.</p>
    pub key: ObjectKey,
    /// <p>Sets the maximum number of parts to return.</p>
    pub max_parts: MaxParts,
    /// <p>An XML header that specifies the fields at the root level that you want returned in
    /// the response. Fields that you do not specify are not returned.</p>
    pub object_attributes: ObjectAttributesList,
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers
    /// will be listed.</p>
    pub part_number_marker: Option<PartNumberMarker>,
    pub request_payer: Option<RequestPayer>,
    /// <p>Specifies the algorithm to use when encrypting the object (for example,
    /// AES256).</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This
    /// value is used to store the object and then it is discarded; Amazon S3 does not store the
    /// encryption key. The key must be appropriate for use with the algorithm specified in the
    /// <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub sse_customer_key: Option<SSECustomerKey>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses
    /// this header for a message integrity check to ensure that the encryption key was transmitted
    /// without error.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>The version ID used to reference a specific version of the object.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetObjectAttributesOutput {
    /// <p>The checksum or digest of the object.</p>
    pub checksum: Option<Checksum>,
    /// <p>Specifies whether the object retrieved was (<code>true</code>) or was not
    /// (<code>false</code>) a delete marker. If <code>false</code>, this response header does
    /// not appear in the response.</p>
    pub delete_marker: DeleteMarker,
    /// <p>An ETag is an opaque identifier assigned by a web server to a specific version of a
    /// resource found at a URL.</p>
    pub e_tag: Option<ETag>,
    /// <p>The creation date of the object.</p>
    pub last_modified: Option<LastModified>,
    /// <p>A collection of parts associated with a multipart upload.</p>
    pub object_parts: Option<GetObjectAttributesParts>,
    /// <p>The size of the object in bytes.</p>
    pub object_size: ObjectSize,
    pub request_charged: Option<RequestCharged>,
    /// <p>Provides the storage class information of the object. Amazon S3 returns this header for all
    /// objects except for S3 Standard storage class objects.</p>
    ///
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html">Storage
    /// Classes</a>.</p>
    pub storage_class: Option<StorageClass>,
    /// <p>The version ID of the object.</p>
    pub version_id: Option<ObjectVersionId>,
}

/// <p>A collection of parts associated with a multipart upload.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetObjectAttributesParts {
    /// <p>Indicates whether the returned list of parts is truncated. A value of
    /// <code>true</code> indicates that the list was truncated. A list can be truncated if the
    /// number of parts exceeds the limit returned in the <code>MaxParts</code> element.</p>
    pub is_truncated: IsTruncated,
    /// <p>The maximum number of parts allowed in the response.</p>
    pub max_parts: MaxParts,
    /// <p>When a list is truncated, this element specifies the last part in the list, as well as
    /// the value to use for the <code>PartNumberMarker</code> request parameter in a subsequent
    /// request.</p>
    pub next_part_number_marker: Option<NextPartNumberMarker>,
    /// <p>The marker for the current part.</p>
    pub part_number_marker: Option<PartNumberMarker>,
    /// <p>A container for elements related to a particular part. A response can contain zero or
    /// more <code>Parts</code> elements.</p>
    pub parts: Option<PartsList>,
    /// <p>The total number of parts.</p>
    pub total_parts_count: PartsCount,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetObjectInput {
    /// <p>The bucket name containing the object. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using an Object Lambda access point the hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-object-lambda.<i>Region</i>.amazonaws.com.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>To retrieve the checksum, this mode must be enabled.</p>
    pub checksum_mode: Option<ChecksumMode>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified;
    /// otherwise, return a 412 (precondition failed) error.</p>
    pub if_match: Option<IfMatch>,
    /// <p>Return the object only if it has been modified since the specified time; otherwise,
    /// return a 304 (not modified) error.</p>
    pub if_modified_since: Option<IfModifiedSince>,
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified;
    /// otherwise, return a 304 (not modified) error.</p>
    pub if_none_match: Option<IfNoneMatch>,
    /// <p>Return the object only if it has not been modified since the specified time; otherwise,
    /// return a 412 (precondition failed) error.</p>
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    /// <p>Key of the object to get.</p>
    pub key: ObjectKey,
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000.
    /// Effectively performs a 'ranged' GET request for the part specified. Useful for downloading
    /// just a part of an object.</p>
    pub part_number: PartNumber,
    /// <p>Downloads the specified range bytes of an object. For more information about the HTTP
    /// Range header, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>.</p>
    /// <note>
    /// <p>Amazon S3 doesn't support retrieving multiple ranges of data per <code>GET</code>
    /// request.</p>
    /// </note>
    pub range: Option<Range>,
    pub request_payer: Option<RequestPayer>,
    /// <p>Sets the <code>Cache-Control</code> header of the response.</p>
    pub response_cache_control: Option<ResponseCacheControl>,
    /// <p>Sets the <code>Content-Disposition</code> header of the response</p>
    pub response_content_disposition: Option<ResponseContentDisposition>,
    /// <p>Sets the <code>Content-Encoding</code> header of the response.</p>
    pub response_content_encoding: Option<ResponseContentEncoding>,
    /// <p>Sets the <code>Content-Language</code> header of the response.</p>
    pub response_content_language: Option<ResponseContentLanguage>,
    /// <p>Sets the <code>Content-Type</code> header of the response.</p>
    pub response_content_type: Option<ResponseContentType>,
    /// <p>Sets the <code>Expires</code> header of the response.</p>
    pub response_expires: Option<ResponseExpires>,
    /// <p>Specifies the algorithm to use to when decrypting the object (for example,
    /// AES256).</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 used to encrypt the data. This
    /// value is used to decrypt the object when recovering it and must match the one used when
    /// storing the data. The key must be appropriate for use with the algorithm specified in the
    /// <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub sse_customer_key: Option<SSECustomerKey>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses
    /// this header for a message integrity check to ensure that the encryption key was transmitted
    /// without error.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetObjectLegalHoldInput {
    /// <p>The bucket name containing the object whose legal hold status you want to retrieve. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The key name for the object whose legal hold status you want to retrieve.</p>
    pub key: ObjectKey,
    pub request_payer: Option<RequestPayer>,
    /// <p>The version ID of the object whose legal hold status you want to retrieve.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetObjectLegalHoldOutput {
    /// <p>The current legal hold status for the specified object.</p>
    pub legal_hold: Option<ObjectLockLegalHold>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetObjectLockConfigurationInput {
    /// <p>The bucket whose Object Lock configuration you want to retrieve.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetObjectLockConfigurationOutput {
    /// <p>The specified bucket's Object Lock configuration.</p>
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetObjectOutput {
    /// <p>Indicates that a range of bytes was specified.</p>
    pub accept_ranges: Option<AcceptRanges>,
    /// <p>Object data.</p>
    pub body: Option<StreamingBlob>,
    /// <p>Indicates whether the object uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<CacheControl>,
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<ContentDisposition>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding
    /// mechanisms must be applied to obtain the media-type referenced by the Content-Type header
    /// field.</p>
    pub content_encoding: Option<ContentEncoding>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<ContentLanguage>,
    /// <p>Size of the body in bytes.</p>
    pub content_length: ContentLength,
    /// <p>The portion of the object returned in the response.</p>
    pub content_range: Option<ContentRange>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<ContentType>,
    /// <p>Specifies whether the object retrieved was (true) or was not (false) a Delete Marker. If
    /// false, this response header does not appear in the response.</p>
    pub delete_marker: DeleteMarker,
    /// <p>An entity tag (ETag) is an opaque identifier assigned by a web server to a specific
    /// version of a resource found at a URL.</p>
    pub e_tag: Option<ETag>,
    /// <p>If the object expiration is configured (see PUT Bucket lifecycle), the response includes
    /// this header. It includes the <code>expiry-date</code> and <code>rule-id</code> key-value
    /// pairs providing object expiration information. The value of the <code>rule-id</code> is
    /// URL-encoded.</p>
    pub expiration: Option<Expiration>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<Expires>,
    /// <p>Creation date of the object.</p>
    pub last_modified: Option<LastModified>,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<Metadata>,
    /// <p>This is set to the number of metadata entries not returned in <code>x-amz-meta</code>
    /// headers. This can happen if you create metadata using an API like SOAP that supports more
    /// flexible metadata than the REST API. For example, using SOAP, you can create metadata whose
    /// values are not legal HTTP headers.</p>
    pub missing_meta: MissingMeta,
    /// <p>Indicates whether this object has an active legal hold. This field is only returned if
    /// you have permission to view an object's legal hold status. </p>
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    /// <p>The Object Lock mode currently in place for this object.</p>
    pub object_lock_mode: Option<ObjectLockMode>,
    /// <p>The date and time when this object's Object Lock will expire.</p>
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    /// <p>The count of parts this object has. This value is only returned if you specify <code>partNumber</code>
    /// in your request and the object was uploaded as a multipart upload.</p>
    pub parts_count: PartsCount,
    /// <p>Amazon S3 can return this if your request involves a bucket that is either a source or
    /// destination in a replication rule.</p>
    pub replication_status: Option<ReplicationStatus>,
    pub request_charged: Option<RequestCharged>,
    /// <p>Provides information about object restoration action and expiration time of the
    /// restored object copy.</p>
    pub restore: Option<Restore>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header to provide round-trip message integrity verification of
    /// the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric
    /// customer managed key that was used for the object.</p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example,
    /// AES256, aws:kms).</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// <p>Provides storage class information of the object. Amazon S3 returns this header for all
    /// objects except for S3 Standard storage class objects.</p>
    pub storage_class: Option<StorageClass>,
    /// <p>The number of tags, if any, on the object.</p>
    pub tag_count: TagCount,
    /// <p>Version of the object.</p>
    pub version_id: Option<ObjectVersionId>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another
    /// object in the same bucket or to an external URL. Amazon S3 stores the value of this header in
    /// the object metadata.</p>
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
}

pub type GetObjectResponseStatusCode = i32;

#[derive(Debug)]
#[non_exhaustive]
pub struct GetObjectRetentionInput {
    /// <p>The bucket name containing the object whose retention settings you want to retrieve. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The key name for the object whose retention settings you want to retrieve.</p>
    pub key: ObjectKey,
    pub request_payer: Option<RequestPayer>,
    /// <p>The version ID for the object whose retention settings you want to retrieve.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetObjectRetentionOutput {
    /// <p>The container element for an object's retention settings.</p>
    pub retention: Option<ObjectLockRetention>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetObjectTaggingInput {
    /// <p>The bucket name containing the object for which to get the tagging information. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Object key for which to get the tagging information.</p>
    pub key: ObjectKey,
    pub request_payer: Option<RequestPayer>,
    /// <p>The versionId of the object for which to get the tagging information.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetObjectTaggingOutput {
    /// <p>Contains the tag set.</p>
    pub tag_set: TagSet,
    /// <p>The versionId of the object for which you got the tagging information.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetObjectTorrentInput {
    /// <p>The name of the bucket containing the object for which to get the torrent files.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The object key for which to get the information.</p>
    pub key: ObjectKey,
    pub request_payer: Option<RequestPayer>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetObjectTorrentOutput {
    /// <p>A Bencoded dictionary as defined by the BitTorrent specification</p>
    pub body: Option<StreamingBlob>,
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GetPublicAccessBlockInput {
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want
    /// to retrieve. </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GetPublicAccessBlockOutput {
    /// <p>The <code>PublicAccessBlock</code> configuration currently in effect for this Amazon S3
    /// bucket.</p>
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}

/// <p>Container for S3 Glacier job parameters.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct GlacierJobParameters {
    /// <p>Retrieval tier at which the restore will be processed.</p>
    pub tier: Tier,
}

/// <p>Container for grant information.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Grant {
    /// <p>The person being granted permissions.</p>
    pub grantee: Option<Grantee>,
    /// <p>Specifies the permission given to the grantee.</p>
    pub permission: Option<Permission>,
}

pub type GrantFullControl = String;

pub type GrantRead = String;

pub type GrantReadACP = String;

pub type GrantWrite = String;

pub type GrantWriteACP = String;

/// <p>Container for the person being granted permissions.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct Grantee {
    /// <p>Screen name of the grantee.</p>
    pub display_name: Option<DisplayName>,
    /// <p>Email address of the grantee.</p>
    /// <note>
    /// <p>Using email addresses to specify a grantee is only supported in the following Amazon Web Services Regions: </p>
    /// <ul>
    /// <li>
    /// <p>US East (N. Virginia)</p>
    /// </li>
    /// <li>
    /// <p>US West (N. California)</p>
    /// </li>
    /// <li>
    /// <p> US West (Oregon)</p>
    /// </li>
    /// <li>
    /// <p> Asia Pacific (Singapore)</p>
    /// </li>
    /// <li>
    /// <p>Asia Pacific (Sydney)</p>
    /// </li>
    /// <li>
    /// <p>Asia Pacific (Tokyo)</p>
    /// </li>
    /// <li>
    /// <p>Europe (Ireland)</p>
    /// </li>
    /// <li>
    /// <p>South America (São Paulo)</p>
    /// </li>
    /// </ul>
    /// <p>For a list of all the Amazon S3 supported Regions and endpoints, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region">Regions and Endpoints</a> in the Amazon Web Services General Reference.</p>
    /// </note>
    pub email_address: Option<EmailAddress>,
    /// <p>The canonical user ID of the grantee.</p>
    pub id: Option<ID>,
    /// <p>Type of grantee</p>
    pub type_: Type,
    /// <p>URI of the grantee group.</p>
    pub uri: Option<URI>,
}

pub type Grants = List<Grant>;

#[derive(Debug)]
#[non_exhaustive]
pub struct HeadBucketInput {
    /// <p>The bucket name.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct HeadObjectInput {
    /// <p>The name of the bucket containing the object.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>To retrieve the checksum, this parameter must be enabled.</p>
    /// <p>In addition, if you enable <code>ChecksumMode</code> and the object is encrypted with
    /// Amazon Web Services Key Management Service (Amazon Web Services KMS), you must have permission to use the
    /// <code>kms:Decrypt</code> action for the request to succeed.</p>
    pub checksum_mode: Option<ChecksumMode>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified;
    /// otherwise, return a 412 (precondition failed) error.</p>
    pub if_match: Option<IfMatch>,
    /// <p>Return the object only if it has been modified since the specified time; otherwise,
    /// return a 304 (not modified) error.</p>
    pub if_modified_since: Option<IfModifiedSince>,
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified;
    /// otherwise, return a 304 (not modified) error.</p>
    pub if_none_match: Option<IfNoneMatch>,
    /// <p>Return the object only if it has not been modified since the specified time; otherwise,
    /// return a 412 (precondition failed) error.</p>
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    /// <p>The object key.</p>
    pub key: ObjectKey,
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000.
    /// Effectively performs a 'ranged' HEAD request for the part specified. Useful querying about
    /// the size of the part and the number of parts in this object.</p>
    pub part_number: PartNumber,
    /// <p>Because <code>HeadObject</code> returns only the metadata for an object, this parameter
    /// has no effect.</p>
    pub range: Option<Range>,
    pub request_payer: Option<RequestPayer>,
    /// <p>Specifies the algorithm to use to when encrypting the object (for example,
    /// AES256).</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This
    /// value is used to store the object and then it is discarded; Amazon S3 does not store the
    /// encryption key. The key must be appropriate for use with the algorithm specified in the
    /// <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub sse_customer_key: Option<SSECustomerKey>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses
    /// this header for a message integrity check to ensure that the encryption key was transmitted
    /// without error.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct HeadObjectOutput {
    /// <p>Indicates that a range of bytes was specified.</p>
    pub accept_ranges: Option<AcceptRanges>,
    /// <p>The archive state of the head object.</p>
    pub archive_status: Option<ArchiveStatus>,
    /// <p>Indicates whether the object uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<CacheControl>,
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<ContentDisposition>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding
    /// mechanisms must be applied to obtain the media-type referenced by the Content-Type header
    /// field.</p>
    pub content_encoding: Option<ContentEncoding>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<ContentLanguage>,
    /// <p>Size of the body in bytes.</p>
    pub content_length: ContentLength,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<ContentType>,
    /// <p>Specifies whether the object retrieved was (true) or was not (false) a Delete Marker. If
    /// false, this response header does not appear in the response.</p>
    pub delete_marker: DeleteMarker,
    /// <p>An entity tag (ETag) is an opaque identifier assigned by a web server to a specific
    /// version of a resource found at a URL.</p>
    pub e_tag: Option<ETag>,
    /// <p>If the object expiration is configured (see PUT Bucket lifecycle), the response includes
    /// this header. It includes the <code>expiry-date</code> and <code>rule-id</code> key-value
    /// pairs providing object expiration information. The value of the <code>rule-id</code> is
    /// URL-encoded.</p>
    pub expiration: Option<Expiration>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<Expires>,
    /// <p>Creation date of the object.</p>
    pub last_modified: Option<LastModified>,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<Metadata>,
    /// <p>This is set to the number of metadata entries not returned in <code>x-amz-meta</code>
    /// headers. This can happen if you create metadata using an API like SOAP that supports more
    /// flexible metadata than the REST API. For example, using SOAP, you can create metadata whose
    /// values are not legal HTTP headers.</p>
    pub missing_meta: MissingMeta,
    /// <p>Specifies whether a legal hold is in effect for this object. This header is only
    /// returned if the requester has the <code>s3:GetObjectLegalHold</code> permission. This
    /// header is not returned if the specified version of this object has never had a legal hold
    /// applied. For more information about S3 Object Lock, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Object Lock</a>.</p>
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    /// <p>The Object Lock mode, if any, that's in effect for this object. This header is only
    /// returned if the requester has the <code>s3:GetObjectRetention</code> permission. For more
    /// information about S3 Object Lock, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Object
    /// Lock</a>. </p>
    pub object_lock_mode: Option<ObjectLockMode>,
    /// <p>The date and time when the Object Lock retention period expires. This header is only
    /// returned if the requester has the <code>s3:GetObjectRetention</code> permission.</p>
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    /// <p>The count of parts this object has. This value is only returned if you specify <code>partNumber</code>
    /// in your request and the object was uploaded as a multipart upload.</p>
    pub parts_count: PartsCount,
    /// <p>Amazon S3 can return this header if your request involves a bucket that is either a source or
    /// a destination in a replication rule.</p>
    ///
    /// <p>In replication, you have a source bucket on which you configure replication and
    /// destination bucket or buckets where Amazon S3 stores object replicas. When you request an object
    /// (<code>GetObject</code>) or object metadata (<code>HeadObject</code>) from these
    /// buckets, Amazon S3 will return the <code>x-amz-replication-status</code> header in the response
    /// as follows:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <b>If requesting an object from the source bucket</b>, Amazon S3 will return the
    /// <code>x-amz-replication-status</code> header if the object in your request is
    /// eligible for replication.</p>
    /// <p> For example, suppose that in your replication configuration, you specify object
    /// prefix <code>TaxDocs</code> requesting Amazon S3 to replicate objects with key prefix
    /// <code>TaxDocs</code>. Any objects you upload with this key name prefix, for
    /// example <code>TaxDocs/document1.pdf</code>, are eligible for replication. For any
    /// object request with this key name prefix, Amazon S3 will return the
    /// <code>x-amz-replication-status</code> header with value PENDING, COMPLETED or
    /// FAILED indicating object replication status.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <b>If requesting an object from a destination bucket</b>, Amazon S3 will return the
    /// <code>x-amz-replication-status</code> header with value REPLICA if the object in
    /// your request is a replica that Amazon S3 created and there is no replica modification
    /// replication in progress.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <b>When replicating objects to multiple destination buckets</b>, the
    /// <code>x-amz-replication-status</code> header acts differently. The header of the
    /// source object will only return a value of COMPLETED when replication is successful to
    /// all destinations. The header will remain at value PENDING until replication has
    /// completed for all destinations. If one or more destinations fails replication the
    /// header will return FAILED. </p>
    /// </li>
    /// </ul>
    ///
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Replication</a>.</p>
    pub replication_status: Option<ReplicationStatus>,
    pub request_charged: Option<RequestCharged>,
    /// <p>If the object is an archived object (an object whose storage class is GLACIER), the
    /// response includes this header if either the archive restoration is in progress (see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_RestoreObject.html">RestoreObject</a> or an archive copy is already restored.</p>
    ///
    /// <p> If an archive copy is already restored, the header value indicates when Amazon S3 is
    /// scheduled to delete the object copy. For example:</p>
    ///
    /// <p>
    /// <code>x-amz-restore: ongoing-request="false", expiry-date="Fri, 21 Dec 2012 00:00:00
    /// GMT"</code>
    /// </p>
    ///
    /// <p>If the object restoration is in progress, the header returns the value
    /// <code>ongoing-request="true"</code>.</p>
    ///
    /// <p>For more information about archiving objects, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html#lifecycle-transition-general-considerations">Transitioning Objects: General Considerations</a>.</p>
    pub restore: Option<Restore>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header to provide round-trip message integrity verification of
    /// the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric
    /// customer managed key that was used for the object.</p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p>If the object is stored using server-side encryption either with an Amazon Web Services KMS key or
    /// an Amazon S3-managed encryption key, the response includes this header with
    /// the value of the server-side encryption algorithm used when storing this object in Amazon
    /// S3 (for example, AES256, aws:kms).</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// <p>Provides storage class information of the object. Amazon S3 returns this header for all
    /// objects except for S3 Standard storage class objects.</p>
    ///
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html">Storage
    /// Classes</a>.</p>
    pub storage_class: Option<StorageClass>,
    /// <p>Version of the object.</p>
    pub version_id: Option<ObjectVersionId>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another
    /// object in the same bucket or to an external URL. Amazon S3 stores the value of this header in
    /// the object metadata.</p>
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
}

pub type HostName = String;

pub type HttpErrorCodeReturnedEquals = String;

pub type HttpRedirectCode = String;

pub type ID = String;

pub type IfMatch = String;

pub type IfModifiedSince = Timestamp;

pub type IfNoneMatch = String;

pub type IfUnmodifiedSince = Timestamp;

/// <p>Container for the <code>Suffix</code> element.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct IndexDocument {
    /// <p>A suffix that is appended to a request that is for a directory on the website endpoint
    /// (for example,if the suffix is index.html and you make a request to samplebucket/images/ the
    /// data that is returned will be for the object with the key name images/index.html) The
    /// suffix must not be empty and must not include a slash character.</p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    pub suffix: Suffix,
}

pub type Initiated = Timestamp;

/// <p>Container element that identifies who initiated the multipart upload. </p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Initiator {
    /// <p>Name of the Principal.</p>
    pub display_name: Option<DisplayName>,
    /// <p>If the principal is an Amazon Web Services account, it provides the Canonical User ID. If the principal
    /// is an IAM User, it provides a user ARN value.</p>
    pub id: Option<ID>,
}

/// <p>Describes the serialization format of the object.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct InputSerialization {
    /// <p>Describes the serialization of a CSV-encoded object.</p>
    pub csv: Option<CSVInput>,
    /// <p>Specifies object's compression format. Valid values: NONE, GZIP, BZIP2. Default Value:
    /// NONE.</p>
    pub compression_type: Option<CompressionType>,
    /// <p>Specifies JSON as object's input serialization format.</p>
    pub json: Option<JSONInput>,
    /// <p>Specifies Parquet as object's input serialization format.</p>
    pub parquet: Option<ParquetInput>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntelligentTieringAccessTier {
    ArchiveAccess,
    DeepArchiveAccess,
}

impl IntelligentTieringAccessTier {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ArchiveAccess => "ARCHIVE_ACCESS",
            Self::DeepArchiveAccess => "DEEP_ARCHIVE_ACCESS",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"ARCHIVE_ACCESS" => Some(Self::ArchiveAccess),
            b"DEEP_ARCHIVE_ACCESS" => Some(Self::DeepArchiveAccess),
            _ => None,
        }
    }
}

impl FromStr for IntelligentTieringAccessTier {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>A container for specifying S3 Intelligent-Tiering filters. The filters determine the
/// subset of objects to which the rule applies.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct IntelligentTieringAndOperator {
    /// <p>An object key name prefix that identifies the subset of objects to which the
    /// configuration applies.</p>
    pub prefix: Option<Prefix>,
    /// <p>All of these tags must exist in the object's tag set in order for the configuration to
    /// apply.</p>
    pub tags: Option<TagSet>,
}

/// <p>Specifies the S3 Intelligent-Tiering configuration for an Amazon S3 bucket.</p>
/// <p>For information about the S3 Intelligent-Tiering storage class, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for
/// automatically optimizing frequently and infrequently accessed objects</a>.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct IntelligentTieringConfiguration {
    /// <p>Specifies a bucket filter. The configuration only includes objects that meet the
    /// filter's criteria.</p>
    pub filter: Option<IntelligentTieringFilter>,
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub id: IntelligentTieringId,
    /// <p>Specifies the status of the configuration.</p>
    pub status: IntelligentTieringStatus,
    /// <p>Specifies the S3 Intelligent-Tiering storage class tier of the configuration.</p>
    pub tierings: TieringList,
}

pub type IntelligentTieringConfigurationList = List<IntelligentTieringConfiguration>;

pub type IntelligentTieringDays = i32;

/// <p>The <code>Filter</code> is used to identify objects that the S3 Intelligent-Tiering
/// configuration applies to.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct IntelligentTieringFilter {
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter.
    /// The operator must have at least two predicates, and an object must match all of the
    /// predicates in order for the filter to apply.</p>
    pub and: Option<IntelligentTieringAndOperator>,
    /// <p>An object key name prefix that identifies the subset of objects to which the rule
    /// applies.</p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    pub prefix: Option<Prefix>,
    pub tag: Option<Tag>,
}

pub type IntelligentTieringId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntelligentTieringStatus {
    Disabled,
    Enabled,
}

impl IntelligentTieringStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Disabled" => Some(Self::Disabled),
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for IntelligentTieringStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>Object is archived and inaccessible until restored.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct InvalidObjectState {
    pub access_tier: Option<IntelligentTieringAccessTier>,
    pub storage_class: Option<StorageClass>,
}

/// <p>Specifies the inventory configuration for an Amazon S3 bucket. For more information, see
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketGETInventoryConfig.html">GET Bucket inventory</a> in the <i>Amazon S3 API Reference</i>.
/// </p>
#[derive(Debug)]
#[non_exhaustive]
pub struct InventoryConfiguration {
    /// <p>Contains information about where to publish the inventory results.</p>
    pub destination: InventoryDestination,
    /// <p>Specifies an inventory filter. The inventory only includes objects that meet the
    /// filter's criteria.</p>
    pub filter: Option<InventoryFilter>,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: InventoryId,
    /// <p>Object versions to include in the inventory list. If set to <code>All</code>, the list
    /// includes all the object versions, which adds the version-related fields
    /// <code>VersionId</code>, <code>IsLatest</code>, and <code>DeleteMarker</code> to the
    /// list. If set to <code>Current</code>, the list does not contain these version-related
    /// fields.</p>
    pub included_object_versions: InventoryIncludedObjectVersions,
    /// <p>Specifies whether the inventory is enabled or disabled. If set to <code>True</code>, an
    /// inventory list is generated. If set to <code>False</code>, no inventory list is
    /// generated.</p>
    pub is_enabled: IsEnabled,
    /// <p>Contains the optional fields that are included in the inventory results.</p>
    pub optional_fields: Option<InventoryOptionalFields>,
    /// <p>Specifies the schedule for generating inventory results.</p>
    pub schedule: InventorySchedule,
}

pub type InventoryConfigurationList = List<InventoryConfiguration>;

/// <p>Specifies the inventory configuration for an Amazon S3 bucket.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct InventoryDestination {
    /// <p>Contains the bucket name, file format, bucket owner (optional), and prefix (optional)
    /// where inventory results are published.</p>
    pub s3_bucket_destination: InventoryS3BucketDestination,
}

/// <p>Contains the type of server-side encryption used to encrypt the inventory
/// results.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct InventoryEncryption {
    /// <p>Specifies the use of SSE-KMS to encrypt delivered inventory reports.</p>
    pub ssekms: Option<SSEKMS>,
    /// <p>Specifies the use of SSE-S3 to encrypt delivered inventory reports.</p>
    pub sses3: Option<SSES3>,
}

/// <p>Specifies an inventory filter. The inventory only includes objects that meet the
/// filter's criteria.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct InventoryFilter {
    /// <p>The prefix that an object must have to be included in the inventory results.</p>
    pub prefix: Prefix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryFormat {
    Csv,
    Orc,
    Parquet,
}

impl InventoryFormat {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Csv => "CSV",
            Self::Orc => "ORC",
            Self::Parquet => "Parquet",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"CSV" => Some(Self::Csv),
            b"ORC" => Some(Self::Orc),
            b"Parquet" => Some(Self::Parquet),
            _ => None,
        }
    }
}

impl FromStr for InventoryFormat {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryFrequency {
    Daily,
    Weekly,
}

impl InventoryFrequency {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Daily => "Daily",
            Self::Weekly => "Weekly",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Daily" => Some(Self::Daily),
            b"Weekly" => Some(Self::Weekly),
            _ => None,
        }
    }
}

impl FromStr for InventoryFrequency {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type InventoryId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryIncludedObjectVersions {
    All,
    Current,
}

impl InventoryIncludedObjectVersions {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Current => "Current",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"All" => Some(Self::All),
            b"Current" => Some(Self::Current),
            _ => None,
        }
    }
}

impl FromStr for InventoryIncludedObjectVersions {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryOptionalField {
    BucketKeyStatus,
    ChecksumAlgorithm,
    ETag,
    EncryptionStatus,
    IntelligentTieringAccessTier,
    IsMultipartUploaded,
    LastModifiedDate,
    ObjectLockLegalHoldStatus,
    ObjectLockMode,
    ObjectLockRetainUntilDate,
    ReplicationStatus,
    Size,
    StorageClass,
}

impl InventoryOptionalField {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BucketKeyStatus => "BucketKeyStatus",
            Self::ChecksumAlgorithm => "ChecksumAlgorithm",
            Self::ETag => "ETag",
            Self::EncryptionStatus => "EncryptionStatus",
            Self::IntelligentTieringAccessTier => "IntelligentTieringAccessTier",
            Self::IsMultipartUploaded => "IsMultipartUploaded",
            Self::LastModifiedDate => "LastModifiedDate",
            Self::ObjectLockLegalHoldStatus => "ObjectLockLegalHoldStatus",
            Self::ObjectLockMode => "ObjectLockMode",
            Self::ObjectLockRetainUntilDate => "ObjectLockRetainUntilDate",
            Self::ReplicationStatus => "ReplicationStatus",
            Self::Size => "Size",
            Self::StorageClass => "StorageClass",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"BucketKeyStatus" => Some(Self::BucketKeyStatus),
            b"ChecksumAlgorithm" => Some(Self::ChecksumAlgorithm),
            b"ETag" => Some(Self::ETag),
            b"EncryptionStatus" => Some(Self::EncryptionStatus),
            b"IntelligentTieringAccessTier" => Some(Self::IntelligentTieringAccessTier),
            b"IsMultipartUploaded" => Some(Self::IsMultipartUploaded),
            b"LastModifiedDate" => Some(Self::LastModifiedDate),
            b"ObjectLockLegalHoldStatus" => Some(Self::ObjectLockLegalHoldStatus),
            b"ObjectLockMode" => Some(Self::ObjectLockMode),
            b"ObjectLockRetainUntilDate" => Some(Self::ObjectLockRetainUntilDate),
            b"ReplicationStatus" => Some(Self::ReplicationStatus),
            b"Size" => Some(Self::Size),
            b"StorageClass" => Some(Self::StorageClass),
            _ => None,
        }
    }
}

impl FromStr for InventoryOptionalField {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type InventoryOptionalFields = List<InventoryOptionalField>;

/// <p>Contains the bucket name, file format, bucket owner (optional), and prefix (optional)
/// where inventory results are published.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct InventoryS3BucketDestination {
    /// <p>The account ID that owns the destination S3 bucket. If no account ID is provided, the
    /// owner is not validated before exporting data. </p>
    /// <note>
    /// <p> Although this value is optional, we strongly recommend that you set it to help
    /// prevent problems if the destination bucket ownership changes. </p>
    /// </note>
    pub account_id: Option<AccountId>,
    /// <p>The Amazon Resource Name (ARN) of the bucket where inventory results will be
    /// published.</p>
    pub bucket: BucketName,
    /// <p>Contains the type of server-side encryption used to encrypt the inventory
    /// results.</p>
    pub encryption: Option<InventoryEncryption>,
    /// <p>Specifies the output format of the inventory results.</p>
    pub format: InventoryFormat,
    /// <p>The prefix that is prepended to all inventory results.</p>
    pub prefix: Option<Prefix>,
}

/// <p>Specifies the schedule for generating inventory results.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct InventorySchedule {
    /// <p>Specifies how frequently inventory results are produced.</p>
    pub frequency: InventoryFrequency,
}

pub type IsEnabled = bool;

pub type IsLatest = bool;

pub type IsPublic = bool;

pub type IsTruncated = bool;

/// <p>Specifies JSON as object's input serialization format.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct JSONInput {
    /// <p>The type of JSON. Valid values: Document, Lines.</p>
    pub type_: Option<JSONType>,
}

/// <p>Specifies JSON as request's output serialization format.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct JSONOutput {
    /// <p>The value used to separate individual records in the output. If no value is specified,
    /// Amazon S3 uses a newline character ('\n').</p>
    pub record_delimiter: Option<RecordDelimiter>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JSONType {
    Document,
    Lines,
}

impl JSONType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Document => "DOCUMENT",
            Self::Lines => "LINES",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"DOCUMENT" => Some(Self::Document),
            b"LINES" => Some(Self::Lines),
            _ => None,
        }
    }
}

impl FromStr for JSONType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type KMSContext = String;

pub type KeyCount = i32;

pub type KeyMarker = String;

pub type KeyPrefixEquals = String;

pub type LambdaFunctionArn = String;

/// <p>A container for specifying the configuration for Lambda notifications.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct LambdaFunctionConfiguration {
    /// <p>The Amazon S3 bucket event for which to invoke the Lambda function. For more information,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Supported
    /// Event Types</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub events: EventList,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<NotificationId>,
    /// <p>The Amazon Resource Name (ARN) of the Lambda function that Amazon S3 invokes when the
    /// specified event type occurs.</p>
    pub lambda_function_arn: LambdaFunctionArn,
}

pub type LambdaFunctionConfigurationList = List<LambdaFunctionConfiguration>;

pub type LastModified = Timestamp;

/// <p>Container for the expiration for the lifecycle of the object.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct LifecycleExpiration {
    /// <p>Indicates at what date the object is to be moved or deleted. Should be in GMT ISO 8601
    /// Format.</p>
    pub date: Option<Date>,
    /// <p>Indicates the lifetime, in days, of the objects that are subject to the rule. The value
    /// must be a non-zero positive integer.</p>
    pub days: Days,
    /// <p>Indicates whether Amazon S3 will remove a delete marker with no noncurrent versions. If set
    /// to true, the delete marker will be expired; if set to false the policy takes no action.
    /// This cannot be specified with Days or Date in a Lifecycle Expiration Policy.</p>
    pub expired_object_delete_marker: ExpiredObjectDeleteMarker,
}

/// <p>A lifecycle rule for individual objects in an Amazon S3 bucket.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct LifecycleRule {
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    /// <p>Specifies the expiration for the lifecycle of the object in the form of date, days and,
    /// whether the object has a delete marker.</p>
    pub expiration: Option<LifecycleExpiration>,
    /// <p>The <code>Filter</code> is used to identify objects that a Lifecycle Rule applies to. A
    /// <code>Filter</code> must have exactly one of <code>Prefix</code>, <code>Tag</code>, or
    /// <code>And</code> specified. <code>Filter</code> is required if the
    /// <code>LifecycleRule</code> does not contain a <code>Prefix</code> element.</p>
    pub filter: Option<LifecycleRuleFilter>,
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub id: Option<ID>,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    /// <p> Specifies the transition rule for the lifecycle rule that describes when noncurrent
    /// objects transition to a specific storage class. If your bucket is versioning-enabled (or
    /// versioning is suspended), you can set this action to request that Amazon S3 transition
    /// noncurrent object versions to a specific storage class at a set period in the object's
    /// lifetime. </p>
    pub noncurrent_version_transitions: Option<NoncurrentVersionTransitionList>,
    /// <p>Prefix identifying one or more objects to which the rule applies. This is
    /// no longer used; use <code>Filter</code> instead.</p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    pub prefix: Option<Prefix>,
    /// <p>If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not
    /// currently being applied.</p>
    pub status: ExpirationStatus,
    /// <p>Specifies when an Amazon S3 object transitions to a specified storage class.</p>
    pub transitions: Option<TransitionList>,
}

/// <p>This is used in a Lifecycle Rule Filter to apply a logical AND to two or more
/// predicates. The Lifecycle Rule will apply to any object matching all of the predicates
/// configured inside the And operator.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct LifecycleRuleAndOperator {
    /// <p>Minimum object size to which the rule applies.</p>
    pub object_size_greater_than: ObjectSizeGreaterThanBytes,
    /// <p>Maximum object size to which the rule applies.</p>
    pub object_size_less_than: ObjectSizeLessThanBytes,
    /// <p>Prefix identifying one or more objects to which the rule applies.</p>
    pub prefix: Option<Prefix>,
    /// <p>All of these tags must exist in the object's tag set in order for the rule to
    /// apply.</p>
    pub tags: Option<TagSet>,
}

/// <p>The <code>Filter</code> is used to identify objects that a Lifecycle Rule applies to. A
/// <code>Filter</code> must have exactly one of <code>Prefix</code>, <code>Tag</code>, or
/// <code>And</code> specified.</p>
#[derive(Debug)]
#[non_exhaustive]
pub enum LifecycleRuleFilter {
    And(LifecycleRuleAndOperator),
    /// <p>Minimum object size to which the rule applies.</p>
    ObjectSizeGreaterThan(ObjectSizeGreaterThanBytes),
    /// <p>Maximum object size to which the rule applies.</p>
    ObjectSizeLessThan(ObjectSizeLessThanBytes),
    /// <p>Prefix identifying one or more objects to which the rule applies.</p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    Prefix(Prefix),
    /// <p>This tag must exist in the object's tag set in order for the rule to apply.</p>
    Tag(Tag),
}

pub type LifecycleRules = List<LifecycleRule>;

#[derive(Debug)]
#[non_exhaustive]
pub struct ListBucketAnalyticsConfigurationsInput {
    /// <p>The name of the bucket from which analytics configurations are retrieved.</p>
    pub bucket: BucketName,
    /// <p>The ContinuationToken that represents a placeholder from where this request should
    /// begin.</p>
    pub continuation_token: Option<Token>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ListBucketAnalyticsConfigurationsOutput {
    /// <p>The list of analytics configurations for a bucket.</p>
    pub analytics_configuration_list: Option<AnalyticsConfigurationList>,
    /// <p>The marker that is used as a starting point for this analytics configuration list
    /// response. This value is present if it was sent in the request.</p>
    pub continuation_token: Option<Token>,
    /// <p>Indicates whether the returned list of analytics configurations is complete. A value of
    /// true indicates that the list is not complete and the NextContinuationToken will be provided
    /// for a subsequent request.</p>
    pub is_truncated: IsTruncated,
    /// <p>
    /// <code>NextContinuationToken</code> is sent when <code>isTruncated</code> is true, which
    /// indicates that there are more analytics configurations to list. The next request must
    /// include this <code>NextContinuationToken</code>. The token is obfuscated and is not a
    /// usable value.</p>
    pub next_continuation_token: Option<NextToken>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ListBucketIntelligentTieringConfigurationsInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub bucket: BucketName,
    /// <p>The <code>ContinuationToken</code> that represents a placeholder from where this request
    /// should begin.</p>
    pub continuation_token: Option<Token>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ListBucketIntelligentTieringConfigurationsOutput {
    /// <p>The <code>ContinuationToken</code> that represents a placeholder from where this request
    /// should begin.</p>
    pub continuation_token: Option<Token>,
    /// <p>The list of S3 Intelligent-Tiering configurations for a bucket.</p>
    pub intelligent_tiering_configuration_list: Option<IntelligentTieringConfigurationList>,
    /// <p>Indicates whether the returned list of analytics configurations is complete. A value of
    /// <code>true</code> indicates that the list is not complete and the
    /// <code>NextContinuationToken</code> will be provided for a subsequent request.</p>
    pub is_truncated: IsTruncated,
    /// <p>The marker used to continue this inventory configuration listing. Use the
    /// <code>NextContinuationToken</code> from this response to continue the listing in a
    /// subsequent request. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub next_continuation_token: Option<NextToken>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ListBucketInventoryConfigurationsInput {
    /// <p>The name of the bucket containing the inventory configurations to retrieve.</p>
    pub bucket: BucketName,
    /// <p>The marker used to continue an inventory configuration listing that has been truncated.
    /// Use the NextContinuationToken from a previously truncated list response to continue the
    /// listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub continuation_token: Option<Token>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ListBucketInventoryConfigurationsOutput {
    /// <p>If sent in the request, the marker that is used as a starting point for this inventory
    /// configuration list response.</p>
    pub continuation_token: Option<Token>,
    /// <p>The list of inventory configurations for a bucket.</p>
    pub inventory_configuration_list: Option<InventoryConfigurationList>,
    /// <p>Tells whether the returned list of inventory configurations is complete. A value of true
    /// indicates that the list is not complete and the NextContinuationToken is provided for a
    /// subsequent request.</p>
    pub is_truncated: IsTruncated,
    /// <p>The marker used to continue this inventory configuration listing. Use the
    /// <code>NextContinuationToken</code> from this response to continue the listing in a
    /// subsequent request. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub next_continuation_token: Option<NextToken>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ListBucketMetricsConfigurationsInput {
    /// <p>The name of the bucket containing the metrics configurations to retrieve.</p>
    pub bucket: BucketName,
    /// <p>The marker that is used to continue a metrics configuration listing that has been
    /// truncated. Use the NextContinuationToken from a previously truncated list response to
    /// continue the listing. The continuation token is an opaque value that Amazon S3
    /// understands.</p>
    pub continuation_token: Option<Token>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ListBucketMetricsConfigurationsOutput {
    /// <p>The marker that is used as a starting point for this metrics configuration list
    /// response. This value is present if it was sent in the request.</p>
    pub continuation_token: Option<Token>,
    /// <p>Indicates whether the returned list of metrics configurations is complete. A value of
    /// true indicates that the list is not complete and the NextContinuationToken will be provided
    /// for a subsequent request.</p>
    pub is_truncated: IsTruncated,
    /// <p>The list of metrics configurations for a bucket.</p>
    pub metrics_configuration_list: Option<MetricsConfigurationList>,
    /// <p>The marker used to continue a metrics configuration listing that has been truncated. Use
    /// the <code>NextContinuationToken</code> from a previously truncated list response to
    /// continue the listing. The continuation token is an opaque value that Amazon S3
    /// understands.</p>
    pub next_continuation_token: Option<NextToken>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ListBucketsOutput {
    /// <p>The list of buckets owned by the requester.</p>
    pub buckets: Option<Buckets>,
    /// <p>The owner of the buckets listed.</p>
    pub owner: Option<Owner>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ListMultipartUploadsInput {
    /// <p>The name of the bucket to which the multipart upload was initiated. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Character you use to group keys.</p>
    /// <p>All keys that contain the same string between the prefix, if specified, and the first
    /// occurrence of the delimiter after the prefix are grouped under a single result element,
    /// <code>CommonPrefixes</code>. If you don't specify the prefix parameter, then the
    /// substring starts at the beginning of the key. The keys that are grouped under
    /// <code>CommonPrefixes</code> result element are not returned elsewhere in the
    /// response.</p>
    pub delimiter: Option<Delimiter>,
    pub encoding_type: Option<EncodingType>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Together with upload-id-marker, this parameter specifies the multipart upload after
    /// which listing should begin.</p>
    /// <p>If <code>upload-id-marker</code> is not specified, only the keys lexicographically
    /// greater than the specified <code>key-marker</code> will be included in the list.</p>
    ///
    /// <p>If <code>upload-id-marker</code> is specified, any multipart uploads for a key equal to
    /// the <code>key-marker</code> might also be included, provided those multipart uploads have
    /// upload IDs lexicographically greater than the specified
    /// <code>upload-id-marker</code>.</p>
    pub key_marker: Option<KeyMarker>,
    /// <p>Sets the maximum number of multipart uploads, from 1 to 1,000, to return in the response
    /// body. 1,000 is the maximum number of uploads that can be returned in a response.</p>
    pub max_uploads: MaxUploads,
    /// <p>Lists in-progress uploads only for those keys that begin with the specified prefix. You
    /// can use prefixes to separate a bucket into different grouping of keys. (You can think of
    /// using prefix to make groups in the same way you'd use a folder in a file system.)</p>
    pub prefix: Option<Prefix>,
    /// <p>Together with key-marker, specifies the multipart upload after which listing should
    /// begin. If key-marker is not specified, the upload-id-marker parameter is ignored.
    /// Otherwise, any multipart uploads for a key equal to the key-marker might be included in the
    /// list only if they have an upload ID lexicographically greater than the specified
    /// <code>upload-id-marker</code>.</p>
    pub upload_id_marker: Option<UploadIdMarker>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ListMultipartUploadsOutput {
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    pub bucket: Option<BucketName>,
    /// <p>If you specify a delimiter in the request, then the result returns each distinct key
    /// prefix containing the delimiter in a <code>CommonPrefixes</code> element. The distinct key
    /// prefixes are returned in the <code>Prefix</code> child element.</p>
    pub common_prefixes: Option<CommonPrefixList>,
    /// <p>Contains the delimiter you specified in the request. If you don't specify a delimiter in
    /// your request, this element is absent from the response.</p>
    pub delimiter: Option<Delimiter>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    /// <p>If you specify <code>encoding-type</code> request parameter, Amazon S3 includes this element
    /// in the response, and returns encoded key name values in the following response
    /// elements:</p>
    ///
    /// <p>
    /// <code>Delimiter</code>, <code>KeyMarker</code>, <code>Prefix</code>,
    /// <code>NextKeyMarker</code>, <code>Key</code>.</p>
    pub encoding_type: Option<EncodingType>,
    /// <p>Indicates whether the returned list of multipart uploads is truncated. A value of true
    /// indicates that the list was truncated. The list can be truncated if the number of multipart
    /// uploads exceeds the limit allowed or specified by max uploads.</p>
    pub is_truncated: IsTruncated,
    /// <p>The key at or after which the listing began.</p>
    pub key_marker: Option<KeyMarker>,
    /// <p>Maximum number of multipart uploads that could have been included in the
    /// response.</p>
    pub max_uploads: MaxUploads,
    /// <p>When a list is truncated, this element specifies the value that should be used for the
    /// key-marker request parameter in a subsequent request.</p>
    pub next_key_marker: Option<NextKeyMarker>,
    /// <p>When a list is truncated, this element specifies the value that should be used for the
    /// <code>upload-id-marker</code> request parameter in a subsequent request.</p>
    pub next_upload_id_marker: Option<NextUploadIdMarker>,
    /// <p>When a prefix is provided in the request, this field contains the specified prefix. The
    /// result contains only keys starting with the specified prefix.</p>
    pub prefix: Option<Prefix>,
    /// <p>Upload ID after which listing began.</p>
    pub upload_id_marker: Option<UploadIdMarker>,
    /// <p>Container for elements related to a particular multipart upload. A response can contain
    /// zero or more <code>Upload</code> elements.</p>
    pub uploads: Option<MultipartUploadList>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ListObjectVersionsInput {
    /// <p>The bucket name that contains the objects. </p>
    pub bucket: BucketName,
    /// <p>A delimiter is a character that you specify to group keys. All keys that contain the
    /// same string between the <code>prefix</code> and the first occurrence of the delimiter are
    /// grouped under a single result element in CommonPrefixes. These groups are counted as one
    /// result against the max-keys limitation. These keys are not returned elsewhere in the
    /// response.</p>
    pub delimiter: Option<Delimiter>,
    pub encoding_type: Option<EncodingType>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Specifies the key to start with when listing objects in a bucket.</p>
    pub key_marker: Option<KeyMarker>,
    /// <p>Sets the maximum number of keys returned in the response. By default the action returns up
    /// to 1,000 key names. The response might contain fewer keys but will never contain more. If
    /// additional keys satisfy the search criteria, but were not returned because max-keys was
    /// exceeded, the response contains <isTruncated>true</isTruncated>. To return the
    /// additional keys, see key-marker and version-id-marker.</p>
    pub max_keys: MaxKeys,
    /// <p>Use this parameter to select only those keys that begin with the specified prefix. You
    /// can use prefixes to separate a bucket into different groupings of keys. (You can think of
    /// using prefix to make groups in the same way you'd use a folder in a file system.) You can
    /// use prefix with delimiter to roll up numerous objects into a single result under
    /// CommonPrefixes. </p>
    pub prefix: Option<Prefix>,
    /// <p>Specifies the object version you want to start listing from.</p>
    pub version_id_marker: Option<VersionIdMarker>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ListObjectVersionsOutput {
    /// <p>All of the keys rolled up into a common prefix count as a single return when calculating
    /// the number of returns.</p>
    pub common_prefixes: Option<CommonPrefixList>,
    /// <p>Container for an object that is a delete marker.</p>
    pub delete_markers: Option<DeleteMarkers>,
    /// <p>The delimiter grouping the included keys. A delimiter is a character that you specify to
    /// group keys. All keys that contain the same string between the prefix and the first
    /// occurrence of the delimiter are grouped under a single result element in
    /// <code>CommonPrefixes</code>. These groups are counted as one result against the max-keys
    /// limitation. These keys are not returned elsewhere in the response.</p>
    pub delimiter: Option<Delimiter>,
    /// <p> Encoding type used by Amazon S3 to encode object key names in the XML response.</p>
    ///
    /// <p>If you specify encoding-type request parameter, Amazon S3 includes this element in the
    /// response, and returns encoded key name values in the following response elements:</p>
    ///
    /// <p>
    /// <code>KeyMarker, NextKeyMarker, Prefix, Key</code>, and <code>Delimiter</code>.</p>
    pub encoding_type: Option<EncodingType>,
    /// <p>A flag that indicates whether Amazon S3 returned all of the results that satisfied the search
    /// criteria. If your results were truncated, you can make a follow-up paginated request using
    /// the NextKeyMarker and NextVersionIdMarker response parameters as a starting place in
    /// another request to return the rest of the results.</p>
    pub is_truncated: IsTruncated,
    /// <p>Marks the last key returned in a truncated response.</p>
    pub key_marker: Option<KeyMarker>,
    /// <p>Specifies the maximum number of objects to return.</p>
    pub max_keys: MaxKeys,
    /// <p>The bucket name.</p>
    pub name: Option<BucketName>,
    /// <p>When the number of responses exceeds the value of <code>MaxKeys</code>,
    /// <code>NextKeyMarker</code> specifies the first key not returned that satisfies the
    /// search criteria. Use this value for the key-marker request parameter in a subsequent
    /// request.</p>
    pub next_key_marker: Option<NextKeyMarker>,
    /// <p>When the number of responses exceeds the value of <code>MaxKeys</code>,
    /// <code>NextVersionIdMarker</code> specifies the first object version not returned that
    /// satisfies the search criteria. Use this value for the version-id-marker request parameter
    /// in a subsequent request.</p>
    pub next_version_id_marker: Option<NextVersionIdMarker>,
    /// <p>Selects objects that start with the value supplied by this parameter.</p>
    pub prefix: Option<Prefix>,
    /// <p>Marks the last version of the key returned in a truncated response.</p>
    pub version_id_marker: Option<VersionIdMarker>,
    /// <p>Container for version information.</p>
    pub versions: Option<ObjectVersionList>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ListObjectsInput {
    /// <p>The name of the bucket containing the objects.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>A delimiter is a character you use to group keys.</p>
    pub delimiter: Option<Delimiter>,
    pub encoding_type: Option<EncodingType>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Marker is where you want Amazon S3 to start listing from. Amazon S3 starts listing after
    /// this specified key. Marker can be any key in the bucket.</p>
    pub marker: Option<Marker>,
    /// <p>Sets the maximum number of keys returned in the response. By default the action returns up
    /// to 1,000 key names. The response might contain fewer keys but will never contain more.
    /// </p>
    pub max_keys: MaxKeys,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub prefix: Option<Prefix>,
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects
    /// request. Bucket owners need not specify this parameter in their requests.</p>
    pub request_payer: Option<RequestPayer>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ListObjectsOutput {
    /// <p>All of the keys (up to 1,000) rolled up in a common prefix count as a single return when calculating
    /// the number of returns. </p>
    ///
    /// <p>A response can contain CommonPrefixes only if you specify a delimiter.</p>
    ///
    /// <p>CommonPrefixes contains all (if there are any) keys between Prefix and the next
    /// occurrence of the string specified by the delimiter.</p>
    ///
    /// <p> CommonPrefixes lists keys that act like subdirectories in the directory specified by
    /// Prefix.</p>
    ///
    /// <p>For example, if the prefix is notes/ and the delimiter is a slash (/) as in
    /// notes/summer/july, the common prefix is notes/summer/. All of the keys that roll up into a
    /// common prefix count as a single return when calculating the number of returns.</p>
    pub common_prefixes: Option<CommonPrefixList>,
    /// <p>Metadata about each object returned.</p>
    pub contents: Option<ObjectList>,
    /// <p>Causes keys that contain the same string between the prefix and the first occurrence of
    /// the delimiter to be rolled up into a single result element in the
    /// <code>CommonPrefixes</code> collection. These rolled-up keys are not returned elsewhere
    /// in the response. Each rolled-up result counts as only one return against the
    /// <code>MaxKeys</code> value.</p>
    pub delimiter: Option<Delimiter>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<EncodingType>,
    /// <p>A flag that indicates whether Amazon S3 returned all of the results that satisfied the search
    /// criteria.</p>
    pub is_truncated: IsTruncated,
    /// <p>Indicates where in the bucket listing begins. Marker is included in the response if it
    /// was sent with the request.</p>
    pub marker: Option<Marker>,
    /// <p>The maximum number of keys returned in the response body.</p>
    pub max_keys: MaxKeys,
    /// <p>The bucket name.</p>
    pub name: Option<BucketName>,
    /// <p>When response is truncated (the IsTruncated element value in the response is true), you
    /// can use the key name in this field as marker in the subsequent request to get next set of
    /// objects. Amazon S3 lists objects in alphabetical order Note: This element is returned only if
    /// you have delimiter request parameter specified. If response does not include the NextMarker
    /// and it is truncated, you can use the value of the last Key in the response as the marker in
    /// the subsequent request to get the next set of object keys.</p>
    pub next_marker: Option<NextMarker>,
    /// <p>Keys that begin with the indicated prefix.</p>
    pub prefix: Option<Prefix>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ListObjectsV2Input {
    /// <p>Bucket name to list. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>ContinuationToken indicates Amazon S3 that the list is being continued on this bucket with a
    /// token. ContinuationToken is obfuscated and is not a real key.</p>
    pub continuation_token: Option<Token>,
    /// <p>A delimiter is a character you use to group keys.</p>
    pub delimiter: Option<Delimiter>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<EncodingType>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The owner field is not present in listV2 by default, if you want to return owner field
    /// with each key in the result then set the fetch owner field to true.</p>
    pub fetch_owner: FetchOwner,
    /// <p>Sets the maximum number of keys returned in the response. By default the action returns up
    /// to 1,000 key names. The response might contain fewer keys but will never contain
    /// more.</p>
    pub max_keys: MaxKeys,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub prefix: Option<Prefix>,
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects
    /// request in V2 style. Bucket owners need not specify this parameter in their
    /// requests.</p>
    pub request_payer: Option<RequestPayer>,
    /// <p>StartAfter is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this
    /// specified key. StartAfter can be any key in the bucket.</p>
    pub start_after: Option<StartAfter>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ListObjectsV2Output {
    /// <p>All of the keys (up to 1,000) rolled up into a common prefix count as a single return when calculating
    /// the number of returns.</p>
    ///
    /// <p>A response can contain <code>CommonPrefixes</code> only if you specify a
    /// delimiter.</p>
    ///
    /// <p>
    /// <code>CommonPrefixes</code> contains all (if there are any) keys between
    /// <code>Prefix</code> and the next occurrence of the string specified by a
    /// delimiter.</p>
    ///
    /// <p>
    /// <code>CommonPrefixes</code> lists keys that act like subdirectories in the directory
    /// specified by <code>Prefix</code>.</p>
    ///
    /// <p>For example, if the prefix is <code>notes/</code> and the delimiter is a slash
    /// (<code>/</code>) as in <code>notes/summer/july</code>, the common prefix is
    /// <code>notes/summer/</code>. All of the keys that roll up into a common prefix count as a
    /// single return when calculating the number of returns. </p>
    pub common_prefixes: Option<CommonPrefixList>,
    /// <p>Metadata about each object returned.</p>
    pub contents: Option<ObjectList>,
    /// <p> If ContinuationToken was sent with the request, it is included in the response.</p>
    pub continuation_token: Option<Token>,
    /// <p>Causes keys that contain the same string between the prefix and the first occurrence of
    /// the delimiter to be rolled up into a single result element in the CommonPrefixes
    /// collection. These rolled-up keys are not returned elsewhere in the response. Each rolled-up
    /// result counts as only one return against the <code>MaxKeys</code> value.</p>
    pub delimiter: Option<Delimiter>,
    /// <p>Encoding type used by Amazon S3 to encode object key names in the XML response.</p>
    ///
    /// <p>If you specify the encoding-type request parameter, Amazon S3 includes this element in the
    /// response, and returns encoded key name values in the following response elements:</p>
    ///
    /// <p>
    /// <code>Delimiter, Prefix, Key,</code> and <code>StartAfter</code>.</p>
    pub encoding_type: Option<EncodingType>,
    /// <p>Set to false if all of the results were returned. Set to true if more keys are available
    /// to return. If the number of results exceeds that specified by MaxKeys, all of the results
    /// might not be returned.</p>
    pub is_truncated: IsTruncated,
    /// <p>KeyCount is the number of keys returned with this request. KeyCount will always be less
    /// than or equals to MaxKeys field. Say you ask for 50 keys, your result will include less than
    /// equals 50 keys </p>
    pub key_count: KeyCount,
    /// <p>Sets the maximum number of keys returned in the response. By default the action returns up
    /// to 1,000 key names. The response might contain fewer keys but will never contain
    /// more.</p>
    pub max_keys: MaxKeys,
    /// <p>The bucket name.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub name: Option<BucketName>,
    /// <p>
    /// <code>NextContinuationToken</code> is sent when <code>isTruncated</code> is true, which
    /// means there are more keys in the bucket that can be listed. The next list requests to Amazon S3
    /// can be continued with this <code>NextContinuationToken</code>.
    /// <code>NextContinuationToken</code> is obfuscated and is not a real key</p>
    pub next_continuation_token: Option<NextToken>,
    /// <p> Keys that begin with the indicated prefix.</p>
    pub prefix: Option<Prefix>,
    /// <p>If StartAfter was sent with the request, it is included in the response.</p>
    pub start_after: Option<StartAfter>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ListPartsInput {
    /// <p>The name of the bucket to which the parts are being uploaded. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: ObjectKey,
    /// <p>Sets the maximum number of parts to return.</p>
    pub max_parts: MaxParts,
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers
    /// will be listed.</p>
    pub part_number_marker: Option<PartNumberMarker>,
    pub request_payer: Option<RequestPayer>,
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created
    /// using a checksum algorithm. For more information,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm.
    /// For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub sse_customer_key: Option<SSECustomerKey>,
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum
    /// algorithm. For more information,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub upload_id: MultipartUploadId,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ListPartsOutput {
    /// <p>If the bucket has a lifecycle rule configured with an action to abort incomplete
    /// multipart uploads and the prefix in the lifecycle rule matches the object name in the
    /// request, then the response includes this header indicating when the initiated multipart
    /// upload will become eligible for abort operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config">Aborting
    /// Incomplete Multipart Uploads Using a Bucket Lifecycle Policy</a>.</p>
    ///
    /// <p>The response will also include the <code>x-amz-abort-rule-id</code> header that will
    /// provide the ID of the lifecycle configuration rule that defines this action.</p>
    pub abort_date: Option<AbortDate>,
    /// <p>This header is returned along with the <code>x-amz-abort-date</code> header. It
    /// identifies applicable lifecycle configuration rule that defines the action to abort
    /// incomplete multipart uploads.</p>
    pub abort_rule_id: Option<AbortRuleId>,
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    pub bucket: Option<BucketName>,
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>Container element that identifies who initiated the multipart upload. If the initiator
    /// is an Amazon Web Services account, this element provides the same information as the <code>Owner</code>
    /// element. If the initiator is an IAM User, this element provides the user ARN and display
    /// name.</p>
    pub initiator: Option<Initiator>,
    /// <p> Indicates whether the returned list of parts is truncated. A true value indicates that
    /// the list was truncated. A list can be truncated if the number of parts exceeds the limit
    /// returned in the MaxParts element.</p>
    pub is_truncated: IsTruncated,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: Option<ObjectKey>,
    /// <p>Maximum number of parts that were allowed in the response.</p>
    pub max_parts: MaxParts,
    /// <p>When a list is truncated, this element specifies the last part in the list, as well as
    /// the value to use for the part-number-marker request parameter in a subsequent
    /// request.</p>
    pub next_part_number_marker: Option<NextPartNumberMarker>,
    /// <p> Container element that identifies the object owner, after the object is created. If
    /// multipart upload is initiated by an IAM user, this element provides the parent account ID
    /// and display name.</p>
    pub owner: Option<Owner>,
    /// <p>When a list is truncated, this element specifies the last part in the list, as well as
    /// the value to use for the part-number-marker request parameter in a subsequent
    /// request.</p>
    pub part_number_marker: Option<PartNumberMarker>,
    /// <p> Container for elements related to a particular part. A response can contain zero or
    /// more <code>Part</code> elements.</p>
    pub parts: Option<Parts>,
    pub request_charged: Option<RequestCharged>,
    /// <p>Class of storage (STANDARD or REDUCED_REDUNDANCY) used to store the uploaded
    /// object.</p>
    pub storage_class: Option<StorageClass>,
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub upload_id: Option<MultipartUploadId>,
}

pub type Location = String;

pub type LocationPrefix = String;

/// <p>Describes where logs are stored and the prefix that Amazon S3 assigns to all log object keys
/// for a bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTlogging.html">PUT Bucket logging</a> in the
/// <i>Amazon S3 API Reference</i>.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct LoggingEnabled {
    /// <p>Specifies the bucket where you want Amazon S3 to store server access logs. You can have your
    /// logs delivered to any bucket that you own, including the same bucket that is being logged.
    /// You can also configure multiple buckets to deliver their logs to the same target bucket. In
    /// this case, you should choose a different <code>TargetPrefix</code> for each source bucket
    /// so that the delivered log files can be distinguished by key.</p>
    pub target_bucket: TargetBucket,
    /// <p>Container for granting information.</p>
    /// <p>Buckets that use the bucket owner enforced setting for Object
    /// Ownership don't support target grants. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/enable-server-access-logging.html#grant-log-delivery-permissions-general">Permissions for server access log delivery</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub target_grants: Option<TargetGrants>,
    /// <p>A prefix for all log object keys. If you store log files from multiple Amazon S3 buckets in a
    /// single bucket, you can use a prefix to distinguish which log files came from which
    /// bucket.</p>
    pub target_prefix: TargetPrefix,
}

pub type MFA = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MFADelete {
    Disabled,
    Enabled,
}

impl MFADelete {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Disabled" => Some(Self::Disabled),
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for MFADelete {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MFADeleteStatus {
    Disabled,
    Enabled,
}

impl MFADeleteStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Disabled" => Some(Self::Disabled),
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for MFADeleteStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type Marker = String;

pub type MaxAgeSeconds = i32;

pub type MaxKeys = i32;

pub type MaxParts = i32;

pub type MaxUploads = i32;

pub type Message = String;

pub type Metadata = Map<MetadataKey, MetadataValue>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataDirective {
    Copy,
    Replace,
}

impl MetadataDirective {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Copy => "COPY",
            Self::Replace => "REPLACE",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"COPY" => Some(Self::Copy),
            b"REPLACE" => Some(Self::Replace),
            _ => None,
        }
    }
}

impl FromStr for MetadataDirective {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>A metadata key-value pair to store with an object.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct MetadataEntry {
    /// <p>Name of the Object.</p>
    pub name: Option<MetadataKey>,
    /// <p>Value of the Object.</p>
    pub value: Option<MetadataValue>,
}

pub type MetadataKey = String;

pub type MetadataValue = String;

/// <p> A container specifying replication metrics-related settings enabling replication
/// metrics and events.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct Metrics {
    /// <p> A container specifying the time threshold for emitting the
    /// <code>s3:Replication:OperationMissedThreshold</code> event. </p>
    pub event_threshold: Option<ReplicationTimeValue>,
    /// <p> Specifies whether the replication metrics are enabled. </p>
    pub status: MetricsStatus,
}

/// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter.
/// The operator must have at least two predicates, and an object must match all of the
/// predicates in order for the filter to apply.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct MetricsAndOperator {
    /// <p>The access point ARN used when evaluating an <code>AND</code> predicate.</p>
    pub access_point_arn: Option<AccessPointArn>,
    /// <p>The prefix used when evaluating an AND predicate.</p>
    pub prefix: Option<Prefix>,
    /// <p>The list of tags used when evaluating an AND predicate.</p>
    pub tags: Option<TagSet>,
}

/// <p>Specifies a metrics configuration for the CloudWatch request metrics (specified by the
/// metrics configuration ID) from an Amazon S3 bucket. If you're updating an existing metrics
/// configuration, note that this is a full replacement of the existing metrics configuration.
/// If you don't include the elements you want to keep, they are erased. For more information,
/// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTMetricConfiguration.html">PutBucketMetricsConfiguration</a>.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct MetricsConfiguration {
    /// <p>Specifies a metrics configuration filter. The metrics configuration will only include
    /// objects that meet the filter's criteria. A filter must be a prefix, an object tag, an access point ARN, or a conjunction
    /// (MetricsAndOperator).</p>
    pub filter: Option<MetricsFilter>,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: MetricsId,
}

pub type MetricsConfigurationList = List<MetricsConfiguration>;

/// <p>Specifies a metrics configuration filter. The metrics configuration only includes
/// objects that meet the filter's criteria. A filter must be a prefix, an object tag, an access point ARN, or a conjunction
/// (MetricsAndOperator). For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketMetricsConfiguration.html">PutBucketMetricsConfiguration</a>.</p>
#[derive(Debug)]
#[non_exhaustive]
pub enum MetricsFilter {
    /// <p>The access point ARN used when evaluating a metrics filter.</p>
    AccessPointArn(AccessPointArn),
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter.
    /// The operator must have at least two predicates, and an object must match all of the
    /// predicates in order for the filter to apply.</p>
    And(MetricsAndOperator),
    /// <p>The prefix used when evaluating a metrics filter.</p>
    Prefix(Prefix),
    /// <p>The tag used when evaluating a metrics filter.</p>
    Tag(Tag),
}

pub type MetricsId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricsStatus {
    Disabled,
    Enabled,
}

impl MetricsStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Disabled" => Some(Self::Disabled),
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for MetricsStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type Minutes = i32;

pub type MissingMeta = i32;

/// <p>Container for the <code>MultipartUpload</code> for the Amazon S3 object.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct MultipartUpload {
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>Date and time at which the multipart upload was initiated.</p>
    pub initiated: Option<Initiated>,
    /// <p>Identifies who initiated the multipart upload.</p>
    pub initiator: Option<Initiator>,
    /// <p>Key of the object for which the multipart upload was initiated.</p>
    pub key: Option<ObjectKey>,
    /// <p>Specifies the owner of the object that is part of the multipart upload. </p>
    pub owner: Option<Owner>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<StorageClass>,
    /// <p>Upload ID that identifies the multipart upload.</p>
    pub upload_id: Option<MultipartUploadId>,
}

pub type MultipartUploadId = String;

pub type MultipartUploadList = List<MultipartUpload>;

pub type NextKeyMarker = String;

pub type NextMarker = String;

pub type NextPartNumberMarker = String;

pub type NextToken = String;

pub type NextUploadIdMarker = String;

pub type NextVersionIdMarker = String;

/// <p>The specified bucket does not exist.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct NoSuchBucket {}

/// <p>The specified key does not exist.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct NoSuchKey {}

/// <p>The specified multipart upload does not exist.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct NoSuchUpload {}

/// <p>Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently
/// deletes the noncurrent object versions. You set this lifecycle configuration action on a
/// bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent
/// object versions at a specific period in the object's lifetime.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct NoncurrentVersionExpiration {
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. If there are this many more recent
    /// noncurrent versions, Amazon S3 will take the associated action. For more information about noncurrent
    /// versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a>
    /// in the <i>Amazon S3 User Guide</i>.</p>
    pub newer_noncurrent_versions: VersionCount,
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the
    /// associated action. The value must be a non-zero positive integer. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How
    /// Amazon S3 Calculates When an Object Became Noncurrent</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub noncurrent_days: Days,
}

/// <p>Container for the transition rule that describes when noncurrent objects transition to
/// the <code>STANDARD_IA</code>, <code>ONEZONE_IA</code>, <code>INTELLIGENT_TIERING</code>,
/// <code>GLACIER_IR</code>, <code>GLACIER</code>, or <code>DEEP_ARCHIVE</code> storage class. If your bucket is
/// versioning-enabled (or versioning is suspended), you can set this action to request that
/// Amazon S3 transition noncurrent object versions to the <code>STANDARD_IA</code>,
/// <code>ONEZONE_IA</code>, <code>INTELLIGENT_TIERING</code>, <code>GLACIER_IR</code>, <code>GLACIER</code>, or
/// <code>DEEP_ARCHIVE</code> storage class at a specific period in the object's
/// lifetime.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct NoncurrentVersionTransition {
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. If there are this many more recent
    /// noncurrent versions, Amazon S3 will take the associated action. For more information about noncurrent
    /// versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a>
    /// in the <i>Amazon S3 User Guide</i>.</p>
    pub newer_noncurrent_versions: VersionCount,
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the
    /// associated action. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How
    /// Amazon S3 Calculates How Long an Object Has Been Noncurrent</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub noncurrent_days: Days,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<TransitionStorageClass>,
}

pub type NoncurrentVersionTransitionList = List<NoncurrentVersionTransition>;

/// <p>The specified content does not exist.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct NotFound {}

/// <p>A container for specifying the notification configuration of the bucket. If this element
/// is empty, notifications are turned off for the bucket.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct NotificationConfiguration {
    /// <p>Enables delivery of events to Amazon EventBridge.</p>
    pub event_bridge_configuration: Option<EventBridgeConfiguration>,
    /// <p>Describes the Lambda functions to invoke and the events for which to invoke
    /// them.</p>
    pub lambda_function_configurations: Option<LambdaFunctionConfigurationList>,
    /// <p>The Amazon Simple Queue Service queues to publish messages to and the events for which
    /// to publish messages.</p>
    pub queue_configurations: Option<QueueConfigurationList>,
    /// <p>The topic to which notifications are sent and the events for which notifications are
    /// generated.</p>
    pub topic_configurations: Option<TopicConfigurationList>,
}

/// <p>Specifies object key name filtering rules. For information about key name filtering, see
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring
/// Event Notifications</a> in the <i>Amazon S3 User Guide</i>.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct NotificationConfigurationFilter {
    pub key: Option<S3KeyFilter>,
}

/// <p>An optional unique identifier for configurations in a notification configuration. If you
/// don't provide one, Amazon S3 will assign an ID.</p>
pub type NotificationId = String;

/// <p>An object consists of data and its descriptive metadata.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Object {
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithmList>,
    /// <p>The entity tag is a hash of the object. The ETag reflects changes only to the contents
    /// of an object, not its metadata. The ETag may or may not be an MD5 digest of the object
    /// data. Whether or not it is depends on how the object was created and how it is encrypted as
    /// described below:</p>
    /// <ul>
    /// <li>
    /// <p>Objects created by the PUT Object, POST Object, or Copy operation, or through the
    /// Amazon Web Services Management Console, and are encrypted by SSE-S3 or plaintext, have ETags that are
    /// an MD5 digest of their object data.</p>
    /// </li>
    /// <li>
    /// <p>Objects created by the PUT Object, POST Object, or Copy operation, or through the
    /// Amazon Web Services Management Console, and are encrypted by SSE-C or SSE-KMS, have ETags that are
    /// not an MD5 digest of their object data.</p>
    /// </li>
    /// <li>
    /// <p>If an object is created by either the Multipart Upload or Part Copy operation, the
    /// ETag is not an MD5 digest, regardless of the method of encryption. If an object
    /// is larger than 16 MB, the Amazon Web Services Management Console will upload or copy that object as a
    /// Multipart Upload, and therefore the ETag will not be an MD5 digest.</p>
    /// </li>
    /// </ul>
    pub e_tag: Option<ETag>,
    /// <p>The name that you assign to an object. You use the object key to retrieve the
    /// object.</p>
    pub key: Option<ObjectKey>,
    /// <p>Creation date of the object.</p>
    pub last_modified: Option<LastModified>,
    /// <p>The owner of the object</p>
    pub owner: Option<Owner>,
    /// <p>Size in bytes of the object</p>
    pub size: Size,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<ObjectStorageClass>,
}

/// <p>This action is not allowed against this storage tier.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ObjectAlreadyInActiveTierError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectAttributes {
    Checksum,
    Etag,
    ObjectParts,
    ObjectSize,
    StorageClass,
}

impl ObjectAttributes {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Checksum => "Checksum",
            Self::Etag => "ETag",
            Self::ObjectParts => "ObjectParts",
            Self::ObjectSize => "ObjectSize",
            Self::StorageClass => "StorageClass",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Checksum" => Some(Self::Checksum),
            b"ETag" => Some(Self::Etag),
            b"ObjectParts" => Some(Self::ObjectParts),
            b"ObjectSize" => Some(Self::ObjectSize),
            b"StorageClass" => Some(Self::StorageClass),
            _ => None,
        }
    }
}

impl FromStr for ObjectAttributes {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type ObjectAttributesList = List<ObjectAttributes>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectCannedACL {
    AuthenticatedRead,
    AwsExecRead,
    BucketOwnerFullControl,
    BucketOwnerRead,
    Private,
    PublicRead,
    PublicReadWrite,
}

impl ObjectCannedACL {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AuthenticatedRead => "authenticated-read",
            Self::AwsExecRead => "aws-exec-read",
            Self::BucketOwnerFullControl => "bucket-owner-full-control",
            Self::BucketOwnerRead => "bucket-owner-read",
            Self::Private => "private",
            Self::PublicRead => "public-read",
            Self::PublicReadWrite => "public-read-write",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"authenticated-read" => Some(Self::AuthenticatedRead),
            b"aws-exec-read" => Some(Self::AwsExecRead),
            b"bucket-owner-full-control" => Some(Self::BucketOwnerFullControl),
            b"bucket-owner-read" => Some(Self::BucketOwnerRead),
            b"private" => Some(Self::Private),
            b"public-read" => Some(Self::PublicRead),
            b"public-read-write" => Some(Self::PublicReadWrite),
            _ => None,
        }
    }
}

impl FromStr for ObjectCannedACL {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>Object Identifier is unique value to identify objects.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct ObjectIdentifier {
    /// <p>Key name of the object.</p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    pub key: ObjectKey,
    /// <p>VersionId for the specific version of the object to delete.</p>
    pub version_id: Option<ObjectVersionId>,
}

pub type ObjectIdentifierList = List<ObjectIdentifier>;

pub type ObjectKey = String;

pub type ObjectList = List<Object>;

/// <p>The container element for Object Lock configuration parameters.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ObjectLockConfiguration {
    /// <p>Indicates whether this bucket has an Object Lock configuration enabled.
    /// Enable <code>ObjectLockEnabled</code> when you apply <code>ObjectLockConfiguration</code>
    /// to a bucket. </p>
    pub object_lock_enabled: Option<ObjectLockEnabled>,
    /// <p>Specifies the Object Lock rule for the specified object. Enable the this rule when you apply
    /// <code>ObjectLockConfiguration</code> to a bucket. Bucket settings require both a mode and a period.
    /// The period can be either <code>Days</code> or <code>Years</code> but you must select one.
    /// You cannot specify <code>Days</code> and <code>Years</code> at the same time.</p>
    pub rule: Option<ObjectLockRule>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLockEnabled {
    Enabled,
}

impl ObjectLockEnabled {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for ObjectLockEnabled {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type ObjectLockEnabledForBucket = bool;

/// <p>A legal hold configuration for an object.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ObjectLockLegalHold {
    /// <p>Indicates whether the specified object has a legal hold in place.</p>
    pub status: Option<ObjectLockLegalHoldStatus>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLockLegalHoldStatus {
    Off,
    On,
}

impl ObjectLockLegalHoldStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Off => "OFF",
            Self::On => "ON",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"OFF" => Some(Self::Off),
            b"ON" => Some(Self::On),
            _ => None,
        }
    }
}

impl FromStr for ObjectLockLegalHoldStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLockMode {
    Compliance,
    Governance,
}

impl ObjectLockMode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Compliance => "COMPLIANCE",
            Self::Governance => "GOVERNANCE",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"COMPLIANCE" => Some(Self::Compliance),
            b"GOVERNANCE" => Some(Self::Governance),
            _ => None,
        }
    }
}

impl FromStr for ObjectLockMode {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type ObjectLockRetainUntilDate = Timestamp;

/// <p>A Retention configuration for an object.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ObjectLockRetention {
    /// <p>Indicates the Retention mode for the specified object.</p>
    pub mode: Option<ObjectLockRetentionMode>,
    /// <p>The date on which this Object Lock Retention will expire.</p>
    pub retain_until_date: Option<Date>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLockRetentionMode {
    Compliance,
    Governance,
}

impl ObjectLockRetentionMode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Compliance => "COMPLIANCE",
            Self::Governance => "GOVERNANCE",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"COMPLIANCE" => Some(Self::Compliance),
            b"GOVERNANCE" => Some(Self::Governance),
            _ => None,
        }
    }
}

impl FromStr for ObjectLockRetentionMode {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>The container element for an Object Lock rule.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ObjectLockRule {
    /// <p>The default Object Lock retention mode and period that you want to apply to new objects
    /// placed in the specified bucket. Bucket settings require both a mode and a period.
    /// The period can be either <code>Days</code> or <code>Years</code> but you must select one.
    /// You cannot specify <code>Days</code> and <code>Years</code> at the same time.</p>
    pub default_retention: Option<DefaultRetention>,
}

pub type ObjectLockToken = String;

/// <p>The source object of the COPY action is not in the active tier and is only stored in
/// Amazon S3 Glacier.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ObjectNotInActiveTierError {}

/// <p>The container element for object ownership for a bucket's ownership controls.</p>
/// <p>BucketOwnerPreferred - Objects uploaded to the bucket change ownership to the bucket
/// owner if the objects are uploaded with the <code>bucket-owner-full-control</code> canned
/// ACL.</p>
/// <p>ObjectWriter - The uploading account will own the object if the object is uploaded with
/// the <code>bucket-owner-full-control</code> canned ACL.</p>
/// <p>BucketOwnerEnforced - Access control lists (ACLs) are disabled and no longer affect permissions.
/// The bucket owner automatically owns and has full control over every object in the bucket. The bucket only
/// accepts PUT requests that don't specify an ACL or bucket owner full control
/// ACLs, such as the <code>bucket-owner-full-control</code> canned
/// ACL or an equivalent form of this ACL expressed in the XML format.</p>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectOwnership {
    BucketOwnerEnforced,
    BucketOwnerPreferred,
    ObjectWriter,
}

impl ObjectOwnership {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BucketOwnerEnforced => "BucketOwnerEnforced",
            Self::BucketOwnerPreferred => "BucketOwnerPreferred",
            Self::ObjectWriter => "ObjectWriter",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"BucketOwnerEnforced" => Some(Self::BucketOwnerEnforced),
            b"BucketOwnerPreferred" => Some(Self::BucketOwnerPreferred),
            b"ObjectWriter" => Some(Self::ObjectWriter),
            _ => None,
        }
    }
}

impl FromStr for ObjectOwnership {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>A container for elements related to an individual part.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ObjectPart {
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>The part number identifying the part. This value is a positive integer between 1 and
    /// 10,000.</p>
    pub part_number: PartNumber,
    /// <p>The size of the uploaded part in bytes.</p>
    pub size: Size,
}

pub type ObjectSize = i64;

pub type ObjectSizeGreaterThanBytes = i64;

pub type ObjectSizeLessThanBytes = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectStorageClass {
    DeepArchive,
    Glacier,
    GlacierIr,
    IntelligentTiering,
    OnezoneIa,
    Outposts,
    ReducedRedundancy,
    Standard,
    StandardIa,
}

impl ObjectStorageClass {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DeepArchive => "DEEP_ARCHIVE",
            Self::Glacier => "GLACIER",
            Self::GlacierIr => "GLACIER_IR",
            Self::IntelligentTiering => "INTELLIGENT_TIERING",
            Self::OnezoneIa => "ONEZONE_IA",
            Self::Outposts => "OUTPOSTS",
            Self::ReducedRedundancy => "REDUCED_REDUNDANCY",
            Self::Standard => "STANDARD",
            Self::StandardIa => "STANDARD_IA",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"DEEP_ARCHIVE" => Some(Self::DeepArchive),
            b"GLACIER" => Some(Self::Glacier),
            b"GLACIER_IR" => Some(Self::GlacierIr),
            b"INTELLIGENT_TIERING" => Some(Self::IntelligentTiering),
            b"ONEZONE_IA" => Some(Self::OnezoneIa),
            b"OUTPOSTS" => Some(Self::Outposts),
            b"REDUCED_REDUNDANCY" => Some(Self::ReducedRedundancy),
            b"STANDARD" => Some(Self::Standard),
            b"STANDARD_IA" => Some(Self::StandardIa),
            _ => None,
        }
    }
}

impl FromStr for ObjectStorageClass {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>The version of an object.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ObjectVersion {
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithmList>,
    /// <p>The entity tag is an MD5 hash of that version of the object.</p>
    pub e_tag: Option<ETag>,
    /// <p>Specifies whether the object is (true) or is not (false) the latest version of an
    /// object.</p>
    pub is_latest: IsLatest,
    /// <p>The object key.</p>
    pub key: Option<ObjectKey>,
    /// <p>Date and time the object was last modified.</p>
    pub last_modified: Option<LastModified>,
    /// <p>Specifies the owner of the object.</p>
    pub owner: Option<Owner>,
    /// <p>Size in bytes of the object.</p>
    pub size: Size,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<ObjectVersionStorageClass>,
    /// <p>Version ID of an object.</p>
    pub version_id: Option<ObjectVersionId>,
}

pub type ObjectVersionId = String;

pub type ObjectVersionList = List<ObjectVersion>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectVersionStorageClass {
    Standard,
}

impl ObjectVersionStorageClass {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "STANDARD",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"STANDARD" => Some(Self::Standard),
            _ => None,
        }
    }
}

impl FromStr for ObjectVersionStorageClass {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>Describes the location where the restore job's output is stored.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct OutputLocation {
    /// <p>Describes an S3 location that will receive the results of the restore request.</p>
    pub s3: Option<S3Location>,
}

/// <p>Describes how results of the Select job are serialized.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct OutputSerialization {
    /// <p>Describes the serialization of CSV-encoded Select results.</p>
    pub csv: Option<CSVOutput>,
    /// <p>Specifies JSON as request's output serialization format.</p>
    pub json: Option<JSONOutput>,
}

/// <p>Container for the owner's display name and ID.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Owner {
    /// <p>Container for the display name of the owner.</p>
    pub display_name: Option<DisplayName>,
    /// <p>Container for the ID of the owner.</p>
    pub id: Option<ID>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerOverride {
    Destination,
}

impl OwnerOverride {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Destination => "Destination",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Destination" => Some(Self::Destination),
            _ => None,
        }
    }
}

impl FromStr for OwnerOverride {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>The container element for a bucket's ownership controls.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct OwnershipControls {
    /// <p>The container element for an ownership control rule.</p>
    pub rules: OwnershipControlsRules,
}

/// <p>The container element for an ownership control rule.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct OwnershipControlsRule {
    pub object_ownership: ObjectOwnership,
}

pub type OwnershipControlsRules = List<OwnershipControlsRule>;

/// <p>Container for Parquet.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ParquetInput {}

/// <p>Container for elements related to a part.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Part {
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub e_tag: Option<ETag>,
    /// <p>Date and time at which the part was uploaded.</p>
    pub last_modified: Option<LastModified>,
    /// <p>Part number identifying the part. This is a positive integer between 1 and
    /// 10,000.</p>
    pub part_number: PartNumber,
    /// <p>Size in bytes of the uploaded part data.</p>
    pub size: Size,
}

pub type PartNumber = i32;

pub type PartNumberMarker = String;

pub type Parts = List<Part>;

pub type PartsCount = i32;

pub type PartsList = List<ObjectPart>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payer {
    BucketOwner,
    Requester,
}

impl Payer {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BucketOwner => "BucketOwner",
            Self::Requester => "Requester",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"BucketOwner" => Some(Self::BucketOwner),
            b"Requester" => Some(Self::Requester),
            _ => None,
        }
    }
}

impl FromStr for Payer {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    FullControl,
    Read,
    ReadAcp,
    Write,
    WriteAcp,
}

impl Permission {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::FullControl => "FULL_CONTROL",
            Self::Read => "READ",
            Self::ReadAcp => "READ_ACP",
            Self::Write => "WRITE",
            Self::WriteAcp => "WRITE_ACP",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"FULL_CONTROL" => Some(Self::FullControl),
            b"READ" => Some(Self::Read),
            b"READ_ACP" => Some(Self::ReadAcp),
            b"WRITE" => Some(Self::Write),
            b"WRITE_ACP" => Some(Self::WriteAcp),
            _ => None,
        }
    }
}

impl FromStr for Permission {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type Policy = String;

/// <p>The container element for a bucket's policy status.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct PolicyStatus {
    /// <p>The policy status for this bucket. <code>TRUE</code> indicates that this bucket is
    /// public. <code>FALSE</code> indicates that the bucket is not public.</p>
    pub is_public: IsPublic,
}

pub type Prefix = String;

pub type Priority = i32;

/// <p>This data type contains information about progress of an operation.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Progress {
    /// <p>The current number of uncompressed object bytes processed.</p>
    pub bytes_processed: BytesProcessed,
    /// <p>The current number of bytes of records payload data returned.</p>
    pub bytes_returned: BytesReturned,
    /// <p>The current number of object bytes scanned.</p>
    pub bytes_scanned: BytesScanned,
}

/// <p>This data type contains information about the progress event of an operation.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ProgressEvent {
    /// <p>The Progress event details.</p>
    pub details: Option<Progress>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Http,
    Https,
}

impl Protocol {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"http" => Some(Self::Http),
            b"https" => Some(Self::Https),
            _ => None,
        }
    }
}

impl FromStr for Protocol {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>The PublicAccessBlock configuration that you want to apply to this Amazon S3 bucket. You can
/// enable the configuration options in any combination. For more information about when Amazon S3
/// considers a bucket or object public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a> in the <i>Amazon S3 User Guide</i>. </p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct PublicAccessBlockConfiguration {
    /// <p>Specifies whether Amazon S3 should block public access control lists (ACLs) for this bucket
    /// and objects in this bucket. Setting this element to <code>TRUE</code> causes the following
    /// behavior:</p>
    /// <ul>
    /// <li>
    /// <p>PUT Bucket ACL and PUT Object ACL calls fail if the specified ACL is
    /// public.</p>
    /// </li>
    /// <li>
    /// <p>PUT Object calls fail if the request includes a public ACL.</p>
    /// </li>
    /// <li>
    /// <p>PUT Bucket calls fail if the request includes a public ACL.</p>
    /// </li>
    /// </ul>
    /// <p>Enabling this setting doesn't affect existing policies or ACLs.</p>
    pub block_public_acls: Setting,
    /// <p>Specifies whether Amazon S3 should block public bucket policies for this bucket. Setting this
    /// element to <code>TRUE</code> causes Amazon S3 to reject calls to PUT Bucket policy if the
    /// specified bucket policy allows public access. </p>
    /// <p>Enabling this setting doesn't affect existing bucket policies.</p>
    pub block_public_policy: Setting,
    /// <p>Specifies whether Amazon S3 should ignore public ACLs for this bucket and objects in this
    /// bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to ignore all public ACLs on
    /// this bucket and objects in this bucket.</p>
    /// <p>Enabling this setting doesn't affect the persistence of any existing ACLs and doesn't
    /// prevent new public ACLs from being set.</p>
    pub ignore_public_acls: Setting,
    /// <p>Specifies whether Amazon S3 should restrict public bucket policies for this bucket. Setting
    /// this element to <code>TRUE</code> restricts access to this bucket to only Amazon Web Service
    /// principals and authorized users within this account if the bucket has a public
    /// policy.</p>
    /// <p>Enabling this setting doesn't affect previously stored bucket policies, except that
    /// public and cross-account access within any public bucket policy, including non-public
    /// delegation to specific accounts, is blocked.</p>
    pub restrict_public_buckets: Setting,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketAccelerateConfigurationInput {
    /// <p>Container for setting the transfer acceleration state.</p>
    pub accelerate_configuration: AccelerateConfiguration,
    /// <p>The name of the bucket for which the accelerate configuration is set.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketAclInput {
    /// <p>The canned ACL to apply to the bucket.</p>
    pub acl: Option<BucketCannedACL>,
    /// <p>Contains the elements that set the ACL permissions for an object per grantee.</p>
    pub access_control_policy: Option<AccessControlPolicy>,
    /// <p>The bucket to which to apply the ACL.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The base64-encoded 128-bit MD5 digest of the data. This header must be used as a message
    /// integrity check to verify that the request body was not corrupted in transit. For more
    /// information, go to <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC
    /// 1864.</a>
    /// </p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the
    /// bucket.</p>
    pub grant_full_control: Option<GrantFullControl>,
    /// <p>Allows grantee to list the objects in the bucket.</p>
    pub grant_read: Option<GrantRead>,
    /// <p>Allows grantee to read the bucket ACL.</p>
    pub grant_read_acp: Option<GrantReadACP>,
    /// <p>Allows grantee to create new objects in the bucket.</p>
    /// <p>For the bucket and object owners of existing objects, also allows deletions and overwrites of those objects.</p>
    pub grant_write: Option<GrantWrite>,
    /// <p>Allows grantee to write the ACL for the applicable bucket.</p>
    pub grant_write_acp: Option<GrantWriteACP>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketAnalyticsConfigurationInput {
    /// <p>The configuration and any analyses for the analytics filter.</p>
    pub analytics_configuration: AnalyticsConfiguration,
    /// <p>The name of the bucket to which an analytics configuration is stored.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID that identifies the analytics configuration.</p>
    pub id: AnalyticsId,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketCorsInput {
    /// <p>Specifies the bucket impacted by the <code>cors</code>configuration.</p>
    pub bucket: BucketName,
    /// <p>Describes the cross-origin access configuration for objects in an Amazon S3 bucket. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cors.html">Enabling Cross-Origin Resource
    /// Sharing</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub cors_configuration: CORSConfiguration,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The base64-encoded 128-bit MD5 digest of the data. This header must be used as a message
    /// integrity check to verify that the request body was not corrupted in transit. For more
    /// information, go to <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC
    /// 1864.</a>
    /// </p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketEncryptionInput {
    /// <p>Specifies default encryption for a bucket using server-side encryption with Amazon S3-managed
    /// keys (SSE-S3) or customer managed keys (SSE-KMS). For information about
    /// the Amazon S3 default encryption feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 Default Bucket Encryption</a>
    /// in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The base64-encoded 128-bit MD5 digest of the server-side encryption configuration.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    pub server_side_encryption_configuration: ServerSideEncryptionConfiguration,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketIntelligentTieringConfigurationInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub bucket: BucketName,
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub id: IntelligentTieringId,
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub intelligent_tiering_configuration: IntelligentTieringConfiguration,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketInventoryConfigurationInput {
    /// <p>The name of the bucket where the inventory configuration will be stored.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: InventoryId,
    /// <p>Specifies the inventory configuration.</p>
    pub inventory_configuration: InventoryConfiguration,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketLifecycleConfigurationInput {
    /// <p>The name of the bucket for which to set the configuration.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Container for lifecycle rules. You can add as many as 1,000 rules.</p>
    pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketLoggingInput {
    /// <p>The name of the bucket for which to set the logging parameters.</p>
    pub bucket: BucketName,
    /// <p>Container for logging status information.</p>
    pub bucket_logging_status: BucketLoggingStatus,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The MD5 hash of the <code>PutBucketLogging</code> request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketMetricsConfigurationInput {
    /// <p>The name of the bucket for which the metrics configuration is set.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: MetricsId,
    /// <p>Specifies the metrics configuration.</p>
    pub metrics_configuration: MetricsConfiguration,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketNotificationConfigurationInput {
    /// <p>The name of the bucket.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    pub notification_configuration: NotificationConfiguration,
    /// <p>Skips validation of Amazon SQS, Amazon SNS, and Lambda destinations. True or false value.</p>
    pub skip_destination_validation: SkipValidation,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketOwnershipControlsInput {
    /// <p>The name of the Amazon S3 bucket whose <code>OwnershipControls</code> you want to set.</p>
    pub bucket: BucketName,
    /// <p>The MD5 hash of the <code>OwnershipControls</code> request body. </p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The <code>OwnershipControls</code> (BucketOwnerEnforced, BucketOwnerPreferred, or ObjectWriter) that you want
    /// to apply to this Amazon S3 bucket.</p>
    pub ownership_controls: OwnershipControls,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketPolicyInput {
    /// <p>The name of the bucket.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>Set this parameter to true to confirm that you want to remove your permissions to change
    /// this bucket policy in the future.</p>
    pub confirm_remove_self_bucket_access: ConfirmRemoveSelfBucketAccess,
    /// <p>The MD5 hash of the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The bucket policy as a JSON document.</p>
    pub policy: Policy,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketReplicationInput {
    /// <p>The name of the bucket</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message
    /// integrity check to verify that the request body was not corrupted in transit. For more
    /// information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    pub replication_configuration: ReplicationConfiguration,
    /// <p>A token to allow Object Lock to be enabled for an existing bucket.</p>
    pub token: Option<ObjectLockToken>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketRequestPaymentInput {
    /// <p>The bucket name.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The base64-encoded 128-bit MD5 digest of the data. You must use this header as a
    /// message integrity check to verify that the request body was not corrupted in transit. For
    /// more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC
    /// 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Container for Payer.</p>
    pub request_payment_configuration: RequestPaymentConfiguration,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketTaggingInput {
    /// <p>The bucket name.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message
    /// integrity check to verify that the request body was not corrupted in transit. For more
    /// information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Container for the <code>TagSet</code> and <code>Tag</code> elements.</p>
    pub tagging: Tagging,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketVersioningInput {
    /// <p>The bucket name.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>>The base64-encoded 128-bit MD5 digest of the data. You must use this header as a
    /// message integrity check to verify that the request body was not corrupted in transit. For
    /// more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC
    /// 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The concatenation of the authentication device's serial number, a space, and the value
    /// that is displayed on your authentication device.</p>
    pub mfa: Option<MFA>,
    /// <p>Container for setting the versioning state.</p>
    pub versioning_configuration: VersioningConfiguration,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutBucketWebsiteInput {
    /// <p>The bucket name.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message
    /// integrity check to verify that the request body was not corrupted in transit. For more
    /// information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Container for the request.</p>
    pub website_configuration: WebsiteConfiguration,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutObjectAclInput {
    /// <p>The canned ACL to apply to the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL">Canned ACL</a>.</p>
    pub acl: Option<ObjectCannedACL>,
    /// <p>Contains the elements that set the ACL permissions for an object per grantee.</p>
    pub access_control_policy: Option<AccessControlPolicy>,
    /// <p>The bucket name that contains the object to which you want to attach the ACL. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The base64-encoded 128-bit MD5 digest of the data. This header must be used as a message
    /// integrity check to verify that the request body was not corrupted in transit. For more
    /// information, go to <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC
    /// 1864.></a>
    /// </p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the
    /// bucket.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_full_control: Option<GrantFullControl>,
    /// <p>Allows grantee to list the objects in the
    /// bucket.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_read: Option<GrantRead>,
    /// <p>Allows grantee to read the bucket ACL.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_read_acp: Option<GrantReadACP>,
    /// <p>Allows grantee to create new objects in the bucket.</p>
    /// <p>For the bucket and object owners of existing objects, also allows deletions and overwrites of those objects.</p>
    pub grant_write: Option<GrantWrite>,
    /// <p>Allows grantee to write the ACL for the applicable
    /// bucket.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_write_acp: Option<GrantWriteACP>,
    /// <p>Key for which the PUT action was initiated.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub key: ObjectKey,
    pub request_payer: Option<RequestPayer>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct PutObjectAclOutput {
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutObjectInput {
    /// <p>The canned ACL to apply to the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL">Canned
    /// ACL</a>.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub acl: Option<ObjectCannedACL>,
    /// <p>Object data.</p>
    pub body: Option<StreamingBlob>,
    /// <p>The bucket name to which the PUT action was initiated. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Specifies whether Amazon S3 should use an S3 Bucket Key for object encryption with server-side encryption using AWS KMS (SSE-KMS). Setting this header to <code>true</code> causes Amazon S3 to use an S3 Bucket Key for object encryption with SSE-KMS.</p>
    /// <p>Specifying this header with a PUT action doesn’t affect bucket-level settings for S3 Bucket Key.</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p> Can be used to specify caching behavior along the request/reply chain. For more
    /// information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
    pub cache_control: Option<CacheControl>,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 32-bit CRC32C checksum of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 160-bit SHA-1 digest of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Specifies presentational information for the object. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1">http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1</a>.</p>
    pub content_disposition: Option<ContentDisposition>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding
    /// mechanisms must be applied to obtain the media-type referenced by the Content-Type header
    /// field. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11</a>.</p>
    pub content_encoding: Option<ContentEncoding>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<ContentLanguage>,
    /// <p>Size of the body in bytes. This parameter is useful when the size of the body cannot be
    /// determined automatically. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.13">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.13</a>.</p>
    pub content_length: ContentLength,
    /// <p>The base64-encoded 128-bit MD5 digest of the message (without the headers) according to
    /// RFC 1864. This header can be used as a message integrity check to verify that the data is
    /// the same data that was originally sent. Although it is optional, we recommend using the
    /// Content-MD5 mechanism as an end-to-end integrity check. For more information about REST
    /// request authentication, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAuthentication.html">REST
    /// Authentication</a>.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>A standard MIME type describing the format of the contents. For more information, see
    /// <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17</a>.</p>
    pub content_type: Option<ContentType>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The date and time at which the object is no longer cacheable. For more information, see
    /// <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.21">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.21</a>.</p>
    pub expires: Option<Expires>,
    /// <p>Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the
    /// object.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_full_control: Option<GrantFullControl>,
    /// <p>Allows grantee to read the object data and its
    /// metadata.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_read: Option<GrantRead>,
    /// <p>Allows grantee to read the object ACL.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_read_acp: Option<GrantReadACP>,
    /// <p>Allows grantee to write the ACL for the applicable
    /// object.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    pub grant_write_acp: Option<GrantWriteACP>,
    /// <p>Object key for which the PUT action was initiated.</p>
    pub key: ObjectKey,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<Metadata>,
    /// <p>Specifies whether a legal hold will be applied to this object. For more information
    /// about S3 Object Lock, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Object
    /// Lock</a>.</p>
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    /// <p>The Object Lock mode that you want to apply to this object.</p>
    pub object_lock_mode: Option<ObjectLockMode>,
    /// <p>The date and time when you want this object's Object Lock to expire. Must be formatted
    /// as a timestamp parameter.</p>
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub request_payer: Option<RequestPayer>,
    /// <p>Specifies the algorithm to use to when encrypting the object (for example,
    /// AES256).</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This
    /// value is used to store the object and then it is discarded; Amazon S3 does not store the
    /// encryption key. The key must be appropriate for use with the algorithm specified in the
    /// <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub sse_customer_key: Option<SSECustomerKey>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses
    /// this header for a message integrity check to ensure that the encryption key was transmitted
    /// without error.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>Specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this
    /// header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value
    /// pairs.</p>
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    /// <p>If <code>x-amz-server-side-encryption</code> is present and has the value of
    /// <code>aws:kms</code>, this header specifies the ID of the Amazon Web Services Key Management Service
    /// (Amazon Web Services KMS) symmetrical customer managed key that was used for the
    /// object. If you specify <code>x-amz-server-side-encryption:aws:kms</code>, but do not
    /// provide<code> x-amz-server-side-encryption-aws-kms-key-id</code>, Amazon S3 uses the Amazon Web Services
    /// managed key to protect the data. If the KMS key does not exist in the same account
    /// issuing the command, you must use the full ARN and not just the ID.
    /// </p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example,
    /// AES256, aws:kms).</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// <p>By default, Amazon S3 uses the STANDARD Storage Class to store newly created objects. The
    /// STANDARD storage class provides high durability and high availability. Depending on
    /// performance needs, you can specify a different Storage Class. Amazon S3 on Outposts only uses
    /// the OUTPOSTS Storage Class. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html">Storage Classes</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub storage_class: Option<StorageClass>,
    /// <p>The tag-set for the object. The tag-set must be encoded as URL Query parameters. (For
    /// example, "Key1=Value1")</p>
    pub tagging: Option<TaggingHeader>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another
    /// object in the same bucket or to an external URL. Amazon S3 stores the value of this header in
    /// the object metadata. For information about object metadata, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html">Object Key and Metadata</a>.</p>
    ///
    /// <p>In the following example, the request header sets the redirect to an object
    /// (anotherPage.html) in the same bucket:</p>
    ///
    /// <p>
    /// <code>x-amz-website-redirect-location: /anotherPage.html</code>
    /// </p>
    ///
    /// <p>In the following example, the request header sets the object redirect to another
    /// website:</p>
    ///
    /// <p>
    /// <code>x-amz-website-redirect-location: http://www.example.com/</code>
    /// </p>
    ///
    /// <p>For more information about website hosting in Amazon S3, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html">Hosting Websites on Amazon S3</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html">How to Configure Website Page
    /// Redirects</a>. </p>
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutObjectLegalHoldInput {
    /// <p>The bucket name containing the object that you want to place a legal hold on. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The MD5 hash for the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The key name for the object that you want to place a legal hold on.</p>
    pub key: ObjectKey,
    /// <p>Container element for the legal hold configuration you want to apply to the specified
    /// object.</p>
    pub legal_hold: Option<ObjectLockLegalHold>,
    pub request_payer: Option<RequestPayer>,
    /// <p>The version ID of the object that you want to place a legal hold on.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct PutObjectLegalHoldOutput {
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutObjectLockConfigurationInput {
    /// <p>The bucket whose Object Lock configuration you want to create or replace.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The MD5 hash for the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The Object Lock configuration that you want to apply to the specified bucket.</p>
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
    pub request_payer: Option<RequestPayer>,
    /// <p>A token to allow Object Lock to be enabled for an existing bucket.</p>
    pub token: Option<ObjectLockToken>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct PutObjectLockConfigurationOutput {
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct PutObjectOutput {
    /// <p>Indicates whether the uploaded object uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Entity tag for the uploaded object.</p>
    pub e_tag: Option<ETag>,
    /// <p>If the expiration is configured for the object (see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLifecycleConfiguration.html">PutBucketLifecycleConfiguration</a>), the response includes this header. It
    /// includes the <code>expiry-date</code> and <code>rule-id</code> key-value pairs that provide
    /// information about object expiration. The value of the <code>rule-id</code> is
    /// URL-encoded.</p>
    pub expiration: Option<Expiration>,
    pub request_charged: Option<RequestCharged>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header to provide round-trip message integrity verification of
    /// the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>If present, specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The
    /// value of this header is a base64-encoded UTF-8 string holding JSON with the encryption
    /// context key-value pairs.</p>
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    /// <p>If <code>x-amz-server-side-encryption</code> is present and has the value of
    /// <code>aws:kms</code>, this header specifies the ID of the Amazon Web Services Key Management Service
    /// (Amazon Web Services KMS) symmetric customer managed key that was used for the
    /// object. </p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p>If you specified server-side encryption either with an Amazon Web Services KMS key
    /// or Amazon S3-managed encryption key in your PUT request, the response includes this header. It
    /// confirms the encryption algorithm that Amazon S3 used to encrypt the object.</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// <p>Version of the object.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutObjectRetentionInput {
    /// <p>The bucket name that contains the object you want to apply this Object Retention
    /// configuration to. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Indicates whether this action should bypass Governance-mode restrictions.</p>
    pub bypass_governance_retention: BypassGovernanceRetention,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The MD5 hash for the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The key name for the object that you want to apply this Object Retention configuration
    /// to.</p>
    pub key: ObjectKey,
    pub request_payer: Option<RequestPayer>,
    /// <p>The container element for the Object Retention configuration.</p>
    pub retention: Option<ObjectLockRetention>,
    /// <p>The version ID for the object that you want to apply this Object Retention configuration
    /// to.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct PutObjectRetentionOutput {
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutObjectTaggingInput {
    /// <p>The bucket name containing the object. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The MD5 hash for the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Name of the object key.</p>
    pub key: ObjectKey,
    pub request_payer: Option<RequestPayer>,
    /// <p>Container for the <code>TagSet</code> and <code>Tag</code> elements</p>
    pub tagging: Tagging,
    /// <p>The versionId of the object that the tag-set will be added to.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct PutObjectTaggingOutput {
    /// <p>The versionId of the object the tag-set was added to.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct PutPublicAccessBlockInput {
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want
    /// to set.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The MD5 hash of the <code>PutPublicAccessBlock</code> request body. </p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The <code>PublicAccessBlock</code> configuration that you want to apply to this Amazon S3
    /// bucket. You can enable the configuration options in any combination. For more information
    /// about when Amazon S3 considers a bucket or object public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

pub type QueueArn = String;

/// <p>Specifies the configuration for publishing messages to an Amazon Simple Queue Service
/// (Amazon SQS) queue when Amazon S3 detects specified events.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct QueueConfiguration {
    /// <p>A collection of bucket events for which to send notifications</p>
    pub events: EventList,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<NotificationId>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SQS queue to which Amazon S3 publishes a message
    /// when it detects events of the specified type.</p>
    pub queue_arn: QueueArn,
}

pub type QueueConfigurationList = List<QueueConfiguration>;

pub type Quiet = bool;

pub type QuoteCharacter = String;

pub type QuoteEscapeCharacter = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteFields {
    Always,
    Asneeded,
}

impl QuoteFields {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Always => "ALWAYS",
            Self::Asneeded => "ASNEEDED",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"ALWAYS" => Some(Self::Always),
            b"ASNEEDED" => Some(Self::Asneeded),
            _ => None,
        }
    }
}

impl FromStr for QuoteFields {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type RecordDelimiter = String;

/// <p>The container for the records event.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct RecordsEvent {
    /// <p>The byte array of partial, one or more result records.</p>
    pub payload: Option<Body>,
}

/// <p>Specifies how requests are redirected. In the event of an error, you can specify a
/// different error code to return.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Redirect {
    /// <p>The host name to use in the redirect request.</p>
    pub host_name: Option<HostName>,
    /// <p>The HTTP redirect code to use on the response. Not required if one of the siblings is
    /// present.</p>
    pub http_redirect_code: Option<HttpRedirectCode>,
    /// <p>Protocol to use when redirecting requests. The default is the protocol that is used in
    /// the original request.</p>
    pub protocol: Option<Protocol>,
    /// <p>The object key prefix to use in the redirect request. For example, to redirect requests
    /// for all pages with prefix <code>docs/</code> (objects in the <code>docs/</code> folder) to
    /// <code>documents/</code>, you can set a condition block with <code>KeyPrefixEquals</code>
    /// set to <code>docs/</code> and in the Redirect set <code>ReplaceKeyPrefixWith</code> to
    /// <code>/documents</code>. Not required if one of the siblings is present. Can be present
    /// only if <code>ReplaceKeyWith</code> is not provided.</p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    pub replace_key_prefix_with: Option<ReplaceKeyPrefixWith>,
    /// <p>The specific object key to use in the redirect request. For example, redirect request to
    /// <code>error.html</code>. Not required if one of the siblings is present. Can be present
    /// only if <code>ReplaceKeyPrefixWith</code> is not provided.</p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    pub replace_key_with: Option<ReplaceKeyWith>,
}

/// <p>Specifies the redirect behavior of all requests to a website endpoint of an Amazon S3
/// bucket.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct RedirectAllRequestsTo {
    /// <p>Name of the host where requests are redirected.</p>
    pub host_name: HostName,
    /// <p>Protocol to use when redirecting requests. The default is the protocol that is used in
    /// the original request.</p>
    pub protocol: Option<Protocol>,
}

pub type ReplaceKeyPrefixWith = String;

pub type ReplaceKeyWith = String;

pub type ReplicaKmsKeyID = String;

/// <p>A filter that you can specify for selection for modifications on replicas. Amazon S3 doesn't
/// replicate replica modifications by default. In the latest version of replication
/// configuration (when <code>Filter</code> is specified), you can specify this element and set
/// the status to <code>Enabled</code> to replicate modifications on replicas. </p>
/// <note>
/// <p> If you don't specify the <code>Filter</code> element, Amazon S3 assumes that the
/// replication configuration is the earlier version, V1. In the earlier version, this
/// element is not allowed.</p>
/// </note>
#[derive(Debug)]
#[non_exhaustive]
pub struct ReplicaModifications {
    /// <p>Specifies whether Amazon S3 replicates modifications on replicas.</p>
    pub status: ReplicaModificationsStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicaModificationsStatus {
    Disabled,
    Enabled,
}

impl ReplicaModificationsStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Disabled" => Some(Self::Disabled),
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for ReplicaModificationsStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>A container for replication rules. You can add up to 1,000 rules. The maximum size of a
/// replication configuration is 2 MB.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct ReplicationConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role that
    /// Amazon S3 assumes when replicating objects. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-how-setup.html">How to Set Up
    /// Replication</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub role: Role,
    /// <p>A container for one or more replication rules. A replication configuration must have at
    /// least one rule and can contain a maximum of 1,000 rules. </p>
    pub rules: ReplicationRules,
}

/// <p>Specifies which Amazon S3 objects to replicate and where to store the replicas.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct ReplicationRule {
    pub delete_marker_replication: Option<DeleteMarkerReplication>,
    /// <p>A container for information about the replication destination and its configurations
    /// including enabling the S3 Replication Time Control (S3 RTC).</p>
    pub destination: Destination,
    /// <p></p>
    pub existing_object_replication: Option<ExistingObjectReplication>,
    pub filter: Option<ReplicationRuleFilter>,
    /// <p>A unique identifier for the rule. The maximum value is 255 characters.</p>
    pub id: Option<ID>,
    /// <p>An object key name prefix that identifies the object or objects to which the rule
    /// applies. The maximum prefix length is 1,024 characters. To include all objects in a bucket,
    /// specify an empty string. </p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    pub prefix: Option<Prefix>,
    /// <p>The priority indicates which rule has precedence whenever two or more replication rules
    /// conflict. Amazon S3 will attempt to replicate objects according to all replication rules.
    /// However, if there are two or more rules with the same destination bucket, then objects will
    /// be replicated according to the rule with the highest priority. The higher the number, the
    /// higher the priority. </p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication.html">Replication</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub priority: Priority,
    /// <p>A container that describes additional filters for identifying the source objects that
    /// you want to replicate. You can choose to enable or disable the replication of these
    /// objects. Currently, Amazon S3 supports only the filter that you can specify for objects created
    /// with server-side encryption using a customer managed key stored in Amazon Web Services Key Management
    /// Service (SSE-KMS).</p>
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    /// <p>Specifies whether the rule is enabled.</p>
    pub status: ReplicationRuleStatus,
}

/// <p>A container for specifying rule filters. The filters determine the subset of objects to
/// which the rule applies. This element is required only if you specify more than one filter. </p>
/// <p>For example:</p>
/// <ul>
/// <li>
/// <p>If you specify both a <code>Prefix</code> and a <code>Tag</code> filter, wrap
/// these filters in an <code>And</code> tag. </p>
/// </li>
/// <li>
/// <p>If you specify a filter based on multiple tags, wrap the <code>Tag</code> elements
/// in an <code>And</code> tag.</p>
/// </li>
/// </ul>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ReplicationRuleAndOperator {
    /// <p>An object key name prefix that identifies the subset of objects to which the rule
    /// applies.</p>
    pub prefix: Option<Prefix>,
    /// <p>An array of tags containing key and value pairs.</p>
    pub tags: Option<TagSet>,
}

/// <p>A filter that identifies the subset of objects to which the replication rule applies. A
/// <code>Filter</code> must specify exactly one <code>Prefix</code>, <code>Tag</code>, or
/// an <code>And</code> child element.</p>
#[derive(Debug)]
#[non_exhaustive]
pub enum ReplicationRuleFilter {
    /// <p>A container for specifying rule filters. The filters determine the subset of objects to
    /// which the rule applies. This element is required only if you specify more than one filter.
    /// For example: </p>
    /// <ul>
    /// <li>
    /// <p>If you specify both a <code>Prefix</code> and a <code>Tag</code> filter, wrap
    /// these filters in an <code>And</code> tag.</p>
    /// </li>
    /// <li>
    /// <p>If you specify a filter based on multiple tags, wrap the <code>Tag</code> elements
    /// in an <code>And</code> tag.</p>
    /// </li>
    /// </ul>
    And(ReplicationRuleAndOperator),
    /// <p>An object key name prefix that identifies the subset of objects to which the rule
    /// applies.</p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    Prefix(Prefix),
    /// <p>A container for specifying a tag key and value. </p>
    /// <p>The rule applies only to objects that have the tag in their tag set.</p>
    Tag(Tag),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationRuleStatus {
    Disabled,
    Enabled,
}

impl ReplicationRuleStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Disabled" => Some(Self::Disabled),
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for ReplicationRuleStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type ReplicationRules = List<ReplicationRule>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationStatus {
    Complete,
    Failed,
    Pending,
    Replica,
}

impl ReplicationStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Complete => "COMPLETE",
            Self::Failed => "FAILED",
            Self::Pending => "PENDING",
            Self::Replica => "REPLICA",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"COMPLETE" => Some(Self::Complete),
            b"FAILED" => Some(Self::Failed),
            b"PENDING" => Some(Self::Pending),
            b"REPLICA" => Some(Self::Replica),
            _ => None,
        }
    }
}

impl FromStr for ReplicationStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p> A container specifying S3 Replication Time Control (S3 RTC) related information, including whether S3 RTC is
/// enabled and the time when all objects and operations on objects must be replicated. Must be
/// specified together with a <code>Metrics</code> block. </p>
#[derive(Debug)]
#[non_exhaustive]
pub struct ReplicationTime {
    /// <p> Specifies whether the replication time is enabled. </p>
    pub status: ReplicationTimeStatus,
    /// <p> A container specifying the time by which replication should be complete for all objects
    /// and operations on objects. </p>
    pub time: ReplicationTimeValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationTimeStatus {
    Disabled,
    Enabled,
}

impl ReplicationTimeStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Disabled" => Some(Self::Disabled),
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for ReplicationTimeStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p> A container specifying the time value for S3 Replication Time Control (S3 RTC) and replication metrics
/// <code>EventThreshold</code>. </p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ReplicationTimeValue {
    /// <p> Contains an integer specifying time in minutes. </p>
    /// <p> Valid value: 15</p>
    pub minutes: Minutes,
}

/// <p>If present, indicates that the requester was successfully charged for the
/// request.</p>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestCharged {
    Requester,
}

impl RequestCharged {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Requester => "requester",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"requester" => Some(Self::Requester),
            _ => None,
        }
    }
}

impl FromStr for RequestCharged {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>Confirms that the requester knows that they will be charged for the request. Bucket
/// owners need not specify this parameter in their requests. For information about downloading
/// objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in
/// Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestPayer {
    Requester,
}

impl RequestPayer {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Requester => "requester",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"requester" => Some(Self::Requester),
            _ => None,
        }
    }
}

impl FromStr for RequestPayer {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>Container for Payer.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct RequestPaymentConfiguration {
    /// <p>Specifies who pays for the download and request fees.</p>
    pub payer: Payer,
}

/// <p>Container for specifying if periodic <code>QueryProgress</code> messages should be
/// sent.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct RequestProgress {
    /// <p>Specifies whether periodic QueryProgress frames should be sent. Valid values: TRUE,
    /// FALSE. Default value: FALSE.</p>
    pub enabled: EnableRequestProgress,
}

pub type RequestRoute = String;

pub type RequestToken = String;

pub type ResponseCacheControl = String;

pub type ResponseContentDisposition = String;

pub type ResponseContentEncoding = String;

pub type ResponseContentLanguage = String;

pub type ResponseContentType = String;

pub type ResponseExpires = Timestamp;

pub type Restore = String;

#[derive(Debug)]
#[non_exhaustive]
pub struct RestoreObjectInput {
    /// <p>The bucket name containing the object to restore. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Object key for which the action was initiated.</p>
    pub key: ObjectKey,
    pub request_payer: Option<RequestPayer>,
    pub restore_request: Option<RestoreRequest>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct RestoreObjectOutput {
    pub request_charged: Option<RequestCharged>,
    /// <p>Indicates the path in the provided S3 output location where Select results will be
    /// restored to.</p>
    pub restore_output_path: Option<RestoreOutputPath>,
}

pub type RestoreOutputPath = String;

/// <p>Container for restore job parameters.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct RestoreRequest {
    /// <p>Lifetime of the active copy in days. Do not use with restores that specify
    /// <code>OutputLocation</code>.</p>
    /// <p>The Days element is required for regular restores, and must not be provided for select
    /// requests.</p>
    pub days: Days,
    /// <p>The optional description for the job.</p>
    pub description: Option<Description>,
    /// <p>S3 Glacier related parameters pertaining to this job. Do not use with restores that
    /// specify <code>OutputLocation</code>.</p>
    pub glacier_job_parameters: Option<GlacierJobParameters>,
    /// <p>Describes the location where the restore job's output is stored.</p>
    pub output_location: Option<OutputLocation>,
    /// <p>Describes the parameters for Select job types.</p>
    pub select_parameters: Option<SelectParameters>,
    /// <p>Retrieval tier at which the restore will be processed.</p>
    pub tier: Option<Tier>,
    /// <p>Type of restore request.</p>
    pub type_: Option<RestoreRequestType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreRequestType {
    Select,
}

impl RestoreRequestType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Select => "SELECT",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"SELECT" => Some(Self::Select),
            _ => None,
        }
    }
}

impl FromStr for RestoreRequestType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type Role = String;

/// <p>Specifies the redirect behavior and when a redirect is applied. For more information
/// about routing rules, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html#advanced-conditional-redirects">Configuring advanced conditional redirects</a> in the
/// <i>Amazon S3 User Guide</i>.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct RoutingRule {
    /// <p>A container for describing a condition that must be met for the specified redirect to
    /// apply. For example, 1. If request is for pages in the <code>/docs</code> folder, redirect
    /// to the <code>/documents</code> folder. 2. If request results in HTTP error 4xx, redirect
    /// request to another host where you might process the error.</p>
    pub condition: Option<Condition>,
    /// <p>Container for redirect information. You can redirect requests to another host, to
    /// another page, or with another protocol. In the event of an error, you can specify a
    /// different error code to return.</p>
    pub redirect: Redirect,
}

pub type RoutingRules = List<RoutingRule>;

/// <p>A container for object key name prefix and suffix filtering rules.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct S3KeyFilter {
    pub filter_rules: Option<FilterRuleList>,
}

/// <p>Describes an Amazon S3 location that will receive the results of the restore request.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct S3Location {
    /// <p>A list of grants that control access to the staged results.</p>
    pub access_control_list: Option<Grants>,
    /// <p>The name of the bucket where the restore results will be placed.</p>
    pub bucket_name: BucketName,
    /// <p>The canned ACL to apply to the restore results.</p>
    pub canned_acl: Option<ObjectCannedACL>,
    pub encryption: Option<Encryption>,
    /// <p>The prefix that is prepended to the restore results for this request.</p>
    pub prefix: LocationPrefix,
    /// <p>The class of storage used to store the restore results.</p>
    pub storage_class: Option<StorageClass>,
    /// <p>The tag-set that is applied to the restore results.</p>
    pub tagging: Option<Tagging>,
    /// <p>A list of metadata to store with the restore results in S3.</p>
    pub user_metadata: Option<UserMetadata>,
}

pub type SSECustomerAlgorithm = String;

pub type SSECustomerKey = String;

pub type SSECustomerKeyMD5 = String;

/// <p>Specifies the use of SSE-KMS to encrypt delivered inventory reports.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct SSEKMS {
    /// <p>Specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key
    /// to use for encrypting inventory reports.</p>
    pub key_id: SSEKMSKeyId,
}

pub type SSEKMSEncryptionContext = String;

pub type SSEKMSKeyId = String;

/// <p>Specifies the use of SSE-S3 to encrypt delivered inventory reports.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct SSES3 {}

/// <p>Specifies the byte range of the object to get the records from. A record is processed
/// when its first byte is contained by the range. This parameter is optional, but when
/// specified, it must not be empty. See RFC 2616, Section 14.35.1 about how to specify the
/// start and end of the range.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ScanRange {
    /// <p>Specifies the end of the byte range. This parameter is optional. Valid values:
    /// non-negative integers. The default value is one less than the size of the object being
    /// queried. If only the End parameter is supplied, it is interpreted to mean scan the last N
    /// bytes of the file. For example,
    /// <code><scanrange><end>50</end></scanrange></code> means scan the
    /// last 50 bytes.</p>
    pub end: End,
    /// <p>Specifies the start of the byte range. This parameter is optional. Valid values:
    /// non-negative integers. The default value is 0. If only <code>start</code> is supplied, it
    /// means scan from that point to the end of the file. For example,
    /// <code><scanrange><start>50</start></scanrange></code> means scan
    /// from byte 50 until the end of the file.</p>
    pub start: Start,
}

/// <p>Describes the parameters for Select job types.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct SelectParameters {
    /// <p>The expression that is used to query the object.</p>
    pub expression: Expression,
    /// <p>The type of the provided expression (for example, SQL).</p>
    pub expression_type: ExpressionType,
    /// <p>Describes the serialization format of the object.</p>
    pub input_serialization: InputSerialization,
    /// <p>Describes how the results of the Select job are serialized.</p>
    pub output_serialization: OutputSerialization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerSideEncryption {
    Aes256,
    AwsKms,
}

impl ServerSideEncryption {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Aes256 => "AES256",
            Self::AwsKms => "aws:kms",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"AES256" => Some(Self::Aes256),
            b"aws:kms" => Some(Self::AwsKms),
            _ => None,
        }
    }
}

impl FromStr for ServerSideEncryption {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>Describes the default server-side encryption to apply to new objects in the bucket. If a
/// PUT Object request doesn't specify any server-side encryption, this default encryption will
/// be applied. If you don't specify a customer managed key at configuration, Amazon S3 automatically creates
/// an Amazon Web Services KMS key in your Amazon Web Services account the first time that you add an object encrypted with
/// SSE-KMS to a bucket. By default, Amazon S3 uses this KMS key for SSE-KMS. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTencryption.html">PUT Bucket encryption</a> in
/// the <i>Amazon S3 API Reference</i>.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct ServerSideEncryptionByDefault {
    /// <p>Amazon Web Services Key Management Service (KMS) customer Amazon Web Services KMS key ID to use for the default
    /// encryption. This parameter is allowed if and only if <code>SSEAlgorithm</code> is set to
    /// <code>aws:kms</code>.</p>
    /// <p>You can specify the key ID or the Amazon Resource Name (ARN) of the KMS key. However, if
    /// you are using encryption with cross-account or Amazon Web Services service operations you must use a fully qualified KMS
    /// key ARN. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html#bucket-encryption-update-bucket-policy">Using encryption for cross-account operations</a>. </p>
    /// <p>
    /// <b>For example:</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>Key ARN:
    /// <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>
    /// </p>
    /// </li>
    /// </ul>
    /// <important>
    /// <p>Amazon S3 only supports symmetric KMS keys and not asymmetric KMS keys. For more information, see
    /// <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using symmetric and
    /// asymmetric keys</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    /// </important>
    pub kms_master_key_id: Option<SSEKMSKeyId>,
    /// <p>Server-side encryption algorithm to use for the default encryption.</p>
    pub sse_algorithm: ServerSideEncryption,
}

/// <p>Specifies the default server-side-encryption configuration.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct ServerSideEncryptionConfiguration {
    /// <p>Container for information about a particular server-side encryption configuration
    /// rule.</p>
    pub rules: ServerSideEncryptionRules,
}

/// <p>Specifies the default server-side encryption configuration.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ServerSideEncryptionRule {
    /// <p>Specifies the default server-side encryption to apply to new objects in the bucket. If a
    /// PUT Object request doesn't specify any server-side encryption, this default encryption will
    /// be applied.</p>
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
    /// <p>Specifies whether Amazon S3 should use an S3 Bucket Key with server-side encryption using KMS (SSE-KMS) for new objects in the bucket. Existing objects are not affected. Setting the <code>BucketKeyEnabled</code> element to <code>true</code> causes Amazon S3 to use an S3 Bucket Key. By default, S3 Bucket Key is not enabled.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html">Amazon S3 Bucket Keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket_key_enabled: BucketKeyEnabled,
}

pub type ServerSideEncryptionRules = List<ServerSideEncryptionRule>;

pub type Setting = bool;

pub type Size = i64;

pub type SkipValidation = bool;

/// <p>A container that describes additional filters for identifying the source objects that
/// you want to replicate. You can choose to enable or disable the replication of these
/// objects. Currently, Amazon S3 supports only the filter that you can specify for objects created
/// with server-side encryption using a customer managed key stored in Amazon Web Services Key Management
/// Service (SSE-KMS).</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct SourceSelectionCriteria {
    /// <p>A filter that you can specify for selections for modifications on replicas. Amazon S3 doesn't
    /// replicate replica modifications by default. In the latest version of replication
    /// configuration (when <code>Filter</code> is specified), you can specify this element and set
    /// the status to <code>Enabled</code> to replicate modifications on replicas. </p>
    /// <note>
    /// <p> If you don't specify the <code>Filter</code> element, Amazon S3 assumes that the
    /// replication configuration is the earlier version, V1. In the earlier version, this
    /// element is not allowed</p>
    /// </note>
    pub replica_modifications: Option<ReplicaModifications>,
    /// <p> A container for filter information for the selection of Amazon S3 objects encrypted with Amazon Web Services
    /// KMS. If you include <code>SourceSelectionCriteria</code> in the replication configuration,
    /// this element is required. </p>
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
}

/// <p>A container for filter information for the selection of S3 objects encrypted with Amazon Web Services
/// KMS.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct SseKmsEncryptedObjects {
    /// <p>Specifies whether Amazon S3 replicates objects created with server-side encryption using an
    /// Amazon Web Services KMS key stored in Amazon Web Services Key Management Service.</p>
    pub status: SseKmsEncryptedObjectsStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SseKmsEncryptedObjectsStatus {
    Disabled,
    Enabled,
}

impl SseKmsEncryptedObjectsStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Disabled" => Some(Self::Disabled),
            b"Enabled" => Some(Self::Enabled),
            _ => None,
        }
    }
}

impl FromStr for SseKmsEncryptedObjectsStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type Start = i64;

pub type StartAfter = String;

/// <p>Container for the stats details.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Stats {
    /// <p>The total number of uncompressed object bytes processed.</p>
    pub bytes_processed: BytesProcessed,
    /// <p>The total number of bytes of records payload data returned.</p>
    pub bytes_returned: BytesReturned,
    /// <p>The total number of object bytes scanned.</p>
    pub bytes_scanned: BytesScanned,
}

/// <p>Container for the Stats Event.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct StatsEvent {
    /// <p>The Stats event details.</p>
    pub details: Option<Stats>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageClass {
    DeepArchive,
    Glacier,
    GlacierIr,
    IntelligentTiering,
    OnezoneIa,
    Outposts,
    ReducedRedundancy,
    Standard,
    StandardIa,
}

impl StorageClass {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DeepArchive => "DEEP_ARCHIVE",
            Self::Glacier => "GLACIER",
            Self::GlacierIr => "GLACIER_IR",
            Self::IntelligentTiering => "INTELLIGENT_TIERING",
            Self::OnezoneIa => "ONEZONE_IA",
            Self::Outposts => "OUTPOSTS",
            Self::ReducedRedundancy => "REDUCED_REDUNDANCY",
            Self::Standard => "STANDARD",
            Self::StandardIa => "STANDARD_IA",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"DEEP_ARCHIVE" => Some(Self::DeepArchive),
            b"GLACIER" => Some(Self::Glacier),
            b"GLACIER_IR" => Some(Self::GlacierIr),
            b"INTELLIGENT_TIERING" => Some(Self::IntelligentTiering),
            b"ONEZONE_IA" => Some(Self::OnezoneIa),
            b"OUTPOSTS" => Some(Self::Outposts),
            b"REDUCED_REDUNDANCY" => Some(Self::ReducedRedundancy),
            b"STANDARD" => Some(Self::Standard),
            b"STANDARD_IA" => Some(Self::StandardIa),
            _ => None,
        }
    }
}

impl FromStr for StorageClass {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>Specifies data related to access patterns to be collected and made available to analyze
/// the tradeoffs between different storage classes for an Amazon S3 bucket.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct StorageClassAnalysis {
    /// <p>Specifies how data related to the storage class analysis for an Amazon S3 bucket should be
    /// exported.</p>
    pub data_export: Option<StorageClassAnalysisDataExport>,
}

/// <p>Container for data related to the storage class analysis for an Amazon S3 bucket for
/// export.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct StorageClassAnalysisDataExport {
    /// <p>The place to store the data for an analysis.</p>
    pub destination: AnalyticsExportDestination,
    /// <p>The version of the output schema to use when exporting data. Must be
    /// <code>V_1</code>.</p>
    pub output_schema_version: StorageClassAnalysisSchemaVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageClassAnalysisSchemaVersion {
    V1,
}

impl StorageClassAnalysisSchemaVersion {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::V1 => "V_1",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"V_1" => Some(Self::V1),
            _ => None,
        }
    }
}

impl FromStr for StorageClassAnalysisSchemaVersion {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type Suffix = String;

/// <p>A container of a key value name pair.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct Tag {
    /// <p>Name of the object key.</p>
    pub key: ObjectKey,
    /// <p>Value of the tag.</p>
    pub value: Value,
}

pub type TagCount = i32;

pub type TagSet = List<Tag>;

/// <p>Container for <code>TagSet</code> elements.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct Tagging {
    /// <p>A collection for a set of tags</p>
    pub tag_set: TagSet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaggingDirective {
    Copy,
    Replace,
}

impl TaggingDirective {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Copy => "COPY",
            Self::Replace => "REPLACE",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"COPY" => Some(Self::Copy),
            b"REPLACE" => Some(Self::Replace),
            _ => None,
        }
    }
}

impl FromStr for TaggingDirective {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type TaggingHeader = String;

pub type TargetBucket = String;

/// <p>Container for granting information.</p>
/// <p>Buckets that use the bucket owner enforced setting for Object
/// Ownership don't support target grants. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/enable-server-access-logging.html#grant-log-delivery-permissions-general">Permissions server access log delivery</a> in the
/// <i>Amazon S3 User Guide</i>.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct TargetGrant {
    /// <p>Container for the person being granted permissions.</p>
    pub grantee: Option<Grantee>,
    /// <p>Logging permissions assigned to the grantee for the bucket.</p>
    pub permission: Option<BucketLogsPermission>,
}

pub type TargetGrants = List<TargetGrant>;

pub type TargetPrefix = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Bulk,
    Expedited,
    Standard,
}

impl Tier {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Bulk => "Bulk",
            Self::Expedited => "Expedited",
            Self::Standard => "Standard",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"Bulk" => Some(Self::Bulk),
            b"Expedited" => Some(Self::Expedited),
            b"Standard" => Some(Self::Standard),
            _ => None,
        }
    }
}

impl FromStr for Tier {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

/// <p>The S3 Intelligent-Tiering storage class is designed to optimize storage costs by
/// automatically moving data to the most cost-effective storage access tier, without
/// additional operational overhead.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct Tiering {
    /// <p>S3 Intelligent-Tiering access tier. See <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for
    /// automatically optimizing frequently and infrequently accessed objects</a> for a list
    /// of access tiers in the S3 Intelligent-Tiering storage class.</p>
    pub access_tier: IntelligentTieringAccessTier,
    /// <p>The number of consecutive days of no access after which an object will be eligible to be
    /// transitioned to the corresponding tier. The minimum number of days specified for
    /// Archive Access tier must be at least 90 days and Deep Archive Access tier must be at least
    /// 180 days. The maximum can be up to 2 years (730 days).</p>
    pub days: IntelligentTieringDays,
}

pub type TieringList = List<Tiering>;

pub type Token = String;

pub type TopicArn = String;

/// <p>A container for specifying the configuration for publication of messages to an Amazon
/// Simple Notification Service (Amazon SNS) topic when Amazon S3 detects specified events.</p>
#[derive(Debug)]
#[non_exhaustive]
pub struct TopicConfiguration {
    /// <p>The Amazon S3 bucket event about which to send notifications. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Supported
    /// Event Types</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub events: EventList,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<NotificationId>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which Amazon S3 publishes a message
    /// when it detects events of the specified type.</p>
    pub topic_arn: TopicArn,
}

pub type TopicConfigurationList = List<TopicConfiguration>;

/// <p>Specifies when an object transitions to a specified storage class. For more information
/// about Amazon S3 lifecycle configuration rules, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/lifecycle-transition-general-considerations.html">Transitioning
/// Objects Using Amazon S3 Lifecycle</a> in the <i>Amazon S3 User Guide</i>.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Transition {
    /// <p>Indicates when objects are transitioned to the specified storage class. The date value
    /// must be in ISO 8601 format. The time is always midnight UTC.</p>
    pub date: Option<Date>,
    /// <p>Indicates the number of days after creation when objects are transitioned to the
    /// specified storage class. The value must be a positive integer.</p>
    pub days: Days,
    /// <p>The storage class to which you want the object to transition.</p>
    pub storage_class: Option<TransitionStorageClass>,
}

pub type TransitionList = List<Transition>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionStorageClass {
    DeepArchive,
    Glacier,
    GlacierIr,
    IntelligentTiering,
    OnezoneIa,
    StandardIa,
}

impl TransitionStorageClass {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DeepArchive => "DEEP_ARCHIVE",
            Self::Glacier => "GLACIER",
            Self::GlacierIr => "GLACIER_IR",
            Self::IntelligentTiering => "INTELLIGENT_TIERING",
            Self::OnezoneIa => "ONEZONE_IA",
            Self::StandardIa => "STANDARD_IA",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"DEEP_ARCHIVE" => Some(Self::DeepArchive),
            b"GLACIER" => Some(Self::Glacier),
            b"GLACIER_IR" => Some(Self::GlacierIr),
            b"INTELLIGENT_TIERING" => Some(Self::IntelligentTiering),
            b"ONEZONE_IA" => Some(Self::OnezoneIa),
            b"STANDARD_IA" => Some(Self::StandardIa),
            _ => None,
        }
    }
}

impl FromStr for TransitionStorageClass {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    AmazonCustomerByEmail,
    CanonicalUser,
    Group,
}

impl Type {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AmazonCustomerByEmail => "AmazonCustomerByEmail",
            Self::CanonicalUser => "CanonicalUser",
            Self::Group => "Group",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"AmazonCustomerByEmail" => Some(Self::AmazonCustomerByEmail),
            b"CanonicalUser" => Some(Self::CanonicalUser),
            b"Group" => Some(Self::Group),
            _ => None,
        }
    }
}

impl FromStr for Type {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))
    }
}

pub type URI = String;

pub type UploadIdMarker = String;

#[derive(Debug)]
#[non_exhaustive]
pub struct UploadPartCopyInput {
    /// <p>The bucket name.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Specifies the source object for the copy operation. You specify the value in one of two
    /// formats, depending on whether you want to access the source object through an <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points.html">access point</a>:</p>
    /// <ul>
    /// <li>
    /// <p>For objects not accessed through an access point, specify the name of the source bucket
    /// and key of the source object, separated by a slash (/). For example, to copy the
    /// object <code>reports/january.pdf</code> from the bucket
    /// <code>awsexamplebucket</code>, use <code>awsexamplebucket/reports/january.pdf</code>.
    /// The value must be URL-encoded.</p>
    /// </li>
    /// <li>
    /// <p>For objects accessed through access points, specify the Amazon Resource Name (ARN) of the object as accessed through the access point, in the format <code>arn:aws:s3:&lt;Region&gt;:&lt;account-id&gt;:accesspoint/&lt;access-point-name&gt;/object/&lt;key&gt;</code>. For example, to copy the object <code>reports/january.pdf</code> through access point <code>my-access-point</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3:us-west-2:123456789012:accesspoint/my-access-point/object/reports/january.pdf</code>. The value must be URL encoded.</p>
    /// <note>
    /// <p>Amazon S3 supports copy operations using access points only when the source and destination buckets are in the same Amazon Web Services Region.</p>
    /// </note>
    /// <p>Alternatively, for objects accessed through Amazon S3 on Outposts, specify the ARN of the object as accessed in the format <code>arn:aws:s3-outposts:&lt;Region&gt;:&lt;account-id&gt;:outpost/&lt;outpost-id&gt;/object/&lt;key&gt;</code>. For example, to copy the object <code>reports/january.pdf</code> through outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/object/reports/january.pdf</code>. The value must be URL-encoded.  </p>
    /// </li>
    /// </ul>
    /// <p>To copy a specific version of an object, append <code>?versionId=&lt;version-id&gt;</code>
    /// to the value (for example,
    /// <code>awsexamplebucket/reports/january.pdf?versionId=QUpfdndhfd8438MNFDN93jdnJFkdmqnh893</code>).
    /// If you don't specify a version ID, Amazon S3 copies the latest version of the source
    /// object.</p>
    pub copy_source: CopySource,
    /// <p>Copies the object if its entity tag (ETag) matches the specified tag.</p>
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    /// <p>Copies the object if it has been modified since the specified time.</p>
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    /// <p>Copies the object if its entity tag (ETag) is different than the specified ETag.</p>
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    /// <p>Copies the object if it hasn't been modified since the specified time.</p>
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    /// <p>The range of bytes to copy from the source object. The range value must use the form
    /// bytes=first-last, where the first and last are the zero-based byte offsets to copy. For
    /// example, bytes=0-9 indicates that you want to copy the first 10 bytes of the source. You
    /// can copy a range only if the source object is greater than 5 MB.</p>
    pub copy_source_range: Option<CopySourceRange>,
    /// <p>Specifies the algorithm to use when decrypting the source object (for example,
    /// AES256).</p>
    pub copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source
    /// object. The encryption key provided in this header must be one that was used when the
    /// source object was created.</p>
    pub copy_source_sse_customer_key: Option<CopySourceSSECustomerKey>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses
    /// this header for a message integrity check to ensure that the encryption key was transmitted
    /// without error.</p>
    pub copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5>,
    /// <p>The account ID of the expected destination bucket owner. If the destination bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The account ID of the expected source bucket owner. If the source bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_source_bucket_owner: Option<AccountId>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: ObjectKey,
    /// <p>Part number of part being copied. This is a positive integer between 1 and
    /// 10,000.</p>
    pub part_number: PartNumber,
    pub request_payer: Option<RequestPayer>,
    /// <p>Specifies the algorithm to use to when encrypting the object (for example,
    /// AES256).</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This
    /// value is used to store the object and then it is discarded; Amazon S3 does not store the
    /// encryption key. The key must be appropriate for use with the algorithm specified in the
    /// <code>x-amz-server-side-encryption-customer-algorithm</code> header. This must be the
    /// same encryption key specified in the initiate multipart upload request.</p>
    pub sse_customer_key: Option<SSECustomerKey>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses
    /// this header for a message integrity check to ensure that the encryption key was transmitted
    /// without error.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>Upload ID identifying the multipart upload whose part is being copied.</p>
    pub upload_id: MultipartUploadId,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct UploadPartCopyOutput {
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p>Container for all response elements.</p>
    pub copy_part_result: Option<CopyPartResult>,
    /// <p>The version of the source object that was copied, if you have enabled versioning on the
    /// source bucket.</p>
    pub copy_source_version_id: Option<CopySourceVersionId>,
    pub request_charged: Option<RequestCharged>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header to provide round-trip message integrity verification of
    /// the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric
    /// customer managed key that was used for the object.</p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example,
    /// AES256, aws:kms).</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct UploadPartInput {
    /// <p>Object data.</p>
    pub body: Option<StreamingBlob>,
    /// <p>The name of the bucket to which the multipart upload was initiated.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any
    /// additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or
    /// <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided
    /// <code>ChecksumAlgorithm</code> parameter.</p>
    /// <p>This checksum algorithm must be the same for all parts and it match the checksum
    /// value supplied in the <code>CreateMultipartUpload</code> request.</p>
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 32-bit CRC32C checksum of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 160-bit SHA-1 digest of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent.
    /// This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Size of the body in bytes. This parameter is useful when the size of the body cannot be
    /// determined automatically.</p>
    pub content_length: ContentLength,
    /// <p>The base64-encoded 128-bit MD5 digest of the part data. This parameter is auto-populated
    /// when using the command from the CLI. This parameter is required if object lock parameters
    /// are specified.</p>
    pub content_md5: Option<ContentMD5>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: ObjectKey,
    /// <p>Part number of part being uploaded. This is a positive integer between 1 and
    /// 10,000.</p>
    pub part_number: PartNumber,
    pub request_payer: Option<RequestPayer>,
    /// <p>Specifies the algorithm to use to when encrypting the object (for example,
    /// AES256).</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This
    /// value is used to store the object and then it is discarded; Amazon S3 does not store the
    /// encryption key. The key must be appropriate for use with the algorithm specified in the
    /// <code>x-amz-server-side-encryption-customer-algorithm header</code>. This must be the
    /// same encryption key specified in the initiate multipart upload request.</p>
    pub sse_customer_key: Option<SSECustomerKey>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses
    /// this header for a message integrity check to ensure that the encryption key was transmitted
    /// without error.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>Upload ID identifying the multipart upload whose part is being uploaded.</p>
    pub upload_id: MultipartUploadId,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct UploadPartOutput {
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded
    /// with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated
    /// with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums">
    /// Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Entity tag for the uploaded object.</p>
    pub e_tag: Option<ETag>,
    pub request_charged: Option<RequestCharged>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the
    /// response will include this header to provide round-trip message integrity verification of
    /// the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric
    /// customer managed key was used for the object.</p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example,
    /// AES256, aws:kms).</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
}

pub type UserMetadata = List<MetadataEntry>;

pub type Value = String;

pub type VersionCount = i32;

pub type VersionIdMarker = String;

/// <p>Describes the versioning state of an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTVersioningStatus.html">PUT
/// Bucket versioning</a> in the <i>Amazon S3 API Reference</i>.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct VersioningConfiguration {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This
    /// element is only returned if the bucket has been configured with MFA delete. If the bucket
    /// has never been so configured, this element is not returned.</p>
    pub mfa_delete: Option<MFADelete>,
    /// <p>The versioning state of the bucket.</p>
    pub status: Option<BucketVersioningStatus>,
}

/// <p>Specifies website configuration parameters for an Amazon S3 bucket.</p>
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct WebsiteConfiguration {
    /// <p>The name of the error document for the website.</p>
    pub error_document: Option<ErrorDocument>,
    /// <p>The name of the index document for the website.</p>
    pub index_document: Option<IndexDocument>,
    /// <p>The redirect behavior for every request to this bucket's website endpoint.</p>
    /// <important>
    /// <p>If you specify this property, you can't specify any other property.</p>
    /// </important>
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    /// <p>Rules that define when a redirect is applied and the redirect behavior.</p>
    pub routing_rules: Option<RoutingRules>,
}

pub type WebsiteRedirectLocation = String;

#[derive(Debug)]
#[non_exhaustive]
pub struct WriteGetObjectResponseInput {
    /// <p>Indicates that a range of bytes was specified.</p>
    pub accept_ranges: Option<AcceptRanges>,
    /// <p>The object data.</p>
    pub body: Option<StreamingBlob>,
    /// <p> Indicates whether the object stored in Amazon S3 uses an S3 bucket key for server-side
    /// encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub bucket_key_enabled: BucketKeyEnabled,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<CacheControl>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the
    /// same data that was originally sent. This specifies the base64-encoded, 32-bit CRC32 checksum
    /// of the object returned by the Object Lambda function. This may not match the checksum for the
    /// object stored in Amazon S3. Amazon S3 will perform validation of the checksum values only when the original
    /// <code>GetObject</code> request required checksum validation. For more information about checksums, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking
    /// object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>Only one checksum header can be specified at a time. If you supply multiple
    /// checksum headers, this request will fail.</p>
    /// <p></p>
    pub checksum_crc32: Option<ChecksumCRC32>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the
    /// same data that was originally sent. This specifies the base64-encoded, 32-bit CRC32C checksum
    /// of the object returned by the Object Lambda function. This may not match the checksum for the
    /// object stored in Amazon S3. Amazon S3 will perform validation of the checksum values only when the original
    /// <code>GetObject</code> request required checksum validation. For more information about checksums, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking
    /// object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>Only one checksum header can be specified at a time. If you supply multiple
    /// checksum headers, this request will fail.</p>
    pub checksum_crc32c: Option<ChecksumCRC32C>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the
    /// same data that was originally sent. This specifies the base64-encoded, 160-bit SHA-1 digest
    /// of the object returned by the Object Lambda function. This may not match the checksum for the
    /// object stored in Amazon S3. Amazon S3 will perform validation of the checksum values only when the original
    /// <code>GetObject</code> request required checksum validation. For more information about checksums, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking
    /// object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>Only one checksum header can be specified at a time. If you supply multiple
    /// checksum headers, this request will fail.</p>
    pub checksum_sha1: Option<ChecksumSHA1>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the
    /// same data that was originally sent. This specifies the base64-encoded, 256-bit SHA-256 digest
    /// of the object returned by the Object Lambda function. This may not match the checksum for the
    /// object stored in Amazon S3. Amazon S3 will perform validation of the checksum values only when the original
    /// <code>GetObject</code> request required checksum validation. For more information about checksums, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking
    /// object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>Only one checksum header can be specified at a time. If you supply multiple
    /// checksum headers, this request will fail.</p>
    pub checksum_sha256: Option<ChecksumSHA256>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<ContentDisposition>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding
    /// mechanisms must be applied to obtain the media-type referenced by the Content-Type header
    /// field.</p>
    pub content_encoding: Option<ContentEncoding>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<ContentLanguage>,
    /// <p>The size of the content body in bytes.</p>
    pub content_length: ContentLength,
    /// <p>The portion of the object returned in the response.</p>
    pub content_range: Option<ContentRange>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<ContentType>,
    /// <p>Specifies whether an object stored in Amazon S3 is (<code>true</code>) or is not
    /// (<code>false</code>) a delete marker. </p>
    pub delete_marker: DeleteMarker,
    /// <p>An opaque identifier assigned by a web server to a specific version of a resource found
    /// at a URL. </p>
    pub e_tag: Option<ETag>,
    /// <p>A string that uniquely identifies an error condition. Returned in the &lt;Code&gt; tag
    /// of the error XML response for a corresponding <code>GetObject</code> call. Cannot be used
    /// with a successful <code>StatusCode</code> header or when the transformed object is provided
    /// in the body. All error codes from S3 are sentence-cased. The regular expression (regex)
    /// value is <code>"^[A-Z][a-zA-Z]+$"</code>.</p>
    pub error_code: Option<ErrorCode>,
    /// <p>Contains a generic description of the error condition. Returned in the &lt;Message&gt;
    /// tag of the error XML response for a corresponding <code>GetObject</code> call. Cannot be
    /// used with a successful <code>StatusCode</code> header or when the transformed object is
    /// provided in body.</p>
    pub error_message: Option<ErrorMessage>,
    /// <p>If the object expiration is configured (see PUT Bucket lifecycle), the response
    /// includes this header. It includes the <code>expiry-date</code> and <code>rule-id</code>
    /// key-value pairs that provide the object expiration information. The value of the
    /// <code>rule-id</code> is URL-encoded. </p>
    pub expiration: Option<Expiration>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<Expires>,
    /// <p>The date and time that the object was last modified.</p>
    pub last_modified: Option<LastModified>,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<Metadata>,
    /// <p>Set to the number of metadata entries not returned in <code>x-amz-meta</code> headers.
    /// This can happen if you create metadata using an API like SOAP that supports more flexible
    /// metadata than the REST API. For example, using SOAP, you can create metadata whose values
    /// are not legal HTTP headers.</p>
    pub missing_meta: MissingMeta,
    /// <p>Indicates whether an object stored in Amazon S3 has an active legal hold.</p>
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    /// <p>Indicates whether an object stored in Amazon S3 has Object Lock enabled. For more
    /// information about S3 Object Lock, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lock.html">Object Lock</a>.</p>
    pub object_lock_mode: Option<ObjectLockMode>,
    /// <p>The date and time when Object Lock is configured to expire.</p>
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    /// <p>The count of parts this object has.</p>
    pub parts_count: PartsCount,
    /// <p>Indicates if request involves bucket that is either a source or destination in a Replication rule. For more
    /// information about S3 Replication, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/replication.html">Replication</a>.</p>
    pub replication_status: Option<ReplicationStatus>,
    pub request_charged: Option<RequestCharged>,
    /// <p>Route prefix to the HTTP URL generated.</p>
    pub request_route: RequestRoute,
    /// <p>A single use encrypted token that maps <code>WriteGetObjectResponse</code> to the end
    /// user <code>GetObject</code> request.</p>
    pub request_token: RequestToken,
    /// <p>Provides information about object restoration operation and expiration time of the
    /// restored object copy.</p>
    pub restore: Option<Restore>,
    /// <p>Encryption algorithm used if server-side encryption with a customer-provided encryption key was specified for object stored in Amazon S3.</p>
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// <p> 128-bit MD5 digest of customer-provided encryption key used in Amazon S3 to encrypt data
    /// stored in S3. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/ServerSideEncryptionCustomerKeys.html">Protecting data
    /// using server-side encryption with customer-provided encryption keys
    /// (SSE-C)</a>.</p>
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// <p> If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for stored in Amazon S3 object. </p>
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// <p> The server-side encryption algorithm used when storing requested object in Amazon S3 (for example, AES256, aws:kms).</p>
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// <p>The integer status code for an HTTP response of a corresponding <code>GetObject</code>
    /// request.</p>
    /// <p class="title">
    /// <b>Status Codes</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>200 - OK</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>206 - Partial Content</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>304 - Not Modified</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>400 - Bad Request</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>401 - Unauthorized</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>403 - Forbidden</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>404 - Not Found</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>405 - Method Not Allowed</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>409 - Conflict</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>411 - Length Required</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>412 - Precondition Failed</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>416 - Range Not Satisfiable</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>500 - Internal Server Error</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>503 - Service Unavailable</code>
    /// </p>
    /// </li>
    /// </ul>
    pub status_code: GetObjectResponseStatusCode,
    /// <p>Provides storage class information of the object. Amazon S3 returns this header for all
    /// objects except for S3 Standard storage class objects.</p>
    ///
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html">Storage
    /// Classes</a>.</p>
    pub storage_class: Option<StorageClass>,
    /// <p>The number of tags, if any, on the object.</p>
    pub tag_count: TagCount,
    /// <p>An ID used to reference a specific version of the object.</p>
    pub version_id: Option<ObjectVersionId>,
}

pub type Years = i32;
