use hyper::StatusCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// Reduce your request rate.
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
}

impl S3ErrorCode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AccessDenied => "AccessDenied ",
            Self::AccountProblem => "AccountProblem",
            Self::AllAccessDisabled => "AllAccessDisabled",
            Self::AmbiguousGrantByEmailAddress => "AmbiguousGrantByEmailAddress",
            Self::AuthorizationHeaderMalformed => "AuthorizationHeaderMalformed",
            Self::BadDigest => "BadDigest",
            Self::BucketAlreadyExists => "BucketAlreadyExists",
            Self::BucketAlreadyOwnedByYou => "BucketAlreadyOwnedByYou",
            Self::BucketNotEmpty => "BucketNotEmpty",
            Self::CredentialsNotSupported => "CredentialsNotSupported",
            Self::CrossLocationLoggingProhibited => "CrossLocationLoggingProhibited",
            Self::EntityTooLarge => "EntityTooLarge",
            Self::EntityTooSmall => "EntityTooSmall",
            Self::ExpiredToken => "ExpiredToken",
            Self::IllegalVersioningConfigurationException => "IllegalVersioningConfigurationException ",
            Self::IncompleteBody => "IncompleteBody",
            Self::IncorrectNumberOfFilesInPostRequest => "IncorrectNumberOfFilesInPostRequest",
            Self::InlineDataTooLarge => "InlineDataTooLarge",
            Self::InternalError => "InternalError",
            Self::InvalidAccessKeyId => "InvalidAccessKeyId",
            Self::InvalidAddressingHeader => "InvalidAddressingHeader",
            Self::InvalidArgument => "InvalidArgument",
            Self::InvalidBucketName => "InvalidBucketName",
            Self::InvalidBucketState => "InvalidBucketState",
            Self::InvalidDigest => "InvalidDigest",
            Self::InvalidEncryptionAlgorithmError => "InvalidEncryptionAlgorithmError",
            Self::InvalidLocationConstraint => "InvalidLocationConstraint",
            Self::InvalidObjectState => "InvalidObjectState",
            Self::InvalidPart => "InvalidPart",
            Self::InvalidPartOrder => "InvalidPartOrder",
            Self::InvalidPayer => "InvalidPayer",
            Self::InvalidPolicyDocument => "InvalidPolicyDocument",
            Self::InvalidRange => "InvalidRange",
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidSOAPRequest => "InvalidSOAPRequest",
            Self::InvalidSecurity => "InvalidSecurity",
            Self::InvalidStorageClass => "InvalidStorageClass",
            Self::InvalidTargetBucketForLogging => "InvalidTargetBucketForLogging",
            Self::InvalidToken => "InvalidToken",
            Self::InvalidURI => "InvalidURI",
            Self::KeyTooLongError => "KeyTooLongError",
            Self::MalformedACLError => "MalformedACLError",
            Self::MalformedPOSTRequest => "MalformedPOSTRequest ",
            Self::MalformedXML => "MalformedXML",
            Self::MaxMessageLengthExceeded => "MaxMessageLengthExceeded",
            Self::MaxPostPreDataLengthExceededError => "MaxPostPreDataLengthExceededError",
            Self::MetadataTooLarge => "MetadataTooLarge",
            Self::MethodNotAllowed => "MethodNotAllowed",
            Self::MissingAttachment => "MissingAttachment",
            Self::MissingContentLength => "MissingContentLength",
            Self::MissingRequestBodyError => "MissingRequestBodyError",
            Self::MissingSecurityElement => "MissingSecurityElement",
            Self::MissingSecurityHeader => "MissingSecurityHeader",
            Self::NoLoggingStatusForKey => "NoLoggingStatusForKey",
            Self::NoSuchBucket => "NoSuchBucket",
            Self::NoSuchBucketPolicy => "NoSuchBucketPolicy",
            Self::NoSuchKey => "NoSuchKey",
            Self::NoSuchLifecycleConfiguration => "NoSuchLifecycleConfiguration",
            Self::NoSuchUpload => "NoSuchUpload",
            Self::NoSuchVersion => "NoSuchVersion ",
            Self::NotImplemented => "NotImplemented",
            Self::NotSignedUp => "NotSignedUp",
            Self::OperationAborted => "OperationAborted",
            Self::PermanentRedirect => "PermanentRedirect",
            Self::PreconditionFailed => "PreconditionFailed",
            Self::Redirect => "Redirect",
            Self::RequestIsNotMultiPartContent => "RequestIsNotMultiPartContent",
            Self::RequestTimeTooSkewed => "RequestTimeTooSkewed",
            Self::RequestTimeout => "RequestTimeout",
            Self::RequestTorrentOfBucketError => "RequestTorrentOfBucketError",
            Self::RestoreAlreadyInProgress => "RestoreAlreadyInProgress",
            Self::ServiceUnavailable => "ServiceUnavailable",
            Self::SignatureDoesNotMatch => "SignatureDoesNotMatch",
            Self::SlowDown => "SlowDown",
            Self::TemporaryRedirect => "TemporaryRedirect",
            Self::TokenRefreshRequired => "TokenRefreshRequired",
            Self::TooManyBuckets => "TooManyBuckets",
            Self::UnexpectedContent => "UnexpectedContent",
            Self::UnresolvableGrantByEmailAddress => "UnresolvableGrantByEmailAddress",
            Self::UserKeyMustBeSpecified => "UserKeyMustBeSpecified",
        }
    }

    #[must_use]
    pub const fn from_bytes(s: &[u8]) -> Option<Self> {
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
            _ => None,
        }
    }

    #[must_use]
    pub const fn status_code(self) -> Option<StatusCode> {
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
        }
    }
}
