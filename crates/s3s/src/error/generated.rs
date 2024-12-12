//! Auto generated by `codegen/src/v1/error.rs:147`

#![allow(clippy::doc_markdown)]

use bytestring::ByteString;
use hyper::StatusCode;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum S3ErrorCode {
    /// Access Denied
    ///
    /// HTTP Status Code: 403 Forbidden
    ///
    AccessDenied,

    /// There is a problem with your Amazon Web Services account that prevents the action from completing successfully. Contact Amazon Web Services Support for further assistance.
    ///
    /// HTTP Status Code: 403 Forbidden
    ///
    AccountProblem,

    /// All access to this Amazon S3 resource has been disabled. Contact Amazon Web Services Support for further assistance.
    ///
    /// HTTP Status Code: 403 Forbidden
    ///
    AllAccessDisabled,

    /// The email address you provided is associated with more than one account.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    AmbiguousGrantByEmailAddress,

    /// The authorization header you provided is invalid.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    AuthorizationHeaderMalformed,

    /// The Content-MD5 you specified did not match what we received.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    BadDigest,

    /// The requested bucket name is not available. The bucket namespace is shared by all users of the system. Please select a different name and try again.
    ///
    /// HTTP Status Code: 409 Conflict
    ///
    BucketAlreadyExists,

    /// The bucket you tried to create already exists, and you own it. Amazon S3 returns this error in all Amazon Web Services Regions except in the North Virginia Region. For legacy compatibility, if you re-create an existing bucket that you already own in the North Virginia Region, Amazon S3 returns 200 OK and resets the bucket access control lists (ACLs).
    ///
    /// HTTP Status Code: 409 Conflict
    ///
    BucketAlreadyOwnedByYou,

    /// The bucket you tried to delete is not empty.
    ///
    /// HTTP Status Code: 409 Conflict
    ///
    BucketNotEmpty,

    /// This request does not support credentials.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    CredentialsNotSupported,

    /// Cross-location logging not allowed. Buckets in one geographic location cannot log information to a bucket in another location.
    ///
    /// HTTP Status Code: 403 Forbidden
    ///
    CrossLocationLoggingProhibited,

    /// Your proposed upload exceeds the maximum allowed object size.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    EntityTooLarge,

    /// Your proposed upload is smaller than the minimum allowed object size.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    EntityTooSmall,

    /// The provided token has expired.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    ExpiredToken,

    /// Indicates that the versioning configuration specified in the request is invalid.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    IllegalVersioningConfigurationException,

    /// You did not provide the number of bytes specified by the Content-Length HTTP header
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    IncompleteBody,

    /// POST requires exactly one file upload per request.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    IncorrectNumberOfFilesInPostRequest,

    /// Inline data exceeds the maximum allowed size.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InlineDataTooLarge,

    /// We encountered an internal error. Please try again.
    ///
    /// HTTP Status Code: 500 Internal Server Error
    ///
    InternalError,

    /// The Amazon Web Services access key ID you provided does not exist in our records.
    ///
    /// HTTP Status Code: 403 Forbidden
    ///
    InvalidAccessKeyId,

    /// You must specify the Anonymous role.
    ///
    InvalidAddressingHeader,

    /// Invalid Argument
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidArgument,

    /// The specified bucket is not valid.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidBucketName,

    /// The request is not valid with the current state of the bucket.
    ///
    /// HTTP Status Code: 409 Conflict
    ///
    InvalidBucketState,

    /// The Content-MD5 you specified is not valid.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidDigest,

    /// The encryption request you specified is not valid. The valid value is AES256.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidEncryptionAlgorithmError,

    /// The specified location constraint is not valid. For more information about Regions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingBucket.html#access-bucket-intro">How to Select a Region for Your Buckets</a>.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidLocationConstraint,

    /// The action is not valid for the current state of the object.
    ///
    /// HTTP Status Code: 403 Forbidden
    ///
    InvalidObjectState,

    /// One or more of the specified parts could not be found. The part might not have been uploaded, or the specified entity tag might not have matched the part's entity tag.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidPart,

    /// The list of parts was not in ascending order. Parts list must be specified in order by part number.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidPartOrder,

    /// All access to this object has been disabled. Please contact Amazon Web Services Support for further assistance.
    ///
    /// HTTP Status Code: 403 Forbidden
    ///
    InvalidPayer,

    /// The content of the form does not meet the conditions specified in the policy document.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidPolicyDocument,

    /// The requested range cannot be satisfied.
    ///
    /// HTTP Status Code: 416 Requested Range NotSatisfiable
    ///
    InvalidRange,

    /// + Please use <code>AWS4-HMAC-SHA256</code>.
    /// + SOAP requests must be made over an HTTPS connection.
    /// + Amazon S3 Transfer Acceleration is not supported for buckets with non-DNS compliant names.
    /// + Amazon S3 Transfer Acceleration is not supported for buckets with periods (.) in their names.
    /// + Amazon S3 Transfer Accelerate endpoint only supports virtual style requests.
    /// + Amazon S3 Transfer Accelerate is not configured on this bucket.
    /// + Amazon S3 Transfer Accelerate is disabled on this bucket.
    /// + Amazon S3 Transfer Acceleration is not supported on this bucket. Contact Amazon Web Services Support for more information.
    /// + Amazon S3 Transfer Acceleration cannot be enabled on this bucket. Contact Amazon Web Services Support for more information.
    ///
    /// HTTP Status Code: 400 Bad Request
    InvalidRequest,

    /// The SOAP request body is invalid.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidSOAPRequest,

    /// The provided security credentials are not valid.
    ///
    /// HTTP Status Code: 403 Forbidden
    ///
    InvalidSecurity,

    /// The storage class you specified is not valid.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidStorageClass,

    /// The target bucket for logging does not exist, is not owned by you, or does not have the appropriate grants for the log-delivery group.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidTargetBucketForLogging,

    /// The provided token is malformed or otherwise invalid.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidToken,

    /// Couldn't parse the specified URI.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    InvalidURI,

    /// Your key is too long.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    KeyTooLongError,

    /// The XML you provided was not well-formed or did not validate against our published schema.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    MalformedACLError,

    /// The body of your POST request is not well-formed multipart/form-data.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    MalformedPOSTRequest,

    /// This happens when the user sends malformed XML (XML that doesn't conform to the published XSD) for the configuration. The error message is, "The XML you provided was not well-formed or did not validate against our published schema."
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    MalformedXML,

    /// Your request was too big.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    MaxMessageLengthExceeded,

    /// Your POST request fields preceding the upload file were too large.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    MaxPostPreDataLengthExceededError,

    /// Your metadata headers exceed the maximum allowed metadata size.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    MetadataTooLarge,

    /// The specified method is not allowed against this resource.
    ///
    /// HTTP Status Code: 405 Method Not Allowed
    ///
    MethodNotAllowed,

    /// A SOAP attachment was expected, but none were found.
    ///
    MissingAttachment,

    /// You must provide the Content-Length HTTP header.
    ///
    /// HTTP Status Code: 411 Length Required
    ///
    MissingContentLength,

    /// This happens when the user sends an empty XML document as a request. The error message is, "Request body is empty."
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    MissingRequestBodyError,

    /// The SOAP 1.1 request is missing a security element.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    MissingSecurityElement,

    /// Your request is missing a required header.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    MissingSecurityHeader,

    /// There is no such thing as a logging status subresource for a key.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    NoLoggingStatusForKey,

    /// The specified bucket does not exist.
    ///
    /// HTTP Status Code: 404 Not Found
    ///
    NoSuchBucket,

    /// The specified bucket does not have a bucket policy.
    ///
    /// HTTP Status Code: 404 Not Found
    ///
    NoSuchBucketPolicy,

    /// The specified key does not exist.
    ///
    /// HTTP Status Code: 404 Not Found
    ///
    NoSuchKey,

    /// The lifecycle configuration does not exist.
    ///
    /// HTTP Status Code: 404 Not Found
    ///
    NoSuchLifecycleConfiguration,

    /// The specified multipart upload does not exist. The upload ID might be invalid, or the multipart upload might have been aborted or completed.
    ///
    /// HTTP Status Code: 404 Not Found
    ///
    NoSuchUpload,

    /// Indicates that the version ID specified in the request does not match an existing version.
    ///
    /// HTTP Status Code: 404 Not Found
    ///
    NoSuchVersion,

    /// A header you provided implies functionality that is not implemented.
    ///
    /// HTTP Status Code: 501 Not Implemented
    ///
    NotImplemented,

    /// Your account is not signed up for the Amazon S3 service. You must sign up before you can use Amazon S3. You can sign up at the following URL: <a href="http://aws.amazon.com/s3">Amazon S3</a>
    ///
    /// HTTP Status Code: 403 Forbidden
    ///
    NotSignedUp,

    /// A conflicting conditional action is currently in progress against this resource. Try again.
    ///
    /// HTTP Status Code: 409 Conflict
    ///
    OperationAborted,

    /// The bucket you are attempting to access must be addressed using the specified endpoint. Send all future requests to this endpoint.
    ///
    /// HTTP Status Code: 301 Moved Permanently
    ///
    PermanentRedirect,

    /// At least one of the preconditions you specified did not hold.
    ///
    /// HTTP Status Code: 412 Precondition Failed
    ///
    PreconditionFailed,

    /// Temporary redirect.
    ///
    /// HTTP Status Code: 307 Moved Temporarily
    ///
    Redirect,

    /// There is no replication configuration for this bucket.
    ///
    /// HTTP Status Code: 404 Not Found
    ///
    ReplicationConfigurationNotFoundError,

    /// Bucket POST must be of the enclosure-type multipart/form-data.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    RequestIsNotMultiPartContent,

    /// The difference between the request time and the server's time is too large.
    ///
    /// HTTP Status Code: 403 Forbidden
    ///
    RequestTimeTooSkewed,

    /// Your socket connection to the server was not read from or written to within the timeout period.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    RequestTimeout,

    /// Requesting the torrent file of a bucket is not permitted.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    RequestTorrentOfBucketError,

    /// Object restore is already in progress.
    ///
    /// HTTP Status Code: 409 Conflict
    ///
    RestoreAlreadyInProgress,

    /// Service is unable to handle request.
    ///
    /// HTTP Status Code: 503 Service Unavailable
    ///
    ServiceUnavailable,

    /// The request signature we calculated does not match the signature you provided. Check your Amazon Web Services secret access key and signing method. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAuthentication.html">REST Authentication</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/SOAPAuthentication.html">SOAP Authentication</a> for details.
    ///
    /// HTTP Status Code: 403 Forbidden
    ///
    SignatureDoesNotMatch,

    /// Reduce your request rate.
    ///
    /// HTTP Status Code: 503 Slow Down
    ///
    SlowDown,

    /// You are being redirected to the bucket while DNS updates.
    ///
    /// HTTP Status Code: 307 Moved Temporarily
    ///
    TemporaryRedirect,

    /// The provided token must be refreshed.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    TokenRefreshRequired,

    /// You have attempted to create more buckets than allowed.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    TooManyBuckets,

    /// This request does not support content.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    UnexpectedContent,

    /// The email address you provided does not match any account on record.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    UnresolvableGrantByEmailAddress,

    /// The bucket POST must contain the specified field name. If it is specified, check the order of the fields.
    ///
    /// HTTP Status Code: 400 Bad Request
    ///
    UserKeyMustBeSpecified,

    Custom(ByteString),
}

impl S3ErrorCode {
    const STATIC_CODE_LIST: &'static [&'static str] = &[
        "AccessDenied ",
        "AccountProblem",
        "AllAccessDisabled",
        "AmbiguousGrantByEmailAddress",
        "AuthorizationHeaderMalformed",
        "BadDigest",
        "BucketAlreadyExists",
        "BucketAlreadyOwnedByYou",
        "BucketNotEmpty",
        "CredentialsNotSupported",
        "CrossLocationLoggingProhibited",
        "EntityTooLarge",
        "EntityTooSmall",
        "ExpiredToken",
        "IllegalVersioningConfigurationException ",
        "IncompleteBody",
        "IncorrectNumberOfFilesInPostRequest",
        "InlineDataTooLarge",
        "InternalError",
        "InvalidAccessKeyId",
        "InvalidAddressingHeader",
        "InvalidArgument",
        "InvalidBucketName",
        "InvalidBucketState",
        "InvalidDigest",
        "InvalidEncryptionAlgorithmError",
        "InvalidLocationConstraint",
        "InvalidObjectState",
        "InvalidPart",
        "InvalidPartOrder",
        "InvalidPayer",
        "InvalidPolicyDocument",
        "InvalidRange",
        "InvalidRequest",
        "InvalidSOAPRequest",
        "InvalidSecurity",
        "InvalidStorageClass",
        "InvalidTargetBucketForLogging",
        "InvalidToken",
        "InvalidURI",
        "KeyTooLongError",
        "MalformedACLError",
        "MalformedPOSTRequest ",
        "MalformedXML",
        "MaxMessageLengthExceeded",
        "MaxPostPreDataLengthExceededError",
        "MetadataTooLarge",
        "MethodNotAllowed",
        "MissingAttachment",
        "MissingContentLength",
        "MissingRequestBodyError",
        "MissingSecurityElement",
        "MissingSecurityHeader",
        "NoLoggingStatusForKey",
        "NoSuchBucket",
        "NoSuchBucketPolicy",
        "NoSuchKey",
        "NoSuchLifecycleConfiguration",
        "NoSuchUpload",
        "NoSuchVersion ",
        "NotImplemented",
        "NotSignedUp",
        "OperationAborted",
        "PermanentRedirect",
        "PreconditionFailed",
        "Redirect",
        "ReplicationConfigurationNotFoundError",
        "RequestIsNotMultiPartContent",
        "RequestTimeTooSkewed",
        "RequestTimeout",
        "RequestTorrentOfBucketError",
        "RestoreAlreadyInProgress",
        "ServiceUnavailable",
        "SignatureDoesNotMatch",
        "SlowDown",
        "TemporaryRedirect",
        "TokenRefreshRequired",
        "TooManyBuckets",
        "UnexpectedContent",
        "UnresolvableGrantByEmailAddress",
        "UserKeyMustBeSpecified",
    ];

    #[must_use]
    fn as_enum_tag(&self) -> usize {
        match self {
            Self::AccessDenied => 0,
            Self::AccountProblem => 1,
            Self::AllAccessDisabled => 2,
            Self::AmbiguousGrantByEmailAddress => 3,
            Self::AuthorizationHeaderMalformed => 4,
            Self::BadDigest => 5,
            Self::BucketAlreadyExists => 6,
            Self::BucketAlreadyOwnedByYou => 7,
            Self::BucketNotEmpty => 8,
            Self::CredentialsNotSupported => 9,
            Self::CrossLocationLoggingProhibited => 10,
            Self::EntityTooLarge => 11,
            Self::EntityTooSmall => 12,
            Self::ExpiredToken => 13,
            Self::IllegalVersioningConfigurationException => 14,
            Self::IncompleteBody => 15,
            Self::IncorrectNumberOfFilesInPostRequest => 16,
            Self::InlineDataTooLarge => 17,
            Self::InternalError => 18,
            Self::InvalidAccessKeyId => 19,
            Self::InvalidAddressingHeader => 20,
            Self::InvalidArgument => 21,
            Self::InvalidBucketName => 22,
            Self::InvalidBucketState => 23,
            Self::InvalidDigest => 24,
            Self::InvalidEncryptionAlgorithmError => 25,
            Self::InvalidLocationConstraint => 26,
            Self::InvalidObjectState => 27,
            Self::InvalidPart => 28,
            Self::InvalidPartOrder => 29,
            Self::InvalidPayer => 30,
            Self::InvalidPolicyDocument => 31,
            Self::InvalidRange => 32,
            Self::InvalidRequest => 33,
            Self::InvalidSOAPRequest => 34,
            Self::InvalidSecurity => 35,
            Self::InvalidStorageClass => 36,
            Self::InvalidTargetBucketForLogging => 37,
            Self::InvalidToken => 38,
            Self::InvalidURI => 39,
            Self::KeyTooLongError => 40,
            Self::MalformedACLError => 41,
            Self::MalformedPOSTRequest => 42,
            Self::MalformedXML => 43,
            Self::MaxMessageLengthExceeded => 44,
            Self::MaxPostPreDataLengthExceededError => 45,
            Self::MetadataTooLarge => 46,
            Self::MethodNotAllowed => 47,
            Self::MissingAttachment => 48,
            Self::MissingContentLength => 49,
            Self::MissingRequestBodyError => 50,
            Self::MissingSecurityElement => 51,
            Self::MissingSecurityHeader => 52,
            Self::NoLoggingStatusForKey => 53,
            Self::NoSuchBucket => 54,
            Self::NoSuchBucketPolicy => 55,
            Self::NoSuchKey => 56,
            Self::NoSuchLifecycleConfiguration => 57,
            Self::NoSuchUpload => 58,
            Self::NoSuchVersion => 59,
            Self::NotImplemented => 60,
            Self::NotSignedUp => 61,
            Self::OperationAborted => 62,
            Self::PermanentRedirect => 63,
            Self::PreconditionFailed => 64,
            Self::Redirect => 65,
            Self::ReplicationConfigurationNotFoundError => 66,
            Self::RequestIsNotMultiPartContent => 67,
            Self::RequestTimeTooSkewed => 68,
            Self::RequestTimeout => 69,
            Self::RequestTorrentOfBucketError => 70,
            Self::RestoreAlreadyInProgress => 71,
            Self::ServiceUnavailable => 72,
            Self::SignatureDoesNotMatch => 73,
            Self::SlowDown => 74,
            Self::TemporaryRedirect => 75,
            Self::TokenRefreshRequired => 76,
            Self::TooManyBuckets => 77,
            Self::UnexpectedContent => 78,
            Self::UnresolvableGrantByEmailAddress => 79,
            Self::UserKeyMustBeSpecified => 80,
            Self::Custom(_) => usize::MAX,
        }
    }

    pub(crate) fn as_static_str(&self) -> Option<&'static str> {
        Self::STATIC_CODE_LIST.get(self.as_enum_tag()).copied()
    }

    #[must_use]
    pub fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"AccessDenied " => Some(Self::AccessDenied),
            b"AccountProblem" => Some(Self::AccountProblem),
            b"AllAccessDisabled" => Some(Self::AllAccessDisabled),
            b"AmbiguousGrantByEmailAddress" => Some(Self::AmbiguousGrantByEmailAddress),
            b"AuthorizationHeaderMalformed" => Some(Self::AuthorizationHeaderMalformed),
            b"BadDigest" => Some(Self::BadDigest),
            b"BucketAlreadyExists" => Some(Self::BucketAlreadyExists),
            b"BucketAlreadyOwnedByYou" => Some(Self::BucketAlreadyOwnedByYou),
            b"BucketNotEmpty" => Some(Self::BucketNotEmpty),
            b"CredentialsNotSupported" => Some(Self::CredentialsNotSupported),
            b"CrossLocationLoggingProhibited" => Some(Self::CrossLocationLoggingProhibited),
            b"EntityTooLarge" => Some(Self::EntityTooLarge),
            b"EntityTooSmall" => Some(Self::EntityTooSmall),
            b"ExpiredToken" => Some(Self::ExpiredToken),
            b"IllegalVersioningConfigurationException " => Some(Self::IllegalVersioningConfigurationException),
            b"IncompleteBody" => Some(Self::IncompleteBody),
            b"IncorrectNumberOfFilesInPostRequest" => Some(Self::IncorrectNumberOfFilesInPostRequest),
            b"InlineDataTooLarge" => Some(Self::InlineDataTooLarge),
            b"InternalError" => Some(Self::InternalError),
            b"InvalidAccessKeyId" => Some(Self::InvalidAccessKeyId),
            b"InvalidAddressingHeader" => Some(Self::InvalidAddressingHeader),
            b"InvalidArgument" => Some(Self::InvalidArgument),
            b"InvalidBucketName" => Some(Self::InvalidBucketName),
            b"InvalidBucketState" => Some(Self::InvalidBucketState),
            b"InvalidDigest" => Some(Self::InvalidDigest),
            b"InvalidEncryptionAlgorithmError" => Some(Self::InvalidEncryptionAlgorithmError),
            b"InvalidLocationConstraint" => Some(Self::InvalidLocationConstraint),
            b"InvalidObjectState" => Some(Self::InvalidObjectState),
            b"InvalidPart" => Some(Self::InvalidPart),
            b"InvalidPartOrder" => Some(Self::InvalidPartOrder),
            b"InvalidPayer" => Some(Self::InvalidPayer),
            b"InvalidPolicyDocument" => Some(Self::InvalidPolicyDocument),
            b"InvalidRange" => Some(Self::InvalidRange),
            b"InvalidRequest" => Some(Self::InvalidRequest),
            b"InvalidSOAPRequest" => Some(Self::InvalidSOAPRequest),
            b"InvalidSecurity" => Some(Self::InvalidSecurity),
            b"InvalidStorageClass" => Some(Self::InvalidStorageClass),
            b"InvalidTargetBucketForLogging" => Some(Self::InvalidTargetBucketForLogging),
            b"InvalidToken" => Some(Self::InvalidToken),
            b"InvalidURI" => Some(Self::InvalidURI),
            b"KeyTooLongError" => Some(Self::KeyTooLongError),
            b"MalformedACLError" => Some(Self::MalformedACLError),
            b"MalformedPOSTRequest " => Some(Self::MalformedPOSTRequest),
            b"MalformedXML" => Some(Self::MalformedXML),
            b"MaxMessageLengthExceeded" => Some(Self::MaxMessageLengthExceeded),
            b"MaxPostPreDataLengthExceededError" => Some(Self::MaxPostPreDataLengthExceededError),
            b"MetadataTooLarge" => Some(Self::MetadataTooLarge),
            b"MethodNotAllowed" => Some(Self::MethodNotAllowed),
            b"MissingAttachment" => Some(Self::MissingAttachment),
            b"MissingContentLength" => Some(Self::MissingContentLength),
            b"MissingRequestBodyError" => Some(Self::MissingRequestBodyError),
            b"MissingSecurityElement" => Some(Self::MissingSecurityElement),
            b"MissingSecurityHeader" => Some(Self::MissingSecurityHeader),
            b"NoLoggingStatusForKey" => Some(Self::NoLoggingStatusForKey),
            b"NoSuchBucket" => Some(Self::NoSuchBucket),
            b"NoSuchBucketPolicy" => Some(Self::NoSuchBucketPolicy),
            b"NoSuchKey" => Some(Self::NoSuchKey),
            b"NoSuchLifecycleConfiguration" => Some(Self::NoSuchLifecycleConfiguration),
            b"NoSuchUpload" => Some(Self::NoSuchUpload),
            b"NoSuchVersion " => Some(Self::NoSuchVersion),
            b"NotImplemented" => Some(Self::NotImplemented),
            b"NotSignedUp" => Some(Self::NotSignedUp),
            b"OperationAborted" => Some(Self::OperationAborted),
            b"PermanentRedirect" => Some(Self::PermanentRedirect),
            b"PreconditionFailed" => Some(Self::PreconditionFailed),
            b"Redirect" => Some(Self::Redirect),
            b"ReplicationConfigurationNotFoundError" => Some(Self::ReplicationConfigurationNotFoundError),
            b"RequestIsNotMultiPartContent" => Some(Self::RequestIsNotMultiPartContent),
            b"RequestTimeTooSkewed" => Some(Self::RequestTimeTooSkewed),
            b"RequestTimeout" => Some(Self::RequestTimeout),
            b"RequestTorrentOfBucketError" => Some(Self::RequestTorrentOfBucketError),
            b"RestoreAlreadyInProgress" => Some(Self::RestoreAlreadyInProgress),
            b"ServiceUnavailable" => Some(Self::ServiceUnavailable),
            b"SignatureDoesNotMatch" => Some(Self::SignatureDoesNotMatch),
            b"SlowDown" => Some(Self::SlowDown),
            b"TemporaryRedirect" => Some(Self::TemporaryRedirect),
            b"TokenRefreshRequired" => Some(Self::TokenRefreshRequired),
            b"TooManyBuckets" => Some(Self::TooManyBuckets),
            b"UnexpectedContent" => Some(Self::UnexpectedContent),
            b"UnresolvableGrantByEmailAddress" => Some(Self::UnresolvableGrantByEmailAddress),
            b"UserKeyMustBeSpecified" => Some(Self::UserKeyMustBeSpecified),
            _ => std::str::from_utf8(s).ok().map(|s| Self::Custom(s.into())),
        }
    }

    #[allow(clippy::match_same_arms)]
    #[must_use]
    pub fn status_code(&self) -> Option<StatusCode> {
        match self {
            Self::AccessDenied => Some(StatusCode::FORBIDDEN),
            Self::AccountProblem => Some(StatusCode::FORBIDDEN),
            Self::AllAccessDisabled => Some(StatusCode::FORBIDDEN),
            Self::AmbiguousGrantByEmailAddress => Some(StatusCode::BAD_REQUEST),
            Self::AuthorizationHeaderMalformed => Some(StatusCode::BAD_REQUEST),
            Self::BadDigest => Some(StatusCode::BAD_REQUEST),
            Self::BucketAlreadyExists => Some(StatusCode::CONFLICT),
            Self::BucketAlreadyOwnedByYou => Some(StatusCode::CONFLICT),
            Self::BucketNotEmpty => Some(StatusCode::CONFLICT),
            Self::CredentialsNotSupported => Some(StatusCode::BAD_REQUEST),
            Self::CrossLocationLoggingProhibited => Some(StatusCode::FORBIDDEN),
            Self::EntityTooLarge => Some(StatusCode::BAD_REQUEST),
            Self::EntityTooSmall => Some(StatusCode::BAD_REQUEST),
            Self::ExpiredToken => Some(StatusCode::BAD_REQUEST),
            Self::IllegalVersioningConfigurationException => Some(StatusCode::BAD_REQUEST),
            Self::IncompleteBody => Some(StatusCode::BAD_REQUEST),
            Self::IncorrectNumberOfFilesInPostRequest => Some(StatusCode::BAD_REQUEST),
            Self::InlineDataTooLarge => Some(StatusCode::BAD_REQUEST),
            Self::InternalError => Some(StatusCode::INTERNAL_SERVER_ERROR),
            Self::InvalidAccessKeyId => Some(StatusCode::FORBIDDEN),
            Self::InvalidAddressingHeader => None,
            Self::InvalidArgument => Some(StatusCode::BAD_REQUEST),
            Self::InvalidBucketName => Some(StatusCode::BAD_REQUEST),
            Self::InvalidBucketState => Some(StatusCode::CONFLICT),
            Self::InvalidDigest => Some(StatusCode::BAD_REQUEST),
            Self::InvalidEncryptionAlgorithmError => Some(StatusCode::BAD_REQUEST),
            Self::InvalidLocationConstraint => Some(StatusCode::BAD_REQUEST),
            Self::InvalidObjectState => Some(StatusCode::FORBIDDEN),
            Self::InvalidPart => Some(StatusCode::BAD_REQUEST),
            Self::InvalidPartOrder => Some(StatusCode::BAD_REQUEST),
            Self::InvalidPayer => Some(StatusCode::FORBIDDEN),
            Self::InvalidPolicyDocument => Some(StatusCode::BAD_REQUEST),
            Self::InvalidRange => Some(StatusCode::RANGE_NOT_SATISFIABLE),
            Self::InvalidRequest => Some(StatusCode::BAD_REQUEST),
            Self::InvalidSOAPRequest => Some(StatusCode::BAD_REQUEST),
            Self::InvalidSecurity => Some(StatusCode::FORBIDDEN),
            Self::InvalidStorageClass => Some(StatusCode::BAD_REQUEST),
            Self::InvalidTargetBucketForLogging => Some(StatusCode::BAD_REQUEST),
            Self::InvalidToken => Some(StatusCode::BAD_REQUEST),
            Self::InvalidURI => Some(StatusCode::BAD_REQUEST),
            Self::KeyTooLongError => Some(StatusCode::BAD_REQUEST),
            Self::MalformedACLError => Some(StatusCode::BAD_REQUEST),
            Self::MalformedPOSTRequest => Some(StatusCode::BAD_REQUEST),
            Self::MalformedXML => Some(StatusCode::BAD_REQUEST),
            Self::MaxMessageLengthExceeded => Some(StatusCode::BAD_REQUEST),
            Self::MaxPostPreDataLengthExceededError => Some(StatusCode::BAD_REQUEST),
            Self::MetadataTooLarge => Some(StatusCode::BAD_REQUEST),
            Self::MethodNotAllowed => Some(StatusCode::METHOD_NOT_ALLOWED),
            Self::MissingAttachment => None,
            Self::MissingContentLength => Some(StatusCode::LENGTH_REQUIRED),
            Self::MissingRequestBodyError => Some(StatusCode::BAD_REQUEST),
            Self::MissingSecurityElement => Some(StatusCode::BAD_REQUEST),
            Self::MissingSecurityHeader => Some(StatusCode::BAD_REQUEST),
            Self::NoLoggingStatusForKey => Some(StatusCode::BAD_REQUEST),
            Self::NoSuchBucket => Some(StatusCode::NOT_FOUND),
            Self::NoSuchBucketPolicy => Some(StatusCode::NOT_FOUND),
            Self::NoSuchKey => Some(StatusCode::NOT_FOUND),
            Self::NoSuchLifecycleConfiguration => Some(StatusCode::NOT_FOUND),
            Self::NoSuchUpload => Some(StatusCode::NOT_FOUND),
            Self::NoSuchVersion => Some(StatusCode::NOT_FOUND),
            Self::NotImplemented => Some(StatusCode::NOT_IMPLEMENTED),
            Self::NotSignedUp => Some(StatusCode::FORBIDDEN),
            Self::OperationAborted => Some(StatusCode::CONFLICT),
            Self::PermanentRedirect => Some(StatusCode::MOVED_PERMANENTLY),
            Self::PreconditionFailed => Some(StatusCode::PRECONDITION_FAILED),
            Self::Redirect => Some(StatusCode::TEMPORARY_REDIRECT),
            Self::ReplicationConfigurationNotFoundError => Some(StatusCode::NOT_FOUND),
            Self::RequestIsNotMultiPartContent => Some(StatusCode::BAD_REQUEST),
            Self::RequestTimeTooSkewed => Some(StatusCode::FORBIDDEN),
            Self::RequestTimeout => Some(StatusCode::BAD_REQUEST),
            Self::RequestTorrentOfBucketError => Some(StatusCode::BAD_REQUEST),
            Self::RestoreAlreadyInProgress => Some(StatusCode::CONFLICT),
            Self::ServiceUnavailable => Some(StatusCode::SERVICE_UNAVAILABLE),
            Self::SignatureDoesNotMatch => Some(StatusCode::FORBIDDEN),
            Self::SlowDown => Some(StatusCode::SERVICE_UNAVAILABLE),
            Self::TemporaryRedirect => Some(StatusCode::TEMPORARY_REDIRECT),
            Self::TokenRefreshRequired => Some(StatusCode::BAD_REQUEST),
            Self::TooManyBuckets => Some(StatusCode::BAD_REQUEST),
            Self::UnexpectedContent => Some(StatusCode::BAD_REQUEST),
            Self::UnresolvableGrantByEmailAddress => Some(StatusCode::BAD_REQUEST),
            Self::UserKeyMustBeSpecified => Some(StatusCode::BAD_REQUEST),
            Self::Custom(_) => None,
        }
    }
}
