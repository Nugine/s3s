//! Auto-generated definitions
#![allow(clippy::empty_structs_with_brackets)]

use super::*;

use std::borrow::Cow;
use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

pub type AbortDate = Timestamp;

/// <p>Specifies the days since the initiation of an incomplete multipart upload that Amazon S3 will
/// wait before permanently removing all parts of the upload. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config">
/// Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Policy</a> in the
/// <i>Amazon S3 User Guide</i>.</p>
#[derive(Default)]
pub struct AbortIncompleteMultipartUpload {
    /// <p>Specifies the number of days after which Amazon S3 aborts an incomplete multipart
    /// upload.</p>
    pub days_after_initiation: DaysAfterInitiation,
}

impl fmt::Debug for AbortIncompleteMultipartUpload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("AbortIncompleteMultipartUpload");
        d.field("days_after_initiation", &self.days_after_initiation);
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for AbortMultipartUploadInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("AbortMultipartUploadInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        d.field("upload_id", &self.upload_id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct AbortMultipartUploadOutput {
    pub request_charged: Option<RequestCharged>,
}

impl fmt::Debug for AbortMultipartUploadOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("AbortMultipartUploadOutput");
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type AbortRuleId = String;

/// <p>Configures the transfer acceleration state for an Amazon S3 bucket. For more information, see
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/transfer-acceleration.html">Amazon S3
/// Transfer Acceleration</a> in the <i>Amazon S3 User Guide</i>.</p>
#[derive(Default)]
pub struct AccelerateConfiguration {
    /// <p>Specifies the transfer acceleration status of the bucket.</p>
    pub status: Option<BucketAccelerateStatus>,
}

impl fmt::Debug for AccelerateConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("AccelerateConfiguration");
        if let Some(ref val) = self.status {
            d.field("status", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type AcceptRanges = String;

/// <p>Contains the elements that set the ACL permissions for an object per grantee.</p>
#[derive(Default)]
pub struct AccessControlPolicy {
    /// <p>A list of grants.</p>
    pub grants: Option<Grants>,
    /// <p>Container for the bucket owner's display name and ID.</p>
    pub owner: Option<Owner>,
}

impl fmt::Debug for AccessControlPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("AccessControlPolicy");
        if let Some(ref val) = self.grants {
            d.field("grants", val);
        }
        if let Some(ref val) = self.owner {
            d.field("owner", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>A container for information about access control for replicas.</p>
pub struct AccessControlTranslation {
    /// <p>Specifies the replica ownership. For default and valid values, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTreplication.html">PUT bucket
    /// replication</a> in the <i>Amazon S3 API Reference</i>.</p>
    pub owner: OwnerOverride,
}

impl fmt::Debug for AccessControlTranslation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("AccessControlTranslation");
        d.field("owner", &self.owner);
        d.finish_non_exhaustive()
    }
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
#[derive(Default)]
pub struct AnalyticsAndOperator {
    /// <p>The prefix to use when evaluating an AND predicate: The prefix that an object must have
    /// to be included in the metrics results.</p>
    pub prefix: Option<Prefix>,
    /// <p>The list of tags to use when evaluating an AND predicate.</p>
    pub tags: Option<TagSet>,
}

impl fmt::Debug for AnalyticsAndOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("AnalyticsAndOperator");
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.tags {
            d.field("tags", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies the configuration and any analyses for the analytics filter of an Amazon S3 bucket.</p>
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

impl fmt::Debug for AnalyticsConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("AnalyticsConfiguration");
        if let Some(ref val) = self.filter {
            d.field("filter", val);
        }
        d.field("id", &self.id);
        d.field("storage_class_analysis", &self.storage_class_analysis);
        d.finish_non_exhaustive()
    }
}

pub type AnalyticsConfigurationList = List<AnalyticsConfiguration>;

/// <p>Where to publish the analytics results.</p>
pub struct AnalyticsExportDestination {
    /// <p>A destination signifying output to an S3 bucket.</p>
    pub s3_bucket_destination: AnalyticsS3BucketDestination,
}

impl fmt::Debug for AnalyticsExportDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("AnalyticsExportDestination");
        d.field("s3_bucket_destination", &self.s3_bucket_destination);
        d.finish_non_exhaustive()
    }
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

impl fmt::Debug for AnalyticsS3BucketDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("AnalyticsS3BucketDestination");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.bucket_account_id {
            d.field("bucket_account_id", val);
        }
        d.field("format", &self.format);
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyticsS3ExportFileFormat(Cow<'static, str>);

impl AnalyticsS3ExportFileFormat {
    pub const CSV: &str = "CSV";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for AnalyticsS3ExportFileFormat {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<AnalyticsS3ExportFileFormat> for Cow<'static, str> {
    fn from(s: AnalyticsS3ExportFileFormat) -> Self {
        s.0
    }
}

impl FromStr for AnalyticsS3ExportFileFormat {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveStatus(Cow<'static, str>);

impl ArchiveStatus {
    pub const ARCHIVE_ACCESS: &str = "ARCHIVE_ACCESS";

    pub const DEEP_ARCHIVE_ACCESS: &str = "DEEP_ARCHIVE_ACCESS";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ArchiveStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ArchiveStatus> for Cow<'static, str> {
    fn from(s: ArchiveStatus) -> Self {
        s.0
    }
}

impl FromStr for ArchiveStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p> In terms of implementation, a Bucket is a resource. An Amazon S3 bucket name is globally
/// unique, and the namespace is shared by all Amazon Web Services accounts. </p>
#[derive(Default)]
pub struct Bucket {
    /// <p>Date the bucket was created. This date can change when making changes to your bucket, such as editing its bucket policy.</p>
    pub creation_date: Option<CreationDate>,
    /// <p>The name of the bucket.</p>
    pub name: Option<BucketName>,
}

impl fmt::Debug for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Bucket");
        if let Some(ref val) = self.creation_date {
            d.field("creation_date", val);
        }
        if let Some(ref val) = self.name {
            d.field("name", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BucketAccelerateStatus(Cow<'static, str>);

impl BucketAccelerateStatus {
    pub const ENABLED: &str = "Enabled";

    pub const SUSPENDED: &str = "Suspended";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for BucketAccelerateStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<BucketAccelerateStatus> for Cow<'static, str> {
    fn from(s: BucketAccelerateStatus) -> Self {
        s.0
    }
}

impl FromStr for BucketAccelerateStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>The requested bucket name is not available. The bucket namespace is shared by all users
/// of the system. Select a different name and try again.</p>
#[derive(Default)]
pub struct BucketAlreadyExists {}

impl fmt::Debug for BucketAlreadyExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("BucketAlreadyExists");
        d.finish_non_exhaustive()
    }
}

/// <p>The bucket you tried to create already exists, and you own it. Amazon S3 returns this error
/// in all Amazon Web Services Regions except in the North Virginia Region. For legacy compatibility, if you
/// re-create an existing bucket that you already own in the North Virginia Region, Amazon S3
/// returns 200 OK and resets the bucket access control lists (ACLs).</p>
#[derive(Default)]
pub struct BucketAlreadyOwnedByYou {}

impl fmt::Debug for BucketAlreadyOwnedByYou {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("BucketAlreadyOwnedByYou");
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BucketCannedACL(Cow<'static, str>);

impl BucketCannedACL {
    pub const AUTHENTICATED_READ: &str = "authenticated-read";

    pub const PRIVATE: &str = "private";

    pub const PUBLIC_READ: &str = "public-read";

    pub const PUBLIC_READ_WRITE: &str = "public-read-write";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for BucketCannedACL {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<BucketCannedACL> for Cow<'static, str> {
    fn from(s: BucketCannedACL) -> Self {
        s.0
    }
}

impl FromStr for BucketCannedACL {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type BucketKeyEnabled = bool;

/// <p>Specifies the lifecycle configuration for objects in an Amazon S3 bucket. For more
/// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html">Object Lifecycle Management</a>
/// in the <i>Amazon S3 User Guide</i>.</p>
pub struct BucketLifecycleConfiguration {
    /// <p>A lifecycle rule for individual objects in an Amazon S3 bucket.</p>
    pub rules: LifecycleRules,
}

impl fmt::Debug for BucketLifecycleConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("BucketLifecycleConfiguration");
        d.field("rules", &self.rules);
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BucketLocationConstraint(Cow<'static, str>);

impl BucketLocationConstraint {
    pub const EU: &str = "EU";

    pub const AF_SOUTH_1: &str = "af-south-1";

    pub const AP_EAST_1: &str = "ap-east-1";

    pub const AP_NORTHEAST_1: &str = "ap-northeast-1";

    pub const AP_NORTHEAST_2: &str = "ap-northeast-2";

    pub const AP_NORTHEAST_3: &str = "ap-northeast-3";

    pub const AP_SOUTH_1: &str = "ap-south-1";

    pub const AP_SOUTHEAST_1: &str = "ap-southeast-1";

    pub const AP_SOUTHEAST_2: &str = "ap-southeast-2";

    pub const AP_SOUTHEAST_3: &str = "ap-southeast-3";

    pub const CA_CENTRAL_1: &str = "ca-central-1";

    pub const CN_NORTH_1: &str = "cn-north-1";

    pub const CN_NORTHWEST_1: &str = "cn-northwest-1";

    pub const EU_CENTRAL_1: &str = "eu-central-1";

    pub const EU_NORTH_1: &str = "eu-north-1";

    pub const EU_SOUTH_1: &str = "eu-south-1";

    pub const EU_WEST_1: &str = "eu-west-1";

    pub const EU_WEST_2: &str = "eu-west-2";

    pub const EU_WEST_3: &str = "eu-west-3";

    pub const ME_SOUTH_1: &str = "me-south-1";

    pub const SA_EAST_1: &str = "sa-east-1";

    pub const US_EAST_2: &str = "us-east-2";

    pub const US_GOV_EAST_1: &str = "us-gov-east-1";

    pub const US_GOV_WEST_1: &str = "us-gov-west-1";

    pub const US_WEST_1: &str = "us-west-1";

    pub const US_WEST_2: &str = "us-west-2";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for BucketLocationConstraint {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<BucketLocationConstraint> for Cow<'static, str> {
    fn from(s: BucketLocationConstraint) -> Self {
        s.0
    }
}

impl FromStr for BucketLocationConstraint {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>Container for logging status information.</p>
#[derive(Default)]
pub struct BucketLoggingStatus {
    pub logging_enabled: Option<LoggingEnabled>,
}

impl fmt::Debug for BucketLoggingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("BucketLoggingStatus");
        if let Some(ref val) = self.logging_enabled {
            d.field("logging_enabled", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BucketLogsPermission(Cow<'static, str>);

impl BucketLogsPermission {
    pub const FULL_CONTROL: &str = "FULL_CONTROL";

    pub const READ: &str = "READ";

    pub const WRITE: &str = "WRITE";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for BucketLogsPermission {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<BucketLogsPermission> for Cow<'static, str> {
    fn from(s: BucketLogsPermission) -> Self {
        s.0
    }
}

impl FromStr for BucketLogsPermission {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type BucketName = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BucketVersioningStatus(Cow<'static, str>);

impl BucketVersioningStatus {
    pub const ENABLED: &str = "Enabled";

    pub const SUSPENDED: &str = "Suspended";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for BucketVersioningStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<BucketVersioningStatus> for Cow<'static, str> {
    fn from(s: BucketVersioningStatus) -> Self {
        s.0
    }
}

impl FromStr for BucketVersioningStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
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
pub struct CORSConfiguration {
    /// <p>A set of origins and methods (cross-origin access that you want to allow). You can add
    /// up to 100 rules to the configuration.</p>
    pub cors_rules: CORSRules,
}

impl fmt::Debug for CORSConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CORSConfiguration");
        d.field("cors_rules", &self.cors_rules);
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies a cross-origin access rule for an Amazon S3 bucket.</p>
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

impl fmt::Debug for CORSRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CORSRule");
        if let Some(ref val) = self.allowed_headers {
            d.field("allowed_headers", val);
        }
        d.field("allowed_methods", &self.allowed_methods);
        d.field("allowed_origins", &self.allowed_origins);
        if let Some(ref val) = self.expose_headers {
            d.field("expose_headers", val);
        }
        if let Some(ref val) = self.id {
            d.field("id", val);
        }
        d.field("max_age_seconds", &self.max_age_seconds);
        d.finish_non_exhaustive()
    }
}

pub type CORSRules = List<CORSRule>;

/// <p>Describes how an uncompressed comma-separated values (CSV)-formatted input object is
/// formatted.</p>
#[derive(Default)]
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

impl fmt::Debug for CSVInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CSVInput");
        d.field("allow_quoted_record_delimiter", &self.allow_quoted_record_delimiter);
        if let Some(ref val) = self.comments {
            d.field("comments", val);
        }
        if let Some(ref val) = self.field_delimiter {
            d.field("field_delimiter", val);
        }
        if let Some(ref val) = self.file_header_info {
            d.field("file_header_info", val);
        }
        if let Some(ref val) = self.quote_character {
            d.field("quote_character", val);
        }
        if let Some(ref val) = self.quote_escape_character {
            d.field("quote_escape_character", val);
        }
        if let Some(ref val) = self.record_delimiter {
            d.field("record_delimiter", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Describes how uncompressed comma-separated values (CSV)-formatted results are
/// formatted.</p>
#[derive(Default)]
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

impl fmt::Debug for CSVOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CSVOutput");
        if let Some(ref val) = self.field_delimiter {
            d.field("field_delimiter", val);
        }
        if let Some(ref val) = self.quote_character {
            d.field("quote_character", val);
        }
        if let Some(ref val) = self.quote_escape_character {
            d.field("quote_escape_character", val);
        }
        if let Some(ref val) = self.quote_fields {
            d.field("quote_fields", val);
        }
        if let Some(ref val) = self.record_delimiter {
            d.field("record_delimiter", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type CacheControl = String;

/// <p>Contains all the possible checksum or digest values for an object.</p>
#[derive(Default)]
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

impl fmt::Debug for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Checksum");
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumAlgorithm(Cow<'static, str>);

impl ChecksumAlgorithm {
    pub const CRC32: &str = "CRC32";

    pub const CRC32C: &str = "CRC32C";

    pub const SHA1: &str = "SHA1";

    pub const SHA256: &str = "SHA256";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ChecksumAlgorithm {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ChecksumAlgorithm> for Cow<'static, str> {
    fn from(s: ChecksumAlgorithm) -> Self {
        s.0
    }
}

impl FromStr for ChecksumAlgorithm {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type ChecksumAlgorithmList = List<ChecksumAlgorithm>;

pub type ChecksumCRC32 = String;

pub type ChecksumCRC32C = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumMode(Cow<'static, str>);

impl ChecksumMode {
    pub const ENABLED: &str = "ENABLED";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ChecksumMode {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ChecksumMode> for Cow<'static, str> {
    fn from(s: ChecksumMode) -> Self {
        s.0
    }
}

impl FromStr for ChecksumMode {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
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
#[derive(Default)]
pub struct CommonPrefix {
    /// <p>Container for the specified common prefix.</p>
    pub prefix: Option<Prefix>,
}

impl fmt::Debug for CommonPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CommonPrefix");
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type CommonPrefixList = List<CommonPrefix>;

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

impl fmt::Debug for CompleteMultipartUploadInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CompleteMultipartUploadInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.multipart_upload {
            d.field("multipart_upload", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key {
            d.field("sse_customer_key", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        d.field("upload_id", &self.upload_id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for CompleteMultipartUploadOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CompleteMultipartUploadOutput");
        if let Some(ref val) = self.bucket {
            d.field("bucket", val);
        }
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        if let Some(ref val) = self.expiration {
            d.field("expiration", val);
        }
        if let Some(ref val) = self.key {
            d.field("key", val);
        }
        if let Some(ref val) = self.location {
            d.field("location", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>The container for the completed multipart upload details.</p>
#[derive(Default)]
pub struct CompletedMultipartUpload {
    /// <p>Array of CompletedPart data types.</p>
    /// <p>If you do not supply a valid <code>Part</code> with your request, the service sends back an HTTP
    /// 400 response.</p>
    pub parts: Option<CompletedPartList>,
}

impl fmt::Debug for CompletedMultipartUpload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CompletedMultipartUpload");
        if let Some(ref val) = self.parts {
            d.field("parts", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Details of the parts that were uploaded.</p>
#[derive(Default)]
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

impl fmt::Debug for CompletedPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CompletedPart");
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        d.field("part_number", &self.part_number);
        d.finish_non_exhaustive()
    }
}

pub type CompletedPartList = List<CompletedPart>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompressionType(Cow<'static, str>);

impl CompressionType {
    pub const BZIP2: &str = "BZIP2";

    pub const GZIP: &str = "GZIP";

    pub const NONE: &str = "NONE";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for CompressionType {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<CompressionType> for Cow<'static, str> {
    fn from(s: CompressionType) -> Self {
        s.0
    }
}

impl FromStr for CompressionType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>A container for describing a condition that must be met for the specified redirect to
/// apply. For example, 1. If request is for pages in the <code>/docs</code> folder, redirect
/// to the <code>/documents</code> folder. 2. If request results in HTTP error 4xx, redirect
/// request to another host where you might process the error.</p>
#[derive(Default)]
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

impl fmt::Debug for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Condition");
        if let Some(ref val) = self.http_error_code_returned_equals {
            d.field("http_error_code_returned_equals", val);
        }
        if let Some(ref val) = self.key_prefix_equals {
            d.field("key_prefix_equals", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type ConfirmRemoveSelfBucketAccess = bool;

pub type ContentDisposition = String;

pub type ContentEncoding = String;

pub type ContentLanguage = String;

pub type ContentLength = i64;

pub type ContentMD5 = String;

pub type ContentRange = String;

/// <p></p>
#[derive(Default)]
pub struct ContinuationEvent {}

impl fmt::Debug for ContinuationEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ContinuationEvent");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for CopyObjectInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CopyObjectInput");
        if let Some(ref val) = self.acl {
            d.field("acl", val);
        }
        d.field("bucket", &self.bucket);
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.cache_control {
            d.field("cache_control", val);
        }
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_disposition {
            d.field("content_disposition", val);
        }
        if let Some(ref val) = self.content_encoding {
            d.field("content_encoding", val);
        }
        if let Some(ref val) = self.content_language {
            d.field("content_language", val);
        }
        if let Some(ref val) = self.content_type {
            d.field("content_type", val);
        }
        d.field("copy_source", &self.copy_source);
        if let Some(ref val) = self.copy_source_if_match {
            d.field("copy_source_if_match", val);
        }
        if let Some(ref val) = self.copy_source_if_modified_since {
            d.field("copy_source_if_modified_since", val);
        }
        if let Some(ref val) = self.copy_source_if_none_match {
            d.field("copy_source_if_none_match", val);
        }
        if let Some(ref val) = self.copy_source_if_unmodified_since {
            d.field("copy_source_if_unmodified_since", val);
        }
        if let Some(ref val) = self.copy_source_sse_customer_algorithm {
            d.field("copy_source_sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.copy_source_sse_customer_key {
            d.field("copy_source_sse_customer_key", val);
        }
        if let Some(ref val) = self.copy_source_sse_customer_key_md5 {
            d.field("copy_source_sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.expected_source_bucket_owner {
            d.field("expected_source_bucket_owner", val);
        }
        if let Some(ref val) = self.expires {
            d.field("expires", val);
        }
        if let Some(ref val) = self.grant_full_control {
            d.field("grant_full_control", val);
        }
        if let Some(ref val) = self.grant_read {
            d.field("grant_read", val);
        }
        if let Some(ref val) = self.grant_read_acp {
            d.field("grant_read_acp", val);
        }
        if let Some(ref val) = self.grant_write_acp {
            d.field("grant_write_acp", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.metadata {
            d.field("metadata", val);
        }
        if let Some(ref val) = self.metadata_directive {
            d.field("metadata_directive", val);
        }
        if let Some(ref val) = self.object_lock_legal_hold_status {
            d.field("object_lock_legal_hold_status", val);
        }
        if let Some(ref val) = self.object_lock_mode {
            d.field("object_lock_mode", val);
        }
        if let Some(ref val) = self.object_lock_retain_until_date {
            d.field("object_lock_retain_until_date", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key {
            d.field("sse_customer_key", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.ssekms_encryption_context {
            d.field("ssekms_encryption_context", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        if let Some(ref val) = self.tagging {
            d.field("tagging", val);
        }
        if let Some(ref val) = self.tagging_directive {
            d.field("tagging_directive", val);
        }
        if let Some(ref val) = self.website_redirect_location {
            d.field("website_redirect_location", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for CopyObjectOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CopyObjectOutput");
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.copy_object_result {
            d.field("copy_object_result", val);
        }
        if let Some(ref val) = self.copy_source_version_id {
            d.field("copy_source_version_id", val);
        }
        if let Some(ref val) = self.expiration {
            d.field("expiration", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.ssekms_encryption_context {
            d.field("ssekms_encryption_context", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Container for all response elements.</p>
#[derive(Default)]
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

impl fmt::Debug for CopyObjectResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CopyObjectResult");
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        if let Some(ref val) = self.last_modified {
            d.field("last_modified", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Container for all response elements.</p>
#[derive(Default)]
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

impl fmt::Debug for CopyPartResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CopyPartResult");
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        if let Some(ref val) = self.last_modified {
            d.field("last_modified", val);
        }
        d.finish_non_exhaustive()
    }
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
#[derive(Default)]
pub struct CreateBucketConfiguration {
    /// <p>Specifies the Region where the bucket will be created. If you don't specify a Region,
    /// the bucket is created in the US East (N. Virginia) Region (us-east-1).</p>
    pub location_constraint: Option<BucketLocationConstraint>,
}

impl fmt::Debug for CreateBucketConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CreateBucketConfiguration");
        if let Some(ref val) = self.location_constraint {
            d.field("location_constraint", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for CreateBucketInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CreateBucketInput");
        if let Some(ref val) = self.acl {
            d.field("acl", val);
        }
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.create_bucket_configuration {
            d.field("create_bucket_configuration", val);
        }
        if let Some(ref val) = self.grant_full_control {
            d.field("grant_full_control", val);
        }
        if let Some(ref val) = self.grant_read {
            d.field("grant_read", val);
        }
        if let Some(ref val) = self.grant_read_acp {
            d.field("grant_read_acp", val);
        }
        if let Some(ref val) = self.grant_write {
            d.field("grant_write", val);
        }
        if let Some(ref val) = self.grant_write_acp {
            d.field("grant_write_acp", val);
        }
        d.field("object_lock_enabled_for_bucket", &self.object_lock_enabled_for_bucket);
        if let Some(ref val) = self.object_ownership {
            d.field("object_ownership", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct CreateBucketOutput {
    /// <p>A forward slash followed by the name of the bucket.</p>
    pub location: Option<Location>,
}

impl fmt::Debug for CreateBucketOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CreateBucketOutput");
        if let Some(ref val) = self.location {
            d.field("location", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for CreateMultipartUploadInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CreateMultipartUploadInput");
        if let Some(ref val) = self.acl {
            d.field("acl", val);
        }
        d.field("bucket", &self.bucket);
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.cache_control {
            d.field("cache_control", val);
        }
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_disposition {
            d.field("content_disposition", val);
        }
        if let Some(ref val) = self.content_encoding {
            d.field("content_encoding", val);
        }
        if let Some(ref val) = self.content_language {
            d.field("content_language", val);
        }
        if let Some(ref val) = self.content_type {
            d.field("content_type", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.expires {
            d.field("expires", val);
        }
        if let Some(ref val) = self.grant_full_control {
            d.field("grant_full_control", val);
        }
        if let Some(ref val) = self.grant_read {
            d.field("grant_read", val);
        }
        if let Some(ref val) = self.grant_read_acp {
            d.field("grant_read_acp", val);
        }
        if let Some(ref val) = self.grant_write_acp {
            d.field("grant_write_acp", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.metadata {
            d.field("metadata", val);
        }
        if let Some(ref val) = self.object_lock_legal_hold_status {
            d.field("object_lock_legal_hold_status", val);
        }
        if let Some(ref val) = self.object_lock_mode {
            d.field("object_lock_mode", val);
        }
        if let Some(ref val) = self.object_lock_retain_until_date {
            d.field("object_lock_retain_until_date", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key {
            d.field("sse_customer_key", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.ssekms_encryption_context {
            d.field("ssekms_encryption_context", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        if let Some(ref val) = self.tagging {
            d.field("tagging", val);
        }
        if let Some(ref val) = self.website_redirect_location {
            d.field("website_redirect_location", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for CreateMultipartUploadOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("CreateMultipartUploadOutput");
        if let Some(ref val) = self.abort_date {
            d.field("abort_date", val);
        }
        if let Some(ref val) = self.abort_rule_id {
            d.field("abort_rule_id", val);
        }
        if let Some(ref val) = self.bucket {
            d.field("bucket", val);
        }
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.key {
            d.field("key", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.ssekms_encryption_context {
            d.field("ssekms_encryption_context", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        if let Some(ref val) = self.upload_id {
            d.field("upload_id", val);
        }
        d.finish_non_exhaustive()
    }
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
#[derive(Default)]
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

impl fmt::Debug for DefaultRetention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DefaultRetention");
        d.field("days", &self.days);
        if let Some(ref val) = self.mode {
            d.field("mode", val);
        }
        d.field("years", &self.years);
        d.finish_non_exhaustive()
    }
}

/// <p>Container for the objects to delete.</p>
pub struct Delete {
    /// <p>The objects to delete.</p>
    pub objects: ObjectIdentifierList,
    /// <p>Element to enable quiet mode for the request. When you add this element, you must set
    /// its value to true.</p>
    pub quiet: Quiet,
}

impl fmt::Debug for Delete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Delete");
        d.field("objects", &self.objects);
        d.field("quiet", &self.quiet);
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketAnalyticsConfigurationInput {
    /// <p>The name of the bucket from which an analytics configuration is deleted.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID that identifies the analytics configuration.</p>
    pub id: AnalyticsId,
}

impl fmt::Debug for DeleteBucketAnalyticsConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketAnalyticsConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("id", &self.id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketAnalyticsConfigurationOutput {}

impl fmt::Debug for DeleteBucketAnalyticsConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketAnalyticsConfigurationOutput");
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketCorsInput {
    /// <p>Specifies the bucket whose <code>cors</code> configuration is being deleted.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for DeleteBucketCorsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketCorsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketCorsOutput {}

impl fmt::Debug for DeleteBucketCorsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketCorsOutput");
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketEncryptionInput {
    /// <p>The name of the bucket containing the server-side encryption configuration to
    /// delete.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for DeleteBucketEncryptionInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketEncryptionInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketEncryptionOutput {}

impl fmt::Debug for DeleteBucketEncryptionOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketEncryptionOutput");
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketInput {
    /// <p>Specifies the bucket being deleted.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for DeleteBucketInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketIntelligentTieringConfigurationInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub bucket: BucketName,
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub id: IntelligentTieringId,
}

impl fmt::Debug for DeleteBucketIntelligentTieringConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketIntelligentTieringConfigurationInput");
        d.field("bucket", &self.bucket);
        d.field("id", &self.id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketIntelligentTieringConfigurationOutput {}

impl fmt::Debug for DeleteBucketIntelligentTieringConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketIntelligentTieringConfigurationOutput");
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketInventoryConfigurationInput {
    /// <p>The name of the bucket containing the inventory configuration to delete.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: InventoryId,
}

impl fmt::Debug for DeleteBucketInventoryConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketInventoryConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("id", &self.id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketInventoryConfigurationOutput {}

impl fmt::Debug for DeleteBucketInventoryConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketInventoryConfigurationOutput");
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketLifecycleInput {
    /// <p>The bucket name of the lifecycle to delete.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for DeleteBucketLifecycleInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketLifecycleInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketLifecycleOutput {}

impl fmt::Debug for DeleteBucketLifecycleOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketLifecycleOutput");
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketMetricsConfigurationInput {
    /// <p>The name of the bucket containing the metrics configuration to delete.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: MetricsId,
}

impl fmt::Debug for DeleteBucketMetricsConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketMetricsConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("id", &self.id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketMetricsConfigurationOutput {}

impl fmt::Debug for DeleteBucketMetricsConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketMetricsConfigurationOutput");
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketOutput {}

impl fmt::Debug for DeleteBucketOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketOutput");
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketOwnershipControlsInput {
    /// <p>The Amazon S3 bucket whose <code>OwnershipControls</code> you want to delete. </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for DeleteBucketOwnershipControlsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketOwnershipControlsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketOwnershipControlsOutput {}

impl fmt::Debug for DeleteBucketOwnershipControlsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketOwnershipControlsOutput");
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketPolicyInput {
    /// <p>The bucket name.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for DeleteBucketPolicyInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketPolicyInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketPolicyOutput {}

impl fmt::Debug for DeleteBucketPolicyOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketPolicyOutput");
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketReplicationInput {
    /// <p> The bucket name. </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for DeleteBucketReplicationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketReplicationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketReplicationOutput {}

impl fmt::Debug for DeleteBucketReplicationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketReplicationOutput");
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketTaggingInput {
    /// <p>The bucket that has the tag set to be removed.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for DeleteBucketTaggingInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketTaggingInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketTaggingOutput {}

impl fmt::Debug for DeleteBucketTaggingOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketTaggingOutput");
        d.finish_non_exhaustive()
    }
}

pub struct DeleteBucketWebsiteInput {
    /// <p>The bucket name for which you want to remove the website configuration. </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for DeleteBucketWebsiteInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketWebsiteInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteBucketWebsiteOutput {}

impl fmt::Debug for DeleteBucketWebsiteOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteBucketWebsiteOutput");
        d.finish_non_exhaustive()
    }
}

pub type DeleteMarker = bool;

/// <p>Information about the delete marker.</p>
#[derive(Default)]
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

impl fmt::Debug for DeleteMarkerEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteMarkerEntry");
        d.field("is_latest", &self.is_latest);
        if let Some(ref val) = self.key {
            d.field("key", val);
        }
        if let Some(ref val) = self.last_modified {
            d.field("last_modified", val);
        }
        if let Some(ref val) = self.owner {
            d.field("owner", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
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
#[derive(Default)]
pub struct DeleteMarkerReplication {
    /// <p>Indicates whether to replicate delete markers.</p>
    /// <note>
    /// <p>Indicates whether to replicate delete markers.</p>
    /// </note>
    pub status: Option<DeleteMarkerReplicationStatus>,
}

impl fmt::Debug for DeleteMarkerReplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteMarkerReplication");
        if let Some(ref val) = self.status {
            d.field("status", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteMarkerReplicationStatus(Cow<'static, str>);

impl DeleteMarkerReplicationStatus {
    pub const DISABLED: &str = "Disabled";

    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for DeleteMarkerReplicationStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<DeleteMarkerReplicationStatus> for Cow<'static, str> {
    fn from(s: DeleteMarkerReplicationStatus) -> Self {
        s.0
    }
}

impl FromStr for DeleteMarkerReplicationStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type DeleteMarkerVersionId = String;

pub type DeleteMarkers = List<DeleteMarkerEntry>;

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

impl fmt::Debug for DeleteObjectInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteObjectInput");
        d.field("bucket", &self.bucket);
        d.field("bypass_governance_retention", &self.bypass_governance_retention);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.mfa {
            d.field("mfa", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteObjectOutput {
    /// <p>Specifies whether the versioned object that was permanently deleted was (true) or was
    /// not (false) a delete marker.</p>
    pub delete_marker: DeleteMarker,
    pub request_charged: Option<RequestCharged>,
    /// <p>Returns the version ID of the delete marker created as a result of the DELETE
    /// operation.</p>
    pub version_id: Option<ObjectVersionId>,
}

impl fmt::Debug for DeleteObjectOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteObjectOutput");
        d.field("delete_marker", &self.delete_marker);
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for DeleteObjectTaggingInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteObjectTaggingInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteObjectTaggingOutput {
    /// <p>The versionId of the object the tag-set was removed from.</p>
    pub version_id: Option<ObjectVersionId>,
}

impl fmt::Debug for DeleteObjectTaggingOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteObjectTaggingOutput");
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for DeleteObjectsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteObjectsInput");
        d.field("bucket", &self.bucket);
        d.field("bypass_governance_retention", &self.bypass_governance_retention);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        d.field("delete", &self.delete);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.mfa {
            d.field("mfa", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeleteObjectsOutput {
    /// <p>Container element for a successful delete. It identifies the object that was
    /// successfully deleted.</p>
    pub deleted: Option<DeletedObjects>,
    /// <p>Container for a failed delete action that describes the object that Amazon S3 attempted to
    /// delete and the error it encountered.</p>
    pub errors: Option<Errors>,
    pub request_charged: Option<RequestCharged>,
}

impl fmt::Debug for DeleteObjectsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeleteObjectsOutput");
        if let Some(ref val) = self.deleted {
            d.field("deleted", val);
        }
        if let Some(ref val) = self.errors {
            d.field("errors", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct DeletePublicAccessBlockInput {
    /// <p>The Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to delete.
    /// </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for DeletePublicAccessBlockInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeletePublicAccessBlockInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DeletePublicAccessBlockOutput {}

impl fmt::Debug for DeletePublicAccessBlockOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeletePublicAccessBlockOutput");
        d.finish_non_exhaustive()
    }
}

/// <p>Information about the deleted object.</p>
#[derive(Default)]
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

impl fmt::Debug for DeletedObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DeletedObject");
        d.field("delete_marker", &self.delete_marker);
        if let Some(ref val) = self.delete_marker_version_id {
            d.field("delete_marker_version_id", val);
        }
        if let Some(ref val) = self.key {
            d.field("key", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type DeletedObjects = List<DeletedObject>;

pub type Delimiter = String;

pub type Description = String;

/// <p>Specifies information about where to publish analysis or configuration results for an
/// Amazon S3 bucket and S3 Replication Time Control (S3 RTC).</p>
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

impl fmt::Debug for Destination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Destination");
        if let Some(ref val) = self.access_control_translation {
            d.field("access_control_translation", val);
        }
        if let Some(ref val) = self.account {
            d.field("account", val);
        }
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.encryption_configuration {
            d.field("encryption_configuration", val);
        }
        if let Some(ref val) = self.metrics {
            d.field("metrics", val);
        }
        if let Some(ref val) = self.replication_time {
            d.field("replication_time", val);
        }
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        d.finish_non_exhaustive()
    }
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodingType(Cow<'static, str>);

impl EncodingType {
    pub const URL: &str = "url";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for EncodingType {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<EncodingType> for Cow<'static, str> {
    fn from(s: EncodingType) -> Self {
        s.0
    }
}

impl FromStr for EncodingType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>Contains the type of server-side encryption used.</p>
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

impl fmt::Debug for Encryption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Encryption");
        d.field("encryption_type", &self.encryption_type);
        if let Some(ref val) = self.kms_context {
            d.field("kms_context", val);
        }
        if let Some(ref val) = self.kms_key_id {
            d.field("kms_key_id", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies encryption-related information for an Amazon S3 bucket that is a destination for
/// replicated objects.</p>
#[derive(Default)]
pub struct EncryptionConfiguration {
    /// <p>Specifies the ID (Key ARN or Alias ARN) of the customer managed Amazon Web Services KMS key
    /// stored in Amazon Web Services Key Management Service (KMS) for the destination bucket. Amazon S3 uses
    /// this key to encrypt replica objects. Amazon S3 only supports symmetric, customer managed KMS keys.
    /// For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using symmetric and
    /// asymmetric keys</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub replica_kms_key_id: Option<ReplicaKmsKeyID>,
}

impl fmt::Debug for EncryptionConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("EncryptionConfiguration");
        if let Some(ref val) = self.replica_kms_key_id {
            d.field("replica_kms_key_id", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type End = i64;

/// <p>A message that indicates the request is complete and no more messages will be sent. You
/// should not assume that the request is complete until the client receives an
/// <code>EndEvent</code>.</p>
#[derive(Default)]
pub struct EndEvent {}

impl fmt::Debug for EndEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("EndEvent");
        d.finish_non_exhaustive()
    }
}

/// <p>Container for all error elements.</p>
#[derive(Default)]
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

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Error");
        if let Some(ref val) = self.code {
            d.field("code", val);
        }
        if let Some(ref val) = self.key {
            d.field("key", val);
        }
        if let Some(ref val) = self.message {
            d.field("message", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type ErrorCode = String;

/// <p>The error information.</p>
pub struct ErrorDocument {
    /// <p>The object key name to use when a 4XX class error occurs.</p>
    /// <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints">
    /// XML related object key constraints</a>.</p>
    /// </important>
    pub key: ObjectKey,
}

impl fmt::Debug for ErrorDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ErrorDocument");
        d.field("key", &self.key);
        d.finish_non_exhaustive()
    }
}

pub type ErrorMessage = String;

pub type Errors = List<Error>;

/// <p>A container for specifying the configuration for Amazon EventBridge.</p>
#[derive(Default)]
pub struct EventBridgeConfiguration {}

impl fmt::Debug for EventBridgeConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("EventBridgeConfiguration");
        d.finish_non_exhaustive()
    }
}

pub type EventList = List<Event>;

/// <p>Optional configuration to replicate existing source bucket objects. For more
/// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-what-is-isnot-replicated.html#existing-object-replication">Replicating Existing Objects</a> in the <i>Amazon S3 User Guide</i>.
/// </p>
pub struct ExistingObjectReplication {
    /// <p></p>
    pub status: ExistingObjectReplicationStatus,
}

impl fmt::Debug for ExistingObjectReplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ExistingObjectReplication");
        d.field("status", &self.status);
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExistingObjectReplicationStatus(Cow<'static, str>);

impl ExistingObjectReplicationStatus {
    pub const DISABLED: &str = "Disabled";

    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ExistingObjectReplicationStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ExistingObjectReplicationStatus> for Cow<'static, str> {
    fn from(s: ExistingObjectReplicationStatus) -> Self {
        s.0
    }
}

impl FromStr for ExistingObjectReplicationStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type Expiration = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpirationStatus(Cow<'static, str>);

impl ExpirationStatus {
    pub const DISABLED: &str = "Disabled";

    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ExpirationStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ExpirationStatus> for Cow<'static, str> {
    fn from(s: ExpirationStatus) -> Self {
        s.0
    }
}

impl FromStr for ExpirationStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type ExpiredObjectDeleteMarker = bool;

pub type Expires = Timestamp;

pub type ExposeHeader = String;

pub type ExposeHeaders = List<ExposeHeader>;

pub type Expression = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionType(Cow<'static, str>);

impl ExpressionType {
    pub const SQL: &str = "SQL";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ExpressionType {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ExpressionType> for Cow<'static, str> {
    fn from(s: ExpressionType) -> Self {
        s.0
    }
}

impl FromStr for ExpressionType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type FetchOwner = bool;

pub type FieldDelimiter = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileHeaderInfo(Cow<'static, str>);

impl FileHeaderInfo {
    pub const IGNORE: &str = "IGNORE";

    pub const NONE: &str = "NONE";

    pub const USE: &str = "USE";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for FileHeaderInfo {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<FileHeaderInfo> for Cow<'static, str> {
    fn from(s: FileHeaderInfo) -> Self {
        s.0
    }
}

impl FromStr for FileHeaderInfo {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>Specifies the Amazon S3 object key name to filter on and whether to filter on the suffix or
/// prefix of the key name.</p>
#[derive(Default)]
pub struct FilterRule {
    /// <p>The object key name prefix or suffix identifying one or more objects to which the
    /// filtering rule applies. The maximum length is 1,024 characters. Overlapping prefixes and
    /// suffixes are not supported. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring Event Notifications</a>
    /// in the <i>Amazon S3 User Guide</i>.</p>
    pub name: Option<FilterRuleName>,
    /// <p>The value that the filter searches for in object key names.</p>
    pub value: Option<FilterRuleValue>,
}

impl fmt::Debug for FilterRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("FilterRule");
        if let Some(ref val) = self.name {
            d.field("name", val);
        }
        if let Some(ref val) = self.value {
            d.field("value", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>A list of containers for the key-value pair that defines the criteria for the filter
/// rule.</p>
pub type FilterRuleList = List<FilterRule>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterRuleName(Cow<'static, str>);

impl FilterRuleName {
    pub const PREFIX: &str = "prefix";

    pub const SUFFIX: &str = "suffix";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for FilterRuleName {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<FilterRuleName> for Cow<'static, str> {
    fn from(s: FilterRuleName) -> Self {
        s.0
    }
}

impl FromStr for FilterRuleName {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type FilterRuleValue = String;

pub struct GetBucketAccelerateConfigurationInput {
    /// <p>The name of the bucket for which the accelerate configuration is retrieved.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketAccelerateConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketAccelerateConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketAccelerateConfigurationOutput {
    /// <p>The accelerate configuration of the bucket.</p>
    pub status: Option<BucketAccelerateStatus>,
}

impl fmt::Debug for GetBucketAccelerateConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketAccelerateConfigurationOutput");
        if let Some(ref val) = self.status {
            d.field("status", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketAclInput {
    /// <p>Specifies the S3 bucket whose ACL is being requested.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketAclInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketAclInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketAclOutput {
    /// <p>A list of grants.</p>
    pub grants: Option<Grants>,
    /// <p>Container for the bucket owner's display name and ID.</p>
    pub owner: Option<Owner>,
}

impl fmt::Debug for GetBucketAclOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketAclOutput");
        if let Some(ref val) = self.grants {
            d.field("grants", val);
        }
        if let Some(ref val) = self.owner {
            d.field("owner", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketAnalyticsConfigurationInput {
    /// <p>The name of the bucket from which an analytics configuration is retrieved.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID that identifies the analytics configuration.</p>
    pub id: AnalyticsId,
}

impl fmt::Debug for GetBucketAnalyticsConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketAnalyticsConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("id", &self.id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketAnalyticsConfigurationOutput {
    /// <p>The configuration and any analyses for the analytics filter.</p>
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}

impl fmt::Debug for GetBucketAnalyticsConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketAnalyticsConfigurationOutput");
        if let Some(ref val) = self.analytics_configuration {
            d.field("analytics_configuration", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketCorsInput {
    /// <p>The bucket name for which to get the cors configuration.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketCorsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketCorsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketCorsOutput {
    /// <p>A set of origins and methods (cross-origin access that you want to allow). You can add
    /// up to 100 rules to the configuration.</p>
    pub cors_rules: Option<CORSRules>,
}

impl fmt::Debug for GetBucketCorsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketCorsOutput");
        if let Some(ref val) = self.cors_rules {
            d.field("cors_rules", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketEncryptionInput {
    /// <p>The name of the bucket from which the server-side encryption configuration is
    /// retrieved.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketEncryptionInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketEncryptionInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketEncryptionOutput {
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

impl fmt::Debug for GetBucketEncryptionOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketEncryptionOutput");
        if let Some(ref val) = self.server_side_encryption_configuration {
            d.field("server_side_encryption_configuration", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketIntelligentTieringConfigurationInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub bucket: BucketName,
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub id: IntelligentTieringId,
}

impl fmt::Debug for GetBucketIntelligentTieringConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketIntelligentTieringConfigurationInput");
        d.field("bucket", &self.bucket);
        d.field("id", &self.id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketIntelligentTieringConfigurationOutput {
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
}

impl fmt::Debug for GetBucketIntelligentTieringConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketIntelligentTieringConfigurationOutput");
        if let Some(ref val) = self.intelligent_tiering_configuration {
            d.field("intelligent_tiering_configuration", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketInventoryConfigurationInput {
    /// <p>The name of the bucket containing the inventory configuration to retrieve.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: InventoryId,
}

impl fmt::Debug for GetBucketInventoryConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketInventoryConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("id", &self.id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketInventoryConfigurationOutput {
    /// <p>Specifies the inventory configuration.</p>
    pub inventory_configuration: Option<InventoryConfiguration>,
}

impl fmt::Debug for GetBucketInventoryConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketInventoryConfigurationOutput");
        if let Some(ref val) = self.inventory_configuration {
            d.field("inventory_configuration", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketLifecycleConfigurationInput {
    /// <p>The name of the bucket for which to get the lifecycle information.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketLifecycleConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketLifecycleConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketLifecycleConfigurationOutput {
    /// <p>Container for a lifecycle rule.</p>
    pub rules: Option<LifecycleRules>,
}

impl fmt::Debug for GetBucketLifecycleConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketLifecycleConfigurationOutput");
        if let Some(ref val) = self.rules {
            d.field("rules", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketLocationInput {
    /// <p>The name of the bucket for which to get the location.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketLocationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketLocationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketLocationOutput {
    /// <p>Specifies the Region where the bucket resides. For a list of all the Amazon S3 supported
    /// location constraints by Region, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region">Regions and Endpoints</a>.
    /// Buckets in Region <code>us-east-1</code> have a LocationConstraint of
    /// <code>null</code>.</p>
    pub location_constraint: Option<BucketLocationConstraint>,
}

impl fmt::Debug for GetBucketLocationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketLocationOutput");
        if let Some(ref val) = self.location_constraint {
            d.field("location_constraint", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketLoggingInput {
    /// <p>The bucket name for which to get the logging information.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketLoggingInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketLoggingInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketLoggingOutput {
    pub logging_enabled: Option<LoggingEnabled>,
}

impl fmt::Debug for GetBucketLoggingOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketLoggingOutput");
        if let Some(ref val) = self.logging_enabled {
            d.field("logging_enabled", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketMetricsConfigurationInput {
    /// <p>The name of the bucket containing the metrics configuration to retrieve.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: MetricsId,
}

impl fmt::Debug for GetBucketMetricsConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketMetricsConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("id", &self.id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketMetricsConfigurationOutput {
    /// <p>Specifies the metrics configuration.</p>
    pub metrics_configuration: Option<MetricsConfiguration>,
}

impl fmt::Debug for GetBucketMetricsConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketMetricsConfigurationOutput");
        if let Some(ref val) = self.metrics_configuration {
            d.field("metrics_configuration", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketNotificationConfigurationInput {
    /// <p>The name of the bucket for which to get the notification configuration.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketNotificationConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketNotificationConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>A container for specifying the notification configuration of the bucket. If this element
/// is empty, notifications are turned off for the bucket.</p>
#[derive(Default)]
pub struct GetBucketNotificationConfigurationOutput {
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

impl fmt::Debug for GetBucketNotificationConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketNotificationConfigurationOutput");
        if let Some(ref val) = self.event_bridge_configuration {
            d.field("event_bridge_configuration", val);
        }
        if let Some(ref val) = self.lambda_function_configurations {
            d.field("lambda_function_configurations", val);
        }
        if let Some(ref val) = self.queue_configurations {
            d.field("queue_configurations", val);
        }
        if let Some(ref val) = self.topic_configurations {
            d.field("topic_configurations", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketOwnershipControlsInput {
    /// <p>The name of the Amazon S3 bucket whose <code>OwnershipControls</code> you want to retrieve.
    /// </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketOwnershipControlsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketOwnershipControlsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketOwnershipControlsOutput {
    /// <p>The <code>OwnershipControls</code> (BucketOwnerEnforced, BucketOwnerPreferred, or ObjectWriter) currently in
    /// effect for this Amazon S3 bucket.</p>
    pub ownership_controls: Option<OwnershipControls>,
}

impl fmt::Debug for GetBucketOwnershipControlsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketOwnershipControlsOutput");
        if let Some(ref val) = self.ownership_controls {
            d.field("ownership_controls", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketPolicyInput {
    /// <p>The bucket name for which to get the bucket policy.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketPolicyInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketPolicyInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketPolicyOutput {
    /// <p>The bucket policy as a JSON document.</p>
    pub policy: Option<Policy>,
}

impl fmt::Debug for GetBucketPolicyOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketPolicyOutput");
        if let Some(ref val) = self.policy {
            d.field("policy", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketPolicyStatusInput {
    /// <p>The name of the Amazon S3 bucket whose policy status you want to retrieve.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketPolicyStatusInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketPolicyStatusInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketPolicyStatusOutput {
    /// <p>The policy status for the specified bucket.</p>
    pub policy_status: Option<PolicyStatus>,
}

impl fmt::Debug for GetBucketPolicyStatusOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketPolicyStatusOutput");
        if let Some(ref val) = self.policy_status {
            d.field("policy_status", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketReplicationInput {
    /// <p>The bucket name for which to get the replication information.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketReplicationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketReplicationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketReplicationOutput {
    pub replication_configuration: Option<ReplicationConfiguration>,
}

impl fmt::Debug for GetBucketReplicationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketReplicationOutput");
        if let Some(ref val) = self.replication_configuration {
            d.field("replication_configuration", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketRequestPaymentInput {
    /// <p>The name of the bucket for which to get the payment request configuration</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketRequestPaymentInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketRequestPaymentInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketRequestPaymentOutput {
    /// <p>Specifies who pays for the download and request fees.</p>
    pub payer: Option<Payer>,
}

impl fmt::Debug for GetBucketRequestPaymentOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketRequestPaymentOutput");
        if let Some(ref val) = self.payer {
            d.field("payer", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketTaggingInput {
    /// <p>The name of the bucket for which to get the tagging information.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketTaggingInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketTaggingInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketTaggingOutput {
    /// <p>Contains the tag set.</p>
    pub tag_set: TagSet,
}

impl fmt::Debug for GetBucketTaggingOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketTaggingOutput");
        d.field("tag_set", &self.tag_set);
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketVersioningInput {
    /// <p>The name of the bucket for which to get the versioning information.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketVersioningInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketVersioningInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetBucketVersioningOutput {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This
    /// element is only returned if the bucket has been configured with MFA delete. If the bucket
    /// has never been so configured, this element is not returned.</p>
    pub mfa_delete: Option<MFADeleteStatus>,
    /// <p>The versioning state of the bucket.</p>
    pub status: Option<BucketVersioningStatus>,
}

impl fmt::Debug for GetBucketVersioningOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketVersioningOutput");
        if let Some(ref val) = self.mfa_delete {
            d.field("mfa_delete", val);
        }
        if let Some(ref val) = self.status {
            d.field("status", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetBucketWebsiteInput {
    /// <p>The bucket name for which to get the website configuration.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetBucketWebsiteInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketWebsiteInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for GetBucketWebsiteOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetBucketWebsiteOutput");
        if let Some(ref val) = self.error_document {
            d.field("error_document", val);
        }
        if let Some(ref val) = self.index_document {
            d.field("index_document", val);
        }
        if let Some(ref val) = self.redirect_all_requests_to {
            d.field("redirect_all_requests_to", val);
        }
        if let Some(ref val) = self.routing_rules {
            d.field("routing_rules", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for GetObjectAclInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectAclInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetObjectAclOutput {
    /// <p>A list of grants.</p>
    pub grants: Option<Grants>,
    /// <p> Container for the bucket owner's display name and ID.</p>
    pub owner: Option<Owner>,
    pub request_charged: Option<RequestCharged>,
}

impl fmt::Debug for GetObjectAclOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectAclOutput");
        if let Some(ref val) = self.grants {
            d.field("grants", val);
        }
        if let Some(ref val) = self.owner {
            d.field("owner", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for GetObjectAttributesInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectAttributesInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        d.field("max_parts", &self.max_parts);
        d.field("object_attributes", &self.object_attributes);
        if let Some(ref val) = self.part_number_marker {
            d.field("part_number_marker", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key {
            d.field("sse_customer_key", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for GetObjectAttributesOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectAttributesOutput");
        if let Some(ref val) = self.checksum {
            d.field("checksum", val);
        }
        d.field("delete_marker", &self.delete_marker);
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        if let Some(ref val) = self.last_modified {
            d.field("last_modified", val);
        }
        if let Some(ref val) = self.object_parts {
            d.field("object_parts", val);
        }
        d.field("object_size", &self.object_size);
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>A collection of parts associated with a multipart upload.</p>
#[derive(Default)]
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

impl fmt::Debug for GetObjectAttributesParts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectAttributesParts");
        d.field("is_truncated", &self.is_truncated);
        d.field("max_parts", &self.max_parts);
        if let Some(ref val) = self.next_part_number_marker {
            d.field("next_part_number_marker", val);
        }
        if let Some(ref val) = self.part_number_marker {
            d.field("part_number_marker", val);
        }
        if let Some(ref val) = self.parts {
            d.field("parts", val);
        }
        d.field("total_parts_count", &self.total_parts_count);
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for GetObjectInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_mode {
            d.field("checksum_mode", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.if_match {
            d.field("if_match", val);
        }
        if let Some(ref val) = self.if_modified_since {
            d.field("if_modified_since", val);
        }
        if let Some(ref val) = self.if_none_match {
            d.field("if_none_match", val);
        }
        if let Some(ref val) = self.if_unmodified_since {
            d.field("if_unmodified_since", val);
        }
        d.field("key", &self.key);
        d.field("part_number", &self.part_number);
        if let Some(ref val) = self.range {
            d.field("range", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.response_cache_control {
            d.field("response_cache_control", val);
        }
        if let Some(ref val) = self.response_content_disposition {
            d.field("response_content_disposition", val);
        }
        if let Some(ref val) = self.response_content_encoding {
            d.field("response_content_encoding", val);
        }
        if let Some(ref val) = self.response_content_language {
            d.field("response_content_language", val);
        }
        if let Some(ref val) = self.response_content_type {
            d.field("response_content_type", val);
        }
        if let Some(ref val) = self.response_expires {
            d.field("response_expires", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key {
            d.field("sse_customer_key", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for GetObjectLegalHoldInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectLegalHoldInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetObjectLegalHoldOutput {
    /// <p>The current legal hold status for the specified object.</p>
    pub legal_hold: Option<ObjectLockLegalHold>,
}

impl fmt::Debug for GetObjectLegalHoldOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectLegalHoldOutput");
        if let Some(ref val) = self.legal_hold {
            d.field("legal_hold", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetObjectLockConfigurationInput {
    /// <p>The bucket whose Object Lock configuration you want to retrieve.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetObjectLockConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectLockConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetObjectLockConfigurationOutput {
    /// <p>The specified bucket's Object Lock configuration.</p>
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
}

impl fmt::Debug for GetObjectLockConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectLockConfigurationOutput");
        if let Some(ref val) = self.object_lock_configuration {
            d.field("object_lock_configuration", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for GetObjectOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectOutput");
        if let Some(ref val) = self.accept_ranges {
            d.field("accept_ranges", val);
        }
        if let Some(ref val) = self.body {
            d.field("body", val);
        }
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.cache_control {
            d.field("cache_control", val);
        }
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.content_disposition {
            d.field("content_disposition", val);
        }
        if let Some(ref val) = self.content_encoding {
            d.field("content_encoding", val);
        }
        if let Some(ref val) = self.content_language {
            d.field("content_language", val);
        }
        d.field("content_length", &self.content_length);
        if let Some(ref val) = self.content_range {
            d.field("content_range", val);
        }
        if let Some(ref val) = self.content_type {
            d.field("content_type", val);
        }
        d.field("delete_marker", &self.delete_marker);
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        if let Some(ref val) = self.expiration {
            d.field("expiration", val);
        }
        if let Some(ref val) = self.expires {
            d.field("expires", val);
        }
        if let Some(ref val) = self.last_modified {
            d.field("last_modified", val);
        }
        if let Some(ref val) = self.metadata {
            d.field("metadata", val);
        }
        d.field("missing_meta", &self.missing_meta);
        if let Some(ref val) = self.object_lock_legal_hold_status {
            d.field("object_lock_legal_hold_status", val);
        }
        if let Some(ref val) = self.object_lock_mode {
            d.field("object_lock_mode", val);
        }
        if let Some(ref val) = self.object_lock_retain_until_date {
            d.field("object_lock_retain_until_date", val);
        }
        d.field("parts_count", &self.parts_count);
        if let Some(ref val) = self.replication_status {
            d.field("replication_status", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.restore {
            d.field("restore", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        d.field("tag_count", &self.tag_count);
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        if let Some(ref val) = self.website_redirect_location {
            d.field("website_redirect_location", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type GetObjectResponseStatusCode = i32;

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

impl fmt::Debug for GetObjectRetentionInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectRetentionInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetObjectRetentionOutput {
    /// <p>The container element for an object's retention settings.</p>
    pub retention: Option<ObjectLockRetention>,
}

impl fmt::Debug for GetObjectRetentionOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectRetentionOutput");
        if let Some(ref val) = self.retention {
            d.field("retention", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for GetObjectTaggingInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectTaggingInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetObjectTaggingOutput {
    /// <p>Contains the tag set.</p>
    pub tag_set: TagSet,
    /// <p>The versionId of the object for which you got the tagging information.</p>
    pub version_id: Option<ObjectVersionId>,
}

impl fmt::Debug for GetObjectTaggingOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectTaggingOutput");
        d.field("tag_set", &self.tag_set);
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetObjectTorrentInput {
    /// <p>The name of the bucket containing the object for which to get the torrent files.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    /// <p>The object key for which to get the information.</p>
    pub key: ObjectKey,
    pub request_payer: Option<RequestPayer>,
}

impl fmt::Debug for GetObjectTorrentInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectTorrentInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetObjectTorrentOutput {
    /// <p>A Bencoded dictionary as defined by the BitTorrent specification</p>
    pub body: Option<StreamingBlob>,
    pub request_charged: Option<RequestCharged>,
}

impl fmt::Debug for GetObjectTorrentOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetObjectTorrentOutput");
        if let Some(ref val) = self.body {
            d.field("body", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct GetPublicAccessBlockInput {
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want
    /// to retrieve. </p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for GetPublicAccessBlockInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetPublicAccessBlockInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct GetPublicAccessBlockOutput {
    /// <p>The <code>PublicAccessBlock</code> configuration currently in effect for this Amazon S3
    /// bucket.</p>
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}

impl fmt::Debug for GetPublicAccessBlockOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GetPublicAccessBlockOutput");
        if let Some(ref val) = self.public_access_block_configuration {
            d.field("public_access_block_configuration", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Container for S3 Glacier job parameters.</p>
pub struct GlacierJobParameters {
    /// <p>Retrieval tier at which the restore will be processed.</p>
    pub tier: Tier,
}

impl fmt::Debug for GlacierJobParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("GlacierJobParameters");
        d.field("tier", &self.tier);
        d.finish_non_exhaustive()
    }
}

/// <p>Container for grant information.</p>
#[derive(Default)]
pub struct Grant {
    /// <p>The person being granted permissions.</p>
    pub grantee: Option<Grantee>,
    /// <p>Specifies the permission given to the grantee.</p>
    pub permission: Option<Permission>,
}

impl fmt::Debug for Grant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Grant");
        if let Some(ref val) = self.grantee {
            d.field("grantee", val);
        }
        if let Some(ref val) = self.permission {
            d.field("permission", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type GrantFullControl = String;

pub type GrantRead = String;

pub type GrantReadACP = String;

pub type GrantWrite = String;

pub type GrantWriteACP = String;

/// <p>Container for the person being granted permissions.</p>
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

impl fmt::Debug for Grantee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Grantee");
        if let Some(ref val) = self.display_name {
            d.field("display_name", val);
        }
        if let Some(ref val) = self.email_address {
            d.field("email_address", val);
        }
        if let Some(ref val) = self.id {
            d.field("id", val);
        }
        d.field("type_", &self.type_);
        if let Some(ref val) = self.uri {
            d.field("uri", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type Grants = List<Grant>;

pub struct HeadBucketInput {
    /// <p>The bucket name.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code>
    /// <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for HeadBucketInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("HeadBucketInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct HeadBucketOutput {}

impl fmt::Debug for HeadBucketOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("HeadBucketOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for HeadObjectInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("HeadObjectInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_mode {
            d.field("checksum_mode", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.if_match {
            d.field("if_match", val);
        }
        if let Some(ref val) = self.if_modified_since {
            d.field("if_modified_since", val);
        }
        if let Some(ref val) = self.if_none_match {
            d.field("if_none_match", val);
        }
        if let Some(ref val) = self.if_unmodified_since {
            d.field("if_unmodified_since", val);
        }
        d.field("key", &self.key);
        d.field("part_number", &self.part_number);
        if let Some(ref val) = self.range {
            d.field("range", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key {
            d.field("sse_customer_key", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for HeadObjectOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("HeadObjectOutput");
        if let Some(ref val) = self.accept_ranges {
            d.field("accept_ranges", val);
        }
        if let Some(ref val) = self.archive_status {
            d.field("archive_status", val);
        }
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.cache_control {
            d.field("cache_control", val);
        }
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.content_disposition {
            d.field("content_disposition", val);
        }
        if let Some(ref val) = self.content_encoding {
            d.field("content_encoding", val);
        }
        if let Some(ref val) = self.content_language {
            d.field("content_language", val);
        }
        d.field("content_length", &self.content_length);
        if let Some(ref val) = self.content_type {
            d.field("content_type", val);
        }
        d.field("delete_marker", &self.delete_marker);
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        if let Some(ref val) = self.expiration {
            d.field("expiration", val);
        }
        if let Some(ref val) = self.expires {
            d.field("expires", val);
        }
        if let Some(ref val) = self.last_modified {
            d.field("last_modified", val);
        }
        if let Some(ref val) = self.metadata {
            d.field("metadata", val);
        }
        d.field("missing_meta", &self.missing_meta);
        if let Some(ref val) = self.object_lock_legal_hold_status {
            d.field("object_lock_legal_hold_status", val);
        }
        if let Some(ref val) = self.object_lock_mode {
            d.field("object_lock_mode", val);
        }
        if let Some(ref val) = self.object_lock_retain_until_date {
            d.field("object_lock_retain_until_date", val);
        }
        d.field("parts_count", &self.parts_count);
        if let Some(ref val) = self.replication_status {
            d.field("replication_status", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.restore {
            d.field("restore", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        if let Some(ref val) = self.website_redirect_location {
            d.field("website_redirect_location", val);
        }
        d.finish_non_exhaustive()
    }
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

impl fmt::Debug for IndexDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("IndexDocument");
        d.field("suffix", &self.suffix);
        d.finish_non_exhaustive()
    }
}

pub type Initiated = Timestamp;

/// <p>Container element that identifies who initiated the multipart upload. </p>
#[derive(Default)]
pub struct Initiator {
    /// <p>Name of the Principal.</p>
    pub display_name: Option<DisplayName>,
    /// <p>If the principal is an Amazon Web Services account, it provides the Canonical User ID. If the principal
    /// is an IAM User, it provides a user ARN value.</p>
    pub id: Option<ID>,
}

impl fmt::Debug for Initiator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Initiator");
        if let Some(ref val) = self.display_name {
            d.field("display_name", val);
        }
        if let Some(ref val) = self.id {
            d.field("id", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Describes the serialization format of the object.</p>
#[derive(Default)]
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

impl fmt::Debug for InputSerialization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("InputSerialization");
        if let Some(ref val) = self.csv {
            d.field("csv", val);
        }
        if let Some(ref val) = self.compression_type {
            d.field("compression_type", val);
        }
        if let Some(ref val) = self.json {
            d.field("json", val);
        }
        if let Some(ref val) = self.parquet {
            d.field("parquet", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntelligentTieringAccessTier(Cow<'static, str>);

impl IntelligentTieringAccessTier {
    pub const ARCHIVE_ACCESS: &str = "ARCHIVE_ACCESS";

    pub const DEEP_ARCHIVE_ACCESS: &str = "DEEP_ARCHIVE_ACCESS";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for IntelligentTieringAccessTier {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<IntelligentTieringAccessTier> for Cow<'static, str> {
    fn from(s: IntelligentTieringAccessTier) -> Self {
        s.0
    }
}

impl FromStr for IntelligentTieringAccessTier {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>A container for specifying S3 Intelligent-Tiering filters. The filters determine the
/// subset of objects to which the rule applies.</p>
#[derive(Default)]
pub struct IntelligentTieringAndOperator {
    /// <p>An object key name prefix that identifies the subset of objects to which the
    /// configuration applies.</p>
    pub prefix: Option<Prefix>,
    /// <p>All of these tags must exist in the object's tag set in order for the configuration to
    /// apply.</p>
    pub tags: Option<TagSet>,
}

impl fmt::Debug for IntelligentTieringAndOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("IntelligentTieringAndOperator");
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.tags {
            d.field("tags", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies the S3 Intelligent-Tiering configuration for an Amazon S3 bucket.</p>
/// <p>For information about the S3 Intelligent-Tiering storage class, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for
/// automatically optimizing frequently and infrequently accessed objects</a>.</p>
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

impl fmt::Debug for IntelligentTieringConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("IntelligentTieringConfiguration");
        if let Some(ref val) = self.filter {
            d.field("filter", val);
        }
        d.field("id", &self.id);
        d.field("status", &self.status);
        d.field("tierings", &self.tierings);
        d.finish_non_exhaustive()
    }
}

pub type IntelligentTieringConfigurationList = List<IntelligentTieringConfiguration>;

pub type IntelligentTieringDays = i32;

/// <p>The <code>Filter</code> is used to identify objects that the S3 Intelligent-Tiering
/// configuration applies to.</p>
#[derive(Default)]
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

impl fmt::Debug for IntelligentTieringFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("IntelligentTieringFilter");
        if let Some(ref val) = self.and {
            d.field("and", val);
        }
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.tag {
            d.field("tag", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type IntelligentTieringId = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntelligentTieringStatus(Cow<'static, str>);

impl IntelligentTieringStatus {
    pub const DISABLED: &str = "Disabled";

    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for IntelligentTieringStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<IntelligentTieringStatus> for Cow<'static, str> {
    fn from(s: IntelligentTieringStatus) -> Self {
        s.0
    }
}

impl FromStr for IntelligentTieringStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>Object is archived and inaccessible until restored.</p>
#[derive(Default)]
pub struct InvalidObjectState {
    pub access_tier: Option<IntelligentTieringAccessTier>,
    pub storage_class: Option<StorageClass>,
}

impl fmt::Debug for InvalidObjectState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("InvalidObjectState");
        if let Some(ref val) = self.access_tier {
            d.field("access_tier", val);
        }
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies the inventory configuration for an Amazon S3 bucket. For more information, see
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketGETInventoryConfig.html">GET Bucket inventory</a> in the <i>Amazon S3 API Reference</i>.
/// </p>
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

impl fmt::Debug for InventoryConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("InventoryConfiguration");
        d.field("destination", &self.destination);
        if let Some(ref val) = self.filter {
            d.field("filter", val);
        }
        d.field("id", &self.id);
        d.field("included_object_versions", &self.included_object_versions);
        d.field("is_enabled", &self.is_enabled);
        if let Some(ref val) = self.optional_fields {
            d.field("optional_fields", val);
        }
        d.field("schedule", &self.schedule);
        d.finish_non_exhaustive()
    }
}

pub type InventoryConfigurationList = List<InventoryConfiguration>;

/// <p>Specifies the inventory configuration for an Amazon S3 bucket.</p>
pub struct InventoryDestination {
    /// <p>Contains the bucket name, file format, bucket owner (optional), and prefix (optional)
    /// where inventory results are published.</p>
    pub s3_bucket_destination: InventoryS3BucketDestination,
}

impl fmt::Debug for InventoryDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("InventoryDestination");
        d.field("s3_bucket_destination", &self.s3_bucket_destination);
        d.finish_non_exhaustive()
    }
}

/// <p>Contains the type of server-side encryption used to encrypt the inventory
/// results.</p>
#[derive(Default)]
pub struct InventoryEncryption {
    /// <p>Specifies the use of SSE-KMS to encrypt delivered inventory reports.</p>
    pub ssekms: Option<SSEKMS>,
    /// <p>Specifies the use of SSE-S3 to encrypt delivered inventory reports.</p>
    pub sses3: Option<SSES3>,
}

impl fmt::Debug for InventoryEncryption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("InventoryEncryption");
        if let Some(ref val) = self.ssekms {
            d.field("ssekms", val);
        }
        if let Some(ref val) = self.sses3 {
            d.field("sses3", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies an inventory filter. The inventory only includes objects that meet the
/// filter's criteria.</p>
pub struct InventoryFilter {
    /// <p>The prefix that an object must have to be included in the inventory results.</p>
    pub prefix: Prefix,
}

impl fmt::Debug for InventoryFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("InventoryFilter");
        d.field("prefix", &self.prefix);
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryFormat(Cow<'static, str>);

impl InventoryFormat {
    pub const CSV: &str = "CSV";

    pub const ORC: &str = "ORC";

    pub const PARQUET: &str = "Parquet";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for InventoryFormat {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<InventoryFormat> for Cow<'static, str> {
    fn from(s: InventoryFormat) -> Self {
        s.0
    }
}

impl FromStr for InventoryFormat {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryFrequency(Cow<'static, str>);

impl InventoryFrequency {
    pub const DAILY: &str = "Daily";

    pub const WEEKLY: &str = "Weekly";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for InventoryFrequency {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<InventoryFrequency> for Cow<'static, str> {
    fn from(s: InventoryFrequency) -> Self {
        s.0
    }
}

impl FromStr for InventoryFrequency {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type InventoryId = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryIncludedObjectVersions(Cow<'static, str>);

impl InventoryIncludedObjectVersions {
    pub const ALL: &str = "All";

    pub const CURRENT: &str = "Current";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for InventoryIncludedObjectVersions {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<InventoryIncludedObjectVersions> for Cow<'static, str> {
    fn from(s: InventoryIncludedObjectVersions) -> Self {
        s.0
    }
}

impl FromStr for InventoryIncludedObjectVersions {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryOptionalField(Cow<'static, str>);

impl InventoryOptionalField {
    pub const BUCKET_KEY_STATUS: &str = "BucketKeyStatus";

    pub const CHECKSUM_ALGORITHM: &str = "ChecksumAlgorithm";

    pub const E_TAG: &str = "ETag";

    pub const ENCRYPTION_STATUS: &str = "EncryptionStatus";

    pub const INTELLIGENT_TIERING_ACCESS_TIER: &str = "IntelligentTieringAccessTier";

    pub const IS_MULTIPART_UPLOADED: &str = "IsMultipartUploaded";

    pub const LAST_MODIFIED_DATE: &str = "LastModifiedDate";

    pub const OBJECT_LOCK_LEGAL_HOLD_STATUS: &str = "ObjectLockLegalHoldStatus";

    pub const OBJECT_LOCK_MODE: &str = "ObjectLockMode";

    pub const OBJECT_LOCK_RETAIN_UNTIL_DATE: &str = "ObjectLockRetainUntilDate";

    pub const REPLICATION_STATUS: &str = "ReplicationStatus";

    pub const SIZE: &str = "Size";

    pub const STORAGE_CLASS: &str = "StorageClass";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for InventoryOptionalField {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<InventoryOptionalField> for Cow<'static, str> {
    fn from(s: InventoryOptionalField) -> Self {
        s.0
    }
}

impl FromStr for InventoryOptionalField {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type InventoryOptionalFields = List<InventoryOptionalField>;

/// <p>Contains the bucket name, file format, bucket owner (optional), and prefix (optional)
/// where inventory results are published.</p>
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

impl fmt::Debug for InventoryS3BucketDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("InventoryS3BucketDestination");
        if let Some(ref val) = self.account_id {
            d.field("account_id", val);
        }
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.encryption {
            d.field("encryption", val);
        }
        d.field("format", &self.format);
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies the schedule for generating inventory results.</p>
pub struct InventorySchedule {
    /// <p>Specifies how frequently inventory results are produced.</p>
    pub frequency: InventoryFrequency,
}

impl fmt::Debug for InventorySchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("InventorySchedule");
        d.field("frequency", &self.frequency);
        d.finish_non_exhaustive()
    }
}

pub type IsEnabled = bool;

pub type IsLatest = bool;

pub type IsPublic = bool;

pub type IsTruncated = bool;

/// <p>Specifies JSON as object's input serialization format.</p>
#[derive(Default)]
pub struct JSONInput {
    /// <p>The type of JSON. Valid values: Document, Lines.</p>
    pub type_: Option<JSONType>,
}

impl fmt::Debug for JSONInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("JSONInput");
        if let Some(ref val) = self.type_ {
            d.field("type_", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies JSON as request's output serialization format.</p>
#[derive(Default)]
pub struct JSONOutput {
    /// <p>The value used to separate individual records in the output. If no value is specified,
    /// Amazon S3 uses a newline character ('\n').</p>
    pub record_delimiter: Option<RecordDelimiter>,
}

impl fmt::Debug for JSONOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("JSONOutput");
        if let Some(ref val) = self.record_delimiter {
            d.field("record_delimiter", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JSONType(Cow<'static, str>);

impl JSONType {
    pub const DOCUMENT: &str = "DOCUMENT";

    pub const LINES: &str = "LINES";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for JSONType {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<JSONType> for Cow<'static, str> {
    fn from(s: JSONType) -> Self {
        s.0
    }
}

impl FromStr for JSONType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type KMSContext = String;

pub type KeyCount = i32;

pub type KeyMarker = String;

pub type KeyPrefixEquals = String;

pub type LambdaFunctionArn = String;

/// <p>A container for specifying the configuration for Lambda notifications.</p>
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

impl fmt::Debug for LambdaFunctionConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("LambdaFunctionConfiguration");
        d.field("events", &self.events);
        if let Some(ref val) = self.filter {
            d.field("filter", val);
        }
        if let Some(ref val) = self.id {
            d.field("id", val);
        }
        d.field("lambda_function_arn", &self.lambda_function_arn);
        d.finish_non_exhaustive()
    }
}

pub type LambdaFunctionConfigurationList = List<LambdaFunctionConfiguration>;

pub type LastModified = Timestamp;

/// <p>Container for the expiration for the lifecycle of the object.</p>
#[derive(Default)]
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

impl fmt::Debug for LifecycleExpiration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("LifecycleExpiration");
        if let Some(ref val) = self.date {
            d.field("date", val);
        }
        d.field("days", &self.days);
        d.field("expired_object_delete_marker", &self.expired_object_delete_marker);
        d.finish_non_exhaustive()
    }
}

/// <p>A lifecycle rule for individual objects in an Amazon S3 bucket.</p>
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

impl fmt::Debug for LifecycleRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("LifecycleRule");
        if let Some(ref val) = self.abort_incomplete_multipart_upload {
            d.field("abort_incomplete_multipart_upload", val);
        }
        if let Some(ref val) = self.expiration {
            d.field("expiration", val);
        }
        if let Some(ref val) = self.filter {
            d.field("filter", val);
        }
        if let Some(ref val) = self.id {
            d.field("id", val);
        }
        if let Some(ref val) = self.noncurrent_version_expiration {
            d.field("noncurrent_version_expiration", val);
        }
        if let Some(ref val) = self.noncurrent_version_transitions {
            d.field("noncurrent_version_transitions", val);
        }
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        d.field("status", &self.status);
        if let Some(ref val) = self.transitions {
            d.field("transitions", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>This is used in a Lifecycle Rule Filter to apply a logical AND to two or more
/// predicates. The Lifecycle Rule will apply to any object matching all of the predicates
/// configured inside the And operator.</p>
#[derive(Default)]
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

impl fmt::Debug for LifecycleRuleAndOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("LifecycleRuleAndOperator");
        d.field("object_size_greater_than", &self.object_size_greater_than);
        d.field("object_size_less_than", &self.object_size_less_than);
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.tags {
            d.field("tags", val);
        }
        d.finish_non_exhaustive()
    }
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

pub struct ListBucketAnalyticsConfigurationsInput {
    /// <p>The name of the bucket from which analytics configurations are retrieved.</p>
    pub bucket: BucketName,
    /// <p>The ContinuationToken that represents a placeholder from where this request should
    /// begin.</p>
    pub continuation_token: Option<Token>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
}

impl fmt::Debug for ListBucketAnalyticsConfigurationsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListBucketAnalyticsConfigurationsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.continuation_token {
            d.field("continuation_token", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for ListBucketAnalyticsConfigurationsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListBucketAnalyticsConfigurationsOutput");
        if let Some(ref val) = self.analytics_configuration_list {
            d.field("analytics_configuration_list", val);
        }
        if let Some(ref val) = self.continuation_token {
            d.field("continuation_token", val);
        }
        d.field("is_truncated", &self.is_truncated);
        if let Some(ref val) = self.next_continuation_token {
            d.field("next_continuation_token", val);
        }
        d.finish_non_exhaustive()
    }
}

pub struct ListBucketIntelligentTieringConfigurationsInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub bucket: BucketName,
    /// <p>The <code>ContinuationToken</code> that represents a placeholder from where this request
    /// should begin.</p>
    pub continuation_token: Option<Token>,
}

impl fmt::Debug for ListBucketIntelligentTieringConfigurationsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListBucketIntelligentTieringConfigurationsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.continuation_token {
            d.field("continuation_token", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for ListBucketIntelligentTieringConfigurationsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListBucketIntelligentTieringConfigurationsOutput");
        if let Some(ref val) = self.continuation_token {
            d.field("continuation_token", val);
        }
        if let Some(ref val) = self.intelligent_tiering_configuration_list {
            d.field("intelligent_tiering_configuration_list", val);
        }
        d.field("is_truncated", &self.is_truncated);
        if let Some(ref val) = self.next_continuation_token {
            d.field("next_continuation_token", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for ListBucketInventoryConfigurationsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListBucketInventoryConfigurationsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.continuation_token {
            d.field("continuation_token", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for ListBucketInventoryConfigurationsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListBucketInventoryConfigurationsOutput");
        if let Some(ref val) = self.continuation_token {
            d.field("continuation_token", val);
        }
        if let Some(ref val) = self.inventory_configuration_list {
            d.field("inventory_configuration_list", val);
        }
        d.field("is_truncated", &self.is_truncated);
        if let Some(ref val) = self.next_continuation_token {
            d.field("next_continuation_token", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for ListBucketMetricsConfigurationsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListBucketMetricsConfigurationsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.continuation_token {
            d.field("continuation_token", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for ListBucketMetricsConfigurationsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListBucketMetricsConfigurationsOutput");
        if let Some(ref val) = self.continuation_token {
            d.field("continuation_token", val);
        }
        d.field("is_truncated", &self.is_truncated);
        if let Some(ref val) = self.metrics_configuration_list {
            d.field("metrics_configuration_list", val);
        }
        if let Some(ref val) = self.next_continuation_token {
            d.field("next_continuation_token", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct ListBucketsInput {}

impl fmt::Debug for ListBucketsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListBucketsInput");
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct ListBucketsOutput {
    /// <p>The list of buckets owned by the requester.</p>
    pub buckets: Option<Buckets>,
    /// <p>The owner of the buckets listed.</p>
    pub owner: Option<Owner>,
}

impl fmt::Debug for ListBucketsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListBucketsOutput");
        if let Some(ref val) = self.buckets {
            d.field("buckets", val);
        }
        if let Some(ref val) = self.owner {
            d.field("owner", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for ListMultipartUploadsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListMultipartUploadsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.delimiter {
            d.field("delimiter", val);
        }
        if let Some(ref val) = self.encoding_type {
            d.field("encoding_type", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.key_marker {
            d.field("key_marker", val);
        }
        d.field("max_uploads", &self.max_uploads);
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.upload_id_marker {
            d.field("upload_id_marker", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for ListMultipartUploadsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListMultipartUploadsOutput");
        if let Some(ref val) = self.bucket {
            d.field("bucket", val);
        }
        if let Some(ref val) = self.common_prefixes {
            d.field("common_prefixes", val);
        }
        if let Some(ref val) = self.delimiter {
            d.field("delimiter", val);
        }
        if let Some(ref val) = self.encoding_type {
            d.field("encoding_type", val);
        }
        d.field("is_truncated", &self.is_truncated);
        if let Some(ref val) = self.key_marker {
            d.field("key_marker", val);
        }
        d.field("max_uploads", &self.max_uploads);
        if let Some(ref val) = self.next_key_marker {
            d.field("next_key_marker", val);
        }
        if let Some(ref val) = self.next_upload_id_marker {
            d.field("next_upload_id_marker", val);
        }
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.upload_id_marker {
            d.field("upload_id_marker", val);
        }
        if let Some(ref val) = self.uploads {
            d.field("uploads", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for ListObjectVersionsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListObjectVersionsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.delimiter {
            d.field("delimiter", val);
        }
        if let Some(ref val) = self.encoding_type {
            d.field("encoding_type", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.key_marker {
            d.field("key_marker", val);
        }
        d.field("max_keys", &self.max_keys);
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.version_id_marker {
            d.field("version_id_marker", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for ListObjectVersionsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListObjectVersionsOutput");
        if let Some(ref val) = self.common_prefixes {
            d.field("common_prefixes", val);
        }
        if let Some(ref val) = self.delete_markers {
            d.field("delete_markers", val);
        }
        if let Some(ref val) = self.delimiter {
            d.field("delimiter", val);
        }
        if let Some(ref val) = self.encoding_type {
            d.field("encoding_type", val);
        }
        d.field("is_truncated", &self.is_truncated);
        if let Some(ref val) = self.key_marker {
            d.field("key_marker", val);
        }
        d.field("max_keys", &self.max_keys);
        if let Some(ref val) = self.name {
            d.field("name", val);
        }
        if let Some(ref val) = self.next_key_marker {
            d.field("next_key_marker", val);
        }
        if let Some(ref val) = self.next_version_id_marker {
            d.field("next_version_id_marker", val);
        }
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.version_id_marker {
            d.field("version_id_marker", val);
        }
        if let Some(ref val) = self.versions {
            d.field("versions", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for ListObjectsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListObjectsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.delimiter {
            d.field("delimiter", val);
        }
        if let Some(ref val) = self.encoding_type {
            d.field("encoding_type", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.marker {
            d.field("marker", val);
        }
        d.field("max_keys", &self.max_keys);
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for ListObjectsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListObjectsOutput");
        if let Some(ref val) = self.common_prefixes {
            d.field("common_prefixes", val);
        }
        if let Some(ref val) = self.contents {
            d.field("contents", val);
        }
        if let Some(ref val) = self.delimiter {
            d.field("delimiter", val);
        }
        if let Some(ref val) = self.encoding_type {
            d.field("encoding_type", val);
        }
        d.field("is_truncated", &self.is_truncated);
        if let Some(ref val) = self.marker {
            d.field("marker", val);
        }
        d.field("max_keys", &self.max_keys);
        if let Some(ref val) = self.name {
            d.field("name", val);
        }
        if let Some(ref val) = self.next_marker {
            d.field("next_marker", val);
        }
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for ListObjectsV2Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListObjectsV2Input");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.continuation_token {
            d.field("continuation_token", val);
        }
        if let Some(ref val) = self.delimiter {
            d.field("delimiter", val);
        }
        if let Some(ref val) = self.encoding_type {
            d.field("encoding_type", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("fetch_owner", &self.fetch_owner);
        d.field("max_keys", &self.max_keys);
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.start_after {
            d.field("start_after", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for ListObjectsV2Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListObjectsV2Output");
        if let Some(ref val) = self.common_prefixes {
            d.field("common_prefixes", val);
        }
        if let Some(ref val) = self.contents {
            d.field("contents", val);
        }
        if let Some(ref val) = self.continuation_token {
            d.field("continuation_token", val);
        }
        if let Some(ref val) = self.delimiter {
            d.field("delimiter", val);
        }
        if let Some(ref val) = self.encoding_type {
            d.field("encoding_type", val);
        }
        d.field("is_truncated", &self.is_truncated);
        d.field("key_count", &self.key_count);
        d.field("max_keys", &self.max_keys);
        if let Some(ref val) = self.name {
            d.field("name", val);
        }
        if let Some(ref val) = self.next_continuation_token {
            d.field("next_continuation_token", val);
        }
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.start_after {
            d.field("start_after", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for ListPartsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListPartsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        d.field("max_parts", &self.max_parts);
        if let Some(ref val) = self.part_number_marker {
            d.field("part_number_marker", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key {
            d.field("sse_customer_key", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        d.field("upload_id", &self.upload_id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for ListPartsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ListPartsOutput");
        if let Some(ref val) = self.abort_date {
            d.field("abort_date", val);
        }
        if let Some(ref val) = self.abort_rule_id {
            d.field("abort_rule_id", val);
        }
        if let Some(ref val) = self.bucket {
            d.field("bucket", val);
        }
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.initiator {
            d.field("initiator", val);
        }
        d.field("is_truncated", &self.is_truncated);
        if let Some(ref val) = self.key {
            d.field("key", val);
        }
        d.field("max_parts", &self.max_parts);
        if let Some(ref val) = self.next_part_number_marker {
            d.field("next_part_number_marker", val);
        }
        if let Some(ref val) = self.owner {
            d.field("owner", val);
        }
        if let Some(ref val) = self.part_number_marker {
            d.field("part_number_marker", val);
        }
        if let Some(ref val) = self.parts {
            d.field("parts", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        if let Some(ref val) = self.upload_id {
            d.field("upload_id", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type Location = String;

pub type LocationPrefix = String;

/// <p>Describes where logs are stored and the prefix that Amazon S3 assigns to all log object keys
/// for a bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTlogging.html">PUT Bucket logging</a> in the
/// <i>Amazon S3 API Reference</i>.</p>
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

impl fmt::Debug for LoggingEnabled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("LoggingEnabled");
        d.field("target_bucket", &self.target_bucket);
        if let Some(ref val) = self.target_grants {
            d.field("target_grants", val);
        }
        d.field("target_prefix", &self.target_prefix);
        d.finish_non_exhaustive()
    }
}

pub type MFA = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MFADelete(Cow<'static, str>);

impl MFADelete {
    pub const DISABLED: &str = "Disabled";

    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for MFADelete {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<MFADelete> for Cow<'static, str> {
    fn from(s: MFADelete) -> Self {
        s.0
    }
}

impl FromStr for MFADelete {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MFADeleteStatus(Cow<'static, str>);

impl MFADeleteStatus {
    pub const DISABLED: &str = "Disabled";

    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for MFADeleteStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<MFADeleteStatus> for Cow<'static, str> {
    fn from(s: MFADeleteStatus) -> Self {
        s.0
    }
}

impl FromStr for MFADeleteStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type Marker = String;

pub type MaxAgeSeconds = i32;

pub type MaxKeys = i32;

pub type MaxParts = i32;

pub type MaxUploads = i32;

pub type Message = String;

pub type Metadata = Map<MetadataKey, MetadataValue>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataDirective(Cow<'static, str>);

impl MetadataDirective {
    pub const COPY: &str = "COPY";

    pub const REPLACE: &str = "REPLACE";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for MetadataDirective {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<MetadataDirective> for Cow<'static, str> {
    fn from(s: MetadataDirective) -> Self {
        s.0
    }
}

impl FromStr for MetadataDirective {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>A metadata key-value pair to store with an object.</p>
#[derive(Default)]
pub struct MetadataEntry {
    /// <p>Name of the Object.</p>
    pub name: Option<MetadataKey>,
    /// <p>Value of the Object.</p>
    pub value: Option<MetadataValue>,
}

impl fmt::Debug for MetadataEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("MetadataEntry");
        if let Some(ref val) = self.name {
            d.field("name", val);
        }
        if let Some(ref val) = self.value {
            d.field("value", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type MetadataKey = String;

pub type MetadataValue = String;

/// <p> A container specifying replication metrics-related settings enabling replication
/// metrics and events.</p>
pub struct Metrics {
    /// <p> A container specifying the time threshold for emitting the
    /// <code>s3:Replication:OperationMissedThreshold</code> event. </p>
    pub event_threshold: Option<ReplicationTimeValue>,
    /// <p> Specifies whether the replication metrics are enabled. </p>
    pub status: MetricsStatus,
}

impl fmt::Debug for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Metrics");
        if let Some(ref val) = self.event_threshold {
            d.field("event_threshold", val);
        }
        d.field("status", &self.status);
        d.finish_non_exhaustive()
    }
}

/// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter.
/// The operator must have at least two predicates, and an object must match all of the
/// predicates in order for the filter to apply.</p>
#[derive(Default)]
pub struct MetricsAndOperator {
    /// <p>The access point ARN used when evaluating an <code>AND</code> predicate.</p>
    pub access_point_arn: Option<AccessPointArn>,
    /// <p>The prefix used when evaluating an AND predicate.</p>
    pub prefix: Option<Prefix>,
    /// <p>The list of tags used when evaluating an AND predicate.</p>
    pub tags: Option<TagSet>,
}

impl fmt::Debug for MetricsAndOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("MetricsAndOperator");
        if let Some(ref val) = self.access_point_arn {
            d.field("access_point_arn", val);
        }
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.tags {
            d.field("tags", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies a metrics configuration for the CloudWatch request metrics (specified by the
/// metrics configuration ID) from an Amazon S3 bucket. If you're updating an existing metrics
/// configuration, note that this is a full replacement of the existing metrics configuration.
/// If you don't include the elements you want to keep, they are erased. For more information,
/// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTMetricConfiguration.html">PutBucketMetricsConfiguration</a>.</p>
pub struct MetricsConfiguration {
    /// <p>Specifies a metrics configuration filter. The metrics configuration will only include
    /// objects that meet the filter's criteria. A filter must be a prefix, an object tag, an access point ARN, or a conjunction
    /// (MetricsAndOperator).</p>
    pub filter: Option<MetricsFilter>,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: MetricsId,
}

impl fmt::Debug for MetricsConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("MetricsConfiguration");
        if let Some(ref val) = self.filter {
            d.field("filter", val);
        }
        d.field("id", &self.id);
        d.finish_non_exhaustive()
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricsStatus(Cow<'static, str>);

impl MetricsStatus {
    pub const DISABLED: &str = "Disabled";

    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for MetricsStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<MetricsStatus> for Cow<'static, str> {
    fn from(s: MetricsStatus) -> Self {
        s.0
    }
}

impl FromStr for MetricsStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type Minutes = i32;

pub type MissingMeta = i32;

/// <p>Container for the <code>MultipartUpload</code> for the Amazon S3 object.</p>
#[derive(Default)]
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

impl fmt::Debug for MultipartUpload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("MultipartUpload");
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.initiated {
            d.field("initiated", val);
        }
        if let Some(ref val) = self.initiator {
            d.field("initiator", val);
        }
        if let Some(ref val) = self.key {
            d.field("key", val);
        }
        if let Some(ref val) = self.owner {
            d.field("owner", val);
        }
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        if let Some(ref val) = self.upload_id {
            d.field("upload_id", val);
        }
        d.finish_non_exhaustive()
    }
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
#[derive(Default)]
pub struct NoSuchBucket {}

impl fmt::Debug for NoSuchBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("NoSuchBucket");
        d.finish_non_exhaustive()
    }
}

/// <p>The specified key does not exist.</p>
#[derive(Default)]
pub struct NoSuchKey {}

impl fmt::Debug for NoSuchKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("NoSuchKey");
        d.finish_non_exhaustive()
    }
}

/// <p>The specified multipart upload does not exist.</p>
#[derive(Default)]
pub struct NoSuchUpload {}

impl fmt::Debug for NoSuchUpload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("NoSuchUpload");
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently
/// deletes the noncurrent object versions. You set this lifecycle configuration action on a
/// bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent
/// object versions at a specific period in the object's lifetime.</p>
#[derive(Default)]
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

impl fmt::Debug for NoncurrentVersionExpiration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("NoncurrentVersionExpiration");
        d.field("newer_noncurrent_versions", &self.newer_noncurrent_versions);
        d.field("noncurrent_days", &self.noncurrent_days);
        d.finish_non_exhaustive()
    }
}

/// <p>Container for the transition rule that describes when noncurrent objects transition to
/// the <code>STANDARD_IA</code>, <code>ONEZONE_IA</code>, <code>INTELLIGENT_TIERING</code>,
/// <code>GLACIER_IR</code>, <code>GLACIER</code>, or <code>DEEP_ARCHIVE</code> storage class. If your bucket is
/// versioning-enabled (or versioning is suspended), you can set this action to request that
/// Amazon S3 transition noncurrent object versions to the <code>STANDARD_IA</code>,
/// <code>ONEZONE_IA</code>, <code>INTELLIGENT_TIERING</code>, <code>GLACIER_IR</code>, <code>GLACIER</code>, or
/// <code>DEEP_ARCHIVE</code> storage class at a specific period in the object's
/// lifetime.</p>
#[derive(Default)]
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

impl fmt::Debug for NoncurrentVersionTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("NoncurrentVersionTransition");
        d.field("newer_noncurrent_versions", &self.newer_noncurrent_versions);
        d.field("noncurrent_days", &self.noncurrent_days);
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type NoncurrentVersionTransitionList = List<NoncurrentVersionTransition>;

/// <p>The specified content does not exist.</p>
#[derive(Default)]
pub struct NotFound {}

impl fmt::Debug for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("NotFound");
        d.finish_non_exhaustive()
    }
}

/// <p>A container for specifying the notification configuration of the bucket. If this element
/// is empty, notifications are turned off for the bucket.</p>
#[derive(Default)]
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

impl fmt::Debug for NotificationConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("NotificationConfiguration");
        if let Some(ref val) = self.event_bridge_configuration {
            d.field("event_bridge_configuration", val);
        }
        if let Some(ref val) = self.lambda_function_configurations {
            d.field("lambda_function_configurations", val);
        }
        if let Some(ref val) = self.queue_configurations {
            d.field("queue_configurations", val);
        }
        if let Some(ref val) = self.topic_configurations {
            d.field("topic_configurations", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies object key name filtering rules. For information about key name filtering, see
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring
/// Event Notifications</a> in the <i>Amazon S3 User Guide</i>.</p>
#[derive(Default)]
pub struct NotificationConfigurationFilter {
    pub key: Option<S3KeyFilter>,
}

impl fmt::Debug for NotificationConfigurationFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("NotificationConfigurationFilter");
        if let Some(ref val) = self.key {
            d.field("key", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>An optional unique identifier for configurations in a notification configuration. If you
/// don't provide one, Amazon S3 will assign an ID.</p>
pub type NotificationId = String;

/// <p>An object consists of data and its descriptive metadata.</p>
#[derive(Default)]
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

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Object");
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        if let Some(ref val) = self.key {
            d.field("key", val);
        }
        if let Some(ref val) = self.last_modified {
            d.field("last_modified", val);
        }
        if let Some(ref val) = self.owner {
            d.field("owner", val);
        }
        d.field("size", &self.size);
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>This action is not allowed against this storage tier.</p>
#[derive(Default)]
pub struct ObjectAlreadyInActiveTierError {}

impl fmt::Debug for ObjectAlreadyInActiveTierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ObjectAlreadyInActiveTierError");
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectAttributes(Cow<'static, str>);

impl ObjectAttributes {
    pub const CHECKSUM: &str = "Checksum";

    pub const ETAG: &str = "ETag";

    pub const OBJECT_PARTS: &str = "ObjectParts";

    pub const OBJECT_SIZE: &str = "ObjectSize";

    pub const STORAGE_CLASS: &str = "StorageClass";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ObjectAttributes {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ObjectAttributes> for Cow<'static, str> {
    fn from(s: ObjectAttributes) -> Self {
        s.0
    }
}

impl FromStr for ObjectAttributes {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type ObjectAttributesList = List<ObjectAttributes>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectCannedACL(Cow<'static, str>);

impl ObjectCannedACL {
    pub const AUTHENTICATED_READ: &str = "authenticated-read";

    pub const AWS_EXEC_READ: &str = "aws-exec-read";

    pub const BUCKET_OWNER_FULL_CONTROL: &str = "bucket-owner-full-control";

    pub const BUCKET_OWNER_READ: &str = "bucket-owner-read";

    pub const PRIVATE: &str = "private";

    pub const PUBLIC_READ: &str = "public-read";

    pub const PUBLIC_READ_WRITE: &str = "public-read-write";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ObjectCannedACL {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ObjectCannedACL> for Cow<'static, str> {
    fn from(s: ObjectCannedACL) -> Self {
        s.0
    }
}

impl FromStr for ObjectCannedACL {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>Object Identifier is unique value to identify objects.</p>
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

impl fmt::Debug for ObjectIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ObjectIdentifier");
        d.field("key", &self.key);
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type ObjectIdentifierList = List<ObjectIdentifier>;

pub type ObjectKey = String;

pub type ObjectList = List<Object>;

/// <p>The container element for Object Lock configuration parameters.</p>
#[derive(Default)]
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

impl fmt::Debug for ObjectLockConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ObjectLockConfiguration");
        if let Some(ref val) = self.object_lock_enabled {
            d.field("object_lock_enabled", val);
        }
        if let Some(ref val) = self.rule {
            d.field("rule", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLockEnabled(Cow<'static, str>);

impl ObjectLockEnabled {
    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ObjectLockEnabled {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ObjectLockEnabled> for Cow<'static, str> {
    fn from(s: ObjectLockEnabled) -> Self {
        s.0
    }
}

impl FromStr for ObjectLockEnabled {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type ObjectLockEnabledForBucket = bool;

/// <p>A legal hold configuration for an object.</p>
#[derive(Default)]
pub struct ObjectLockLegalHold {
    /// <p>Indicates whether the specified object has a legal hold in place.</p>
    pub status: Option<ObjectLockLegalHoldStatus>,
}

impl fmt::Debug for ObjectLockLegalHold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ObjectLockLegalHold");
        if let Some(ref val) = self.status {
            d.field("status", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLockLegalHoldStatus(Cow<'static, str>);

impl ObjectLockLegalHoldStatus {
    pub const OFF: &str = "OFF";

    pub const ON: &str = "ON";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ObjectLockLegalHoldStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ObjectLockLegalHoldStatus> for Cow<'static, str> {
    fn from(s: ObjectLockLegalHoldStatus) -> Self {
        s.0
    }
}

impl FromStr for ObjectLockLegalHoldStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLockMode(Cow<'static, str>);

impl ObjectLockMode {
    pub const COMPLIANCE: &str = "COMPLIANCE";

    pub const GOVERNANCE: &str = "GOVERNANCE";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ObjectLockMode {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ObjectLockMode> for Cow<'static, str> {
    fn from(s: ObjectLockMode) -> Self {
        s.0
    }
}

impl FromStr for ObjectLockMode {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type ObjectLockRetainUntilDate = Timestamp;

/// <p>A Retention configuration for an object.</p>
#[derive(Default)]
pub struct ObjectLockRetention {
    /// <p>Indicates the Retention mode for the specified object.</p>
    pub mode: Option<ObjectLockRetentionMode>,
    /// <p>The date on which this Object Lock Retention will expire.</p>
    pub retain_until_date: Option<Date>,
}

impl fmt::Debug for ObjectLockRetention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ObjectLockRetention");
        if let Some(ref val) = self.mode {
            d.field("mode", val);
        }
        if let Some(ref val) = self.retain_until_date {
            d.field("retain_until_date", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLockRetentionMode(Cow<'static, str>);

impl ObjectLockRetentionMode {
    pub const COMPLIANCE: &str = "COMPLIANCE";

    pub const GOVERNANCE: &str = "GOVERNANCE";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ObjectLockRetentionMode {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ObjectLockRetentionMode> for Cow<'static, str> {
    fn from(s: ObjectLockRetentionMode) -> Self {
        s.0
    }
}

impl FromStr for ObjectLockRetentionMode {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>The container element for an Object Lock rule.</p>
#[derive(Default)]
pub struct ObjectLockRule {
    /// <p>The default Object Lock retention mode and period that you want to apply to new objects
    /// placed in the specified bucket. Bucket settings require both a mode and a period.
    /// The period can be either <code>Days</code> or <code>Years</code> but you must select one.
    /// You cannot specify <code>Days</code> and <code>Years</code> at the same time.</p>
    pub default_retention: Option<DefaultRetention>,
}

impl fmt::Debug for ObjectLockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ObjectLockRule");
        if let Some(ref val) = self.default_retention {
            d.field("default_retention", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type ObjectLockToken = String;

/// <p>The source object of the COPY action is not in the active tier and is only stored in
/// Amazon S3 Glacier.</p>
#[derive(Default)]
pub struct ObjectNotInActiveTierError {}

impl fmt::Debug for ObjectNotInActiveTierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ObjectNotInActiveTierError");
        d.finish_non_exhaustive()
    }
}

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectOwnership(Cow<'static, str>);

impl ObjectOwnership {
    pub const BUCKET_OWNER_ENFORCED: &str = "BucketOwnerEnforced";

    pub const BUCKET_OWNER_PREFERRED: &str = "BucketOwnerPreferred";

    pub const OBJECT_WRITER: &str = "ObjectWriter";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ObjectOwnership {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ObjectOwnership> for Cow<'static, str> {
    fn from(s: ObjectOwnership) -> Self {
        s.0
    }
}

impl FromStr for ObjectOwnership {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>A container for elements related to an individual part.</p>
#[derive(Default)]
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

impl fmt::Debug for ObjectPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ObjectPart");
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        d.field("part_number", &self.part_number);
        d.field("size", &self.size);
        d.finish_non_exhaustive()
    }
}

pub type ObjectSize = i64;

pub type ObjectSizeGreaterThanBytes = i64;

pub type ObjectSizeLessThanBytes = i64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStorageClass(Cow<'static, str>);

impl ObjectStorageClass {
    pub const DEEP_ARCHIVE: &str = "DEEP_ARCHIVE";

    pub const GLACIER: &str = "GLACIER";

    pub const GLACIER_IR: &str = "GLACIER_IR";

    pub const INTELLIGENT_TIERING: &str = "INTELLIGENT_TIERING";

    pub const ONEZONE_IA: &str = "ONEZONE_IA";

    pub const OUTPOSTS: &str = "OUTPOSTS";

    pub const REDUCED_REDUNDANCY: &str = "REDUCED_REDUNDANCY";

    pub const STANDARD: &str = "STANDARD";

    pub const STANDARD_IA: &str = "STANDARD_IA";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ObjectStorageClass {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ObjectStorageClass> for Cow<'static, str> {
    fn from(s: ObjectStorageClass) -> Self {
        s.0
    }
}

impl FromStr for ObjectStorageClass {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>The version of an object.</p>
#[derive(Default)]
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

impl fmt::Debug for ObjectVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ObjectVersion");
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        d.field("is_latest", &self.is_latest);
        if let Some(ref val) = self.key {
            d.field("key", val);
        }
        if let Some(ref val) = self.last_modified {
            d.field("last_modified", val);
        }
        if let Some(ref val) = self.owner {
            d.field("owner", val);
        }
        d.field("size", &self.size);
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type ObjectVersionId = String;

pub type ObjectVersionList = List<ObjectVersion>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectVersionStorageClass(Cow<'static, str>);

impl ObjectVersionStorageClass {
    pub const STANDARD: &str = "STANDARD";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ObjectVersionStorageClass {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ObjectVersionStorageClass> for Cow<'static, str> {
    fn from(s: ObjectVersionStorageClass) -> Self {
        s.0
    }
}

impl FromStr for ObjectVersionStorageClass {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>Describes the location where the restore job's output is stored.</p>
#[derive(Default)]
pub struct OutputLocation {
    /// <p>Describes an S3 location that will receive the results of the restore request.</p>
    pub s3: Option<S3Location>,
}

impl fmt::Debug for OutputLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("OutputLocation");
        if let Some(ref val) = self.s3 {
            d.field("s3", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Describes how results of the Select job are serialized.</p>
#[derive(Default)]
pub struct OutputSerialization {
    /// <p>Describes the serialization of CSV-encoded Select results.</p>
    pub csv: Option<CSVOutput>,
    /// <p>Specifies JSON as request's output serialization format.</p>
    pub json: Option<JSONOutput>,
}

impl fmt::Debug for OutputSerialization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("OutputSerialization");
        if let Some(ref val) = self.csv {
            d.field("csv", val);
        }
        if let Some(ref val) = self.json {
            d.field("json", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Container for the owner's display name and ID.</p>
#[derive(Default)]
pub struct Owner {
    /// <p>Container for the display name of the owner.</p>
    pub display_name: Option<DisplayName>,
    /// <p>Container for the ID of the owner.</p>
    pub id: Option<ID>,
}

impl fmt::Debug for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Owner");
        if let Some(ref val) = self.display_name {
            d.field("display_name", val);
        }
        if let Some(ref val) = self.id {
            d.field("id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerOverride(Cow<'static, str>);

impl OwnerOverride {
    pub const DESTINATION: &str = "Destination";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for OwnerOverride {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<OwnerOverride> for Cow<'static, str> {
    fn from(s: OwnerOverride) -> Self {
        s.0
    }
}

impl FromStr for OwnerOverride {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>The container element for a bucket's ownership controls.</p>
pub struct OwnershipControls {
    /// <p>The container element for an ownership control rule.</p>
    pub rules: OwnershipControlsRules,
}

impl fmt::Debug for OwnershipControls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("OwnershipControls");
        d.field("rules", &self.rules);
        d.finish_non_exhaustive()
    }
}

/// <p>The container element for an ownership control rule.</p>
pub struct OwnershipControlsRule {
    pub object_ownership: ObjectOwnership,
}

impl fmt::Debug for OwnershipControlsRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("OwnershipControlsRule");
        d.field("object_ownership", &self.object_ownership);
        d.finish_non_exhaustive()
    }
}

pub type OwnershipControlsRules = List<OwnershipControlsRule>;

/// <p>Container for Parquet.</p>
#[derive(Default)]
pub struct ParquetInput {}

impl fmt::Debug for ParquetInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ParquetInput");
        d.finish_non_exhaustive()
    }
}

/// <p>Container for elements related to a part.</p>
#[derive(Default)]
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

impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Part");
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        if let Some(ref val) = self.last_modified {
            d.field("last_modified", val);
        }
        d.field("part_number", &self.part_number);
        d.field("size", &self.size);
        d.finish_non_exhaustive()
    }
}

pub type PartNumber = i32;

pub type PartNumberMarker = String;

pub type Parts = List<Part>;

pub type PartsCount = i32;

pub type PartsList = List<ObjectPart>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payer(Cow<'static, str>);

impl Payer {
    pub const BUCKET_OWNER: &str = "BucketOwner";

    pub const REQUESTER: &str = "Requester";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for Payer {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<Payer> for Cow<'static, str> {
    fn from(s: Payer) -> Self {
        s.0
    }
}

impl FromStr for Payer {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permission(Cow<'static, str>);

impl Permission {
    pub const FULL_CONTROL: &str = "FULL_CONTROL";

    pub const READ: &str = "READ";

    pub const READ_ACP: &str = "READ_ACP";

    pub const WRITE: &str = "WRITE";

    pub const WRITE_ACP: &str = "WRITE_ACP";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for Permission {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<Permission> for Cow<'static, str> {
    fn from(s: Permission) -> Self {
        s.0
    }
}

impl FromStr for Permission {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type Policy = String;

/// <p>The container element for a bucket's policy status.</p>
#[derive(Default)]
pub struct PolicyStatus {
    /// <p>The policy status for this bucket. <code>TRUE</code> indicates that this bucket is
    /// public. <code>FALSE</code> indicates that the bucket is not public.</p>
    pub is_public: IsPublic,
}

impl fmt::Debug for PolicyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PolicyStatus");
        d.field("is_public", &self.is_public);
        d.finish_non_exhaustive()
    }
}

pub type Prefix = String;

pub type Priority = i32;

/// <p>This data type contains information about progress of an operation.</p>
#[derive(Default)]
pub struct Progress {
    /// <p>The current number of uncompressed object bytes processed.</p>
    pub bytes_processed: BytesProcessed,
    /// <p>The current number of bytes of records payload data returned.</p>
    pub bytes_returned: BytesReturned,
    /// <p>The current number of object bytes scanned.</p>
    pub bytes_scanned: BytesScanned,
}

impl fmt::Debug for Progress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Progress");
        d.field("bytes_processed", &self.bytes_processed);
        d.field("bytes_returned", &self.bytes_returned);
        d.field("bytes_scanned", &self.bytes_scanned);
        d.finish_non_exhaustive()
    }
}

/// <p>This data type contains information about the progress event of an operation.</p>
#[derive(Default)]
pub struct ProgressEvent {
    /// <p>The Progress event details.</p>
    pub details: Option<Progress>,
}

impl fmt::Debug for ProgressEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ProgressEvent");
        if let Some(ref val) = self.details {
            d.field("details", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Protocol(Cow<'static, str>);

impl Protocol {
    pub const HTTP: &str = "http";

    pub const HTTPS: &str = "https";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for Protocol {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<Protocol> for Cow<'static, str> {
    fn from(s: Protocol) -> Self {
        s.0
    }
}

impl FromStr for Protocol {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>The PublicAccessBlock configuration that you want to apply to this Amazon S3 bucket. You can
/// enable the configuration options in any combination. For more information about when Amazon S3
/// considers a bucket or object public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a> in the <i>Amazon S3 User Guide</i>. </p>
#[derive(Default)]
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

impl fmt::Debug for PublicAccessBlockConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PublicAccessBlockConfiguration");
        d.field("block_public_acls", &self.block_public_acls);
        d.field("block_public_policy", &self.block_public_policy);
        d.field("ignore_public_acls", &self.ignore_public_acls);
        d.field("restrict_public_buckets", &self.restrict_public_buckets);
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketAccelerateConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketAccelerateConfigurationInput");
        d.field("accelerate_configuration", &self.accelerate_configuration);
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketAccelerateConfigurationOutput {}

impl fmt::Debug for PutBucketAccelerateConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketAccelerateConfigurationOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketAclInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketAclInput");
        if let Some(ref val) = self.acl {
            d.field("acl", val);
        }
        if let Some(ref val) = self.access_control_policy {
            d.field("access_control_policy", val);
        }
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.grant_full_control {
            d.field("grant_full_control", val);
        }
        if let Some(ref val) = self.grant_read {
            d.field("grant_read", val);
        }
        if let Some(ref val) = self.grant_read_acp {
            d.field("grant_read_acp", val);
        }
        if let Some(ref val) = self.grant_write {
            d.field("grant_write", val);
        }
        if let Some(ref val) = self.grant_write_acp {
            d.field("grant_write_acp", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketAclOutput {}

impl fmt::Debug for PutBucketAclOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketAclOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketAnalyticsConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketAnalyticsConfigurationInput");
        d.field("analytics_configuration", &self.analytics_configuration);
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("id", &self.id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketAnalyticsConfigurationOutput {}

impl fmt::Debug for PutBucketAnalyticsConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketAnalyticsConfigurationOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketCorsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketCorsInput");
        d.field("bucket", &self.bucket);
        d.field("cors_configuration", &self.cors_configuration);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketCorsOutput {}

impl fmt::Debug for PutBucketCorsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketCorsOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketEncryptionInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketEncryptionInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("server_side_encryption_configuration", &self.server_side_encryption_configuration);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketEncryptionOutput {}

impl fmt::Debug for PutBucketEncryptionOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketEncryptionOutput");
        d.finish_non_exhaustive()
    }
}

pub struct PutBucketIntelligentTieringConfigurationInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub bucket: BucketName,
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub id: IntelligentTieringId,
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub intelligent_tiering_configuration: IntelligentTieringConfiguration,
}

impl fmt::Debug for PutBucketIntelligentTieringConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketIntelligentTieringConfigurationInput");
        d.field("bucket", &self.bucket);
        d.field("id", &self.id);
        d.field("intelligent_tiering_configuration", &self.intelligent_tiering_configuration);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketIntelligentTieringConfigurationOutput {}

impl fmt::Debug for PutBucketIntelligentTieringConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketIntelligentTieringConfigurationOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketInventoryConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketInventoryConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("id", &self.id);
        d.field("inventory_configuration", &self.inventory_configuration);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketInventoryConfigurationOutput {}

impl fmt::Debug for PutBucketInventoryConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketInventoryConfigurationOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketLifecycleConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketLifecycleConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.lifecycle_configuration {
            d.field("lifecycle_configuration", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketLifecycleConfigurationOutput {}

impl fmt::Debug for PutBucketLifecycleConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketLifecycleConfigurationOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketLoggingInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketLoggingInput");
        d.field("bucket", &self.bucket);
        d.field("bucket_logging_status", &self.bucket_logging_status);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketLoggingOutput {}

impl fmt::Debug for PutBucketLoggingOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketLoggingOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketMetricsConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketMetricsConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("id", &self.id);
        d.field("metrics_configuration", &self.metrics_configuration);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketMetricsConfigurationOutput {}

impl fmt::Debug for PutBucketMetricsConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketMetricsConfigurationOutput");
        d.finish_non_exhaustive()
    }
}

pub struct PutBucketNotificationConfigurationInput {
    /// <p>The name of the bucket.</p>
    pub bucket: BucketName,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: Option<AccountId>,
    pub notification_configuration: NotificationConfiguration,
    /// <p>Skips validation of Amazon SQS, Amazon SNS, and Lambda destinations. True or false value.</p>
    pub skip_destination_validation: SkipValidation,
}

impl fmt::Debug for PutBucketNotificationConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketNotificationConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("notification_configuration", &self.notification_configuration);
        d.field("skip_destination_validation", &self.skip_destination_validation);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketNotificationConfigurationOutput {}

impl fmt::Debug for PutBucketNotificationConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketNotificationConfigurationOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketOwnershipControlsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketOwnershipControlsInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("ownership_controls", &self.ownership_controls);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketOwnershipControlsOutput {}

impl fmt::Debug for PutBucketOwnershipControlsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketOwnershipControlsOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketPolicyInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketPolicyInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        d.field("confirm_remove_self_bucket_access", &self.confirm_remove_self_bucket_access);
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("policy", &self.policy);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketPolicyOutput {}

impl fmt::Debug for PutBucketPolicyOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketPolicyOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketReplicationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketReplicationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("replication_configuration", &self.replication_configuration);
        if let Some(ref val) = self.token {
            d.field("token", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketReplicationOutput {}

impl fmt::Debug for PutBucketReplicationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketReplicationOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketRequestPaymentInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketRequestPaymentInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("request_payment_configuration", &self.request_payment_configuration);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketRequestPaymentOutput {}

impl fmt::Debug for PutBucketRequestPaymentOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketRequestPaymentOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketTaggingInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketTaggingInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("tagging", &self.tagging);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketTaggingOutput {}

impl fmt::Debug for PutBucketTaggingOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketTaggingOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketVersioningInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketVersioningInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.mfa {
            d.field("mfa", val);
        }
        d.field("versioning_configuration", &self.versioning_configuration);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketVersioningOutput {}

impl fmt::Debug for PutBucketVersioningOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketVersioningOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutBucketWebsiteInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketWebsiteInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("website_configuration", &self.website_configuration);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutBucketWebsiteOutput {}

impl fmt::Debug for PutBucketWebsiteOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutBucketWebsiteOutput");
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutObjectAclInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectAclInput");
        if let Some(ref val) = self.acl {
            d.field("acl", val);
        }
        if let Some(ref val) = self.access_control_policy {
            d.field("access_control_policy", val);
        }
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.grant_full_control {
            d.field("grant_full_control", val);
        }
        if let Some(ref val) = self.grant_read {
            d.field("grant_read", val);
        }
        if let Some(ref val) = self.grant_read_acp {
            d.field("grant_read_acp", val);
        }
        if let Some(ref val) = self.grant_write {
            d.field("grant_write", val);
        }
        if let Some(ref val) = self.grant_write_acp {
            d.field("grant_write_acp", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutObjectAclOutput {
    pub request_charged: Option<RequestCharged>,
}

impl fmt::Debug for PutObjectAclOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectAclOutput");
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutObjectInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectInput");
        if let Some(ref val) = self.acl {
            d.field("acl", val);
        }
        if let Some(ref val) = self.body {
            d.field("body", val);
        }
        d.field("bucket", &self.bucket);
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.cache_control {
            d.field("cache_control", val);
        }
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.content_disposition {
            d.field("content_disposition", val);
        }
        if let Some(ref val) = self.content_encoding {
            d.field("content_encoding", val);
        }
        if let Some(ref val) = self.content_language {
            d.field("content_language", val);
        }
        d.field("content_length", &self.content_length);
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.content_type {
            d.field("content_type", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.expires {
            d.field("expires", val);
        }
        if let Some(ref val) = self.grant_full_control {
            d.field("grant_full_control", val);
        }
        if let Some(ref val) = self.grant_read {
            d.field("grant_read", val);
        }
        if let Some(ref val) = self.grant_read_acp {
            d.field("grant_read_acp", val);
        }
        if let Some(ref val) = self.grant_write_acp {
            d.field("grant_write_acp", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.metadata {
            d.field("metadata", val);
        }
        if let Some(ref val) = self.object_lock_legal_hold_status {
            d.field("object_lock_legal_hold_status", val);
        }
        if let Some(ref val) = self.object_lock_mode {
            d.field("object_lock_mode", val);
        }
        if let Some(ref val) = self.object_lock_retain_until_date {
            d.field("object_lock_retain_until_date", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key {
            d.field("sse_customer_key", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.ssekms_encryption_context {
            d.field("ssekms_encryption_context", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        if let Some(ref val) = self.tagging {
            d.field("tagging", val);
        }
        if let Some(ref val) = self.website_redirect_location {
            d.field("website_redirect_location", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutObjectLegalHoldInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectLegalHoldInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.legal_hold {
            d.field("legal_hold", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutObjectLegalHoldOutput {
    pub request_charged: Option<RequestCharged>,
}

impl fmt::Debug for PutObjectLegalHoldOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectLegalHoldOutput");
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutObjectLockConfigurationInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectLockConfigurationInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.object_lock_configuration {
            d.field("object_lock_configuration", val);
        }
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.token {
            d.field("token", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutObjectLockConfigurationOutput {
    pub request_charged: Option<RequestCharged>,
}

impl fmt::Debug for PutObjectLockConfigurationOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectLockConfigurationOutput");
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for PutObjectOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectOutput");
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        if let Some(ref val) = self.expiration {
            d.field("expiration", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.ssekms_encryption_context {
            d.field("ssekms_encryption_context", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutObjectRetentionInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectRetentionInput");
        d.field("bucket", &self.bucket);
        d.field("bypass_governance_retention", &self.bypass_governance_retention);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.retention {
            d.field("retention", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutObjectRetentionOutput {
    pub request_charged: Option<RequestCharged>,
}

impl fmt::Debug for PutObjectRetentionOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectRetentionOutput");
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutObjectTaggingInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectTaggingInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        d.field("tagging", &self.tagging);
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutObjectTaggingOutput {
    /// <p>The versionId of the object the tag-set was added to.</p>
    pub version_id: Option<ObjectVersionId>,
}

impl fmt::Debug for PutObjectTaggingOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutObjectTaggingOutput");
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for PutPublicAccessBlockInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutPublicAccessBlockInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("public_access_block_configuration", &self.public_access_block_configuration);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct PutPublicAccessBlockOutput {}

impl fmt::Debug for PutPublicAccessBlockOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("PutPublicAccessBlockOutput");
        d.finish_non_exhaustive()
    }
}

pub type QueueArn = String;

/// <p>Specifies the configuration for publishing messages to an Amazon Simple Queue Service
/// (Amazon SQS) queue when Amazon S3 detects specified events.</p>
pub struct QueueConfiguration {
    /// <p>A collection of bucket events for which to send notifications</p>
    pub events: EventList,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<NotificationId>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SQS queue to which Amazon S3 publishes a message
    /// when it detects events of the specified type.</p>
    pub queue_arn: QueueArn,
}

impl fmt::Debug for QueueConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("QueueConfiguration");
        d.field("events", &self.events);
        if let Some(ref val) = self.filter {
            d.field("filter", val);
        }
        if let Some(ref val) = self.id {
            d.field("id", val);
        }
        d.field("queue_arn", &self.queue_arn);
        d.finish_non_exhaustive()
    }
}

pub type QueueConfigurationList = List<QueueConfiguration>;

pub type Quiet = bool;

pub type QuoteCharacter = String;

pub type QuoteEscapeCharacter = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuoteFields(Cow<'static, str>);

impl QuoteFields {
    pub const ALWAYS: &str = "ALWAYS";

    pub const ASNEEDED: &str = "ASNEEDED";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for QuoteFields {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<QuoteFields> for Cow<'static, str> {
    fn from(s: QuoteFields) -> Self {
        s.0
    }
}

impl FromStr for QuoteFields {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type RecordDelimiter = String;

/// <p>The container for the records event.</p>
#[derive(Default)]
pub struct RecordsEvent {
    /// <p>The byte array of partial, one or more result records.</p>
    pub payload: Option<Body>,
}

impl fmt::Debug for RecordsEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("RecordsEvent");
        if let Some(ref val) = self.payload {
            d.field("payload", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies how requests are redirected. In the event of an error, you can specify a
/// different error code to return.</p>
#[derive(Default)]
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

impl fmt::Debug for Redirect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Redirect");
        if let Some(ref val) = self.host_name {
            d.field("host_name", val);
        }
        if let Some(ref val) = self.http_redirect_code {
            d.field("http_redirect_code", val);
        }
        if let Some(ref val) = self.protocol {
            d.field("protocol", val);
        }
        if let Some(ref val) = self.replace_key_prefix_with {
            d.field("replace_key_prefix_with", val);
        }
        if let Some(ref val) = self.replace_key_with {
            d.field("replace_key_with", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies the redirect behavior of all requests to a website endpoint of an Amazon S3
/// bucket.</p>
pub struct RedirectAllRequestsTo {
    /// <p>Name of the host where requests are redirected.</p>
    pub host_name: HostName,
    /// <p>Protocol to use when redirecting requests. The default is the protocol that is used in
    /// the original request.</p>
    pub protocol: Option<Protocol>,
}

impl fmt::Debug for RedirectAllRequestsTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("RedirectAllRequestsTo");
        d.field("host_name", &self.host_name);
        if let Some(ref val) = self.protocol {
            d.field("protocol", val);
        }
        d.finish_non_exhaustive()
    }
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
pub struct ReplicaModifications {
    /// <p>Specifies whether Amazon S3 replicates modifications on replicas.</p>
    pub status: ReplicaModificationsStatus,
}

impl fmt::Debug for ReplicaModifications {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ReplicaModifications");
        d.field("status", &self.status);
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicaModificationsStatus(Cow<'static, str>);

impl ReplicaModificationsStatus {
    pub const DISABLED: &str = "Disabled";

    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ReplicaModificationsStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ReplicaModificationsStatus> for Cow<'static, str> {
    fn from(s: ReplicaModificationsStatus) -> Self {
        s.0
    }
}

impl FromStr for ReplicaModificationsStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>A container for replication rules. You can add up to 1,000 rules. The maximum size of a
/// replication configuration is 2 MB.</p>
pub struct ReplicationConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role that
    /// Amazon S3 assumes when replicating objects. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-how-setup.html">How to Set Up
    /// Replication</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub role: Role,
    /// <p>A container for one or more replication rules. A replication configuration must have at
    /// least one rule and can contain a maximum of 1,000 rules. </p>
    pub rules: ReplicationRules,
}

impl fmt::Debug for ReplicationConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ReplicationConfiguration");
        d.field("role", &self.role);
        d.field("rules", &self.rules);
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies which Amazon S3 objects to replicate and where to store the replicas.</p>
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

impl fmt::Debug for ReplicationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ReplicationRule");
        if let Some(ref val) = self.delete_marker_replication {
            d.field("delete_marker_replication", val);
        }
        d.field("destination", &self.destination);
        if let Some(ref val) = self.existing_object_replication {
            d.field("existing_object_replication", val);
        }
        if let Some(ref val) = self.filter {
            d.field("filter", val);
        }
        if let Some(ref val) = self.id {
            d.field("id", val);
        }
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        d.field("priority", &self.priority);
        if let Some(ref val) = self.source_selection_criteria {
            d.field("source_selection_criteria", val);
        }
        d.field("status", &self.status);
        d.finish_non_exhaustive()
    }
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
#[derive(Default)]
pub struct ReplicationRuleAndOperator {
    /// <p>An object key name prefix that identifies the subset of objects to which the rule
    /// applies.</p>
    pub prefix: Option<Prefix>,
    /// <p>An array of tags containing key and value pairs.</p>
    pub tags: Option<TagSet>,
}

impl fmt::Debug for ReplicationRuleAndOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ReplicationRuleAndOperator");
        if let Some(ref val) = self.prefix {
            d.field("prefix", val);
        }
        if let Some(ref val) = self.tags {
            d.field("tags", val);
        }
        d.finish_non_exhaustive()
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicationRuleStatus(Cow<'static, str>);

impl ReplicationRuleStatus {
    pub const DISABLED: &str = "Disabled";

    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ReplicationRuleStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ReplicationRuleStatus> for Cow<'static, str> {
    fn from(s: ReplicationRuleStatus) -> Self {
        s.0
    }
}

impl FromStr for ReplicationRuleStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type ReplicationRules = List<ReplicationRule>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicationStatus(Cow<'static, str>);

impl ReplicationStatus {
    pub const COMPLETE: &str = "COMPLETE";

    pub const FAILED: &str = "FAILED";

    pub const PENDING: &str = "PENDING";

    pub const REPLICA: &str = "REPLICA";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ReplicationStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ReplicationStatus> for Cow<'static, str> {
    fn from(s: ReplicationStatus) -> Self {
        s.0
    }
}

impl FromStr for ReplicationStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p> A container specifying S3 Replication Time Control (S3 RTC) related information, including whether S3 RTC is
/// enabled and the time when all objects and operations on objects must be replicated. Must be
/// specified together with a <code>Metrics</code> block. </p>
pub struct ReplicationTime {
    /// <p> Specifies whether the replication time is enabled. </p>
    pub status: ReplicationTimeStatus,
    /// <p> A container specifying the time by which replication should be complete for all objects
    /// and operations on objects. </p>
    pub time: ReplicationTimeValue,
}

impl fmt::Debug for ReplicationTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ReplicationTime");
        d.field("status", &self.status);
        d.field("time", &self.time);
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicationTimeStatus(Cow<'static, str>);

impl ReplicationTimeStatus {
    pub const DISABLED: &str = "Disabled";

    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ReplicationTimeStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ReplicationTimeStatus> for Cow<'static, str> {
    fn from(s: ReplicationTimeStatus) -> Self {
        s.0
    }
}

impl FromStr for ReplicationTimeStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p> A container specifying the time value for S3 Replication Time Control (S3 RTC) and replication metrics
/// <code>EventThreshold</code>. </p>
#[derive(Default)]
pub struct ReplicationTimeValue {
    /// <p> Contains an integer specifying time in minutes. </p>
    /// <p> Valid value: 15</p>
    pub minutes: Minutes,
}

impl fmt::Debug for ReplicationTimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ReplicationTimeValue");
        d.field("minutes", &self.minutes);
        d.finish_non_exhaustive()
    }
}

/// <p>If present, indicates that the requester was successfully charged for the
/// request.</p>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestCharged(Cow<'static, str>);

impl RequestCharged {
    pub const REQUESTER: &str = "requester";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for RequestCharged {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<RequestCharged> for Cow<'static, str> {
    fn from(s: RequestCharged) -> Self {
        s.0
    }
}

impl FromStr for RequestCharged {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>Confirms that the requester knows that they will be charged for the request. Bucket
/// owners need not specify this parameter in their requests. For information about downloading
/// objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in
/// Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestPayer(Cow<'static, str>);

impl RequestPayer {
    pub const REQUESTER: &str = "requester";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for RequestPayer {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<RequestPayer> for Cow<'static, str> {
    fn from(s: RequestPayer) -> Self {
        s.0
    }
}

impl FromStr for RequestPayer {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>Container for Payer.</p>
pub struct RequestPaymentConfiguration {
    /// <p>Specifies who pays for the download and request fees.</p>
    pub payer: Payer,
}

impl fmt::Debug for RequestPaymentConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("RequestPaymentConfiguration");
        d.field("payer", &self.payer);
        d.finish_non_exhaustive()
    }
}

/// <p>Container for specifying if periodic <code>QueryProgress</code> messages should be
/// sent.</p>
#[derive(Default)]
pub struct RequestProgress {
    /// <p>Specifies whether periodic QueryProgress frames should be sent. Valid values: TRUE,
    /// FALSE. Default value: FALSE.</p>
    pub enabled: EnableRequestProgress,
}

impl fmt::Debug for RequestProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("RequestProgress");
        d.field("enabled", &self.enabled);
        d.finish_non_exhaustive()
    }
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

impl fmt::Debug for RestoreObjectInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("RestoreObjectInput");
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.restore_request {
            d.field("restore_request", val);
        }
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct RestoreObjectOutput {
    pub request_charged: Option<RequestCharged>,
    /// <p>Indicates the path in the provided S3 output location where Select results will be
    /// restored to.</p>
    pub restore_output_path: Option<RestoreOutputPath>,
}

impl fmt::Debug for RestoreObjectOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("RestoreObjectOutput");
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.restore_output_path {
            d.field("restore_output_path", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type RestoreOutputPath = String;

/// <p>Container for restore job parameters.</p>
#[derive(Default)]
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

impl fmt::Debug for RestoreRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("RestoreRequest");
        d.field("days", &self.days);
        if let Some(ref val) = self.description {
            d.field("description", val);
        }
        if let Some(ref val) = self.glacier_job_parameters {
            d.field("glacier_job_parameters", val);
        }
        if let Some(ref val) = self.output_location {
            d.field("output_location", val);
        }
        if let Some(ref val) = self.select_parameters {
            d.field("select_parameters", val);
        }
        if let Some(ref val) = self.tier {
            d.field("tier", val);
        }
        if let Some(ref val) = self.type_ {
            d.field("type_", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreRequestType(Cow<'static, str>);

impl RestoreRequestType {
    pub const SELECT: &str = "SELECT";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for RestoreRequestType {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<RestoreRequestType> for Cow<'static, str> {
    fn from(s: RestoreRequestType) -> Self {
        s.0
    }
}

impl FromStr for RestoreRequestType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type Role = String;

/// <p>Specifies the redirect behavior and when a redirect is applied. For more information
/// about routing rules, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html#advanced-conditional-redirects">Configuring advanced conditional redirects</a> in the
/// <i>Amazon S3 User Guide</i>.</p>
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

impl fmt::Debug for RoutingRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("RoutingRule");
        if let Some(ref val) = self.condition {
            d.field("condition", val);
        }
        d.field("redirect", &self.redirect);
        d.finish_non_exhaustive()
    }
}

pub type RoutingRules = List<RoutingRule>;

/// <p>A container for object key name prefix and suffix filtering rules.</p>
#[derive(Default)]
pub struct S3KeyFilter {
    pub filter_rules: Option<FilterRuleList>,
}

impl fmt::Debug for S3KeyFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("S3KeyFilter");
        if let Some(ref val) = self.filter_rules {
            d.field("filter_rules", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Describes an Amazon S3 location that will receive the results of the restore request.</p>
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

impl fmt::Debug for S3Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("S3Location");
        if let Some(ref val) = self.access_control_list {
            d.field("access_control_list", val);
        }
        d.field("bucket_name", &self.bucket_name);
        if let Some(ref val) = self.canned_acl {
            d.field("canned_acl", val);
        }
        if let Some(ref val) = self.encryption {
            d.field("encryption", val);
        }
        d.field("prefix", &self.prefix);
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        if let Some(ref val) = self.tagging {
            d.field("tagging", val);
        }
        if let Some(ref val) = self.user_metadata {
            d.field("user_metadata", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type SSECustomerAlgorithm = String;

pub type SSECustomerKey = String;

pub type SSECustomerKeyMD5 = String;

/// <p>Specifies the use of SSE-KMS to encrypt delivered inventory reports.</p>
pub struct SSEKMS {
    /// <p>Specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key
    /// to use for encrypting inventory reports.</p>
    pub key_id: SSEKMSKeyId,
}

impl fmt::Debug for SSEKMS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("SSEKMS");
        d.field("key_id", &self.key_id);
        d.finish_non_exhaustive()
    }
}

pub type SSEKMSEncryptionContext = String;

pub type SSEKMSKeyId = String;

/// <p>Specifies the use of SSE-S3 to encrypt delivered inventory reports.</p>
#[derive(Default)]
pub struct SSES3 {}

impl fmt::Debug for SSES3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("SSES3");
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies the byte range of the object to get the records from. A record is processed
/// when its first byte is contained by the range. This parameter is optional, but when
/// specified, it must not be empty. See RFC 2616, Section 14.35.1 about how to specify the
/// start and end of the range.</p>
#[derive(Default)]
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

impl fmt::Debug for ScanRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ScanRange");
        d.field("end", &self.end);
        d.field("start", &self.start);
        d.finish_non_exhaustive()
    }
}

/// <p>Describes the parameters for Select job types.</p>
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

impl fmt::Debug for SelectParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("SelectParameters");
        d.field("expression", &self.expression);
        d.field("expression_type", &self.expression_type);
        d.field("input_serialization", &self.input_serialization);
        d.field("output_serialization", &self.output_serialization);
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerSideEncryption(Cow<'static, str>);

impl ServerSideEncryption {
    pub const AES256: &str = "AES256";

    pub const AWS_KMS: &str = "aws:kms";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for ServerSideEncryption {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<ServerSideEncryption> for Cow<'static, str> {
    fn from(s: ServerSideEncryption) -> Self {
        s.0
    }
}

impl FromStr for ServerSideEncryption {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>Describes the default server-side encryption to apply to new objects in the bucket. If a
/// PUT Object request doesn't specify any server-side encryption, this default encryption will
/// be applied. If you don't specify a customer managed key at configuration, Amazon S3 automatically creates
/// an Amazon Web Services KMS key in your Amazon Web Services account the first time that you add an object encrypted with
/// SSE-KMS to a bucket. By default, Amazon S3 uses this KMS key for SSE-KMS. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTencryption.html">PUT Bucket encryption</a> in
/// the <i>Amazon S3 API Reference</i>.</p>
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

impl fmt::Debug for ServerSideEncryptionByDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ServerSideEncryptionByDefault");
        if let Some(ref val) = self.kms_master_key_id {
            d.field("kms_master_key_id", val);
        }
        d.field("sse_algorithm", &self.sse_algorithm);
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies the default server-side-encryption configuration.</p>
pub struct ServerSideEncryptionConfiguration {
    /// <p>Container for information about a particular server-side encryption configuration
    /// rule.</p>
    pub rules: ServerSideEncryptionRules,
}

impl fmt::Debug for ServerSideEncryptionConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ServerSideEncryptionConfiguration");
        d.field("rules", &self.rules);
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies the default server-side encryption configuration.</p>
#[derive(Default)]
pub struct ServerSideEncryptionRule {
    /// <p>Specifies the default server-side encryption to apply to new objects in the bucket. If a
    /// PUT Object request doesn't specify any server-side encryption, this default encryption will
    /// be applied.</p>
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
    /// <p>Specifies whether Amazon S3 should use an S3 Bucket Key with server-side encryption using KMS (SSE-KMS) for new objects in the bucket. Existing objects are not affected. Setting the <code>BucketKeyEnabled</code> element to <code>true</code> causes Amazon S3 to use an S3 Bucket Key. By default, S3 Bucket Key is not enabled.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html">Amazon S3 Bucket Keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket_key_enabled: BucketKeyEnabled,
}

impl fmt::Debug for ServerSideEncryptionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("ServerSideEncryptionRule");
        if let Some(ref val) = self.apply_server_side_encryption_by_default {
            d.field("apply_server_side_encryption_by_default", val);
        }
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        d.finish_non_exhaustive()
    }
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
#[derive(Default)]
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

impl fmt::Debug for SourceSelectionCriteria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("SourceSelectionCriteria");
        if let Some(ref val) = self.replica_modifications {
            d.field("replica_modifications", val);
        }
        if let Some(ref val) = self.sse_kms_encrypted_objects {
            d.field("sse_kms_encrypted_objects", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>A container for filter information for the selection of S3 objects encrypted with Amazon Web Services
/// KMS.</p>
pub struct SseKmsEncryptedObjects {
    /// <p>Specifies whether Amazon S3 replicates objects created with server-side encryption using an
    /// Amazon Web Services KMS key stored in Amazon Web Services Key Management Service.</p>
    pub status: SseKmsEncryptedObjectsStatus,
}

impl fmt::Debug for SseKmsEncryptedObjects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("SseKmsEncryptedObjects");
        d.field("status", &self.status);
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SseKmsEncryptedObjectsStatus(Cow<'static, str>);

impl SseKmsEncryptedObjectsStatus {
    pub const DISABLED: &str = "Disabled";

    pub const ENABLED: &str = "Enabled";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for SseKmsEncryptedObjectsStatus {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<SseKmsEncryptedObjectsStatus> for Cow<'static, str> {
    fn from(s: SseKmsEncryptedObjectsStatus) -> Self {
        s.0
    }
}

impl FromStr for SseKmsEncryptedObjectsStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type Start = i64;

pub type StartAfter = String;

/// <p>Container for the stats details.</p>
#[derive(Default)]
pub struct Stats {
    /// <p>The total number of uncompressed object bytes processed.</p>
    pub bytes_processed: BytesProcessed,
    /// <p>The total number of bytes of records payload data returned.</p>
    pub bytes_returned: BytesReturned,
    /// <p>The total number of object bytes scanned.</p>
    pub bytes_scanned: BytesScanned,
}

impl fmt::Debug for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Stats");
        d.field("bytes_processed", &self.bytes_processed);
        d.field("bytes_returned", &self.bytes_returned);
        d.field("bytes_scanned", &self.bytes_scanned);
        d.finish_non_exhaustive()
    }
}

/// <p>Container for the Stats Event.</p>
#[derive(Default)]
pub struct StatsEvent {
    /// <p>The Stats event details.</p>
    pub details: Option<Stats>,
}

impl fmt::Debug for StatsEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("StatsEvent");
        if let Some(ref val) = self.details {
            d.field("details", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageClass(Cow<'static, str>);

impl StorageClass {
    pub const DEEP_ARCHIVE: &str = "DEEP_ARCHIVE";

    pub const GLACIER: &str = "GLACIER";

    pub const GLACIER_IR: &str = "GLACIER_IR";

    pub const INTELLIGENT_TIERING: &str = "INTELLIGENT_TIERING";

    pub const ONEZONE_IA: &str = "ONEZONE_IA";

    pub const OUTPOSTS: &str = "OUTPOSTS";

    pub const REDUCED_REDUNDANCY: &str = "REDUCED_REDUNDANCY";

    pub const STANDARD: &str = "STANDARD";

    pub const STANDARD_IA: &str = "STANDARD_IA";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for StorageClass {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<StorageClass> for Cow<'static, str> {
    fn from(s: StorageClass) -> Self {
        s.0
    }
}

impl FromStr for StorageClass {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>Specifies data related to access patterns to be collected and made available to analyze
/// the tradeoffs between different storage classes for an Amazon S3 bucket.</p>
#[derive(Default)]
pub struct StorageClassAnalysis {
    /// <p>Specifies how data related to the storage class analysis for an Amazon S3 bucket should be
    /// exported.</p>
    pub data_export: Option<StorageClassAnalysisDataExport>,
}

impl fmt::Debug for StorageClassAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("StorageClassAnalysis");
        if let Some(ref val) = self.data_export {
            d.field("data_export", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Container for data related to the storage class analysis for an Amazon S3 bucket for
/// export.</p>
pub struct StorageClassAnalysisDataExport {
    /// <p>The place to store the data for an analysis.</p>
    pub destination: AnalyticsExportDestination,
    /// <p>The version of the output schema to use when exporting data. Must be
    /// <code>V_1</code>.</p>
    pub output_schema_version: StorageClassAnalysisSchemaVersion,
}

impl fmt::Debug for StorageClassAnalysisDataExport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("StorageClassAnalysisDataExport");
        d.field("destination", &self.destination);
        d.field("output_schema_version", &self.output_schema_version);
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageClassAnalysisSchemaVersion(Cow<'static, str>);

impl StorageClassAnalysisSchemaVersion {
    pub const V_1: &str = "V_1";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for StorageClassAnalysisSchemaVersion {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<StorageClassAnalysisSchemaVersion> for Cow<'static, str> {
    fn from(s: StorageClassAnalysisSchemaVersion) -> Self {
        s.0
    }
}

impl FromStr for StorageClassAnalysisSchemaVersion {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type Suffix = String;

/// <p>A container of a key value name pair.</p>
pub struct Tag {
    /// <p>Name of the object key.</p>
    pub key: ObjectKey,
    /// <p>Value of the tag.</p>
    pub value: Value,
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Tag");
        d.field("key", &self.key);
        d.field("value", &self.value);
        d.finish_non_exhaustive()
    }
}

pub type TagCount = i32;

pub type TagSet = List<Tag>;

/// <p>Container for <code>TagSet</code> elements.</p>
pub struct Tagging {
    /// <p>A collection for a set of tags</p>
    pub tag_set: TagSet,
}

impl fmt::Debug for Tagging {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Tagging");
        d.field("tag_set", &self.tag_set);
        d.finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaggingDirective(Cow<'static, str>);

impl TaggingDirective {
    pub const COPY: &str = "COPY";

    pub const REPLACE: &str = "REPLACE";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for TaggingDirective {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<TaggingDirective> for Cow<'static, str> {
    fn from(s: TaggingDirective) -> Self {
        s.0
    }
}

impl FromStr for TaggingDirective {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type TaggingHeader = String;

pub type TargetBucket = String;

/// <p>Container for granting information.</p>
/// <p>Buckets that use the bucket owner enforced setting for Object
/// Ownership don't support target grants. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/enable-server-access-logging.html#grant-log-delivery-permissions-general">Permissions server access log delivery</a> in the
/// <i>Amazon S3 User Guide</i>.</p>
#[derive(Default)]
pub struct TargetGrant {
    /// <p>Container for the person being granted permissions.</p>
    pub grantee: Option<Grantee>,
    /// <p>Logging permissions assigned to the grantee for the bucket.</p>
    pub permission: Option<BucketLogsPermission>,
}

impl fmt::Debug for TargetGrant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("TargetGrant");
        if let Some(ref val) = self.grantee {
            d.field("grantee", val);
        }
        if let Some(ref val) = self.permission {
            d.field("permission", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type TargetGrants = List<TargetGrant>;

pub type TargetPrefix = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tier(Cow<'static, str>);

impl Tier {
    pub const BULK: &str = "Bulk";

    pub const EXPEDITED: &str = "Expedited";

    pub const STANDARD: &str = "Standard";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for Tier {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<Tier> for Cow<'static, str> {
    fn from(s: Tier) -> Self {
        s.0
    }
}

impl FromStr for Tier {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

/// <p>The S3 Intelligent-Tiering storage class is designed to optimize storage costs by
/// automatically moving data to the most cost-effective storage access tier, without
/// additional operational overhead.</p>
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

impl fmt::Debug for Tiering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Tiering");
        d.field("access_tier", &self.access_tier);
        d.field("days", &self.days);
        d.finish_non_exhaustive()
    }
}

pub type TieringList = List<Tiering>;

pub type Token = String;

pub type TopicArn = String;

/// <p>A container for specifying the configuration for publication of messages to an Amazon
/// Simple Notification Service (Amazon SNS) topic when Amazon S3 detects specified events.</p>
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

impl fmt::Debug for TopicConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("TopicConfiguration");
        d.field("events", &self.events);
        if let Some(ref val) = self.filter {
            d.field("filter", val);
        }
        if let Some(ref val) = self.id {
            d.field("id", val);
        }
        d.field("topic_arn", &self.topic_arn);
        d.finish_non_exhaustive()
    }
}

pub type TopicConfigurationList = List<TopicConfiguration>;

/// <p>Specifies when an object transitions to a specified storage class. For more information
/// about Amazon S3 lifecycle configuration rules, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/lifecycle-transition-general-considerations.html">Transitioning
/// Objects Using Amazon S3 Lifecycle</a> in the <i>Amazon S3 User Guide</i>.</p>
#[derive(Default)]
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

impl fmt::Debug for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Transition");
        if let Some(ref val) = self.date {
            d.field("date", val);
        }
        d.field("days", &self.days);
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type TransitionList = List<Transition>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionStorageClass(Cow<'static, str>);

impl TransitionStorageClass {
    pub const DEEP_ARCHIVE: &str = "DEEP_ARCHIVE";

    pub const GLACIER: &str = "GLACIER";

    pub const GLACIER_IR: &str = "GLACIER_IR";

    pub const INTELLIGENT_TIERING: &str = "INTELLIGENT_TIERING";

    pub const ONEZONE_IA: &str = "ONEZONE_IA";

    pub const STANDARD_IA: &str = "STANDARD_IA";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for TransitionStorageClass {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<TransitionStorageClass> for Cow<'static, str> {
    fn from(s: TransitionStorageClass) -> Self {
        s.0
    }
}

impl FromStr for TransitionStorageClass {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type(Cow<'static, str>);

impl Type {
    pub const AMAZON_CUSTOMER_BY_EMAIL: &str = "AmazonCustomerByEmail";

    pub const CANONICAL_USER: &str = "CanonicalUser";

    pub const GROUP: &str = "Group";

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn from_static(s: &'static str) -> Self {
        Self(Cow::from(s))
    }
}

impl From<String> for Type {
    fn from(s: String) -> Self {
        Self(Cow::from(s))
    }
}

impl From<Type> for Cow<'static, str> {
    fn from(s: Type) -> Self {
        s.0
    }
}

impl FromStr for Type {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_owned()))
    }
}

pub type URI = String;

pub type UploadIdMarker = String;

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

impl fmt::Debug for UploadPartCopyInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("UploadPartCopyInput");
        d.field("bucket", &self.bucket);
        d.field("copy_source", &self.copy_source);
        if let Some(ref val) = self.copy_source_if_match {
            d.field("copy_source_if_match", val);
        }
        if let Some(ref val) = self.copy_source_if_modified_since {
            d.field("copy_source_if_modified_since", val);
        }
        if let Some(ref val) = self.copy_source_if_none_match {
            d.field("copy_source_if_none_match", val);
        }
        if let Some(ref val) = self.copy_source_if_unmodified_since {
            d.field("copy_source_if_unmodified_since", val);
        }
        if let Some(ref val) = self.copy_source_range {
            d.field("copy_source_range", val);
        }
        if let Some(ref val) = self.copy_source_sse_customer_algorithm {
            d.field("copy_source_sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.copy_source_sse_customer_key {
            d.field("copy_source_sse_customer_key", val);
        }
        if let Some(ref val) = self.copy_source_sse_customer_key_md5 {
            d.field("copy_source_sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        if let Some(ref val) = self.expected_source_bucket_owner {
            d.field("expected_source_bucket_owner", val);
        }
        d.field("key", &self.key);
        d.field("part_number", &self.part_number);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key {
            d.field("sse_customer_key", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        d.field("upload_id", &self.upload_id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for UploadPartCopyOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("UploadPartCopyOutput");
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.copy_part_result {
            d.field("copy_part_result", val);
        }
        if let Some(ref val) = self.copy_source_version_id {
            d.field("copy_source_version_id", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        d.finish_non_exhaustive()
    }
}

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

impl fmt::Debug for UploadPartInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("UploadPartInput");
        if let Some(ref val) = self.body {
            d.field("body", val);
        }
        d.field("bucket", &self.bucket);
        if let Some(ref val) = self.checksum_algorithm {
            d.field("checksum_algorithm", val);
        }
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        d.field("content_length", &self.content_length);
        if let Some(ref val) = self.content_md5 {
            d.field("content_md5", val);
        }
        if let Some(ref val) = self.expected_bucket_owner {
            d.field("expected_bucket_owner", val);
        }
        d.field("key", &self.key);
        d.field("part_number", &self.part_number);
        if let Some(ref val) = self.request_payer {
            d.field("request_payer", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key {
            d.field("sse_customer_key", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        d.field("upload_id", &self.upload_id);
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
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

impl fmt::Debug for UploadPartOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("UploadPartOutput");
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type UserMetadata = List<MetadataEntry>;

pub type Value = String;

pub type VersionCount = i32;

pub type VersionIdMarker = String;

/// <p>Describes the versioning state of an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTVersioningStatus.html">PUT
/// Bucket versioning</a> in the <i>Amazon S3 API Reference</i>.</p>
#[derive(Default)]
pub struct VersioningConfiguration {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This
    /// element is only returned if the bucket has been configured with MFA delete. If the bucket
    /// has never been so configured, this element is not returned.</p>
    pub mfa_delete: Option<MFADelete>,
    /// <p>The versioning state of the bucket.</p>
    pub status: Option<BucketVersioningStatus>,
}

impl fmt::Debug for VersioningConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("VersioningConfiguration");
        if let Some(ref val) = self.mfa_delete {
            d.field("mfa_delete", val);
        }
        if let Some(ref val) = self.status {
            d.field("status", val);
        }
        d.finish_non_exhaustive()
    }
}

/// <p>Specifies website configuration parameters for an Amazon S3 bucket.</p>
#[derive(Default)]
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

impl fmt::Debug for WebsiteConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("WebsiteConfiguration");
        if let Some(ref val) = self.error_document {
            d.field("error_document", val);
        }
        if let Some(ref val) = self.index_document {
            d.field("index_document", val);
        }
        if let Some(ref val) = self.redirect_all_requests_to {
            d.field("redirect_all_requests_to", val);
        }
        if let Some(ref val) = self.routing_rules {
            d.field("routing_rules", val);
        }
        d.finish_non_exhaustive()
    }
}

pub type WebsiteRedirectLocation = String;

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

impl fmt::Debug for WriteGetObjectResponseInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("WriteGetObjectResponseInput");
        if let Some(ref val) = self.accept_ranges {
            d.field("accept_ranges", val);
        }
        if let Some(ref val) = self.body {
            d.field("body", val);
        }
        d.field("bucket_key_enabled", &self.bucket_key_enabled);
        if let Some(ref val) = self.cache_control {
            d.field("cache_control", val);
        }
        if let Some(ref val) = self.checksum_crc32 {
            d.field("checksum_crc32", val);
        }
        if let Some(ref val) = self.checksum_crc32c {
            d.field("checksum_crc32c", val);
        }
        if let Some(ref val) = self.checksum_sha1 {
            d.field("checksum_sha1", val);
        }
        if let Some(ref val) = self.checksum_sha256 {
            d.field("checksum_sha256", val);
        }
        if let Some(ref val) = self.content_disposition {
            d.field("content_disposition", val);
        }
        if let Some(ref val) = self.content_encoding {
            d.field("content_encoding", val);
        }
        if let Some(ref val) = self.content_language {
            d.field("content_language", val);
        }
        d.field("content_length", &self.content_length);
        if let Some(ref val) = self.content_range {
            d.field("content_range", val);
        }
        if let Some(ref val) = self.content_type {
            d.field("content_type", val);
        }
        d.field("delete_marker", &self.delete_marker);
        if let Some(ref val) = self.e_tag {
            d.field("e_tag", val);
        }
        if let Some(ref val) = self.error_code {
            d.field("error_code", val);
        }
        if let Some(ref val) = self.error_message {
            d.field("error_message", val);
        }
        if let Some(ref val) = self.expiration {
            d.field("expiration", val);
        }
        if let Some(ref val) = self.expires {
            d.field("expires", val);
        }
        if let Some(ref val) = self.last_modified {
            d.field("last_modified", val);
        }
        if let Some(ref val) = self.metadata {
            d.field("metadata", val);
        }
        d.field("missing_meta", &self.missing_meta);
        if let Some(ref val) = self.object_lock_legal_hold_status {
            d.field("object_lock_legal_hold_status", val);
        }
        if let Some(ref val) = self.object_lock_mode {
            d.field("object_lock_mode", val);
        }
        if let Some(ref val) = self.object_lock_retain_until_date {
            d.field("object_lock_retain_until_date", val);
        }
        d.field("parts_count", &self.parts_count);
        if let Some(ref val) = self.replication_status {
            d.field("replication_status", val);
        }
        if let Some(ref val) = self.request_charged {
            d.field("request_charged", val);
        }
        d.field("request_route", &self.request_route);
        d.field("request_token", &self.request_token);
        if let Some(ref val) = self.restore {
            d.field("restore", val);
        }
        if let Some(ref val) = self.sse_customer_algorithm {
            d.field("sse_customer_algorithm", val);
        }
        if let Some(ref val) = self.sse_customer_key_md5 {
            d.field("sse_customer_key_md5", val);
        }
        if let Some(ref val) = self.ssekms_key_id {
            d.field("ssekms_key_id", val);
        }
        if let Some(ref val) = self.server_side_encryption {
            d.field("server_side_encryption", val);
        }
        d.field("status_code", &self.status_code);
        if let Some(ref val) = self.storage_class {
            d.field("storage_class", val);
        }
        d.field("tag_count", &self.tag_count);
        if let Some(ref val) = self.version_id {
            d.field("version_id", val);
        }
        d.finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct WriteGetObjectResponseOutput {}

impl fmt::Debug for WriteGetObjectResponseOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("WriteGetObjectResponseOutput");
        d.finish_non_exhaustive()
    }
}

pub type Years = i32;
