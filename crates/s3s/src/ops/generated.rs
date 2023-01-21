#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::borrow_interior_mutable_const)]

use crate::dto::*;
use crate::error::*;
use crate::header::names::*;
use crate::http;
use crate::path::S3Path;
use crate::xml;

use std::io::Write;

/// An async trait which represents the S3 API
#[async_trait::async_trait]
pub trait S3: Send + Sync + 'static {
    /// <p>This action aborts a multipart upload. After a multipart upload is aborted, no
    /// additional parts can be uploaded using that upload ID. The storage consumed by any
    /// previously uploaded parts will be freed. However, if any part uploads are currently in
    /// progress, those part uploads might or might not succeed. As a result, it might be necessary
    /// to abort a given multipart upload multiple times in order to completely free all storage
    /// consumed by all parts. </p>
    /// <p>To verify that all parts have been removed, so you don't get charged for the part
    /// storage, you should call the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a> action and ensure that
    /// the parts list is empty.</p>
    /// <p>For information about permissions required to use the multipart upload, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuAndPermissions.html">Multipart Upload and
    /// Permissions</a>.</p>
    /// <p>The following operations are related to <code>AbortMultipartUpload</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html">CompleteMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListMultipartUploads.html">ListMultipartUploads</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn abort_multipart_upload(&self, _input: AbortMultipartUploadInput) -> S3Result<AbortMultipartUploadOutput> {
        Err(s3_error!(NotImplemented, "AbortMultipartUpload is not implemented yet"))
    }

    /// <p>Completes a multipart upload by assembling previously uploaded parts.</p>
    /// <p>You first initiate the multipart upload and then upload all parts using the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a>
    /// operation. After successfully uploading all relevant parts of an upload, you call this
    /// action to complete the upload. Upon receiving this request, Amazon S3 concatenates all
    /// the parts in ascending order by part number to create a new object. In the Complete
    /// Multipart Upload request, you must provide the parts list. You must ensure that the parts
    /// list is complete. This action concatenates the parts that you provide in the list. For
    /// each part in the list, you must provide the part number and the <code>ETag</code> value,
    /// returned after that part was uploaded.</p>
    /// <p>Processing of a Complete Multipart Upload request could take several minutes to
    /// complete. After Amazon S3 begins processing the request, it sends an HTTP response header that
    /// specifies a 200 OK response. While processing is in progress, Amazon S3 periodically sends white
    /// space characters to keep the connection from timing out. Because a request could fail after
    /// the initial 200 OK response has been sent, it is important that you check the response body
    /// to determine whether the request succeeded.</p>
    /// <p>Note that if <code>CompleteMultipartUpload</code> fails, applications should be prepared
    /// to retry the failed requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ErrorBestPractices.html">Amazon S3 Error Best Practices</a>.</p>
    /// <important>
    /// <p>You cannot use <code>Content-Type: application/x-www-form-urlencoded</code> with Complete
    /// Multipart Upload requests. Also, if you do not provide a <code>Content-Type</code> header, <code>CompleteMultipartUpload</code> returns a 200 OK response.</p>
    /// </important>
    /// <p>For more information about multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/uploadobjusingmpu.html">Uploading Objects Using Multipart
    /// Upload</a>.</p>
    /// <p>For information about permissions required to use the multipart upload API, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuAndPermissions.html">Multipart Upload and
    /// Permissions</a>.</p>
    ///
    ///
    /// <p>
    /// <code>CompleteMultipartUpload</code> has the following special errors:</p>
    /// <ul>
    /// <li>
    /// <p>Error code: <code>EntityTooSmall</code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Description: Your proposed upload is smaller than the minimum allowed object
    /// size. Each part must be at least 5 MB in size, except the last part.</p>
    /// </li>
    /// <li>
    /// <p>400 Bad Request</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p>Error code: <code>InvalidPart</code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Description: One or more of the specified parts could not be found. The part
    /// might not have been uploaded, or the specified entity tag might not have
    /// matched the part's entity tag.</p>
    /// </li>
    /// <li>
    /// <p>400 Bad Request</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p>Error code: <code>InvalidPartOrder</code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Description: The list of parts was not in ascending order. The parts list
    /// must be specified in order by part number.</p>
    /// </li>
    /// <li>
    /// <p>400 Bad Request</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p>Error code: <code>NoSuchUpload</code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Description: The specified multipart upload does not exist. The upload ID
    /// might be invalid, or the multipart upload might have been aborted or
    /// completed.</p>
    /// </li>
    /// <li>
    /// <p>404 Not Found</p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    ///
    /// <p>The following operations are related to <code>CompleteMultipartUpload</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html">AbortMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListMultipartUploads.html">ListMultipartUploads</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn complete_multipart_upload(&self, _input: CompleteMultipartUploadInput) -> S3Result<CompleteMultipartUploadOutput> {
        Err(s3_error!(NotImplemented, "CompleteMultipartUpload is not implemented yet"))
    }

    /// <p>Creates a copy of an object that is already stored in Amazon S3.</p>
    /// <note>
    /// <p>You can store individual objects of up to 5 TB in Amazon S3. You create a copy of your
    /// object up to 5 GB in size in a single atomic action using this API. However, to copy an
    /// object greater than 5 GB, you must use the multipart upload Upload Part - Copy
    /// (UploadPartCopy) API. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/CopyingObjctsUsingRESTMPUapi.html">Copy Object Using the
    /// REST Multipart Upload API</a>.</p>
    /// </note>
    /// <p>All copy requests must be authenticated. Additionally, you must have
    /// <i>read</i> access to the source object and <i>write</i>
    /// access to the destination bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAuthentication.html">REST Authentication</a>. Both the Region
    /// that you want to copy the object from and the Region that you want to copy the object to
    /// must be enabled for your account.</p>
    /// <p>A copy request might return an error when Amazon S3 receives the copy request or while Amazon S3
    /// is copying the files. If the error occurs before the copy action starts, you receive a
    /// standard Amazon S3 error. If the error occurs during the copy operation, the error response is
    /// embedded in the <code>200 OK</code> response. This means that a <code>200 OK</code>
    /// response can contain either a success or an error. Design your application to parse the
    /// contents of the response and handle it appropriately.</p>
    /// <p>If the copy is successful, you receive a response with information about the copied
    /// object.</p>
    /// <note>
    /// <p>If the request is an HTTP 1.1 request, the response is chunk encoded. If it were not,
    /// it would not contain the content-length, and you would need to read the entire
    /// body.</p>
    /// </note>
    /// <p>The copy request charge is based on the storage class and Region that you specify for
    /// the destination object. For pricing information, see <a href="http://aws.amazon.com/s3/pricing/">Amazon S3 pricing</a>.</p>
    /// <important>
    /// <p>Amazon S3 transfer acceleration does not support cross-Region copies. If you request a
    /// cross-Region copy using a transfer acceleration endpoint, you get a 400 <code>Bad
    /// Request</code> error. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/transfer-acceleration.html">Transfer Acceleration</a>.</p>
    /// </important>
    /// <p>
    /// <b>Metadata</b>
    /// </p>
    /// <p>When copying an object, you can preserve all metadata (default) or specify new metadata.
    /// However, the ACL is not preserved and is set to private for the user making the request. To
    /// override the default ACL setting, specify a new ACL when generating a copy request. For
    /// more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/S3_ACLs_UsingACLs.html">Using ACLs</a>. </p>
    /// <p>To specify whether you want the object metadata copied from the source object or
    /// replaced with metadata provided in the request, you can optionally add the
    /// <code>x-amz-metadata-directive</code> header. When you grant permissions, you can use
    /// the <code>s3:x-amz-metadata-directive</code> condition key to enforce certain metadata
    /// behavior when objects are uploaded. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/amazon-s3-policy-keys.html">Specifying Conditions in a
    /// Policy</a> in the <i>Amazon S3 User Guide</i>. For a complete list of
    /// Amazon S3-specific condition keys, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/list_amazons3.html">Actions, Resources, and Condition Keys for
    /// Amazon S3</a>.</p>
    /// <p>
    /// <b>x-amz-copy-source-if Headers</b>
    /// </p>
    /// <p>To only copy an object under certain conditions, such as whether the <code>Etag</code>
    /// matches or whether the object was modified before or after a specified date, use the
    /// following request parameters:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>x-amz-copy-source-if-match</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-copy-source-if-none-match</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-copy-source-if-unmodified-since</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-copy-source-if-modified-since</code>
    /// </p>
    /// </li>
    /// </ul>
    /// <p> If both the <code>x-amz-copy-source-if-match</code> and
    /// <code>x-amz-copy-source-if-unmodified-since</code> headers are present in the request
    /// and evaluate as follows, Amazon S3 returns <code>200 OK</code> and copies the data:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>x-amz-copy-source-if-match</code> condition evaluates to true</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-copy-source-if-unmodified-since</code> condition evaluates to
    /// false</p>
    /// </li>
    /// </ul>
    ///
    /// <p>If both the <code>x-amz-copy-source-if-none-match</code> and
    /// <code>x-amz-copy-source-if-modified-since</code> headers are present in the request and
    /// evaluate as follows, Amazon S3 returns the <code>412 Precondition Failed</code> response
    /// code:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>x-amz-copy-source-if-none-match</code> condition evaluates to false</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-copy-source-if-modified-since</code> condition evaluates to
    /// true</p>
    /// </li>
    /// </ul>
    ///
    /// <note>
    /// <p>All headers with the <code>x-amz-</code> prefix, including
    /// <code>x-amz-copy-source</code>, must be signed.</p>
    /// </note>
    /// <p>
    /// <b>Server-side encryption</b>
    /// </p>
    /// <p>When you perform a CopyObject operation, you can optionally use the appropriate encryption-related
    /// headers to encrypt the object using server-side encryption with Amazon Web Services managed encryption keys
    /// (SSE-S3 or SSE-KMS) or a customer-provided encryption key. With server-side encryption, Amazon S3
    /// encrypts your data as it writes it to disks in its data centers and decrypts the data when
    /// you access it. For more information about server-side encryption, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/serv-side-encryption.html">Using
    /// Server-Side Encryption</a>.</p>
    /// <p>If a target object uses SSE-KMS, you can enable an S3 Bucket Key for the object. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html">Amazon S3 Bucket Keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>
    /// <b>Access Control List (ACL)-Specific Request
    /// Headers</b>
    /// </p>
    /// <p>When copying an object, you can optionally use headers to grant ACL-based permissions.
    /// By default, all objects are private. Only the owner has full access control. When adding a
    /// new object, you can grant permissions to individual Amazon Web Services accounts or to predefined groups
    /// defined by Amazon S3. These permissions are then added to the ACL on the object. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html">Access Control List (ACL) Overview</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-using-rest-api.html">Managing ACLs Using the REST
    /// API</a>. </p>
    /// <p>If the bucket that you're copying objects to uses the bucket owner enforced setting for
    /// S3 Object Ownership, ACLs are disabled and no longer affect permissions. Buckets that
    /// use this setting only accept PUT requests that don't specify an ACL or PUT requests that
    /// specify bucket owner full control ACLs, such as the <code>bucket-owner-full-control</code> canned
    /// ACL or an equivalent form of this ACL expressed in the XML format.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html"> Controlling ownership of
    /// objects and disabling ACLs</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <note>
    /// <p>If your bucket uses the bucket owner enforced setting for Object Ownership,
    /// all objects written to the bucket by any account will be owned by the bucket owner.</p>
    /// </note>
    /// <p>
    /// <b>Checksums</b>
    /// </p>
    /// <p>When copying an object, if it has a checksum, that checksum will be copied to the new object
    /// by default. When you copy the object over, you may optionally specify a different checksum
    /// algorithm to use with the <code>x-amz-checksum-algorithm</code> header.</p>
    /// <p>
    /// <b>Storage Class Options</b>
    /// </p>
    /// <p>You can use the <code>CopyObject</code> action to change the storage class of an
    /// object that is already stored in Amazon S3 using the <code>StorageClass</code> parameter. For
    /// more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html">Storage
    /// Classes</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>
    /// <b>Versioning</b>
    /// </p>
    /// <p>By default, <code>x-amz-copy-source</code> identifies the current version of an object
    /// to copy. If the current version is a delete marker, Amazon S3 behaves as if the object was
    /// deleted. To copy a different version, use the <code>versionId</code> subresource.</p>
    /// <p>If you enable versioning on the target bucket, Amazon S3 generates a unique version ID for
    /// the object being copied. This version ID is different from the version ID of the source
    /// object. Amazon S3 returns the version ID of the copied object in the
    /// <code>x-amz-version-id</code> response header in the response.</p>
    /// <p>If you do not enable versioning or suspend it on the target bucket, the version ID that
    /// Amazon S3 generates is always null.</p>
    /// <p>If the source object's storage class is GLACIER, you must restore a copy of this object
    /// before you can use it as a source object for the copy operation. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_RestoreObject.html">RestoreObject</a>.</p>
    /// <p>The following operations are related to <code>CopyObject</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/CopyingObjectsExamples.html">Copying
    /// Objects</a>.</p>
    async fn copy_object(&self, _input: CopyObjectInput) -> S3Result<CopyObjectOutput> {
        Err(s3_error!(NotImplemented, "CopyObject is not implemented yet"))
    }

    /// <p>Creates a new S3 bucket. To create a bucket, you must register with Amazon S3 and have a
    /// valid Amazon Web Services Access Key ID to authenticate requests. Anonymous requests are never allowed to
    /// create buckets. By creating the bucket, you become the bucket owner.</p>
    /// <p>Not every string is an acceptable bucket name. For information about bucket naming
    /// restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html">Bucket naming rules</a>.</p>
    /// <p>If you want to create an Amazon S3 on Outposts bucket, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_CreateBucket.html">Create Bucket</a>. </p>
    /// <p>By default, the bucket is created in the US East (N. Virginia) Region. You can
    /// optionally specify a Region in the request body. You might choose a Region to optimize
    /// latency, minimize costs, or address regulatory requirements. For example, if you reside in
    /// Europe, you will probably find it advantageous to create buckets in the Europe (Ireland)
    /// Region. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingBucket.html#access-bucket-intro">Accessing a
    /// bucket</a>.</p>
    /// <note>
    /// <p>If you send your create bucket request to the <code>s3.amazonaws.com</code> endpoint,
    /// the request goes to the us-east-1 Region. Accordingly, the signature calculations in
    /// Signature Version 4 must use us-east-1 as the Region, even if the location constraint in
    /// the request specifies another Region where the bucket is to be created. If you create a
    /// bucket in a Region other than US East (N. Virginia), your application must be able to
    /// handle 307 redirect. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/VirtualHosting.html">Virtual hosting of buckets</a>.</p>
    /// </note>
    /// <p>
    /// <b>Access control lists (ACLs)</b>
    /// </p>
    /// <p>When creating a bucket using this operation, you can optionally configure the bucket ACL to specify the accounts or
    /// groups that should be granted specific permissions on the bucket.</p>
    /// <important>
    /// <p>If your CreateBucket request sets bucket owner enforced for S3 Object Ownership and
    /// specifies a bucket ACL that provides access to an external Amazon Web Services account, your request
    /// fails with a <code>400</code> error and returns the
    /// <code>InvalidBucketAclWithObjectOwnership</code> error code. For more information,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">Controlling object
    /// ownership</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    /// <p>There are two ways to grant the appropriate permissions using the request headers.</p>
    /// <ul>
    /// <li>
    /// <p>Specify a canned ACL using the <code>x-amz-acl</code> request header. Amazon S3
    /// supports a set of predefined ACLs, known as <i>canned ACLs</i>. Each
    /// canned ACL has a predefined set of grantees and permissions. For more information,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL">Canned ACL</a>.</p>
    /// </li>
    /// <li>
    /// <p>Specify access permissions explicitly using the <code>x-amz-grant-read</code>,
    /// <code>x-amz-grant-write</code>, <code>x-amz-grant-read-acp</code>,
    /// <code>x-amz-grant-write-acp</code>, and <code>x-amz-grant-full-control</code>
    /// headers. These headers map to the set of permissions Amazon S3 supports in an ACL. For
    /// more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html">Access control list
    /// (ACL) overview</a>.</p>
    /// <p>You specify each grantee as a type=value pair, where the type is one of the
    /// following:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>id</code> – if the value specified is the canonical user ID of an Amazon Web Services account</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>uri</code> – if you are granting permissions to a predefined
    /// group</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>emailAddress</code> – if the value specified is the email address of
    /// an Amazon Web Services account</p>
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
    /// </li>
    /// </ul>
    /// <p>For example, the following <code>x-amz-grant-read</code> header grants the Amazon Web Services accounts identified by account IDs permissions to read object data and its metadata:</p>
    /// <p>
    /// <code>x-amz-grant-read: id="11112222333", id="444455556666" </code>
    /// </p>
    /// </li>
    /// </ul>
    /// <note>
    /// <p>You can use either a canned ACL or specify access permissions explicitly. You cannot
    /// do both.</p>
    /// </note>
    ///
    /// <p>
    /// <b>Permissions</b>
    /// </p>
    /// <p>In addition to <code>s3:CreateBucket</code>, the following permissions are required when your CreateBucket includes specific headers:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <b>ACLs</b> - If your <code>CreateBucket</code> request specifies ACL permissions and the ACL is public-read, public-read-write,
    /// authenticated-read, or if you specify access permissions explicitly through any other ACL, both
    /// <code>s3:CreateBucket</code> and <code>s3:PutBucketAcl</code> permissions are needed. If the ACL the
    /// <code>CreateBucket</code> request is private or doesn't specify any ACLs, only <code>s3:CreateBucket</code> permission is needed. </p>
    /// </li>
    /// <li>
    /// <p>
    /// <b>Object Lock</b> - If
    /// <code>ObjectLockEnabledForBucket</code> is set to true in your
    /// <code>CreateBucket</code> request,
    /// <code>s3:PutBucketObjectLockConfiguration</code> and
    /// <code>s3:PutBucketVersioning</code> permissions are required.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <b>S3 Object Ownership</b> - If your CreateBucket
    /// request includes the the <code>x-amz-object-ownership</code> header,
    /// <code>s3:PutBucketOwnershipControls</code> permission is required.</p>
    /// </li>
    /// </ul>
    /// <p>The following operations are related to <code>CreateBucket</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucket.html">DeleteBucket</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn create_bucket(&self, _input: CreateBucketInput) -> S3Result<CreateBucketOutput> {
        Err(s3_error!(NotImplemented, "CreateBucket is not implemented yet"))
    }

    /// <p>This action initiates a multipart upload and returns an upload ID. This upload ID is
    /// used to associate all of the parts in the specific multipart upload. You specify this
    /// upload ID in each of your subsequent upload part requests (see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a>). You also include this
    /// upload ID in the final request to either complete or abort the multipart upload
    /// request.</p>
    ///
    /// <p>For more information about multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html">Multipart Upload Overview</a>.</p>
    ///
    /// <p>If you have configured a lifecycle rule to abort incomplete multipart uploads, the
    /// upload must complete within the number of days specified in the bucket lifecycle
    /// configuration. Otherwise, the incomplete multipart upload becomes eligible for an abort
    /// action and Amazon S3 aborts the multipart upload. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config">Aborting
    /// Incomplete Multipart Uploads Using a Bucket Lifecycle Policy</a>.</p>
    ///
    /// <p>For information about the permissions required to use the multipart upload API, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuAndPermissions.html">Multipart Upload and
    /// Permissions</a>.</p>
    ///
    /// <p>For request signing, multipart upload is just a series of regular requests. You initiate
    /// a multipart upload, send one or more requests to upload parts, and then complete the
    /// multipart upload process. You sign each request individually. There is nothing special
    /// about signing multipart upload requests. For more information about signing, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html">Authenticating
    /// Requests (Amazon Web Services Signature Version 4)</a>.</p>
    ///
    /// <note>
    /// <p> After you initiate a multipart upload and upload one or more parts, to stop being
    /// charged for storing the uploaded parts, you must either complete or abort the multipart
    /// upload. Amazon S3 frees up the space used to store the parts and stop charging you for
    /// storing them only after you either complete or abort a multipart upload. </p>
    /// </note>
    ///
    /// <p>You can optionally request server-side encryption. For server-side encryption, Amazon S3
    /// encrypts your data as it writes it to disks in its data centers and decrypts it when you
    /// access it. You can provide your own encryption key, or use Amazon Web Services KMS keys or Amazon S3-managed encryption keys. If you choose to provide
    /// your own encryption key, the request headers you provide in <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPartCopy.html">UploadPartCopy</a> requests must match the headers you used in the request to
    /// initiate the upload by using <code>CreateMultipartUpload</code>. </p>
    /// <p>To perform a multipart upload with encryption using an Amazon Web Services KMS key, the requester must
    /// have permission to the <code>kms:Decrypt</code> and <code>kms:GenerateDataKey*</code>
    /// actions on the key. These permissions are required because Amazon S3 must decrypt and read data
    /// from the encrypted file parts before it completes the multipart upload. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/mpuoverview.html#mpuAndPermissions">Multipart upload API
    /// and permissions</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///
    /// <p>If your Identity and Access Management (IAM) user or role is in the same Amazon Web Services account
    /// as the KMS key, then you must have these permissions on the key policy. If your IAM
    /// user or role belongs to a different account than the key, then you must have the
    /// permissions on both the key policy and your IAM user or role.</p>
    ///
    ///
    /// <p> For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/serv-side-encryption.html">Protecting
    /// Data Using Server-Side Encryption</a>.</p>
    ///
    /// <dl>
    /// <dt>Access Permissions</dt>
    /// <dd>
    /// <p>When copying an object, you can optionally specify the accounts or groups that
    /// should be granted specific permissions on the new object. There are two ways to
    /// grant the permissions using the request headers:</p>
    /// <ul>
    /// <li>
    /// <p>Specify a canned ACL with the <code>x-amz-acl</code> request header. For
    /// more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL">Canned ACL</a>.</p>
    /// </li>
    /// <li>
    /// <p>Specify access permissions explicitly with the
    /// <code>x-amz-grant-read</code>, <code>x-amz-grant-read-acp</code>,
    /// <code>x-amz-grant-write-acp</code>, and
    /// <code>x-amz-grant-full-control</code> headers. These parameters map to
    /// the set of permissions that Amazon S3 supports in an ACL. For more information,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html">Access Control List (ACL)
    /// Overview</a>.</p>
    /// </li>
    /// </ul>
    /// <p>You can use either a canned ACL or specify access permissions explicitly. You
    /// cannot do both.</p>
    /// </dd>
    /// <dt>Server-Side- Encryption-Specific Request Headers</dt>
    /// <dd>
    /// <p>You can optionally tell Amazon S3 to encrypt data at rest using server-side
    /// encryption. Server-side encryption is for data encryption at rest. Amazon S3 encrypts
    /// your data as it writes it to disks in its data centers and decrypts it when you
    /// access it. The option you use depends on whether you want to use Amazon Web Services managed
    /// encryption keys or provide your own encryption key. </p>
    /// <ul>
    /// <li>
    /// <p>Use encryption keys managed by Amazon S3 or customer managed key stored
    /// in Amazon Web Services Key Management Service (Amazon Web Services KMS) – If you want Amazon Web Services to manage the keys
    /// used to encrypt data, specify the following headers in the request.</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>x-amz-server-side-encryption</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-server-side-encryption-aws-kms-key-id</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-server-side-encryption-context</code>
    /// </p>
    /// </li>
    /// </ul>
    /// <note>
    /// <p>If you specify <code>x-amz-server-side-encryption:aws:kms</code>, but
    /// don't provide <code>x-amz-server-side-encryption-aws-kms-key-id</code>,
    /// Amazon S3 uses the Amazon Web Services managed key in Amazon Web Services KMS to protect the data.</p>
    /// </note>
    /// <important>
    /// <p>All GET and PUT requests for an object protected by Amazon Web Services KMS fail if
    /// you don't make them with SSL or by using SigV4.</p>
    /// </important>
    /// <p>For more information about server-side encryption with KMS key (SSE-KMS),
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html">Protecting Data Using Server-Side Encryption with KMS keys</a>.</p>
    /// </li>
    /// <li>
    /// <p>Use customer-provided encryption keys – If you want to manage your own
    /// encryption keys, provide all the following headers in the request.</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>x-amz-server-side-encryption-customer-algorithm</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-server-side-encryption-customer-key</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-server-side-encryption-customer-key-MD5</code>
    /// </p>
    /// </li>
    /// </ul>
    /// <p>For more information about server-side encryption with KMS keys (SSE-KMS),
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html">Protecting Data Using Server-Side Encryption with KMS keys</a>.</p>
    /// </li>
    /// </ul>
    /// </dd>
    /// <dt>Access-Control-List (ACL)-Specific Request Headers</dt>
    /// <dd>
    /// <p>You also can use the following access control–related headers with this
    /// operation. By default, all objects are private. Only the owner has full access
    /// control. When adding a new object, you can grant permissions to individual Amazon Web Services accounts or to predefined groups defined by Amazon S3. These permissions are then added
    /// to the access control list (ACL) on the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/S3_ACLs_UsingACLs.html">Using ACLs</a>. With this
    /// operation, you can grant access permissions using one of the following two
    /// methods:</p>
    /// <ul>
    /// <li>
    /// <p>Specify a canned ACL (<code>x-amz-acl</code>) — Amazon S3 supports a set of
    /// predefined ACLs, known as <i>canned ACLs</i>. Each canned ACL
    /// has a predefined set of grantees and permissions. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL">Canned
    /// ACL</a>.</p>
    /// </li>
    /// <li>
    /// <p>Specify access permissions explicitly — To explicitly grant access
    /// permissions to specific Amazon Web Services accounts or groups, use the following headers.
    /// Each header maps to specific permissions that Amazon S3 supports in an ACL. For
    /// more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html">Access
    /// Control List (ACL) Overview</a>. In the header, you specify a list of
    /// grantees who get the specific permission. To grant permissions explicitly,
    /// use:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>x-amz-grant-read</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-grant-write</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-grant-read-acp</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-grant-write-acp</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-grant-full-control</code>
    /// </p>
    /// </li>
    /// </ul>
    /// <p>You specify each grantee as a type=value pair, where the type is one of
    /// the following:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>id</code> – if the value specified is the canonical user ID
    /// of an Amazon Web Services account</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>uri</code> – if you are granting permissions to a predefined
    /// group</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>emailAddress</code> – if the value specified is the email
    /// address of an Amazon Web Services account</p>
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
    /// </li>
    /// </ul>
    /// <p>For example, the following <code>x-amz-grant-read</code> header grants the Amazon Web Services accounts identified by account IDs permissions to read object data and its metadata:</p>
    /// <p>
    /// <code>x-amz-grant-read: id="11112222333", id="444455556666" </code>
    /// </p>
    /// </li>
    /// </ul>
    ///
    /// </dd>
    /// </dl>
    ///
    /// <p>The following operations are related to <code>CreateMultipartUpload</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html">CompleteMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html">AbortMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListMultipartUploads.html">ListMultipartUploads</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn create_multipart_upload(&self, _input: CreateMultipartUploadInput) -> S3Result<CreateMultipartUploadOutput> {
        Err(s3_error!(NotImplemented, "CreateMultipartUpload is not implemented yet"))
    }

    /// <p>Deletes the S3 bucket. All objects (including all object versions and delete markers) in
    /// the bucket must be deleted before the bucket itself can be deleted.</p>
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObject.html">DeleteObject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket(&self, _input: DeleteBucketInput) -> S3Result {
        Err(s3_error!(NotImplemented, "DeleteBucket is not implemented yet"))
    }

    /// <p>Deletes an analytics configuration for the bucket (specified by the analytics
    /// configuration ID).</p>
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:PutAnalyticsConfiguration</code> action. The bucket owner has this permission
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p>For information about the Amazon S3 analytics feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/analytics-storage-class.html">Amazon S3 Analytics – Storage Class
    /// Analysis</a>. </p>
    ///
    /// <p>The following operations are related to
    /// <code>DeleteBucketAnalyticsConfiguration</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketAnalyticsConfiguration.html">GetBucketAnalyticsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketAnalyticsConfigurations.html">ListBucketAnalyticsConfigurations</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketAnalyticsConfiguration.html">PutBucketAnalyticsConfiguration</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_analytics_configuration(&self, _input: DeleteBucketAnalyticsConfigurationInput) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "DeleteBucketAnalyticsConfiguration is not implemented yet"
        ))
    }

    /// <p>Deletes the <code>cors</code> configuration information set for the bucket.</p>
    /// <p>To use this operation, you must have permission to perform the
    /// <code>s3:PutBucketCORS</code> action. The bucket owner has this permission by default
    /// and can grant this permission to others. </p>
    /// <p>For information about <code>cors</code>, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cors.html">Enabling
    /// Cross-Origin Resource Sharing</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///
    /// <p class="title">
    /// <b>Related Resources:</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketCors.html">PutBucketCors</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTOPTIONSobject.html">RESTOPTIONSobject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_cors(&self, _input: DeleteBucketCorsInput) -> S3Result {
        Err(s3_error!(NotImplemented, "DeleteBucketCors is not implemented yet"))
    }

    /// <p>This implementation of the DELETE action removes default encryption from the bucket.
    /// For information about the Amazon S3 default encryption feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 Default Bucket Encryption</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:PutEncryptionConfiguration</code> action. The bucket owner has this permission
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to your Amazon S3
    /// Resources</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketEncryption.html">PutBucketEncryption</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketEncryption.html">GetBucketEncryption</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_encryption(&self, _input: DeleteBucketEncryptionInput) -> S3Result {
        Err(s3_error!(NotImplemented, "DeleteBucketEncryption is not implemented yet"))
    }

    /// <p>Deletes the S3 Intelligent-Tiering configuration from the specified bucket.</p>
    /// <p>The S3 Intelligent-Tiering storage class is designed to optimize storage costs by automatically moving data to the most cost-effective storage access tier, without performance impact or operational overhead. S3 Intelligent-Tiering delivers automatic cost savings in three low latency and high throughput access tiers. To get the lowest storage cost on data that can be accessed in minutes to hours, you can choose to activate additional archiving capabilities.</p>
    /// <p>The S3 Intelligent-Tiering storage class is  the ideal storage class for data with unknown, changing, or unpredictable access patterns, independent of object size or retention period. If the size of an object is less than 128 KB, it is not monitored and not eligible for auto-tiering. Smaller objects can be stored, but they are always charged at the Frequent Access tier rates in the S3 Intelligent-Tiering storage class.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for automatically optimizing frequently and infrequently accessed objects</a>.</p>
    /// <p>Operations related to
    /// <code>DeleteBucketIntelligentTieringConfiguration</code> include: </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketIntelligentTieringConfiguration.html">GetBucketIntelligentTieringConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketIntelligentTieringConfiguration.html">PutBucketIntelligentTieringConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketIntelligentTieringConfigurations.html">ListBucketIntelligentTieringConfigurations</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_intelligent_tiering_configuration(
        &self,
        _input: DeleteBucketIntelligentTieringConfigurationInput,
    ) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "DeleteBucketIntelligentTieringConfiguration is not implemented yet"
        ))
    }

    /// <p>Deletes an inventory configuration (identified by the inventory ID) from the
    /// bucket.</p>
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:PutInventoryConfiguration</code> action. The bucket owner has this permission
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    /// <p>For information about the Amazon S3 inventory feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-inventory.html">Amazon S3 Inventory</a>.</p>
    /// <p>Operations related to <code>DeleteBucketInventoryConfiguration</code> include: </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketInventoryConfiguration.html">GetBucketInventoryConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketInventoryConfiguration.html">PutBucketInventoryConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketInventoryConfigurations.html">ListBucketInventoryConfigurations</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_inventory_configuration(&self, _input: DeleteBucketInventoryConfigurationInput) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "DeleteBucketInventoryConfiguration is not implemented yet"
        ))
    }

    /// <p>Deletes the lifecycle configuration from the specified bucket. Amazon S3 removes all the
    /// lifecycle configuration rules in the lifecycle subresource associated with the bucket. Your
    /// objects never expire, and Amazon S3 no longer automatically deletes any objects on the basis of
    /// rules contained in the deleted lifecycle configuration.</p>
    /// <p>To use this operation, you must have permission to perform the
    /// <code>s3:PutLifecycleConfiguration</code> action. By default, the bucket owner has this
    /// permission and the bucket owner can grant this permission to others.</p>
    ///
    /// <p>There is usually some time lag before lifecycle configuration deletion is fully
    /// propagated to all the Amazon S3 systems.</p>
    ///
    /// <p>For more information about the object expiration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#intro-lifecycle-rules-actions">Elements to
    /// Describe Lifecycle Actions</a>.</p>
    /// <p>Related actions include:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLifecycleConfiguration.html">PutBucketLifecycleConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLifecycleConfiguration.html">GetBucketLifecycleConfiguration</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_lifecycle(&self, _input: DeleteBucketLifecycleInput) -> S3Result {
        Err(s3_error!(NotImplemented, "DeleteBucketLifecycle is not implemented yet"))
    }

    /// <p>Deletes a metrics configuration for the Amazon CloudWatch request metrics (specified by the
    /// metrics configuration ID) from the bucket. Note that this doesn't include the daily storage
    /// metrics.</p>
    ///
    /// <p> To use this operation, you must have permissions to perform the
    /// <code>s3:PutMetricsConfiguration</code> action. The bucket owner has this permission by
    /// default. The bucket owner can grant this permission to others. For more information about
    /// permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p>For information about CloudWatch request metrics for Amazon S3, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cloudwatch-monitoring.html">Monitoring Metrics with Amazon CloudWatch</a>. </p>
    /// <p>The following operations are related to
    /// <code>DeleteBucketMetricsConfiguration</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketMetricsConfiguration.html">GetBucketMetricsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketMetricsConfiguration.html">PutBucketMetricsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketMetricsConfigurations.html">ListBucketMetricsConfigurations</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cloudwatch-monitoring.html">Monitoring Metrics with Amazon
    /// CloudWatch</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_metrics_configuration(&self, _input: DeleteBucketMetricsConfigurationInput) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "DeleteBucketMetricsConfiguration is not implemented yet"
        ))
    }

    /// <p>Removes <code>OwnershipControls</code> for an Amazon S3 bucket. To use this operation, you
    /// must have the <code>s3:PutBucketOwnershipControls</code> permission. For more information
    /// about Amazon S3 permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying
    /// Permissions in a Policy</a>.</p>
    /// <p>For information about Amazon S3 Object Ownership, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/about-object-ownership.html">Using Object Ownership</a>. </p>
    /// <p>The following operations are related to
    /// <code>DeleteBucketOwnershipControls</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a>GetBucketOwnershipControls</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a>PutBucketOwnershipControls</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_ownership_controls(&self, _input: DeleteBucketOwnershipControlsInput) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "DeleteBucketOwnershipControls is not implemented yet"
        ))
    }

    /// <p>This implementation of the DELETE action uses the policy subresource to delete the
    /// policy of a specified bucket. If you are using an identity other than the root user of the
    /// Amazon Web Services account that owns the bucket, the calling identity must have the
    /// <code>DeleteBucketPolicy</code> permissions on the specified bucket and belong to the
    /// bucket owner's account to use this operation. </p>
    ///
    /// <p>If you don't have <code>DeleteBucketPolicy</code> permissions, Amazon S3 returns a <code>403
    /// Access Denied</code> error. If you have the correct permissions, but you're not using an
    /// identity that belongs to the bucket owner's account, Amazon S3 returns a <code>405 Method Not
    /// Allowed</code> error. </p>
    ///
    /// <important>
    /// <p>As a security precaution, the root user of the Amazon Web Services account that owns a bucket can
    /// always use this operation, even if the policy explicitly denies the root user the
    /// ability to perform this action.</p>
    /// </important>
    ///
    /// <p>For more information about bucket policies, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-iam-policies.html">Using Bucket Policies and
    /// UserPolicies</a>. </p>
    /// <p>The following operations are related to <code>DeleteBucketPolicy</code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObject.html">DeleteObject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_policy(&self, _input: DeleteBucketPolicyInput) -> S3Result {
        Err(s3_error!(NotImplemented, "DeleteBucketPolicy is not implemented yet"))
    }

    /// <p> Deletes the replication configuration from the bucket.</p>
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:PutReplicationConfiguration</code> action. The bucket owner has these
    /// permissions by default and can grant it to others. For more information about permissions,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>. </p>
    /// <note>
    /// <p>It can take a while for the deletion of a replication configuration to fully
    /// propagate.</p>
    /// </note>
    ///
    /// <p> For information about replication configuration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication.html">Replication</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///
    /// <p>The following operations are related to <code>DeleteBucketReplication</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketReplication.html">PutBucketReplication</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketReplication.html">GetBucketReplication</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_replication(&self, _input: DeleteBucketReplicationInput) -> S3Result {
        Err(s3_error!(NotImplemented, "DeleteBucketReplication is not implemented yet"))
    }

    /// <p>Deletes the tags from the bucket.</p>
    ///
    /// <p>To use this operation, you must have permission to perform the
    /// <code>s3:PutBucketTagging</code> action. By default, the bucket owner has this
    /// permission and can grant this permission to others. </p>
    /// <p>The following operations are related to <code>DeleteBucketTagging</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketTagging.html">GetBucketTagging</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketTagging.html">PutBucketTagging</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_tagging(&self, _input: DeleteBucketTaggingInput) -> S3Result {
        Err(s3_error!(NotImplemented, "DeleteBucketTagging is not implemented yet"))
    }

    /// <p>This action removes the website configuration for a bucket. Amazon S3 returns a 200
    /// OK response upon successfully deleting a website configuration on the specified
    /// bucket. You will get a <code>200 OK</code> response if the website configuration you are
    /// trying to delete does not exist on the bucket. Amazon S3 returns a <code>404</code> response if
    /// the bucket specified in the request does not exist.</p>
    ///
    /// <p>This DELETE action requires the <code>S3:DeleteBucketWebsite</code> permission. By
    /// default, only the bucket owner can delete the website configuration attached to a bucket.
    /// However, bucket owners can grant other users permission to delete the website configuration
    /// by writing a bucket policy granting them the <code>S3:DeleteBucketWebsite</code>
    /// permission. </p>
    ///
    /// <p>For more information about hosting websites, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html">Hosting Websites on Amazon S3</a>. </p>
    ///
    /// <p>The following operations are related to <code>DeleteBucketWebsite</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketWebsite.html">GetBucketWebsite</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketWebsite.html">PutBucketWebsite</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_bucket_website(&self, _input: DeleteBucketWebsiteInput) -> S3Result {
        Err(s3_error!(NotImplemented, "DeleteBucketWebsite is not implemented yet"))
    }

    /// <p>Removes the null version (if there is one) of an object and inserts a delete marker,
    /// which becomes the latest version of the object. If there isn't a null version, Amazon S3 does
    /// not remove any objects but will still respond that the command was successful.</p>
    ///
    /// <p>To remove a specific version, you must be the bucket owner and you must use the version
    /// Id subresource. Using this subresource permanently deletes the version. If the object
    /// deleted is a delete marker, Amazon S3 sets the response header,
    /// <code>x-amz-delete-marker</code>, to true. </p>
    ///
    /// <p>If the object you want to delete is in a bucket where the bucket versioning
    /// configuration is MFA Delete enabled, you must include the <code>x-amz-mfa</code> request
    /// header in the DELETE <code>versionId</code> request. Requests that include
    /// <code>x-amz-mfa</code> must use HTTPS. </p>
    ///
    /// <p> For more information about MFA Delete, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMFADelete.html">Using MFA Delete</a>. To see sample requests that use versioning, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectDELETE.html#ExampleVersionObjectDelete">Sample Request</a>. </p>
    ///
    /// <p>You can delete objects by explicitly calling DELETE Object or configure its
    /// lifecycle (<a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLifecycle.html">PutBucketLifecycle</a>) to
    /// enable Amazon S3 to remove them for you. If you want to block users or accounts from removing or
    /// deleting objects from your bucket, you must deny them the <code>s3:DeleteObject</code>,
    /// <code>s3:DeleteObjectVersion</code>, and <code>s3:PutLifeCycleConfiguration</code>
    /// actions. </p>
    ///
    /// <p>The following action is related to <code>DeleteObject</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_object(&self, _input: DeleteObjectInput) -> S3Result<DeleteObjectOutput> {
        Err(s3_error!(NotImplemented, "DeleteObject is not implemented yet"))
    }

    /// <p>Removes the entire tag set from the specified object. For more information about
    /// managing object tags, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-tagging.html"> Object
    /// Tagging</a>.</p>
    ///
    /// <p>To use this operation, you must have permission to perform the
    /// <code>s3:DeleteObjectTagging</code> action.</p>
    ///
    /// <p>To delete tags of a specific object version, add the <code>versionId</code> query
    /// parameter in the request. You will need permission for the
    /// <code>s3:DeleteObjectVersionTagging</code> action.</p>
    ///
    /// <p>The following operations are related to
    /// <code>DeleteBucketMetricsConfiguration</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObjectTagging.html">PutObjectTagging</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectTagging.html">GetObjectTagging</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_object_tagging(&self, _input: DeleteObjectTaggingInput) -> S3Result<DeleteObjectTaggingOutput> {
        Err(s3_error!(NotImplemented, "DeleteObjectTagging is not implemented yet"))
    }

    /// <p>This action enables you to delete multiple objects from a bucket using a single HTTP
    /// request. If you know the object keys that you want to delete, then this action provides
    /// a suitable alternative to sending individual delete requests, reducing per-request
    /// overhead.</p>
    ///
    /// <p>The request contains a list of up to 1000 keys that you want to delete. In the XML, you
    /// provide the object key names, and optionally, version IDs if you want to delete a specific
    /// version of the object from a versioning-enabled bucket. For each key, Amazon S3 performs a
    /// delete action and returns the result of that delete, success, or failure, in the
    /// response. Note that if the object specified in the request is not found, Amazon S3 returns the
    /// result as deleted.</p>
    ///
    /// <p> The action supports two modes for the response: verbose and quiet. By default, the
    /// action uses verbose mode in which the response includes the result of deletion of each
    /// key in your request. In quiet mode the response includes only keys where the delete
    /// action encountered an error. For a successful deletion, the action does not return
    /// any information about the delete in the response body.</p>
    ///
    /// <p>When performing this action on an MFA Delete enabled bucket, that attempts to delete
    /// any versioned objects, you must include an MFA token. If you do not provide one, the entire
    /// request will fail, even if there are non-versioned objects you are trying to delete. If you
    /// provide an invalid token, whether there are versioned keys in the request or not, the
    /// entire Multi-Object Delete request will fail. For information about MFA Delete, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/Versioning.html#MultiFactorAuthenticationDelete"> MFA
    /// Delete</a>.</p>
    ///
    /// <p>Finally, the Content-MD5 header is required for all Multi-Object Delete requests. Amazon
    /// S3 uses the header value to ensure that your request body has not been altered in
    /// transit.</p>
    ///
    /// <p>The following operations are related to <code>DeleteObjects</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html">CompleteMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html">AbortMultipartUpload</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_objects(&self, _input: DeleteObjectsInput) -> S3Result<DeleteObjectsOutput> {
        Err(s3_error!(NotImplemented, "DeleteObjects is not implemented yet"))
    }

    /// <p>Removes the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket. To use this
    /// operation, you must have the <code>s3:PutBucketPublicAccessBlock</code> permission. For
    /// more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p>The following operations are related to <code>DeletePublicAccessBlock</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html">Using Amazon S3 Block
    /// Public Access</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetPublicAccessBlock.html">GetPublicAccessBlock</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutPublicAccessBlock.html">PutPublicAccessBlock</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketPolicyStatus.html">GetBucketPolicyStatus</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn delete_public_access_block(&self, _input: DeletePublicAccessBlockInput) -> S3Result {
        Err(s3_error!(NotImplemented, "DeletePublicAccessBlock is not implemented yet"))
    }

    /// <p>This implementation of the GET action uses the <code>accelerate</code> subresource to
    /// return the Transfer Acceleration state of a bucket, which is either <code>Enabled</code> or
    /// <code>Suspended</code>. Amazon S3 Transfer Acceleration is a bucket-level feature that
    /// enables you to perform faster data transfers to and from Amazon S3.</p>
    /// <p>To use this operation, you must have permission to perform the
    /// <code>s3:GetAccelerateConfiguration</code> action. The bucket owner has this permission
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to your Amazon S3
    /// Resources</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>You set the Transfer Acceleration state of an existing bucket to <code>Enabled</code> or
    /// <code>Suspended</code> by using the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketAccelerateConfiguration.html">PutBucketAccelerateConfiguration</a> operation. </p>
    /// <p>A GET <code>accelerate</code> request does not return a state value for a bucket that
    /// has no transfer acceleration state. A bucket has no Transfer Acceleration state if a state
    /// has never been set on the bucket. </p>
    ///
    /// <p>For more information about transfer acceleration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/transfer-acceleration.html">Transfer Acceleration</a> in the
    /// Amazon S3 User Guide.</p>
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketAccelerateConfiguration.html">PutBucketAccelerateConfiguration</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_accelerate_configuration(
        &self,
        _input: GetBucketAccelerateConfigurationInput,
    ) -> S3Result<GetBucketAccelerateConfigurationOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketAccelerateConfiguration is not implemented yet"
        ))
    }

    /// <p>This implementation of the <code>GET</code> action uses the <code>acl</code>
    /// subresource to return the access control list (ACL) of a bucket. To use <code>GET</code> to
    /// return the ACL of the bucket, you must have <code>READ_ACP</code> access to the bucket. If
    /// <code>READ_ACP</code> permission is granted to the anonymous user, you can return the
    /// ACL of the bucket without using an authorization header.</p>
    /// <note>
    /// <p>If your bucket uses the bucket owner enforced setting for S3 Object Ownership,
    /// requests to read ACLs are still supported and return the <code>bucket-owner-full-control</code>
    /// ACL with the owner being the account that created the bucket. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">
    /// Controlling object ownership and disabling ACLs</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </note>
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjects.html">ListObjects</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_acl(&self, _input: GetBucketAclInput) -> S3Result<GetBucketAclOutput> {
        Err(s3_error!(NotImplemented, "GetBucketAcl is not implemented yet"))
    }

    /// <p>This implementation of the GET action returns an analytics configuration (identified
    /// by the analytics configuration ID) from the bucket.</p>
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:GetAnalyticsConfiguration</code> action. The bucket owner has this permission
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources"> Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a> in the <i>Amazon S3 User Guide</i>. </p>
    /// <p>For information about Amazon S3 analytics feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/analytics-storage-class.html">Amazon S3 Analytics – Storage Class
    /// Analysis</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketAnalyticsConfiguration.html">DeleteBucketAnalyticsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketAnalyticsConfigurations.html">ListBucketAnalyticsConfigurations</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketAnalyticsConfiguration.html">PutBucketAnalyticsConfiguration</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_analytics_configuration(
        &self,
        _input: GetBucketAnalyticsConfigurationInput,
    ) -> S3Result<GetBucketAnalyticsConfigurationOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketAnalyticsConfiguration is not implemented yet"
        ))
    }

    /// <p>Returns the Cross-Origin Resource Sharing (CORS) configuration information set for the
    /// bucket.</p>
    ///
    /// <p> To use this operation, you must have permission to perform the
    /// <code>s3:GetBucketCORS</code> action. By default, the bucket owner has this permission
    /// and can grant it to others.</p>
    ///
    /// <p> For more information about CORS, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cors.html"> Enabling Cross-Origin Resource
    /// Sharing</a>.</p>
    ///
    /// <p>The following operations are related to <code>GetBucketCors</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketCors.html">PutBucketCors</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketCors.html">DeleteBucketCors</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_cors(&self, _input: GetBucketCorsInput) -> S3Result<GetBucketCorsOutput> {
        Err(s3_error!(NotImplemented, "GetBucketCors is not implemented yet"))
    }

    /// <p>Returns the default encryption configuration for an Amazon S3 bucket. If the bucket does not
    /// have a default encryption configuration, GetBucketEncryption returns
    /// <code>ServerSideEncryptionConfigurationNotFoundError</code>. </p>
    /// <p>For information about the Amazon S3 default encryption feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 Default Bucket Encryption</a>.</p>
    /// <p> To use this operation, you must have permission to perform the
    /// <code>s3:GetEncryptionConfiguration</code> action. The bucket owner has this permission
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    /// <p>The following operations are related to <code>GetBucketEncryption</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketEncryption.html">PutBucketEncryption</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketEncryption.html">DeleteBucketEncryption</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_encryption(&self, _input: GetBucketEncryptionInput) -> S3Result<GetBucketEncryptionOutput> {
        Err(s3_error!(NotImplemented, "GetBucketEncryption is not implemented yet"))
    }

    /// <p>Gets the S3 Intelligent-Tiering configuration from the specified bucket.</p>
    /// <p>The S3 Intelligent-Tiering storage class is designed to optimize storage costs by automatically moving data to the most cost-effective storage access tier, without performance impact or operational overhead. S3 Intelligent-Tiering delivers automatic cost savings in three low latency and high throughput access tiers. To get the lowest storage cost on data that can be accessed in minutes to hours, you can choose to activate additional archiving capabilities.</p>
    /// <p>The S3 Intelligent-Tiering storage class is  the ideal storage class for data with unknown, changing, or unpredictable access patterns, independent of object size or retention period. If the size of an object is less than 128 KB, it is not monitored and not eligible for auto-tiering. Smaller objects can be stored, but they are always charged at the Frequent Access tier rates in the S3 Intelligent-Tiering storage class.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for automatically optimizing frequently and infrequently accessed objects</a>.</p>
    /// <p>Operations related to
    /// <code>GetBucketIntelligentTieringConfiguration</code> include: </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketIntelligentTieringConfiguration.html">DeleteBucketIntelligentTieringConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketIntelligentTieringConfiguration.html">PutBucketIntelligentTieringConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketIntelligentTieringConfigurations.html">ListBucketIntelligentTieringConfigurations</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_intelligent_tiering_configuration(
        &self,
        _input: GetBucketIntelligentTieringConfigurationInput,
    ) -> S3Result<GetBucketIntelligentTieringConfigurationOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketIntelligentTieringConfiguration is not implemented yet"
        ))
    }

    /// <p>Returns an inventory configuration (identified by the inventory configuration ID) from
    /// the bucket.</p>
    ///
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:GetInventoryConfiguration</code> action. The bucket owner has this permission
    /// by default and can grant this permission to others. For more information about permissions,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p>For information about the Amazon S3 inventory feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-inventory.html">Amazon S3 Inventory</a>.</p>
    ///
    /// <p>The following operations are related to
    /// <code>GetBucketInventoryConfiguration</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketInventoryConfiguration.html">DeleteBucketInventoryConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketInventoryConfigurations.html">ListBucketInventoryConfigurations</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketInventoryConfiguration.html">PutBucketInventoryConfiguration</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_inventory_configuration(
        &self,
        _input: GetBucketInventoryConfigurationInput,
    ) -> S3Result<GetBucketInventoryConfigurationOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketInventoryConfiguration is not implemented yet"
        ))
    }

    /// <note>
    /// <p>Bucket lifecycle configuration now supports specifying a lifecycle rule using an
    /// object key name prefix, one or more object tags, or a combination of both. Accordingly,
    /// this section describes the latest API. The response describes the new filter element
    /// that you can use to specify a filter to select a subset of objects to which the rule
    /// applies. If you are using a previous version of the lifecycle configuration, it still
    /// works. For the earlier action, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLifecycle.html">GetBucketLifecycle</a>.</p>
    /// </note>
    /// <p>Returns the lifecycle configuration information set on the bucket. For information about
    /// lifecycle configuration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html">Object
    /// Lifecycle Management</a>.</p>
    ///
    /// <p>To use this operation, you must have permission to perform the
    /// <code>s3:GetLifecycleConfiguration</code> action. The bucket owner has this permission,
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p>
    /// <code>GetBucketLifecycleConfiguration</code> has the following special error:</p>
    /// <ul>
    /// <li>
    /// <p>Error code: <code>NoSuchLifecycleConfiguration</code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Description: The lifecycle configuration does not exist.</p>
    /// </li>
    /// <li>
    /// <p>HTTP Status Code: 404 Not Found</p>
    /// </li>
    /// <li>
    /// <p>SOAP Fault Code Prefix: Client</p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    /// <p>The following operations are related to
    /// <code>GetBucketLifecycleConfiguration</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLifecycle.html">GetBucketLifecycle</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLifecycle.html">PutBucketLifecycle</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketLifecycle.html">DeleteBucketLifecycle</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_lifecycle_configuration(
        &self,
        _input: GetBucketLifecycleConfigurationInput,
    ) -> S3Result<GetBucketLifecycleConfigurationOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketLifecycleConfiguration is not implemented yet"
        ))
    }

    /// <p>Returns the Region the bucket resides in. You set the bucket's Region using the
    /// <code>LocationConstraint</code> request parameter in a <code>CreateBucket</code>
    /// request. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>.</p>
    ///
    /// <p>To use this implementation of the operation, you must be the bucket owner.</p>
    ///
    /// <p>To use this API against an access point, provide the alias of the access point in place of the bucket name.</p>
    ///
    /// <p>The following operations are related to <code>GetBucketLocation</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_location(&self, _input: GetBucketLocationInput) -> S3Result<GetBucketLocationOutput> {
        Err(s3_error!(NotImplemented, "GetBucketLocation is not implemented yet"))
    }

    /// <p>Returns the logging status of a bucket and the permissions users have to view and modify
    /// that status. To use GET, you must be the bucket owner.</p>
    ///
    /// <p>The following operations are related to <code>GetBucketLogging</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLogging.html">PutBucketLogging</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_logging(&self, _input: GetBucketLoggingInput) -> S3Result<GetBucketLoggingOutput> {
        Err(s3_error!(NotImplemented, "GetBucketLogging is not implemented yet"))
    }

    /// <p>Gets a metrics configuration (specified by the metrics configuration ID) from the
    /// bucket. Note that this doesn't include the daily storage metrics.</p>
    ///
    /// <p> To use this operation, you must have permissions to perform the
    /// <code>s3:GetMetricsConfiguration</code> action. The bucket owner has this permission by
    /// default. The bucket owner can grant this permission to others. For more information about
    /// permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p> For information about CloudWatch request metrics for Amazon S3, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cloudwatch-monitoring.html">Monitoring Metrics with Amazon
    /// CloudWatch</a>.</p>
    ///
    /// <p>The following operations are related to
    /// <code>GetBucketMetricsConfiguration</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketMetricsConfiguration.html">PutBucketMetricsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketMetricsConfiguration.html">DeleteBucketMetricsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketMetricsConfigurations.html">ListBucketMetricsConfigurations</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cloudwatch-monitoring.html">Monitoring Metrics with Amazon
    /// CloudWatch</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_metrics_configuration(
        &self,
        _input: GetBucketMetricsConfigurationInput,
    ) -> S3Result<GetBucketMetricsConfigurationOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketMetricsConfiguration is not implemented yet"
        ))
    }

    /// <p>Returns the notification configuration of a bucket.</p>
    /// <p>If notifications are not enabled on the bucket, the action returns an empty
    /// <code>NotificationConfiguration</code> element.</p>
    ///
    /// <p>By default, you must be the bucket owner to read the notification configuration of a
    /// bucket. However, the bucket owner can use a bucket policy to grant permission to other
    /// users to read this configuration with the <code>s3:GetBucketNotification</code>
    /// permission.</p>
    ///
    /// <p>For more information about setting and reading the notification configuration on a
    /// bucket, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Setting Up Notification of
    /// Bucket Events</a>. For more information about bucket policies, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-iam-policies.html">Using Bucket Policies</a>.</p>
    ///
    /// <p>The following action is related to <code>GetBucketNotification</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketNotification.html">PutBucketNotification</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_notification_configuration(
        &self,
        _input: GetBucketNotificationConfigurationInput,
    ) -> S3Result<NotificationConfiguration> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketNotificationConfiguration is not implemented yet"
        ))
    }

    /// <p>Retrieves <code>OwnershipControls</code> for an Amazon S3 bucket. To use this operation, you
    /// must have the <code>s3:GetBucketOwnershipControls</code> permission. For more information
    /// about Amazon S3 permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html">Specifying
    /// permissions in a policy</a>. </p>
    /// <p>For information about Amazon S3 Object Ownership, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">Using Object Ownership</a>. </p>
    /// <p>The following operations are related to <code>GetBucketOwnershipControls</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a>PutBucketOwnershipControls</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a>DeleteBucketOwnershipControls</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_ownership_controls(
        &self,
        _input: GetBucketOwnershipControlsInput,
    ) -> S3Result<GetBucketOwnershipControlsOutput> {
        Err(s3_error!(NotImplemented, "GetBucketOwnershipControls is not implemented yet"))
    }

    /// <p>Returns the policy of a specified bucket. If you are using an identity other than the
    /// root user of the Amazon Web Services account that owns the bucket, the calling identity must have the
    /// <code>GetBucketPolicy</code> permissions on the specified bucket and belong to the
    /// bucket owner's account in order to use this operation.</p>
    ///
    /// <p>If you don't have <code>GetBucketPolicy</code> permissions, Amazon S3 returns a <code>403
    /// Access Denied</code> error. If you have the correct permissions, but you're not using an
    /// identity that belongs to the bucket owner's account, Amazon S3 returns a <code>405 Method Not
    /// Allowed</code> error.</p>
    ///
    /// <important>
    /// <p>As a security precaution, the root user of the Amazon Web Services account that owns a bucket can
    /// always use this operation, even if the policy explicitly denies the root user the
    /// ability to perform this action.</p>
    /// </important>
    ///
    /// <p>For more information about bucket policies, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-iam-policies.html">Using Bucket Policies and User
    /// Policies</a>.</p>
    ///
    /// <p>The following action is related to <code>GetBucketPolicy</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_policy(&self, _input: GetBucketPolicyInput) -> S3Result<GetBucketPolicyOutput> {
        Err(s3_error!(NotImplemented, "GetBucketPolicy is not implemented yet"))
    }

    /// <p>Retrieves the policy status for an Amazon S3 bucket, indicating whether the bucket is public.
    /// In order to use this operation, you must have the <code>s3:GetBucketPolicyStatus</code>
    /// permission. For more information about Amazon S3 permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying Permissions in a
    /// Policy</a>.</p>
    ///
    /// <p> For more information about when Amazon S3 considers a bucket public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a>. </p>
    ///
    /// <p>The following operations are related to <code>GetBucketPolicyStatus</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html">Using Amazon S3 Block
    /// Public Access</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetPublicAccessBlock.html">GetPublicAccessBlock</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutPublicAccessBlock.html">PutPublicAccessBlock</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeletePublicAccessBlock.html">DeletePublicAccessBlock</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_policy_status(&self, _input: GetBucketPolicyStatusInput) -> S3Result<GetBucketPolicyStatusOutput> {
        Err(s3_error!(NotImplemented, "GetBucketPolicyStatus is not implemented yet"))
    }

    /// <p>Returns the replication configuration of a bucket.</p>
    /// <note>
    /// <p> It can take a while to propagate the put or delete a replication configuration to
    /// all Amazon S3 systems. Therefore, a get request soon after put or delete can return a wrong
    /// result. </p>
    /// </note>
    /// <p> For information about replication configuration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication.html">Replication</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    ///
    /// <p>This action requires permissions for the <code>s3:GetReplicationConfiguration</code>
    /// action. For more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-iam-policies.html">Using Bucket Policies and User
    /// Policies</a>.</p>
    ///
    /// <p>If you include the <code>Filter</code> element in a replication configuration, you must
    /// also include the <code>DeleteMarkerReplication</code> and <code>Priority</code> elements.
    /// The response also returns those elements.</p>
    ///
    /// <p>For information about <code>GetBucketReplication</code> errors, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html#ReplicationErrorCodeList">List of
    /// replication-related error codes</a>
    /// </p>
    ///
    ///
    /// <p>The following operations are related to <code>GetBucketReplication</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketReplication.html">PutBucketReplication</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketReplication.html">DeleteBucketReplication</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_replication(&self, _input: GetBucketReplicationInput) -> S3Result<GetBucketReplicationOutput> {
        Err(s3_error!(NotImplemented, "GetBucketReplication is not implemented yet"))
    }

    /// <p>Returns the request payment configuration of a bucket. To use this version of the
    /// operation, you must be the bucket owner. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html">Requester Pays Buckets</a>.</p>
    ///
    /// <p>The following operations are related to <code>GetBucketRequestPayment</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjects.html">ListObjects</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_request_payment(&self, _input: GetBucketRequestPaymentInput) -> S3Result<GetBucketRequestPaymentOutput> {
        Err(s3_error!(NotImplemented, "GetBucketRequestPayment is not implemented yet"))
    }

    /// <p>Returns the tag set associated with the bucket.</p>
    /// <p>To use this operation, you must have permission to perform the
    /// <code>s3:GetBucketTagging</code> action. By default, the bucket owner has this
    /// permission and can grant this permission to others.</p>
    ///
    /// <p>
    /// <code>GetBucketTagging</code> has the following special error:</p>
    /// <ul>
    /// <li>
    /// <p>Error code: <code>NoSuchTagSet</code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Description: There is no tag set associated with the bucket.</p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    ///
    /// <p>The following operations are related to <code>GetBucketTagging</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketTagging.html">PutBucketTagging</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketTagging.html">DeleteBucketTagging</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_tagging(&self, _input: GetBucketTaggingInput) -> S3Result<GetBucketTaggingOutput> {
        Err(s3_error!(NotImplemented, "GetBucketTagging is not implemented yet"))
    }

    /// <p>Returns the versioning state of a bucket.</p>
    /// <p>To retrieve the versioning state of a bucket, you must be the bucket owner.</p>
    ///
    /// <p>This implementation also returns the MFA Delete status of the versioning state. If the
    /// MFA Delete status is <code>enabled</code>, the bucket owner must use an authentication
    /// device to change the versioning state of the bucket.</p>
    ///
    /// <p>The following operations are related to <code>GetBucketVersioning</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObject.html">DeleteObject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_versioning(&self, _input: GetBucketVersioningInput) -> S3Result<GetBucketVersioningOutput> {
        Err(s3_error!(NotImplemented, "GetBucketVersioning is not implemented yet"))
    }

    /// <p>Returns the website configuration for a bucket. To host website on Amazon S3, you can
    /// configure a bucket as website by adding a website configuration. For more information about
    /// hosting websites, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html">Hosting Websites on
    /// Amazon S3</a>. </p>
    /// <p>This GET action requires the <code>S3:GetBucketWebsite</code> permission. By default,
    /// only the bucket owner can read the bucket website configuration. However, bucket owners can
    /// allow other users to read the website configuration by writing a bucket policy granting
    /// them the <code>S3:GetBucketWebsite</code> permission.</p>
    /// <p>The following operations are related to <code>DeleteBucketWebsite</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketWebsite.html">DeleteBucketWebsite</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketWebsite.html">PutBucketWebsite</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_bucket_website(&self, _input: GetBucketWebsiteInput) -> S3Result<GetBucketWebsiteOutput> {
        Err(s3_error!(NotImplemented, "GetBucketWebsite is not implemented yet"))
    }

    /// <p>Retrieves objects from Amazon S3. To use <code>GET</code>, you must have <code>READ</code>
    /// access to the object. If you grant <code>READ</code> access to the anonymous user, you can
    /// return the object without using an authorization header.</p>
    ///
    /// <p>An Amazon S3 bucket has no directory hierarchy such as you would find in a typical computer
    /// file system. You can, however, create a logical hierarchy by using object key names that
    /// imply a folder structure. For example, instead of naming an object <code>sample.jpg</code>,
    /// you can name it <code>photos/2006/February/sample.jpg</code>.</p>
    ///
    /// <p>To get an object from such a logical hierarchy, specify the full key name for the object
    /// in the <code>GET</code> operation. For a virtual hosted-style request example, if you have
    /// the object <code>photos/2006/February/sample.jpg</code>, specify the resource as
    /// <code>/photos/2006/February/sample.jpg</code>. For a path-style request example, if you
    /// have the object <code>photos/2006/February/sample.jpg</code> in the bucket named
    /// <code>examplebucket</code>, specify the resource as
    /// <code>/examplebucket/photos/2006/February/sample.jpg</code>. For more information about
    /// request types, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/VirtualHosting.html#VirtualHostingSpecifyBucket">HTTP Host Header Bucket Specification</a>.</p>
    ///
    /// <p>For more information about returning the ACL of an object, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAcl.html">GetObjectAcl</a>.</p>
    ///
    /// <p>If the object you are retrieving is stored in the S3 Glacier or
    /// S3 Glacier Deep Archive storage class, or S3 Intelligent-Tiering Archive or
    /// S3 Intelligent-Tiering Deep Archive tiers, before you can retrieve the object you must first restore a
    /// copy using <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_RestoreObject.html">RestoreObject</a>. Otherwise, this action returns an
    /// <code>InvalidObjectStateError</code> error. For information about restoring archived
    /// objects, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/restoring-objects.html">Restoring Archived
    /// Objects</a>.</p>
    ///
    /// <p>Encryption request headers, like <code>x-amz-server-side-encryption</code>, should not
    /// be sent for GET requests if your object uses server-side encryption with KMS keys (SSE-KMS)
    /// or server-side encryption with Amazon S3–managed encryption keys (SSE-S3). If your
    /// object does use these types of keys, you’ll get an HTTP 400 BadRequest error.</p>
    /// <p>If you encrypt an object by using server-side encryption with customer-provided
    /// encryption keys (SSE-C) when you store the object in Amazon S3, then when you GET the object,
    /// you must use the following headers:</p>
    /// <ul>
    /// <li>
    /// <p>x-amz-server-side-encryption-customer-algorithm</p>
    /// </li>
    /// <li>
    /// <p>x-amz-server-side-encryption-customer-key</p>
    /// </li>
    /// <li>
    /// <p>x-amz-server-side-encryption-customer-key-MD5</p>
    /// </li>
    /// </ul>
    /// <p>For more information about SSE-C, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Server-Side Encryption (Using
    /// Customer-Provided Encryption Keys)</a>.</p>
    ///
    /// <p>Assuming you have the relevant permission to read object tags, the response also returns the
    /// <code>x-amz-tagging-count</code> header that provides the count of number of tags
    /// associated with the object. You can use <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectTagging.html">GetObjectTagging</a> to retrieve
    /// the tag set associated with an object.</p>
    ///
    /// <p>
    /// <b>Permissions</b>
    /// </p>
    /// <p>You need the relevant read object (or version) permission for this operation. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying Permissions
    /// in a Policy</a>. If the object you request does not exist, the error Amazon S3 returns
    /// depends on whether you also have the <code>s3:ListBucket</code> permission.</p>
    /// <ul>
    /// <li>
    /// <p>If you have the <code>s3:ListBucket</code> permission on the bucket, Amazon S3 will
    /// return an HTTP status code 404 ("no such key") error.</p>
    /// </li>
    /// <li>
    /// <p>If you don’t have the <code>s3:ListBucket</code> permission, Amazon S3 will return an
    /// HTTP status code 403 ("access denied") error.</p>
    /// </li>
    /// </ul>
    ///
    ///
    /// <p>
    /// <b>Versioning</b>
    /// </p>
    /// <p>By default, the GET action returns the current version of an object. To return a
    /// different version, use the <code>versionId</code> subresource.</p>
    ///
    /// <note>
    /// <ul>
    /// <li>
    /// <p>
    /// If you supply a <code>versionId</code>, you need the <code>s3:GetObjectVersion</code> permission to
    /// access a specific version of an object. If you request a specific version, you do not need to have
    /// the <code>s3:GetObject</code> permission.
    /// </p>
    /// </li>
    /// <li>
    /// <p>If the current version of the object is a delete marker, Amazon S3 behaves as if the
    /// object was deleted and includes <code>x-amz-delete-marker: true</code> in the
    /// response.</p>
    /// </li>
    /// </ul>
    /// </note>
    ///
    ///
    /// <p>For more information about versioning, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketVersioning.html">PutBucketVersioning</a>. </p>
    ///
    /// <p>
    /// <b>Overriding Response Header Values</b>
    /// </p>
    /// <p>There are times when you want to override certain response header values in a GET
    /// response. For example, you might override the <code>Content-Disposition</code> response
    /// header value in your GET request.</p>
    ///
    /// <p>You can override values for a set of response headers using the following query
    /// parameters. These response header values are sent only on a successful request, that is,
    /// when status code 200 OK is returned. The set of headers you can override using these
    /// parameters is a subset of the headers that Amazon S3 accepts when you create an object. The
    /// response headers that you can override for the GET response are <code>Content-Type</code>,
    /// <code>Content-Language</code>, <code>Expires</code>, <code>Cache-Control</code>,
    /// <code>Content-Disposition</code>, and <code>Content-Encoding</code>. To override these
    /// header values in the GET response, you use the following request parameters.</p>
    ///
    /// <note>
    /// <p>You must sign the request, either using an Authorization header or a presigned URL,
    /// when using these parameters. They cannot be used with an unsigned (anonymous)
    /// request.</p>
    /// </note>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>response-content-type</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>response-content-language</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>response-expires</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>response-cache-control</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>response-content-disposition</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>response-content-encoding</code>
    /// </p>
    /// </li>
    /// </ul>
    ///
    /// <p>
    /// <b>Additional Considerations about Request Headers</b>
    /// </p>
    ///
    /// <p>If both of the <code>If-Match</code> and <code>If-Unmodified-Since</code> headers are
    /// present in the request as follows: <code>If-Match</code> condition evaluates to
    /// <code>true</code>, and; <code>If-Unmodified-Since</code> condition evaluates to
    /// <code>false</code>; then, S3 returns 200 OK and the data requested. </p>
    ///
    /// <p>If both of the <code>If-None-Match</code> and <code>If-Modified-Since</code> headers are
    /// present in the request as follows:<code> If-None-Match</code> condition evaluates to
    /// <code>false</code>, and; <code>If-Modified-Since</code> condition evaluates to
    /// <code>true</code>; then, S3 returns 304 Not Modified response code.</p>
    ///
    /// <p>For more information about conditional requests, see <a href="https://tools.ietf.org/html/rfc7232">RFC 7232</a>.</p>
    ///
    /// <p>The following operations are related to <code>GetObject</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBuckets.html">ListBuckets</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAcl.html">GetObjectAcl</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_object(&self, _input: GetObjectInput) -> S3Result<GetObjectOutput> {
        Err(s3_error!(NotImplemented, "GetObject is not implemented yet"))
    }

    /// <p>Returns the access control list (ACL) of an object. To use this operation, you must have
    /// <code>s3:GetObjectAcl</code> permissions or <code>READ_ACP</code> access to the object.
    /// For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#acl-access-policy-permission-mapping">Mapping of ACL permissions and access policy permissions</a> in the <i>Amazon S3
    /// User Guide</i>
    /// </p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    /// <p>
    /// <b>Versioning</b>
    /// </p>
    /// <p>By default, GET returns ACL information about the current version of an object. To
    /// return ACL information about a different version, use the versionId subresource.</p>
    /// <note>
    /// <p>If your bucket uses the bucket owner enforced setting for S3 Object Ownership,
    /// requests to read ACLs are still supported and return the <code>bucket-owner-full-control</code>
    /// ACL with the owner being the account that created the bucket. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">
    /// Controlling object ownership and disabling ACLs</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </note>
    /// <p>The following operations are related to <code>GetObjectAcl</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAttributes.html">GetObjectAttributes</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObject.html">DeleteObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_object_acl(&self, _input: GetObjectAclInput) -> S3Result<GetObjectAclOutput> {
        Err(s3_error!(NotImplemented, "GetObjectAcl is not implemented yet"))
    }

    /// <p>Retrieves all the metadata from an object without returning the object itself. This
    /// action is useful if you're interested only in an object's metadata. To use
    /// <code>GetObjectAttributes</code>, you must have READ access to the object.</p>
    ///
    /// <p>
    /// <code>GetObjectAttributes</code> combines the functionality of
    /// <code>GetObjectAcl</code>, <code>GetObjectLegalHold</code>,
    /// <code>GetObjectLockConfiguration</code>, <code>GetObjectRetention</code>,
    /// <code>GetObjectTagging</code>, <code>HeadObject</code>, and <code>ListParts</code>. All
    /// of the data returned with each of those individual calls can be returned with a single call
    /// to <code>GetObjectAttributes</code>.</p>
    ///
    /// <p>If you encrypt an object by using server-side encryption with customer-provided
    /// encryption keys (SSE-C) when you store the object in Amazon S3, then when you retrieve the
    /// metadata from the object, you must use the following headers:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>x-amz-server-side-encryption-customer-algorithm</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-server-side-encryption-customer-key</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>x-amz-server-side-encryption-customer-key-MD5</code>
    /// </p>
    /// </li>
    /// </ul>
    /// <p>For more information about SSE-C, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Server-Side Encryption
    /// (Using Customer-Provided Encryption Keys)</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    /// <note>
    /// <ul>
    /// <li>
    /// <p>Encryption request headers, such as
    /// <code>x-amz-server-side-encryption</code>, should not be sent for GET requests
    /// if your object uses server-side encryption with Amazon Web Services KMS keys stored in Amazon Web Services Key
    /// Management Service (SSE-KMS) or server-side encryption with Amazon S3 managed
    /// encryption keys (SSE-S3). If your object does use these types of keys, you'll get
    /// an HTTP <code>400 Bad Request</code> error.</p>
    /// </li>
    /// <li>
    /// <p>
    /// The last modified property in this case is the creation date of the object.</p>
    /// </li>
    /// </ul>
    /// </note>
    ///
    /// <p>Consider the following when using request headers:</p>
    /// <ul>
    /// <li>
    /// <p> If both of the <code>If-Match</code> and <code>If-Unmodified-Since</code>
    /// headers are present in the request as follows, then Amazon S3 returns the HTTP
    /// status code <code>200 OK</code> and the data requested:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>If-Match</code> condition evaluates to <code>true</code>.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>If-Unmodified-Since</code> condition evaluates to
    /// <code>false</code>.</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p>If both of the <code>If-None-Match</code> and <code>If-Modified-Since</code>
    /// headers are present in the request as follows, then Amazon S3 returns the HTTP status code
    /// <code>304 Not Modified</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>If-None-Match</code> condition evaluates to
    /// <code>false</code>.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>If-Modified-Since</code> condition evaluates to
    /// <code>true</code>.</p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    ///
    /// <p>For more information about conditional requests, see <a href="https://tools.ietf.org/html/rfc7232">RFC 7232</a>.</p>
    ///
    /// <p>
    /// <b>Permissions</b>
    /// </p>
    /// <p>The permissions that you need to use this operation depend on whether the bucket is
    /// versioned. If the bucket is versioned, you need both the <code>s3:GetObjectVersion</code>
    /// and <code>s3:GetObjectVersionAttributes</code> permissions for this operation. If the
    /// bucket is not versioned, you need the <code>s3:GetObject</code> and
    /// <code>s3:GetObjectAttributes</code> permissions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying
    /// Permissions in a Policy</a> in the <i>Amazon S3 User Guide</i>. If the
    /// object that you request does not exist, the error Amazon S3 returns depends on whether you also
    /// have the <code>s3:ListBucket</code> permission.</p>
    /// <ul>
    /// <li>
    /// <p>If you have the <code>s3:ListBucket</code> permission on the bucket, Amazon S3
    /// returns an HTTP status code <code>404 Not Found</code> ("no such key") error.</p>
    /// </li>
    /// <li>
    /// <p>If you don't have the <code>s3:ListBucket</code> permission, Amazon S3 returns an
    /// HTTP status code <code>403 Forbidden</code> ("access denied") error.</p>
    /// </li>
    /// </ul>
    ///
    /// <p>The following actions are related to <code>GetObjectAttributes</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAcl.html">GetObjectAcl</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectLegalHold.html">GetObjectLegalHold</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectLockConfiguration.html">GetObjectLockConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectRetention.html">GetObjectRetention</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectTagging.html">GetObjectTagging</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadObject.html">HeadObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_object_attributes(&self, _input: GetObjectAttributesInput) -> S3Result<GetObjectAttributesOutput> {
        Err(s3_error!(NotImplemented, "GetObjectAttributes is not implemented yet"))
    }

    /// <p>Gets an object's current legal hold status. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Locking
    /// Objects</a>.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    ///
    /// <p>The following action is related to <code>GetObjectLegalHold</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAttributes.html">GetObjectAttributes</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_object_legal_hold(&self, _input: GetObjectLegalHoldInput) -> S3Result<GetObjectLegalHoldOutput> {
        Err(s3_error!(NotImplemented, "GetObjectLegalHold is not implemented yet"))
    }

    /// <p>Gets the Object Lock configuration for a bucket. The rule specified in the Object Lock
    /// configuration will be applied by default to every new object placed in the specified
    /// bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Locking
    /// Objects</a>.</p>
    ///
    /// <p>The following action is related to <code>GetObjectLockConfiguration</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAttributes.html">GetObjectAttributes</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_object_lock_configuration(
        &self,
        _input: GetObjectLockConfigurationInput,
    ) -> S3Result<GetObjectLockConfigurationOutput> {
        Err(s3_error!(NotImplemented, "GetObjectLockConfiguration is not implemented yet"))
    }

    /// <p>Retrieves an object's retention settings. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Locking Objects</a>.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    ///
    /// <p>The following action is related to <code>GetObjectRetention</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAttributes.html">GetObjectAttributes</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_object_retention(&self, _input: GetObjectRetentionInput) -> S3Result<GetObjectRetentionOutput> {
        Err(s3_error!(NotImplemented, "GetObjectRetention is not implemented yet"))
    }

    /// <p>Returns the tag-set of an object. You send the GET request against the tagging
    /// subresource associated with the object.</p>
    ///
    /// <p>To use this operation, you must have permission to perform the
    /// <code>s3:GetObjectTagging</code> action. By default, the GET action returns
    /// information about current version of an object. For a versioned bucket, you can have
    /// multiple versions of an object in your bucket. To retrieve tags of any other version, use
    /// the versionId query parameter. You also need permission for the
    /// <code>s3:GetObjectVersionTagging</code> action.</p>
    ///
    /// <p> By default, the bucket owner has this permission and can grant this permission to
    /// others.</p>
    ///
    /// <p> For information about the Amazon S3 object tagging feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-tagging.html">Object Tagging</a>.</p>
    ///
    /// <p>The following actions are related to <code>GetObjectTagging</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObjectTagging.html">DeleteObjectTagging</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAttributes.html">GetObjectAttributes</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObjectTagging.html">PutObjectTagging</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_object_tagging(&self, _input: GetObjectTaggingInput) -> S3Result<GetObjectTaggingOutput> {
        Err(s3_error!(NotImplemented, "GetObjectTagging is not implemented yet"))
    }

    /// <p>Returns torrent files from a bucket. BitTorrent can save you bandwidth when you're
    /// distributing large files. For more information about BitTorrent, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/S3Torrent.html">Using BitTorrent with Amazon S3</a>.</p>
    /// <note>
    /// <p>You can get torrent only for objects that are less than 5 GB in size, and that are
    /// not encrypted using server-side encryption with a customer-provided encryption
    /// key.</p>
    /// </note>
    /// <p>To use GET, you must have READ access to the object.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    /// <p>The following action is related to <code>GetObjectTorrent</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_object_torrent(&self, _input: GetObjectTorrentInput) -> S3Result<GetObjectTorrentOutput> {
        Err(s3_error!(NotImplemented, "GetObjectTorrent is not implemented yet"))
    }

    /// <p>Retrieves the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket. To use
    /// this operation, you must have the <code>s3:GetBucketPublicAccessBlock</code> permission.
    /// For more information about Amazon S3 permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying Permissions in a
    /// Policy</a>.</p>
    ///
    /// <important>
    /// <p>When Amazon S3 evaluates the <code>PublicAccessBlock</code> configuration for a bucket or
    /// an object, it checks the <code>PublicAccessBlock</code> configuration for both the
    /// bucket (or the bucket that contains the object) and the bucket owner's account. If the
    /// <code>PublicAccessBlock</code> settings are different between the bucket and the
    /// account, Amazon S3 uses the most restrictive combination of the bucket-level and
    /// account-level settings.</p>
    /// </important>
    ///
    /// <p>For more information about when Amazon S3 considers a bucket or an object public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a>.</p>
    ///
    /// <p>The following operations are related to <code>GetPublicAccessBlock</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html">Using Amazon S3 Block
    /// Public Access</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutPublicAccessBlock.html">PutPublicAccessBlock</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetPublicAccessBlock.html">GetPublicAccessBlock</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeletePublicAccessBlock.html">DeletePublicAccessBlock</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn get_public_access_block(&self, _input: GetPublicAccessBlockInput) -> S3Result<GetPublicAccessBlockOutput> {
        Err(s3_error!(NotImplemented, "GetPublicAccessBlock is not implemented yet"))
    }

    /// <p>This action is useful to determine if a bucket exists and you have permission to
    /// access it. The action returns a <code>200 OK</code> if the bucket exists and you have
    /// permission to access it.</p>
    ///
    ///
    /// <p>If the bucket does not exist or you do not have permission to access it, the <code>HEAD</code> request
    /// returns a generic <code>404 Not Found</code> or <code>403 Forbidden</code> code. A message body is not
    /// included, so you cannot determine the exception beyond these error codes.</p>
    ///
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:ListBucket</code> action. The bucket owner has this permission by default and
    /// can grant this permission to others. For more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    ///
    /// <p>To use this API against an access point, you must provide the alias of the access point in place of the bucket name or specify the access point ARN. When using the access point ARN, you must direct requests to the access point hostname. The access point hostname takes the form AccessPointName-AccountId.s3-accesspoint.Region.amazonaws.com. When using the Amazon Web Services SDKs, you provide the ARN in place of the bucket name. For more information see, <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a>.</p>
    async fn head_bucket(&self, _input: HeadBucketInput) -> S3Result {
        Err(s3_error!(NotImplemented, "HeadBucket is not implemented yet"))
    }

    /// <p>The HEAD action retrieves metadata from an object without returning the object
    /// itself. This action is useful if you're only interested in an object's metadata. To use
    /// HEAD, you must have READ access to the object.</p>
    ///
    /// <p>A <code>HEAD</code> request has the same options as a <code>GET</code> action on an
    /// object. The response is identical to the <code>GET</code> response except that there is no
    /// response body. Because of this, if the <code>HEAD</code> request generates an error, it
    /// returns a generic <code>404 Not Found</code> or <code>403 Forbidden</code> code. It is not
    /// possible to retrieve the exact exception beyond these error codes.</p>
    ///
    /// <p>If you encrypt an object by using server-side encryption with customer-provided
    /// encryption keys (SSE-C) when you store the object in Amazon S3, then when you retrieve the
    /// metadata from the object, you must use the following headers:</p>
    /// <ul>
    /// <li>
    /// <p>x-amz-server-side-encryption-customer-algorithm</p>
    /// </li>
    /// <li>
    /// <p>x-amz-server-side-encryption-customer-key</p>
    /// </li>
    /// <li>
    /// <p>x-amz-server-side-encryption-customer-key-MD5</p>
    /// </li>
    /// </ul>
    /// <p>For more information about SSE-C, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Server-Side Encryption (Using
    /// Customer-Provided Encryption Keys)</a>.</p>
    /// <note>
    /// <ul>
    /// <li>
    /// <p>Encryption request headers, like <code>x-amz-server-side-encryption</code>, should
    /// not be sent for GET requests if your object uses server-side encryption with KMS keys (SSE-KMS)
    /// or server-side encryption with Amazon S3–managed encryption keys
    /// (SSE-S3). If your object does use these types of keys, you’ll get an HTTP 400 BadRequest
    /// error.</p>
    /// </li>
    /// <li>
    /// <p>
    /// The last modified property in this case is the creation date of the object.</p>
    /// </li>
    /// </ul>
    /// </note>
    ///
    ///
    /// <p>Request headers are limited to 8 KB in size. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTCommonRequestHeaders.html">Common Request
    /// Headers</a>.</p>
    /// <p>Consider the following when using request headers:</p>
    /// <ul>
    /// <li>
    /// <p> Consideration 1 – If both of the <code>If-Match</code> and
    /// <code>If-Unmodified-Since</code> headers are present in the request as
    /// follows:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>If-Match</code> condition evaluates to <code>true</code>, and;</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>If-Unmodified-Since</code> condition evaluates to
    /// <code>false</code>;</p>
    /// </li>
    /// </ul>
    /// <p>Then Amazon S3 returns <code>200 OK</code> and the data requested.</p>
    /// </li>
    /// <li>
    /// <p> Consideration 2 – If both of the <code>If-None-Match</code> and
    /// <code>If-Modified-Since</code> headers are present in the request as
    /// follows:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>If-None-Match</code> condition evaluates to <code>false</code>,
    /// and;</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>If-Modified-Since</code> condition evaluates to
    /// <code>true</code>;</p>
    /// </li>
    /// </ul>
    /// <p>Then Amazon S3 returns the <code>304 Not Modified</code> response code.</p>
    /// </li>
    /// </ul>
    ///
    /// <p>For more information about conditional requests, see <a href="https://tools.ietf.org/html/rfc7232">RFC 7232</a>.</p>
    ///
    /// <p>
    /// <b>Permissions</b>
    /// </p>
    /// <p>You need the relevant read object (or version) permission for this operation. For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying Permissions
    /// in a Policy</a>. If the object you request does not exist, the error Amazon S3 returns
    /// depends on whether you also have the s3:ListBucket permission.</p>
    /// <ul>
    /// <li>
    /// <p>If you have the <code>s3:ListBucket</code> permission on the bucket, Amazon S3 returns
    /// an HTTP status code 404 ("no such key") error.</p>
    /// </li>
    /// <li>
    /// <p>If you don’t have the <code>s3:ListBucket</code> permission, Amazon S3 returns an HTTP
    /// status code 403 ("access denied") error.</p>
    /// </li>
    /// </ul>
    ///
    /// <p>The following actions are related to <code>HeadObject</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAttributes.html">GetObjectAttributes</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn head_object(&self, _input: HeadObjectInput) -> S3Result<HeadObjectOutput> {
        Err(s3_error!(NotImplemented, "HeadObject is not implemented yet"))
    }

    /// <p>Lists the analytics configurations for the bucket. You can have up to 1,000 analytics
    /// configurations per bucket.</p>
    ///
    /// <p>This action supports list pagination and does not return more than 100 configurations
    /// at a time. You should always check the <code>IsTruncated</code> element in the response. If
    /// there are no more configurations to list, <code>IsTruncated</code> is set to false. If
    /// there are more configurations to list, <code>IsTruncated</code> is set to true, and there
    /// will be a value in <code>NextContinuationToken</code>. You use the
    /// <code>NextContinuationToken</code> value to continue the pagination of the list by
    /// passing the value in continuation-token in the request to <code>GET</code> the next
    /// page.</p>
    ///
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:GetAnalyticsConfiguration</code> action. The bucket owner has this permission
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p>For information about Amazon S3 analytics feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/analytics-storage-class.html">Amazon S3 Analytics – Storage Class
    /// Analysis</a>. </p>
    ///
    /// <p>The following operations are related to
    /// <code>ListBucketAnalyticsConfigurations</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketAnalyticsConfiguration.html">GetBucketAnalyticsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketAnalyticsConfiguration.html">DeleteBucketAnalyticsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketAnalyticsConfiguration.html">PutBucketAnalyticsConfiguration</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn list_bucket_analytics_configurations(
        &self,
        _input: ListBucketAnalyticsConfigurationsInput,
    ) -> S3Result<ListBucketAnalyticsConfigurationsOutput> {
        Err(s3_error!(
            NotImplemented,
            "ListBucketAnalyticsConfigurations is not implemented yet"
        ))
    }

    /// <p>Lists the S3 Intelligent-Tiering configuration from the specified bucket.</p>
    /// <p>The S3 Intelligent-Tiering storage class is designed to optimize storage costs by automatically moving data to the most cost-effective storage access tier, without performance impact or operational overhead. S3 Intelligent-Tiering delivers automatic cost savings in three low latency and high throughput access tiers. To get the lowest storage cost on data that can be accessed in minutes to hours, you can choose to activate additional archiving capabilities.</p>
    /// <p>The S3 Intelligent-Tiering storage class is  the ideal storage class for data with unknown, changing, or unpredictable access patterns, independent of object size or retention period. If the size of an object is less than 128 KB, it is not monitored and not eligible for auto-tiering. Smaller objects can be stored, but they are always charged at the Frequent Access tier rates in the S3 Intelligent-Tiering storage class.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for automatically optimizing frequently and infrequently accessed objects</a>.</p>
    /// <p>Operations related to
    /// <code>ListBucketIntelligentTieringConfigurations</code> include: </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketIntelligentTieringConfiguration.html">DeleteBucketIntelligentTieringConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketIntelligentTieringConfiguration.html">PutBucketIntelligentTieringConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketIntelligentTieringConfiguration.html">GetBucketIntelligentTieringConfiguration</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn list_bucket_intelligent_tiering_configurations(
        &self,
        _input: ListBucketIntelligentTieringConfigurationsInput,
    ) -> S3Result<ListBucketIntelligentTieringConfigurationsOutput> {
        Err(s3_error!(
            NotImplemented,
            "ListBucketIntelligentTieringConfigurations is not implemented yet"
        ))
    }

    /// <p>Returns a list of inventory configurations for the bucket. You can have up to 1,000
    /// analytics configurations per bucket.</p>
    ///
    /// <p>This action supports list pagination and does not return more than 100 configurations
    /// at a time. Always check the <code>IsTruncated</code> element in the response. If there are
    /// no more configurations to list, <code>IsTruncated</code> is set to false. If there are more
    /// configurations to list, <code>IsTruncated</code> is set to true, and there is a value in
    /// <code>NextContinuationToken</code>. You use the <code>NextContinuationToken</code> value
    /// to continue the pagination of the list by passing the value in continuation-token in the
    /// request to <code>GET</code> the next page.</p>
    ///
    /// <p> To use this operation, you must have permissions to perform the
    /// <code>s3:GetInventoryConfiguration</code> action. The bucket owner has this permission
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p>For information about the Amazon S3 inventory feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-inventory.html">Amazon S3 Inventory</a>
    /// </p>
    ///
    /// <p>The following operations are related to
    /// <code>ListBucketInventoryConfigurations</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketInventoryConfiguration.html">GetBucketInventoryConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketInventoryConfiguration.html">DeleteBucketInventoryConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketInventoryConfiguration.html">PutBucketInventoryConfiguration</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn list_bucket_inventory_configurations(
        &self,
        _input: ListBucketInventoryConfigurationsInput,
    ) -> S3Result<ListBucketInventoryConfigurationsOutput> {
        Err(s3_error!(
            NotImplemented,
            "ListBucketInventoryConfigurations is not implemented yet"
        ))
    }

    /// <p>Lists the metrics configurations for the bucket. The metrics configurations are only for
    /// the request metrics of the bucket and do not provide information on daily storage metrics.
    /// You can have up to 1,000 configurations per bucket.</p>
    ///
    /// <p>This action supports list pagination and does not return more than 100 configurations
    /// at a time. Always check the <code>IsTruncated</code> element in the response. If there are
    /// no more configurations to list, <code>IsTruncated</code> is set to false. If there are more
    /// configurations to list, <code>IsTruncated</code> is set to true, and there is a value in
    /// <code>NextContinuationToken</code>. You use the <code>NextContinuationToken</code> value
    /// to continue the pagination of the list by passing the value in
    /// <code>continuation-token</code> in the request to <code>GET</code> the next page.</p>
    ///
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:GetMetricsConfiguration</code> action. The bucket owner has this permission by
    /// default. The bucket owner can grant this permission to others. For more information about
    /// permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p>For more information about metrics configurations and CloudWatch request metrics, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cloudwatch-monitoring.html">Monitoring Metrics with Amazon
    /// CloudWatch</a>.</p>
    ///
    /// <p>The following operations are related to
    /// <code>ListBucketMetricsConfigurations</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketMetricsConfiguration.html">PutBucketMetricsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketMetricsConfiguration.html">GetBucketMetricsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketMetricsConfiguration.html">DeleteBucketMetricsConfiguration</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn list_bucket_metrics_configurations(
        &self,
        _input: ListBucketMetricsConfigurationsInput,
    ) -> S3Result<ListBucketMetricsConfigurationsOutput> {
        Err(s3_error!(
            NotImplemented,
            "ListBucketMetricsConfigurations is not implemented yet"
        ))
    }

    /// <p>Returns a list of all buckets owned by the authenticated sender of the request. To use
    /// this operation, you must have the <code>s3:ListAllMyBuckets</code> permission.</p>
    async fn list_buckets(&self) -> S3Result<ListBucketsOutput> {
        Err(s3_error!(NotImplemented, "ListBuckets is not implemented yet"))
    }

    /// <p>This action lists in-progress multipart uploads. An in-progress multipart upload is a
    /// multipart upload that has been initiated using the Initiate Multipart Upload request, but
    /// has not yet been completed or aborted.</p>
    ///
    /// <p>This action returns at most 1,000 multipart uploads in the response. 1,000 multipart
    /// uploads is the maximum number of uploads a response can include, which is also the default
    /// value. You can further limit the number of uploads in a response by specifying the
    /// <code>max-uploads</code> parameter in the response. If additional multipart uploads
    /// satisfy the list criteria, the response will contain an <code>IsTruncated</code> element
    /// with the value true. To list the additional multipart uploads, use the
    /// <code>key-marker</code> and <code>upload-id-marker</code> request parameters.</p>
    ///
    /// <p>In the response, the uploads are sorted by key. If your application has initiated more
    /// than one multipart upload using the same object key, then uploads in the response are first
    /// sorted by key. Additionally, uploads are sorted in ascending order within each key by the
    /// upload initiation time.</p>
    ///
    /// <p>For more information on multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/uploadobjusingmpu.html">Uploading Objects Using Multipart
    /// Upload</a>.</p>
    ///
    /// <p>For information on permissions required to use the multipart upload API, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuAndPermissions.html">Multipart Upload and
    /// Permissions</a>.</p>
    ///
    /// <p>The following operations are related to <code>ListMultipartUploads</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html">CompleteMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html">AbortMultipartUpload</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn list_multipart_uploads(&self, _input: ListMultipartUploadsInput) -> S3Result<ListMultipartUploadsOutput> {
        Err(s3_error!(NotImplemented, "ListMultipartUploads is not implemented yet"))
    }

    /// <p>Returns metadata about all versions of the objects in a bucket. You can also use request
    /// parameters as selection criteria to return metadata about a subset of all the object
    /// versions.</p>
    /// <important>
    /// <p>
    /// To use this operation, you must have permissions to perform the
    /// <code>s3:ListBucketVersions</code> action. Be aware of the name difference.
    /// </p>
    /// </important>
    /// <note>
    /// <p> A 200 OK response can contain valid or invalid XML. Make sure to design your
    /// application to parse the contents of the response and handle it appropriately.</p>
    /// </note>
    /// <p>To use this operation, you must have READ access to the bucket.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    /// <p>The following operations are related to
    /// <code>ListObjectVersions</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html">ListObjectsV2</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObject.html">DeleteObject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn list_object_versions(&self, _input: ListObjectVersionsInput) -> S3Result<ListObjectVersionsOutput> {
        Err(s3_error!(NotImplemented, "ListObjectVersions is not implemented yet"))
    }

    /// <p>Returns some or all (up to 1,000) of the objects in a bucket. You can use the request
    /// parameters as selection criteria to return a subset of the objects in a bucket. A 200 OK
    /// response can contain valid or invalid XML. Be sure to design your application to parse the
    /// contents of the response and handle it appropriately.</p>
    /// <important>
    /// <p>This action has been revised. We recommend that you use the newer version, <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html">ListObjectsV2</a>, when developing applications. For backward compatibility,
    /// Amazon S3 continues to support <code>ListObjects</code>.</p>
    /// </important>
    ///
    ///
    /// <p>The following operations are related to <code>ListObjects</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html">ListObjectsV2</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBuckets.html">ListBuckets</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn list_objects(&self, _input: ListObjectsInput) -> S3Result<ListObjectsOutput> {
        Err(s3_error!(NotImplemented, "ListObjects is not implemented yet"))
    }

    /// <p>Returns some or all (up to 1,000) of the objects in a bucket with each request. You can use
    /// the request parameters as selection criteria to return a subset of the objects in a bucket. A
    /// <code>200 OK</code> response can contain valid or invalid XML. Make sure to design your
    /// application to parse the contents of the response and handle it appropriately.
    /// Objects are returned sorted in an ascending order of the respective key names in the list.
    /// For more information about listing objects, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/ListingKeysUsingAPIs.html">Listing object keys
    /// programmatically</a>
    /// </p>
    ///
    /// <p>To use this operation, you must have READ access to the bucket.</p>
    ///
    /// <p>To use this action in an Identity and Access Management (IAM) policy, you must
    /// have permissions to perform the <code>s3:ListBucket</code> action. The bucket owner has
    /// this permission by default and can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    /// <important>
    /// <p>This section describes the latest revision of this action. We recommend that you use this
    /// revised API for application development. For backward compatibility, Amazon S3 continues to
    /// support the prior version of this API, <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjects.html">ListObjects</a>.</p>
    /// </important>
    ///
    /// <p>To get a list of your buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBuckets.html">ListBuckets</a>.</p>
    ///
    /// <p>The following operations are related to <code>ListObjectsV2</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn list_objects_v2(&self, _input: ListObjectsV2Input) -> S3Result<ListObjectsV2Output> {
        Err(s3_error!(NotImplemented, "ListObjectsV2 is not implemented yet"))
    }

    /// <p>Lists the parts that have been uploaded for a specific multipart upload. This operation
    /// must include the upload ID, which you obtain by sending the initiate multipart upload
    /// request (see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>).
    /// This request returns a maximum of 1,000 uploaded parts. The default number of parts
    /// returned is 1,000 parts. You can restrict the number of parts returned by specifying the
    /// <code>max-parts</code> request parameter. If your multipart upload consists of more than
    /// 1,000 parts, the response returns an <code>IsTruncated</code> field with the value of true,
    /// and a <code>NextPartNumberMarker</code> element. In subsequent <code>ListParts</code>
    /// requests you can include the part-number-marker query string parameter and set its value to
    /// the <code>NextPartNumberMarker</code> field value from the previous response.</p>
    /// <p>If the upload was created using a checksum algorithm, you will need to have permission
    /// to the <code>kms:Decrypt</code> action for the request to succeed.
    /// </p>
    ///
    /// <p>For more information on multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/uploadobjusingmpu.html">Uploading Objects Using Multipart
    /// Upload</a>.</p>
    ///
    /// <p>For information on permissions required to use the multipart upload API, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuAndPermissions.html">Multipart Upload and
    /// Permissions</a>.</p>
    ///
    /// <p>The following operations are related to <code>ListParts</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html">CompleteMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html">AbortMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAttributes.html">GetObjectAttributes</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListMultipartUploads.html">ListMultipartUploads</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn list_parts(&self, _input: ListPartsInput) -> S3Result<ListPartsOutput> {
        Err(s3_error!(NotImplemented, "ListParts is not implemented yet"))
    }

    /// <p>Sets the accelerate configuration of an existing bucket. Amazon S3 Transfer Acceleration is a
    /// bucket-level feature that enables you to perform faster data transfers to Amazon S3.</p>
    ///
    /// <p> To use this operation, you must have permission to perform the
    /// <code>s3:PutAccelerateConfiguration</code> action. The bucket owner has this permission
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing
    /// Access Permissions to Your Amazon S3 Resources</a>.</p>
    ///
    /// <p> The Transfer Acceleration state of a bucket can be set to one of the following two
    /// values:</p>
    /// <ul>
    /// <li>
    /// <p> Enabled – Enables accelerated data transfers to the bucket.</p>
    /// </li>
    /// <li>
    /// <p> Suspended – Disables accelerated data transfers to the bucket.</p>
    /// </li>
    /// </ul>
    ///
    ///
    /// <p>The <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketAccelerateConfiguration.html">GetBucketAccelerateConfiguration</a> action returns the transfer acceleration
    /// state of a bucket.</p>
    ///
    /// <p>After setting the Transfer Acceleration state of a bucket to Enabled, it might take up
    /// to thirty minutes before the data transfer rates to the bucket increase.</p>
    ///
    /// <p> The name of the bucket used for Transfer Acceleration must be DNS-compliant and must
    /// not contain periods (".").</p>
    ///
    /// <p> For more information about transfer acceleration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/transfer-acceleration.html">Transfer Acceleration</a>.</p>
    ///
    /// <p>The following operations are related to
    /// <code>PutBucketAccelerateConfiguration</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketAccelerateConfiguration.html">GetBucketAccelerateConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_accelerate_configuration(&self, _input: PutBucketAccelerateConfigurationInput) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "PutBucketAccelerateConfiguration is not implemented yet"
        ))
    }

    /// <p>Sets the permissions on an existing bucket using access control lists (ACL). For more
    /// information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/S3_ACLs_UsingACLs.html">Using ACLs</a>. To set
    /// the ACL of a bucket, you must have <code>WRITE_ACP</code> permission.</p>
    ///
    /// <p>You can use one of the following two ways to set a bucket's permissions:</p>
    /// <ul>
    /// <li>
    /// <p>Specify the ACL in the request body</p>
    /// </li>
    /// <li>
    /// <p>Specify permissions using request headers</p>
    /// </li>
    /// </ul>
    ///
    /// <note>
    /// <p>You cannot specify access permission using both the body and the request
    /// headers.</p>
    /// </note>
    ///
    /// <p>Depending on your application needs, you may choose to set the ACL on a bucket using
    /// either the request body or the headers. For example, if you have an existing application
    /// that updates a bucket ACL using the request body, then you can continue to use that
    /// approach.</p>
    ///
    /// <important>
    /// <p>If your bucket uses the bucket owner enforced setting for S3 Object Ownership, ACLs are disabled and no longer affect permissions.
    /// You must use policies to grant access to your bucket and the objects in it. Requests to set ACLs or update ACLs fail and
    /// return the <code>AccessControlListNotSupported</code> error code. Requests to read ACLs are still supported.
    /// For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">Controlling object ownership</a>
    /// in the <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    /// <p>
    /// <b>Access Permissions</b>
    /// </p>
    /// <p>You can set access permissions using one of the following methods:</p>
    /// <ul>
    /// <li>
    /// <p>Specify a canned ACL with the <code>x-amz-acl</code> request header. Amazon S3 supports
    /// a set of predefined ACLs, known as <i>canned ACLs</i>. Each canned ACL
    /// has a predefined set of grantees and permissions. Specify the canned ACL name as the
    /// value of <code>x-amz-acl</code>. If you use this header, you cannot use other access
    /// control-specific headers in your request. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL">Canned ACL</a>.</p>
    /// </li>
    /// <li>
    /// <p>Specify access permissions explicitly with the <code>x-amz-grant-read</code>,
    /// <code>x-amz-grant-read-acp</code>, <code>x-amz-grant-write-acp</code>, and
    /// <code>x-amz-grant-full-control</code> headers. When using these headers, you
    /// specify explicit access permissions and grantees (Amazon Web Services accounts or Amazon S3 groups) who
    /// will receive the permission. If you use these ACL-specific headers, you cannot use
    /// the <code>x-amz-acl</code> header to set a canned ACL. These parameters map to the
    /// set of permissions that Amazon S3 supports in an ACL. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html">Access Control List (ACL)
    /// Overview</a>.</p>
    /// <p>You specify each grantee as a type=value pair, where the type is one of the
    /// following:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>id</code> – if the value specified is the canonical user ID of an Amazon Web Services account</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>uri</code> – if you are granting permissions to a predefined
    /// group</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>emailAddress</code> – if the value specified is the email address of
    /// an Amazon Web Services account</p>
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
    /// </li>
    /// </ul>
    /// <p>For example, the following <code>x-amz-grant-write</code> header grants create,
    /// overwrite, and delete objects permission to LogDelivery group predefined by Amazon S3 and
    /// two Amazon Web Services accounts identified by their email addresses.</p>
    /// <p>
    /// <code>x-amz-grant-write: uri="http://acs.amazonaws.com/groups/s3/LogDelivery",
    /// id="111122223333", id="555566667777" </code>
    /// </p>
    ///
    /// </li>
    /// </ul>
    /// <p>You can use either a canned ACL or specify access permissions explicitly. You cannot do
    /// both.</p>
    /// <p>
    /// <b>Grantee Values</b>
    /// </p>
    /// <p>You can specify the person (grantee) to whom you're assigning access rights (using
    /// request elements) in the following ways:</p>
    /// <ul>
    /// <li>
    /// <p>By the person's ID:</p>
    /// <p>
    /// <code>&lt;Grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    /// xsi:type="CanonicalUser"><ID><>ID<></ID><DisplayName><>GranteesEmail<></DisplayName>
    /// &lt;/Grantee&gt;</code>
    /// </p>
    /// <p>DisplayName is optional and ignored in the request</p>
    /// </li>
    /// <li>
    /// <p>By URI:</p>
    /// <p>
    /// <code>&lt;Grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    /// xsi:type="Group"><URI><>http://acs.amazonaws.com/groups/global/AuthenticatedUsers<></URI>&lt;/Grantee&gt;</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>By Email address:</p>
    /// <p>
    /// <code>&lt;Grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    /// xsi:type="AmazonCustomerByEmail"><EmailAddress><>Grantees@email.com<></EmailAddress>lt;/Grantee></code>
    /// </p>
    /// <p>The grantee is resolved to the CanonicalUser and, in a response to a GET Object
    /// acl request, appears as the CanonicalUser. </p>
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
    /// </li>
    /// </ul>
    ///
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucket.html">DeleteBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAcl.html">GetObjectAcl</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_acl(&self, _input: PutBucketAclInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutBucketAcl is not implemented yet"))
    }

    /// <p>Sets an analytics configuration for the bucket (specified by the analytics configuration
    /// ID). You can have up to 1,000 analytics configurations per bucket.</p>
    ///
    /// <p>You can choose to have storage class analysis export analysis reports sent to a
    /// comma-separated values (CSV) flat file. See the <code>DataExport</code> request element.
    /// Reports are updated daily and are based on the object filters that you configure. When
    /// selecting data export, you specify a destination bucket and an optional destination prefix
    /// where the file is written. You can export the data to a destination bucket in a different
    /// account. However, the destination bucket must be in the same Region as the bucket that you
    /// are making the PUT analytics configuration to. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/analytics-storage-class.html">Amazon S3 Analytics – Storage Class
    /// Analysis</a>. </p>
    ///
    /// <important>
    /// <p>You must create a bucket policy on the destination bucket where the exported file is
    /// written to grant permissions to Amazon S3 to write objects to the bucket. For an example
    /// policy, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/example-bucket-policies.html#example-bucket-policies-use-case-9">Granting Permissions for Amazon S3 Inventory and Storage Class Analysis</a>.</p>
    /// </important>
    ///
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:PutAnalyticsConfiguration</code> action. The bucket owner has this permission
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    ///
    /// <p class="title">
    /// <b>Special Errors</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>HTTP Error: HTTP 400 Bad Request</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code: InvalidArgument</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: Invalid argument.</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>HTTP Error: HTTP 400 Bad Request</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code: TooManyConfigurations</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: You are attempting to create a new configuration but have
    /// already reached the 1,000-configuration limit.</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>HTTP Error: HTTP 403 Forbidden</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Code: AccessDenied</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: You are not the owner of the specified bucket, or you do
    /// not have the s3:PutAnalyticsConfiguration bucket permission to set the
    /// configuration on the bucket.</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    ///
    ///
    ///
    ///
    ///
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketAnalyticsConfiguration.html">GetBucketAnalyticsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketAnalyticsConfiguration.html">DeleteBucketAnalyticsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketAnalyticsConfigurations.html">ListBucketAnalyticsConfigurations</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_analytics_configuration(&self, _input: PutBucketAnalyticsConfigurationInput) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "PutBucketAnalyticsConfiguration is not implemented yet"
        ))
    }

    /// <p>Sets the <code>cors</code> configuration for your bucket. If the configuration exists,
    /// Amazon S3 replaces it.</p>
    /// <p>To use this operation, you must be allowed to perform the <code>s3:PutBucketCORS</code>
    /// action. By default, the bucket owner has this permission and can grant it to others.</p>
    /// <p>You set this configuration on a bucket so that the bucket can service cross-origin
    /// requests. For example, you might want to enable a request whose origin is
    /// <code>http://www.example.com</code> to access your Amazon S3 bucket at
    /// <code>my.example.bucket.com</code> by using the browser's <code>XMLHttpRequest</code>
    /// capability.</p>
    /// <p>To enable cross-origin resource sharing (CORS) on a bucket, you add the
    /// <code>cors</code> subresource to the bucket. The <code>cors</code> subresource is an XML
    /// document in which you configure rules that identify origins and the HTTP methods that can
    /// be executed on your bucket. The document is limited to 64 KB in size. </p>
    /// <p>When Amazon S3 receives a cross-origin request (or a pre-flight OPTIONS request) against a
    /// bucket, it evaluates the <code>cors</code> configuration on the bucket and uses the first
    /// <code>CORSRule</code> rule that matches the incoming browser request to enable a
    /// cross-origin request. For a rule to match, the following conditions must be met:</p>
    /// <ul>
    /// <li>
    /// <p>The request's <code>Origin</code> header must match <code>AllowedOrigin</code>
    /// elements.</p>
    /// </li>
    /// <li>
    /// <p>The request method (for example, GET, PUT, HEAD, and so on) or the
    /// <code>Access-Control-Request-Method</code> header in case of a pre-flight
    /// <code>OPTIONS</code> request must be one of the <code>AllowedMethod</code>
    /// elements. </p>
    /// </li>
    /// <li>
    /// <p>Every header specified in the <code>Access-Control-Request-Headers</code> request
    /// header of a pre-flight request must match an <code>AllowedHeader</code> element.
    /// </p>
    /// </li>
    /// </ul>
    /// <p> For more information about CORS, go to <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cors.html">Enabling
    /// Cross-Origin Resource Sharing</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketCors.html">GetBucketCors</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketCors.html">DeleteBucketCors</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTOPTIONSobject.html">RESTOPTIONSobject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_cors(&self, _input: PutBucketCorsInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutBucketCors is not implemented yet"))
    }

    /// <p>This action uses the <code>encryption</code> subresource to configure default
    /// encryption and Amazon S3 Bucket Key for an existing bucket.</p>
    /// <p>Default encryption for a bucket can use server-side encryption with Amazon S3-managed keys
    /// (SSE-S3) or customer managed keys (SSE-KMS). If you specify default encryption
    /// using SSE-KMS, you can also configure Amazon S3 Bucket Key. When the default encryption is SSE-KMS, if
    /// you upload an object to the bucket and do not specify the KMS key to use for encryption, Amazon S3
    /// uses the default Amazon Web Services managed KMS key for your account. For information about default
    /// encryption, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 default bucket encryption</a>
    /// in the <i>Amazon S3 User Guide</i>. For more information about S3 Bucket Keys,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html">Amazon S3 Bucket Keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <important>
    /// <p>This action requires Amazon Web Services Signature Version 4. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html"> Authenticating Requests (Amazon Web Services Signature
    /// Version 4)</a>. </p>
    /// </important>
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:PutEncryptionConfiguration</code> action. The bucket owner has this permission
    /// by default. The bucket owner can grant this permission to others. For more information
    /// about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a> in the Amazon S3 User Guide. </p>
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketEncryption.html">GetBucketEncryption</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketEncryption.html">DeleteBucketEncryption</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_encryption(&self, _input: PutBucketEncryptionInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutBucketEncryption is not implemented yet"))
    }

    /// <p>Puts a S3 Intelligent-Tiering configuration to the specified bucket.
    /// You can have up to 1,000 S3 Intelligent-Tiering configurations per bucket.</p>
    /// <p>The S3 Intelligent-Tiering storage class is designed to optimize storage costs by automatically moving data to the most cost-effective storage access tier, without performance impact or operational overhead. S3 Intelligent-Tiering delivers automatic cost savings in three low latency and high throughput access tiers. To get the lowest storage cost on data that can be accessed in minutes to hours, you can choose to activate additional archiving capabilities.</p>
    /// <p>The S3 Intelligent-Tiering storage class is  the ideal storage class for data with unknown, changing, or unpredictable access patterns, independent of object size or retention period. If the size of an object is less than 128 KB, it is not monitored and not eligible for auto-tiering. Smaller objects can be stored, but they are always charged at the Frequent Access tier rates in the S3 Intelligent-Tiering storage class.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for automatically optimizing frequently and infrequently accessed objects</a>.</p>
    /// <p>Operations related to
    /// <code>PutBucketIntelligentTieringConfiguration</code> include: </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketIntelligentTieringConfiguration.html">DeleteBucketIntelligentTieringConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketIntelligentTieringConfiguration.html">GetBucketIntelligentTieringConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketIntelligentTieringConfigurations.html">ListBucketIntelligentTieringConfigurations</a>
    /// </p>
    /// </li>
    /// </ul>
    /// <note>
    /// <p>You only need S3 Intelligent-Tiering enabled on a bucket if you want to automatically
    /// move objects stored in the S3 Intelligent-Tiering storage class to the
    /// Archive Access or Deep Archive Access tier.</p>
    /// </note>
    ///
    /// <p class="title">
    /// <b>Special Errors</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p class="title">
    /// <b>HTTP 400 Bad Request Error</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidArgument</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause:</i> Invalid Argument</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p class="title">
    /// <b>HTTP 400 Bad Request Error</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> TooManyConfigurations</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause:</i> You are attempting to create a new configuration
    /// but have already reached the 1,000-configuration limit. </p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p class="title">
    /// <b>HTTP 403 Forbidden Error</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> AccessDenied</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause:</i> You are not the owner of the specified bucket,
    /// or you do not have the <code>s3:PutIntelligentTieringConfiguration</code> bucket
    /// permission to set the configuration on the bucket. </p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    async fn put_bucket_intelligent_tiering_configuration(
        &self,
        _input: PutBucketIntelligentTieringConfigurationInput,
    ) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "PutBucketIntelligentTieringConfiguration is not implemented yet"
        ))
    }

    /// <p>This implementation of the <code>PUT</code> action adds an inventory configuration
    /// (identified by the inventory ID) to the bucket. You can have up to 1,000 inventory
    /// configurations per bucket. </p>
    /// <p>Amazon S3 inventory generates inventories of the objects in the bucket on a daily or weekly
    /// basis, and the results are published to a flat file. The bucket that is inventoried is
    /// called the <i>source</i> bucket, and the bucket where the inventory flat file
    /// is stored is called the <i>destination</i> bucket. The
    /// <i>destination</i> bucket must be in the same Amazon Web Services Region as the
    /// <i>source</i> bucket. </p>
    /// <p>When you configure an inventory for a <i>source</i> bucket, you specify
    /// the <i>destination</i> bucket where you want the inventory to be stored, and
    /// whether to generate the inventory daily or weekly. You can also configure what object
    /// metadata to include and whether to inventory all object versions or only current versions.
    /// For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-inventory.html">Amazon S3
    /// Inventory</a> in the Amazon S3 User Guide.</p>
    /// <important>
    /// <p>You must create a bucket policy on the <i>destination</i> bucket to
    /// grant permissions to Amazon S3 to write objects to the bucket in the defined location. For an
    /// example policy, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/example-bucket-policies.html#example-bucket-policies-use-case-9">
    /// Granting Permissions for Amazon S3 Inventory and Storage Class Analysis</a>.</p>
    /// </important>
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:PutInventoryConfiguration</code> action. The bucket owner has this permission
    /// by default and can grant this permission to others. For more information about permissions,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a> in the Amazon S3 User Guide.</p>
    ///
    /// <p class="title">
    /// <b>Special Errors</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p class="title">
    /// <b>HTTP 400 Bad Request Error</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> InvalidArgument</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause:</i> Invalid Argument</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p class="title">
    /// <b>HTTP 400 Bad Request Error</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> TooManyConfigurations</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause:</i> You are attempting to create a new configuration
    /// but have already reached the 1,000-configuration limit. </p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p class="title">
    /// <b>HTTP 403 Forbidden Error</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code:</i> AccessDenied</p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause:</i> You are not the owner of the specified bucket,
    /// or you do not have the <code>s3:PutInventoryConfiguration</code> bucket
    /// permission to set the configuration on the bucket. </p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketInventoryConfiguration.html">GetBucketInventoryConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketInventoryConfiguration.html">DeleteBucketInventoryConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketInventoryConfigurations.html">ListBucketInventoryConfigurations</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_inventory_configuration(&self, _input: PutBucketInventoryConfigurationInput) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "PutBucketInventoryConfiguration is not implemented yet"
        ))
    }

    /// <p>Creates a new lifecycle configuration for the bucket or replaces an existing lifecycle
    /// configuration. Keep in mind that this will overwrite an existing lifecycle configuration, so if
    /// you want to retain any configuration details, they must be included in the new lifecycle
    /// configuration. For information about lifecycle configuration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lifecycle-mgmt.html">Managing your storage
    /// lifecycle</a>.</p>
    ///
    /// <note>
    /// <p>Bucket lifecycle configuration now supports specifying a lifecycle rule using an
    /// object key name prefix, one or more object tags, or a combination of both. Accordingly,
    /// this section describes the latest API. The previous version of the API supported
    /// filtering based only on an object key name prefix, which is supported for backward
    /// compatibility. For the related API description, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLifecycle.html">PutBucketLifecycle</a>.</p>
    /// </note>
    ///
    ///
    ///
    /// <p>
    /// <b>Rules</b>
    /// </p>
    /// <p>You specify the lifecycle configuration in your request body. The lifecycle
    /// configuration is specified as XML consisting of one or more rules. An Amazon S3 Lifecycle
    /// configuration can have up to 1,000 rules. This limit is not adjustable. Each rule consists
    /// of the following:</p>
    ///
    /// <ul>
    /// <li>
    /// <p>Filter identifying a subset of objects to which the rule applies. The filter can
    /// be based on a key name prefix, object tags, or a combination of both.</p>
    /// </li>
    /// <li>
    /// <p>Status whether the rule is in effect.</p>
    /// </li>
    /// <li>
    /// <p>One or more lifecycle transition and expiration actions that you want Amazon S3 to
    /// perform on the objects identified by the filter. If the state of your bucket is
    /// versioning-enabled or versioning-suspended, you can have many versions of the same
    /// object (one current version and zero or more noncurrent versions). Amazon S3 provides
    /// predefined actions that you can specify for current and noncurrent object
    /// versions.</p>
    /// </li>
    /// </ul>
    ///
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html">Object
    /// Lifecycle Management</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html">Lifecycle Configuration Elements</a>.</p>
    ///
    ///
    /// <p>
    /// <b>Permissions</b>
    /// </p>
    ///
    ///
    /// <p>By default, all Amazon S3 resources are private, including buckets, objects, and related
    /// subresources (for example, lifecycle configuration and website configuration). Only the
    /// resource owner (that is, the Amazon Web Services account that created it) can access the resource. The
    /// resource owner can optionally grant access permissions to others by writing an access
    /// policy. For this operation, a user must get the <code>s3:PutLifecycleConfiguration</code>
    /// permission.</p>
    ///
    /// <p>You can also explicitly deny permissions. Explicit deny also supersedes any other
    /// permissions. If you want to block users or accounts from removing or deleting objects from
    /// your bucket, you must deny them permissions for the following actions:</p>
    ///
    /// <ul>
    /// <li>
    /// <p>
    /// <code>s3:DeleteObject</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>s3:DeleteObjectVersion</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>s3:PutLifecycleConfiguration</code>
    /// </p>
    /// </li>
    /// </ul>
    ///
    ///
    /// <p>For more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p>The following are related to <code>PutBucketLifecycleConfiguration</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/lifecycle-configuration-examples.html">Examples of
    /// Lifecycle Configuration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLifecycleConfiguration.html">GetBucketLifecycleConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketLifecycle.html">DeleteBucketLifecycle</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_lifecycle_configuration(&self, _input: PutBucketLifecycleConfigurationInput) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "PutBucketLifecycleConfiguration is not implemented yet"
        ))
    }

    /// <p>Set the logging parameters for a bucket and to specify permissions for who can view and
    /// modify the logging parameters. All logs are saved to buckets in the same Amazon Web Services Region as the
    /// source bucket. To set the logging status of a bucket, you must be the bucket owner.</p>
    ///
    /// <p>The bucket owner is automatically granted FULL_CONTROL to all logs. You use the <code>Grantee</code> request element to grant access to other people. The
    /// <code>Permissions</code> request element specifies the kind of access the grantee has to
    /// the logs.</p>
    /// <important>
    /// <p>If the target bucket for log delivery uses the bucket owner enforced
    /// setting for S3 Object Ownership, you can't use the <code>Grantee</code> request element
    /// to grant access to others. Permissions can only be granted using policies. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/enable-server-access-logging.html#grant-log-delivery-permissions-general">Permissions for server access log delivery</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    ///
    /// <p>
    /// <b>Grantee Values</b>
    /// </p>
    /// <p>You can specify the person (grantee) to whom you're assigning access rights (using
    /// request elements) in the following ways:</p>
    ///
    /// <ul>
    /// <li>
    /// <p>By the person's ID:</p>
    /// <p>
    /// <code>&lt;Grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    /// xsi:type="CanonicalUser"><ID><>ID<></ID><DisplayName><>GranteesEmail<></DisplayName>
    /// &lt;/Grantee&gt;</code>
    /// </p>
    /// <p>DisplayName is optional and ignored in the request.</p>
    /// </li>
    /// <li>
    /// <p>By Email address:</p>
    /// <p>
    /// <code> &lt;Grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    /// xsi:type="AmazonCustomerByEmail"><EmailAddress><>Grantees@email.com<></EmailAddress>&lt;/Grantee&gt;</code>
    /// </p>
    /// <p>The grantee is resolved to the CanonicalUser and, in a response to a GET Object
    /// acl request, appears as the CanonicalUser.</p>
    /// </li>
    /// <li>
    /// <p>By URI:</p>
    /// <p>
    /// <code>&lt;Grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    /// xsi:type="Group"><URI><>http://acs.amazonaws.com/groups/global/AuthenticatedUsers<></URI>&lt;/Grantee&gt;</code>
    /// </p>
    /// </li>
    /// </ul>
    ///
    ///
    /// <p>To enable logging, you use LoggingEnabled and its children request elements. To disable
    /// logging, you use an empty BucketLoggingStatus request element:</p>
    ///
    /// <p>
    /// <code>&lt;BucketLoggingStatus xmlns="http://doc.s3.amazonaws.com/2006-03-01"
    /// /></code>
    /// </p>
    ///
    /// <p>For more information about server access logging, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/ServerLogs.html">Server Access Logging</a> in the <i>Amazon S3 User Guide</i>. </p>
    ///
    /// <p>For more information about creating a bucket, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>. For more
    /// information about returning the logging status of a bucket, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLogging.html">GetBucketLogging</a>.</p>
    ///
    /// <p>The following operations are related to <code>PutBucketLogging</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucket.html">DeleteBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLogging.html">GetBucketLogging</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_logging(&self, _input: PutBucketLoggingInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutBucketLogging is not implemented yet"))
    }

    /// <p>Sets a metrics configuration (specified by the metrics configuration ID) for the bucket.
    /// You can have up to 1,000 metrics configurations per bucket. If you're updating an existing
    /// metrics configuration, note that this is a full replacement of the existing metrics
    /// configuration. If you don't include the elements you want to keep, they are erased.</p>
    ///
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:PutMetricsConfiguration</code> action. The bucket owner has this permission by
    /// default. The bucket owner can grant this permission to others. For more information about
    /// permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p>For information about CloudWatch request metrics for Amazon S3, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cloudwatch-monitoring.html">Monitoring Metrics with Amazon
    /// CloudWatch</a>.</p>
    ///
    /// <p>The following operations are related to
    /// <code>PutBucketMetricsConfiguration</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketMetricsConfiguration.html">DeleteBucketMetricsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketMetricsConfiguration.html">GetBucketMetricsConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketMetricsConfigurations.html">ListBucketMetricsConfigurations</a>
    /// </p>
    /// </li>
    /// </ul>
    ///
    /// <p>
    /// <code>GetBucketLifecycle</code> has the following special error:</p>
    /// <ul>
    /// <li>
    /// <p>Error code: <code>TooManyConfigurations</code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Description: You are attempting to create a new configuration but have
    /// already reached the 1,000-configuration limit.</p>
    /// </li>
    /// <li>
    /// <p>HTTP Status Code: HTTP 400 Bad Request</p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    async fn put_bucket_metrics_configuration(&self, _input: PutBucketMetricsConfigurationInput) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "PutBucketMetricsConfiguration is not implemented yet"
        ))
    }

    /// <p>Enables notifications of specified events for a bucket. For more information about event
    /// notifications, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring Event
    /// Notifications</a>.</p>
    ///
    /// <p>Using this API, you can replace an existing notification configuration. The
    /// configuration is an XML file that defines the event types that you want Amazon S3 to publish and
    /// the destination where you want Amazon S3 to publish an event notification when it detects an
    /// event of the specified type.</p>
    ///
    /// <p>By default, your bucket has no event notifications configured. That is, the notification
    /// configuration will be an empty <code>NotificationConfiguration</code>.</p>
    ///
    /// <p>
    /// <code>&lt;NotificationConfiguration&gt;</code>
    /// </p>
    /// <p>
    /// <code>&lt;/NotificationConfiguration&gt;</code>
    /// </p>
    /// <p>This action replaces the existing notification configuration with the configuration
    /// you include in the request body.</p>
    ///
    /// <p>After Amazon S3 receives this request, it first verifies that any Amazon Simple Notification
    /// Service (Amazon SNS) or Amazon Simple Queue Service (Amazon SQS) destination exists, and
    /// that the bucket owner has permission to publish to it by sending a test notification. In
    /// the case of Lambda destinations, Amazon S3 verifies that the Lambda function permissions
    /// grant Amazon S3 permission to invoke the function from the Amazon S3 bucket. For more information,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring Notifications for Amazon S3
    /// Events</a>.</p>
    ///
    /// <p>You can disable notifications by adding the empty NotificationConfiguration
    /// element.</p>
    /// <p>For more information about the number of event notification configurations that you can create per bucket, see
    /// <a href="https://docs.aws.amazon.com/general/latest/gr/s3.html#limits_s3">Amazon S3 service quotas</a> in <i>Amazon Web Services General Reference</i>.</p>
    /// <p>By default, only the bucket owner can configure notifications on a bucket. However,
    /// bucket owners can use a bucket policy to grant permission to other users to set this
    /// configuration with <code>s3:PutBucketNotification</code> permission.</p>
    ///
    /// <note>
    /// <p>The PUT notification is an atomic operation. For example, suppose your notification
    /// configuration includes SNS topic, SQS queue, and Lambda function configurations. When
    /// you send a PUT request with this configuration, Amazon S3 sends test messages to your SNS
    /// topic. If the message fails, the entire PUT action will fail, and Amazon S3 will not add
    /// the configuration to your bucket.</p>
    /// </note>
    ///
    /// <p>
    /// <b>Responses</b>
    /// </p>
    /// <p>If the configuration in the request body includes only one
    /// <code>TopicConfiguration</code> specifying only the
    /// <code>s3:ReducedRedundancyLostObject</code> event type, the response will also include
    /// the <code>x-amz-sns-test-message-id</code> header containing the message ID of the test
    /// notification sent to the topic.</p>
    ///
    /// <p>The following action is related to
    /// <code>PutBucketNotificationConfiguration</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketNotificationConfiguration.html">GetBucketNotificationConfiguration</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_notification_configuration(&self, _input: PutBucketNotificationConfigurationInput) -> S3Result {
        Err(s3_error!(
            NotImplemented,
            "PutBucketNotificationConfiguration is not implemented yet"
        ))
    }

    /// <p>Creates or modifies <code>OwnershipControls</code> for an Amazon S3 bucket. To use this
    /// operation, you must have the <code>s3:PutBucketOwnershipControls</code> permission. For
    /// more information about Amazon S3 permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/user-guide/using-with-s3-actions.html">Specifying permissions in a policy</a>. </p>
    /// <p>For information about Amazon S3 Object Ownership, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/user-guide/about-object-ownership.html">Using object ownership</a>. </p>
    /// <p>The following operations are related to <code>PutBucketOwnershipControls</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a>GetBucketOwnershipControls</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a>DeleteBucketOwnershipControls</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_ownership_controls(&self, _input: PutBucketOwnershipControlsInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutBucketOwnershipControls is not implemented yet"))
    }

    /// <p>Applies an Amazon S3 bucket policy to an Amazon S3 bucket. If you are using an identity other than
    /// the root user of the Amazon Web Services account that owns the bucket, the calling identity must have the
    /// <code>PutBucketPolicy</code> permissions on the specified bucket and belong to the
    /// bucket owner's account in order to use this operation.</p>
    ///
    /// <p>If you don't have <code>PutBucketPolicy</code> permissions, Amazon S3 returns a <code>403
    /// Access Denied</code> error. If you have the correct permissions, but you're not using an
    /// identity that belongs to the bucket owner's account, Amazon S3 returns a <code>405 Method Not
    /// Allowed</code> error.</p>
    ///
    /// <important>
    /// <p> As a security precaution, the root user of the Amazon Web Services account that owns a bucket can
    /// always use this operation, even if the policy explicitly denies the root user the
    /// ability to perform this action. </p>
    /// </important>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/example-bucket-policies.html">Bucket policy examples</a>.</p>
    ///
    /// <p>The following operations are related to <code>PutBucketPolicy</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucket.html">DeleteBucket</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_policy(&self, _input: PutBucketPolicyInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutBucketPolicy is not implemented yet"))
    }

    /// <p> Creates a replication configuration or replaces an existing one. For more information,
    /// see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication.html">Replication</a> in the <i>Amazon S3 User Guide</i>. </p>
    ///
    /// <p>Specify the replication configuration in the request body. In the replication
    /// configuration, you provide the name of the destination bucket or buckets where you want
    /// Amazon S3 to replicate objects, the IAM role that Amazon S3 can assume to replicate objects on your
    /// behalf, and other relevant information.</p>
    ///
    ///
    /// <p>A replication configuration must include at least one rule, and can contain a maximum of
    /// 1,000. Each rule identifies a subset of objects to replicate by filtering the objects in
    /// the source bucket. To choose additional subsets of objects to replicate, add a rule for
    /// each subset.</p>
    ///
    /// <p>To specify a subset of the objects in the source bucket to apply a replication rule to,
    /// add the Filter element as a child of the Rule element. You can filter objects based on an
    /// object key prefix, one or more object tags, or both. When you add the Filter element in the
    /// configuration, you must also add the following elements:
    /// <code>DeleteMarkerReplication</code>, <code>Status</code>, and
    /// <code>Priority</code>.</p>
    /// <note>
    /// <p>If you are using an earlier version of the replication configuration, Amazon S3 handles
    /// replication of delete markers differently. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-add-config.html#replication-backward-compat-considerations">Backward Compatibility</a>.</p>
    /// </note>
    /// <p>For information about enabling versioning on a bucket, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/Versioning.html">Using Versioning</a>.</p>
    ///
    /// <p>
    /// <b>Handling Replication of Encrypted Objects</b>
    /// </p>
    /// <p>By default, Amazon S3 doesn't replicate objects that are stored at rest using server-side
    /// encryption with KMS keys. To replicate Amazon Web Services KMS-encrypted objects, add the
    /// following: <code>SourceSelectionCriteria</code>, <code>SseKmsEncryptedObjects</code>,
    /// <code>Status</code>, <code>EncryptionConfiguration</code>, and
    /// <code>ReplicaKmsKeyID</code>. For information about replication configuration, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-config-for-kms-objects.html">Replicating Objects
    /// Created with SSE Using KMS keys</a>.</p>
    ///
    /// <p>For information on <code>PutBucketReplication</code> errors, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html#ReplicationErrorCodeList">List of
    /// replication-related error codes</a>
    /// </p>
    ///
    /// <p>
    /// <b>Permissions</b>
    /// </p>
    /// <p>To create a <code>PutBucketReplication</code> request, you must have <code>s3:PutReplicationConfiguration</code>
    /// permissions for the bucket.
    /// </p>
    /// <p>By default, a resource owner, in this case the Amazon Web Services account that created the bucket, can
    /// perform this operation. The resource owner can also grant others permissions to perform the
    /// operation. For more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying Permissions in a Policy</a>
    /// and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your
    /// Amazon S3 Resources</a>.</p>
    /// <note>
    /// <p>To perform this operation, the user or role performing the action must have the
    /// <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html">iam:PassRole</a> permission.</p>
    /// </note>
    ///
    /// <p>The following operations are related to <code>PutBucketReplication</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketReplication.html">GetBucketReplication</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketReplication.html">DeleteBucketReplication</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_replication(&self, _input: PutBucketReplicationInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutBucketReplication is not implemented yet"))
    }

    /// <p>Sets the request payment configuration for a bucket. By default, the bucket owner pays
    /// for downloads from the bucket. This configuration parameter enables the bucket owner (only)
    /// to specify that the person requesting the download will be charged for the download. For
    /// more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html">Requester Pays
    /// Buckets</a>.</p>
    ///
    /// <p>The following operations are related to <code>PutBucketRequestPayment</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketRequestPayment.html">GetBucketRequestPayment</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_request_payment(&self, _input: PutBucketRequestPaymentInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutBucketRequestPayment is not implemented yet"))
    }

    /// <p>Sets the tags for a bucket.</p>
    /// <p>Use tags to organize your Amazon Web Services bill to reflect your own cost structure. To do this, sign
    /// up to get your Amazon Web Services account bill with tag key values included. Then, to see the cost of
    /// combined resources, organize your billing information according to resources with the same
    /// tag key values. For example, you can tag several resources with a specific application
    /// name, and then organize your billing information to see the total cost of that application
    /// across several services. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Cost Allocation
    /// and Tagging</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/CostAllocTagging.html">Using Cost Allocation in Amazon S3 Bucket
    /// Tags</a>.</p>
    ///
    /// <note>
    /// <p>
    /// When this operation sets the tags for a bucket, it will overwrite any current tags the
    /// bucket already has. You cannot use this operation to add tags to an existing list of tags.</p>
    /// </note>
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:PutBucketTagging</code> action. The bucket owner has this permission by default
    /// and can grant this permission to others. For more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a>.</p>
    ///
    /// <p>
    /// <code>PutBucketTagging</code> has the following special errors:</p>
    /// <ul>
    /// <li>
    /// <p>Error code: <code>InvalidTagError</code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Description: The tag provided was not a valid tag. This error can occur if
    /// the tag did not pass input validation. For information about tag restrictions,
    /// see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> and <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/aws-tag-restrictions.html">Amazon Web Services-Generated Cost Allocation Tag Restrictions</a>.</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p>Error code: <code>MalformedXMLError</code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Description: The XML provided does not match the schema.</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p>Error code: <code>OperationAbortedError </code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Description: A conflicting conditional action is currently in progress
    /// against this resource. Please try again.</p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p>Error code: <code>InternalError</code>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>Description: The service was unable to apply the provided tag to the
    /// bucket.</p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    ///
    ///
    /// <p>The following operations are related to <code>PutBucketTagging</code>:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketTagging.html">GetBucketTagging</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketTagging.html">DeleteBucketTagging</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_tagging(&self, _input: PutBucketTaggingInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutBucketTagging is not implemented yet"))
    }

    /// <p>Sets the versioning state of an existing bucket.</p>
    /// <p>You can set the versioning state with one of the following values:</p>
    ///
    /// <p>
    /// <b>Enabled</b>—Enables versioning for the objects in the
    /// bucket. All objects added to the bucket receive a unique version ID.</p>
    ///
    /// <p>
    /// <b>Suspended</b>—Disables versioning for the objects in the
    /// bucket. All objects added to the bucket receive the version ID null.</p>
    ///
    /// <p>If the versioning state has never been set on a bucket, it has no versioning state; a
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketVersioning.html">GetBucketVersioning</a> request does not return a versioning state value.</p>
    ///
    /// <p>In order to enable MFA Delete, you must be the bucket owner. If you are the bucket owner
    /// and want to enable MFA Delete in the bucket versioning configuration, you must
    /// include the <code>x-amz-mfa request</code> header and the
    /// <code>Status</code> and the <code>MfaDelete</code> request elements in a request to set
    /// the versioning state of the bucket.</p>
    ///
    /// <important>
    /// <p>If you have an object expiration lifecycle policy in your non-versioned bucket and
    /// you want to maintain the same permanent delete behavior when you enable versioning, you
    /// must add a noncurrent expiration policy. The noncurrent expiration lifecycle policy will
    /// manage the deletes of the noncurrent object versions in the version-enabled bucket. (A
    /// version-enabled bucket maintains one current and zero or more noncurrent object
    /// versions.) For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html#lifecycle-and-other-bucket-config">Lifecycle and Versioning</a>.</p>
    /// </important>
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucket.html">DeleteBucket</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketVersioning.html">GetBucketVersioning</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_bucket_versioning(&self, _input: PutBucketVersioningInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutBucketVersioning is not implemented yet"))
    }

    /// <p>Sets the configuration of the website that is specified in the <code>website</code>
    /// subresource. To configure a bucket as a website, you can add this subresource on the bucket
    /// with website configuration information such as the file name of the index document and any
    /// redirect rules. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html">Hosting Websites on Amazon S3</a>.</p>
    ///
    /// <p>This PUT action requires the <code>S3:PutBucketWebsite</code> permission. By default,
    /// only the bucket owner can configure the website attached to a bucket; however, bucket
    /// owners can allow other users to set the website configuration by writing a bucket policy
    /// that grants them the <code>S3:PutBucketWebsite</code> permission.</p>
    ///
    /// <p>To redirect all website requests sent to the bucket's website endpoint, you add a
    /// website configuration with the following elements. Because all requests are sent to another
    /// website, you don't need to provide index document name for the bucket.</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>WebsiteConfiguration</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>RedirectAllRequestsTo</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>HostName</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>Protocol</code>
    /// </p>
    /// </li>
    /// </ul>
    ///
    /// <p>If you want granular control over redirects, you can use the following elements to add
    /// routing rules that describe conditions for redirecting requests and information about the
    /// redirect destination. In this case, the website configuration must provide an index
    /// document for the bucket, because some requests might not be redirected. </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>WebsiteConfiguration</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>IndexDocument</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>Suffix</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>ErrorDocument</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>Key</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>RoutingRules</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>RoutingRule</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>Condition</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>HttpErrorCodeReturnedEquals</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>KeyPrefixEquals</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>Redirect</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>Protocol</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>HostName</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>ReplaceKeyPrefixWith</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>ReplaceKeyWith</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>HttpRedirectCode</code>
    /// </p>
    /// </li>
    /// </ul>
    ///
    /// <p>Amazon S3 has a limitation of 50 routing rules per website configuration. If you require more
    /// than 50 routing rules, you can use object redirect. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html">Configuring an
    /// Object Redirect</a> in the <i>Amazon S3 User Guide</i>.</p>
    async fn put_bucket_website(&self, _input: PutBucketWebsiteInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutBucketWebsite is not implemented yet"))
    }

    /// <p>Adds an object to a bucket. You must have WRITE permissions on a bucket to add an object
    /// to it.</p>
    ///
    ///
    /// <p>Amazon S3 never adds partial objects; if you receive a success response, Amazon S3 added the
    /// entire object to the bucket.</p>
    ///
    /// <p>Amazon S3 is a distributed system. If it receives multiple write requests for the same object
    /// simultaneously, it overwrites all but the last object written. Amazon S3 does not provide object
    /// locking; if you need this, make sure to build it into your application layer or use
    /// versioning instead.</p>
    ///
    /// <p>To ensure that data is not corrupted traversing the network, use the
    /// <code>Content-MD5</code> header. When you use this header, Amazon S3 checks the object
    /// against the provided MD5 value and, if they do not match, returns an error. Additionally,
    /// you can calculate the MD5 while putting an object to Amazon S3 and compare the returned ETag to
    /// the calculated MD5 value.</p>
    /// <note>
    /// <ul>
    /// <li>
    /// <p>To successfully complete the <code>PutObject</code> request, you must have the
    /// <code>s3:PutObject</code> in your IAM permissions.</p>
    /// </li>
    /// <li>
    /// <p>To successfully change the objects acl of your <code>PutObject</code> request,
    /// you must have the <code>s3:PutObjectAcl</code> in your IAM permissions.</p>
    /// </li>
    /// <li>
    /// <p> The <code>Content-MD5</code> header is required for any request to upload an object
    /// with a retention period configured using Amazon S3 Object Lock. For more information about
    /// Amazon S3 Object Lock, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html">Amazon S3 Object Lock Overview</a>
    /// in the <i>Amazon S3 User Guide</i>. </p>
    /// </li>
    /// </ul>
    /// </note>
    /// <p>
    /// <b>Server-side Encryption</b>
    /// </p>
    /// <p>You can optionally request server-side encryption. With server-side encryption, Amazon S3 encrypts
    /// your data as it writes it to disks in its data centers and decrypts the data
    /// when you access it. You have the option to provide your own encryption key or use Amazon Web Services
    /// managed encryption keys (SSE-S3 or SSE-KMS). For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingServerSideEncryption.html">Using Server-Side
    /// Encryption</a>.</p>
    /// <p>If you request server-side encryption using Amazon Web Services Key Management Service (SSE-KMS), you can enable
    /// an S3 Bucket Key at the object-level. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html">Amazon S3 Bucket Keys</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    /// <p>
    /// <b>Access Control List (ACL)-Specific Request
    /// Headers</b>
    /// </p>
    /// <p>You can use headers to grant ACL- based permissions. By default, all objects are
    /// private. Only the owner has full access control. When adding a new object, you can grant
    /// permissions to individual Amazon Web Services accounts or to predefined groups defined by Amazon S3. These
    /// permissions are then added to the ACL on the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html">Access Control List
    /// (ACL) Overview</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-using-rest-api.html">Managing ACLs Using the REST
    /// API</a>. </p>
    /// <p>If the bucket that you're uploading objects to uses the bucket owner enforced setting
    /// for S3 Object Ownership, ACLs are disabled and no longer affect permissions. Buckets that
    /// use this setting only accept PUT requests that don't specify an ACL or PUT requests that
    /// specify bucket owner full control ACLs, such as the <code>bucket-owner-full-control</code> canned
    /// ACL or an equivalent form of this ACL expressed in the XML format. PUT requests that contain other
    /// ACLs (for example, custom grants to certain Amazon Web Services accounts) fail and return a
    /// <code>400</code> error with the error code
    /// <code>AccessControlListNotSupported</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html"> Controlling ownership of
    /// objects and disabling ACLs</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <note>
    /// <p>If your bucket uses the bucket owner enforced setting for Object Ownership,
    /// all objects written to the bucket by any account will be owned by the bucket owner.</p>
    /// </note>
    /// <p>
    /// <b>Storage Class Options</b>
    /// </p>
    /// <p>By default, Amazon S3 uses the STANDARD Storage Class to store newly created objects. The
    /// STANDARD storage class provides high durability and high availability. Depending on
    /// performance needs, you can specify a different Storage Class. Amazon S3 on Outposts only uses
    /// the OUTPOSTS Storage Class. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html">Storage Classes</a> in the
    /// <i>Amazon S3 User Guide</i>.</p>
    ///
    ///
    /// <p>
    /// <b>Versioning</b>
    /// </p>
    /// <p>If you enable versioning for a bucket, Amazon S3 automatically generates a unique version ID
    /// for the object being stored. Amazon S3 returns this ID in the response. When you enable
    /// versioning for a bucket, if Amazon S3 receives multiple write requests for the same object
    /// simultaneously, it stores all of the objects.</p>
    /// <p>For more information about versioning, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/AddingObjectstoVersioningEnabledBuckets.html">Adding Objects to
    /// Versioning Enabled Buckets</a>. For information about returning the versioning state
    /// of a bucket, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketVersioning.html">GetBucketVersioning</a>. </p>
    ///
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CopyObject.html">CopyObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObject.html">DeleteObject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_object(&self, _input: PutObjectInput) -> S3Result<PutObjectOutput> {
        Err(s3_error!(NotImplemented, "PutObject is not implemented yet"))
    }

    /// <p>Uses the <code>acl</code> subresource to set the access control list (ACL) permissions
    /// for a new or existing object in an S3 bucket. You must have <code>WRITE_ACP</code>
    /// permission to set the ACL of an object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#permissions">What
    /// permissions can I grant?</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    /// <p>Depending on your application needs, you can choose to set
    /// the ACL on an object using either the request body or the headers. For example, if you have
    /// an existing application that updates a bucket ACL using the request body, you can continue
    /// to use that approach. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html">Access Control List (ACL) Overview</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <important>
    /// <p>If your bucket uses the bucket owner enforced setting for S3 Object Ownership, ACLs are disabled and no longer affect permissions.
    /// You must use policies to grant access to your bucket and the objects in it. Requests to set ACLs or update ACLs fail and
    /// return the <code>AccessControlListNotSupported</code> error code. Requests to read ACLs are still supported.
    /// For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">Controlling object ownership</a>
    /// in the <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    ///
    /// <p>
    /// <b>Access Permissions</b>
    /// </p>
    /// <p>You can set access permissions using one of the following methods:</p>
    /// <ul>
    /// <li>
    /// <p>Specify a canned ACL with the <code>x-amz-acl</code> request header. Amazon S3 supports
    /// a set of predefined ACLs, known as canned ACLs. Each canned ACL has a predefined set
    /// of grantees and permissions. Specify the canned ACL name as the value of
    /// <code>x-amz-ac</code>l. If you use this header, you cannot use other access
    /// control-specific headers in your request. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL">Canned ACL</a>.</p>
    /// </li>
    /// <li>
    /// <p>Specify access permissions explicitly with the <code>x-amz-grant-read</code>,
    /// <code>x-amz-grant-read-acp</code>, <code>x-amz-grant-write-acp</code>, and
    /// <code>x-amz-grant-full-control</code> headers. When using these headers, you
    /// specify explicit access permissions and grantees (Amazon Web Services accounts or Amazon S3 groups) who
    /// will receive the permission. If you use these ACL-specific headers, you cannot use
    /// <code>x-amz-acl</code> header to set a canned ACL. These parameters map to the set
    /// of permissions that Amazon S3 supports in an ACL. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html">Access Control List (ACL)
    /// Overview</a>.</p>
    ///
    /// <p>You specify each grantee as a type=value pair, where the type is one of the
    /// following:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>id</code> – if the value specified is the canonical user ID of an Amazon Web Services account</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>uri</code> – if you are granting permissions to a predefined
    /// group</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>emailAddress</code> – if the value specified is the email address of
    /// an Amazon Web Services account</p>
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
    /// </li>
    /// </ul>
    /// <p>For example, the following <code>x-amz-grant-read</code> header grants list
    /// objects permission to the two Amazon Web Services accounts identified by their email
    /// addresses.</p>
    /// <p>
    /// <code>x-amz-grant-read: emailAddress="xyz@amazon.com",
    /// emailAddress="abc@amazon.com" </code>
    /// </p>
    ///
    /// </li>
    /// </ul>
    /// <p>You can use either a canned ACL or specify access permissions explicitly. You cannot do
    /// both.</p>
    /// <p>
    /// <b>Grantee Values</b>
    /// </p>
    /// <p>You can specify the person (grantee) to whom you're assigning access rights (using
    /// request elements) in the following ways:</p>
    /// <ul>
    /// <li>
    /// <p>By the person's ID:</p>
    /// <p>
    /// <code>&lt;Grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    /// xsi:type="CanonicalUser"><ID><>ID<></ID><DisplayName><>GranteesEmail<></DisplayName>
    /// &lt;/Grantee&gt;</code>
    /// </p>
    /// <p>DisplayName is optional and ignored in the request.</p>
    /// </li>
    /// <li>
    /// <p>By URI:</p>
    /// <p>
    /// <code>&lt;Grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    /// xsi:type="Group"><URI><>http://acs.amazonaws.com/groups/global/AuthenticatedUsers<></URI>&lt;/Grantee&gt;</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>By Email address:</p>
    /// <p>
    /// <code>&lt;Grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    /// xsi:type="AmazonCustomerByEmail"><EmailAddress><>Grantees@email.com<></EmailAddress>lt;/Grantee></code>
    /// </p>
    /// <p>The grantee is resolved to the CanonicalUser and, in a response to a GET Object
    /// acl request, appears as the CanonicalUser.</p>
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
    /// </li>
    /// </ul>
    /// <p>
    /// <b>Versioning</b>
    /// </p>
    /// <p>The ACL of an object is set at the object version level. By default, PUT sets the ACL of
    /// the current version of an object. To set the ACL of a different version, use the
    /// <code>versionId</code> subresource.</p>
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CopyObject.html">CopyObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_object_acl(&self, _input: PutObjectAclInput) -> S3Result<PutObjectAclOutput> {
        Err(s3_error!(NotImplemented, "PutObjectAcl is not implemented yet"))
    }

    /// <p>Applies a legal hold configuration to the specified object. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Locking
    /// Objects</a>.</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    async fn put_object_legal_hold(&self, _input: PutObjectLegalHoldInput) -> S3Result<PutObjectLegalHoldOutput> {
        Err(s3_error!(NotImplemented, "PutObjectLegalHold is not implemented yet"))
    }

    /// <p>Places an Object Lock configuration on the specified bucket. The rule specified in the
    /// Object Lock configuration will be applied by default to every new object placed in the
    /// specified bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Locking Objects</a>.
    /// </p>
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
    /// <li>
    /// <p>You can only enable Object Lock for new buckets. If you want to turn on
    /// Object Lock for an existing bucket, contact Amazon Web Services Support.</p>
    /// </li>
    /// </ul>
    /// </note>
    async fn put_object_lock_configuration(
        &self,
        _input: PutObjectLockConfigurationInput,
    ) -> S3Result<PutObjectLockConfigurationOutput> {
        Err(s3_error!(NotImplemented, "PutObjectLockConfiguration is not implemented yet"))
    }

    /// <p>Places an Object Retention configuration on an object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Locking Objects</a>.
    /// Users or accounts require the <code>s3:PutObjectRetention</code> permission in order to place
    /// an Object Retention configuration on objects. Bypassing a Governance Retention configuration
    /// requires the <code>s3:BypassGovernanceRetention</code> permission.
    /// </p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    async fn put_object_retention(&self, _input: PutObjectRetentionInput) -> S3Result<PutObjectRetentionOutput> {
        Err(s3_error!(NotImplemented, "PutObjectRetention is not implemented yet"))
    }

    /// <p>Sets the supplied tag-set to an object that already exists in a bucket.</p>
    /// <p>A tag is a key-value pair. You can associate tags with an object by sending a PUT
    /// request against the tagging subresource that is associated with the object. You can
    /// retrieve tags by sending a GET request. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectTagging.html">GetObjectTagging</a>.</p>
    ///
    /// <p>For tagging-related restrictions related to characters and encodings, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">Tag
    /// Restrictions</a>. Note that Amazon S3 limits the maximum number of tags to 10 tags per
    /// object.</p>
    ///
    /// <p>To use this operation, you must have permission to perform the
    /// <code>s3:PutObjectTagging</code> action. By default, the bucket owner has this
    /// permission and can grant this permission to others.</p>
    ///
    /// <p>To put tags of any other version, use the <code>versionId</code> query parameter. You
    /// also need permission for the <code>s3:PutObjectVersionTagging</code> action.</p>
    ///
    /// <p>For information about the Amazon S3 object tagging feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-tagging.html">Object Tagging</a>.</p>
    ///
    ///
    /// <p class="title">
    /// <b>Special Errors</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code: InvalidTagError </i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: The tag provided was not a valid tag. This error can occur
    /// if the tag did not pass input validation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-tagging.html">Object Tagging</a>.</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code: MalformedXMLError </i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: The XML provided does not match the schema.</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code: OperationAbortedError </i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: A conflicting conditional action is currently in
    /// progress against this resource. Please try again.</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code: InternalError</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: The service was unable to apply the provided tag to the
    /// object.</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    ///
    ///
    ///
    ///
    ///
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectTagging.html">GetObjectTagging</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObjectTagging.html">DeleteObjectTagging</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_object_tagging(&self, _input: PutObjectTaggingInput) -> S3Result<PutObjectTaggingOutput> {
        Err(s3_error!(NotImplemented, "PutObjectTagging is not implemented yet"))
    }

    /// <p>Creates or modifies the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket.
    /// To use this operation, you must have the <code>s3:PutBucketPublicAccessBlock</code>
    /// permission. For more information about Amazon S3 permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying Permissions in a
    /// Policy</a>.</p>
    ///
    /// <important>
    /// <p>When Amazon S3 evaluates the <code>PublicAccessBlock</code> configuration for a bucket or
    /// an object, it checks the <code>PublicAccessBlock</code> configuration for both the
    /// bucket (or the bucket that contains the object) and the bucket owner's account. If the
    /// <code>PublicAccessBlock</code> configurations are different between the bucket and
    /// the account, Amazon S3 uses the most restrictive combination of the bucket-level and
    /// account-level settings.</p>
    /// </important>
    ///
    ///
    /// <p>For more information about when Amazon S3 considers a bucket or an object public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a>.</p>
    ///
    ///
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetPublicAccessBlock.html">GetPublicAccessBlock</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeletePublicAccessBlock.html">DeletePublicAccessBlock</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketPolicyStatus.html">GetBucketPolicyStatus</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html">Using Amazon S3 Block
    /// Public Access</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn put_public_access_block(&self, _input: PutPublicAccessBlockInput) -> S3Result {
        Err(s3_error!(NotImplemented, "PutPublicAccessBlock is not implemented yet"))
    }

    /// <p>Restores an archived copy of an object back into Amazon S3</p>
    /// <p>This action is not supported by Amazon S3 on Outposts.</p>
    /// <p>This action performs the following types of requests: </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>select</code> - Perform a select query on an archived object</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>restore an archive</code> - Restore an archived object</p>
    /// </li>
    /// </ul>
    /// <p>To use this operation, you must have permissions to perform the
    /// <code>s3:RestoreObject</code> action. The bucket owner has this permission by default
    /// and can grant this permission to others. For more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3
    /// Resources</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>
    /// <b>Querying Archives with Select Requests</b>
    /// </p>
    /// <p>You use a select type of request to perform SQL queries on archived objects. The
    /// archived objects that are being queried by the select request must be formatted as
    /// uncompressed comma-separated values (CSV) files. You can run queries and custom analytics
    /// on your archived data without having to restore your data to a hotter Amazon S3 tier. For an
    /// overview about select requests, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/querying-glacier-archives.html">Querying Archived Objects</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When making a select request, do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Define an output location for the select query's output. This must be an Amazon S3
    /// bucket in the same Amazon Web Services Region as the bucket that contains the archive object that is
    /// being queried. The Amazon Web Services account that initiates the job must have permissions to write
    /// to the S3 bucket. You can specify the storage class and encryption for the output
    /// objects stored in the bucket. For more information about output, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/querying-glacier-archives.html">Querying Archived Objects</a>
    /// in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>For more information about the <code>S3</code> structure in the request body, see
    /// the following:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/S3_ACLs_UsingACLs.html">Managing Access with
    /// ACLs</a> in the <i>Amazon S3 User Guide</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/serv-side-encryption.html">Protecting Data Using
    /// Server-Side Encryption</a> in the
    /// <i>Amazon S3 User Guide</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <p>Define the SQL expression for the <code>SELECT</code> type of restoration for your
    /// query in the request body's <code>SelectParameters</code> structure. You can use
    /// expressions like the following examples.</p>
    /// <ul>
    /// <li>
    /// <p>The following expression returns all records from the specified
    /// object.</p>
    /// <p>
    /// <code>SELECT * FROM Object</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>Assuming that you are not using any headers for data stored in the object,
    /// you can specify columns with positional headers.</p>
    /// <p>
    /// <code>SELECT s._1, s._2 FROM Object s WHERE s._3 > 100</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>If you have headers and you set the <code>fileHeaderInfo</code> in the
    /// <code>CSV</code> structure in the request body to <code>USE</code>, you can
    /// specify headers in the query. (If you set the <code>fileHeaderInfo</code> field
    /// to <code>IGNORE</code>, the first row is skipped for the query.) You cannot mix
    /// ordinal positions with header column names. </p>
    /// <p>
    /// <code>SELECT s.Id, s.FirstName, s.SSN FROM S3Object s</code>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    /// <p>For more information about using SQL with S3 Glacier Select restore, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-glacier-select-sql-reference.html">SQL Reference for Amazon S3 Select and
    /// S3 Glacier Select</a> in the <i>Amazon S3 User Guide</i>. </p>
    /// <p>When making a select request, you can also do the following:</p>
    /// <ul>
    /// <li>
    /// <p>To expedite your queries, specify the <code>Expedited</code> tier. For more
    /// information about tiers, see "Restoring Archives," later in this topic.</p>
    /// </li>
    /// <li>
    /// <p>Specify details about the data serialization format of both the input object that
    /// is being queried and the serialization of the CSV-encoded query results.</p>
    /// </li>
    /// </ul>
    /// <p>The following are additional important facts about the select feature:</p>
    /// <ul>
    /// <li>
    /// <p>The output results are new Amazon S3 objects. Unlike archive retrievals, they are
    /// stored until explicitly deleted-manually or through a lifecycle policy.</p>
    /// </li>
    /// <li>
    /// <p>You can issue more than one select request on the same Amazon S3 object. Amazon S3 doesn't
    /// deduplicate requests, so avoid issuing duplicate requests.</p>
    /// </li>
    /// <li>
    /// <p> Amazon S3 accepts a select request even if the object has already been restored. A
    /// select request doesn’t return error response <code>409</code>.</p>
    /// </li>
    /// </ul>
    /// <p>
    /// <b>Restoring objects</b>
    /// </p>
    /// <p>Objects that you archive to the S3 Glacier or
    /// S3 Glacier Deep Archive storage class, and S3 Intelligent-Tiering Archive or
    /// S3 Intelligent-Tiering Deep Archive tiers are not accessible in real time. For objects in
    /// Archive Access or Deep Archive Access tiers you must first initiate a restore request, and
    /// then wait until the object is moved into the Frequent Access tier. For objects in
    /// S3 Glacier or S3 Glacier Deep Archive storage classes you must
    /// first initiate a restore request, and then wait until a temporary copy of the object is
    /// available. To access an archived object, you must restore the object for the duration
    /// (number of days) that you specify.</p>
    /// <p>To restore a specific object version, you can provide a version ID. If you don't provide
    /// a version ID, Amazon S3 restores the current version.</p>
    /// <p>When restoring an archived object (or using a select request), you can specify one of
    /// the following data access tier options in the <code>Tier</code> element of the request
    /// body: </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>Expedited</code> - Expedited retrievals allow you to quickly access your
    /// data stored in the S3 Glacier storage class or S3 Intelligent-Tiering Archive
    /// tier when occasional urgent requests for a subset of archives are required. For all
    /// but the largest archived objects (250 MB+), data accessed using Expedited retrievals
    /// is typically made available within 1–5 minutes. Provisioned capacity ensures that
    /// retrieval capacity for Expedited retrievals is available when you need it. Expedited
    /// retrievals and provisioned capacity are not available for objects stored in the
    /// S3 Glacier Deep Archive storage class or S3 Intelligent-Tiering Deep Archive tier.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>Standard</code> - Standard retrievals allow you to access any of your
    /// archived objects within several hours. This is the default option for retrieval
    /// requests that do not specify the retrieval option. Standard retrievals typically
    /// finish within 3–5 hours for objects stored in the S3 Glacier storage
    /// class or S3 Intelligent-Tiering Archive tier. They typically finish within 12 hours for
    /// objects stored in the S3 Glacier Deep Archive storage class or
    /// S3 Intelligent-Tiering Deep Archive tier. Standard retrievals are free for objects stored in
    /// S3 Intelligent-Tiering.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>Bulk</code> - Bulk retrievals are the lowest-cost retrieval option in
    /// S3 Glacier, enabling you to retrieve large amounts, even petabytes, of data
    /// inexpensively. Bulk retrievals typically finish within 5–12 hours for objects stored
    /// in the S3 Glacier storage class or S3 Intelligent-Tiering Archive tier. They
    /// typically finish within 48 hours for objects stored in the
    /// S3 Glacier Deep Archive storage class or S3 Intelligent-Tiering Deep Archive tier. Bulk
    /// retrievals are free for objects stored in S3 Intelligent-Tiering.</p>
    /// </li>
    /// </ul>
    /// <p>For more information about archive retrieval options and provisioned capacity for
    /// <code>Expedited</code> data access, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/restoring-objects.html">Restoring Archived Objects</a> in the <i>Amazon S3 User Guide</i>. </p>
    /// <p>You can use Amazon S3 restore speed upgrade to change the restore speed to a faster speed
    /// while it is in progress. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/restoring-objects.html#restoring-objects-upgrade-tier.title.html">
    /// Upgrading the speed of an in-progress restore</a> in the
    /// <i>Amazon S3 User Guide</i>. </p>
    /// <p>To get the status of object restoration, you can send a <code>HEAD</code> request.
    /// Operations return the <code>x-amz-restore</code> header, which provides information about
    /// the restoration status, in the response. You can use Amazon S3 event notifications to notify you
    /// when a restore is initiated or completed. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring Amazon S3 Event Notifications</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// <p>After restoring an archived object, you can update the restoration period by reissuing
    /// the request with a new period. Amazon S3 updates the restoration period relative to the current
    /// time and charges only for the request-there are no data transfer charges. You cannot
    /// update the restoration period when Amazon S3 is actively processing your current restore request
    /// for the object.</p>
    /// <p>If your bucket has a lifecycle configuration with a rule that includes an expiration
    /// action, the object expiration overrides the life span that you specify in a restore
    /// request. For example, if you restore an object copy for 10 days, but the object is
    /// scheduled to expire in 3 days, Amazon S3 deletes the object in 3 days. For more information
    /// about lifecycle configuration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLifecycleConfiguration.html">PutBucketLifecycleConfiguration</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html">Object Lifecycle Management</a> in
    /// <i>Amazon S3 User Guide</i>.</p>
    /// <p>
    /// <b>Responses</b>
    /// </p>
    /// <p>A successful action returns either the <code>200 OK</code> or <code>202
    /// Accepted</code> status code. </p>
    /// <ul>
    /// <li>
    /// <p>If the object is not previously restored, then Amazon S3 returns <code>202
    /// Accepted</code> in the response. </p>
    /// </li>
    /// <li>
    /// <p>If the object is previously restored, Amazon S3 returns <code>200 OK</code> in the
    /// response. </p>
    /// </li>
    /// </ul>
    /// <p class="title">
    /// <b>Special Errors</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code: RestoreAlreadyInProgress</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: Object restore is already in progress. (This error does not
    /// apply to SELECT type requests.)</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code: 409 Conflict</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix: Client</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code: GlacierExpeditedRetrievalNotAvailable</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: expedited retrievals are currently not available. Try again
    /// later. (Returned if there is insufficient capacity to process the Expedited
    /// request. This error applies only to Expedited retrievals and not to
    /// S3 Standard or Bulk retrievals.)</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code: 503</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix: N/A</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLifecycleConfiguration.html">PutBucketLifecycleConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketNotificationConfiguration.html">GetBucketNotificationConfiguration</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-glacier-select-sql-reference.html">SQL Reference for
    /// Amazon S3 Select and S3 Glacier Select </a> in the
    /// <i>Amazon S3 User Guide</i>
    /// </p>
    /// </li>
    /// </ul>
    async fn restore_object(&self, _input: RestoreObjectInput) -> S3Result<RestoreObjectOutput> {
        Err(s3_error!(NotImplemented, "RestoreObject is not implemented yet"))
    }

    /// <p>Uploads a part in a multipart upload.</p>
    /// <note>
    /// <p>In this operation, you provide part data in your request. However, you have an option
    /// to specify your existing Amazon S3 object as a data source for the part you are uploading. To
    /// upload a part from an existing object, you use the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPartCopy.html">UploadPartCopy</a> operation.
    /// </p>
    /// </note>
    ///
    /// <p>You must initiate a multipart upload (see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>)
    /// before you can upload any part. In response to your initiate request, Amazon S3 returns an
    /// upload ID, a unique identifier, that you must include in your upload part request.</p>
    /// <p>Part numbers can be any number from 1 to 10,000, inclusive. A part number uniquely
    /// identifies a part and also defines its position within the object being created. If you
    /// upload a new part using the same part number that was used with a previous part, the
    /// previously uploaded part is overwritten.</p>
    /// <p>For information about maximum and minimum part sizes and other multipart upload specifications, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/qfacts.html">Multipart upload limits</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>To ensure that data is not corrupted when traversing the network, specify the
    /// <code>Content-MD5</code> header in the upload part request. Amazon S3 checks the part data
    /// against the provided MD5 value. If they do not match, Amazon S3 returns an error. </p>
    ///
    /// <p>If the upload request is signed with Signature Version 4, then Amazon Web Services S3 uses the
    /// <code>x-amz-content-sha256</code> header as a checksum instead of
    /// <code>Content-MD5</code>. For more information see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-auth-using-authorization-header.html">Authenticating Requests: Using the Authorization Header (Amazon Web Services Signature Version
    /// 4)</a>. </p>
    ///
    ///
    ///
    /// <p>
    /// <b>Note:</b> After you initiate multipart upload and upload
    /// one or more parts, you must either complete or abort multipart upload in order to stop
    /// getting charged for storage of the uploaded parts. Only after you either complete or abort
    /// multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts
    /// storage.</p>
    ///
    /// <p>For more information on multipart uploads, go to <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html">Multipart Upload Overview</a> in the
    /// <i>Amazon S3 User Guide </i>.</p>
    /// <p>For information on the permissions required to use the multipart upload API, go to
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuAndPermissions.html">Multipart Upload and
    /// Permissions</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///
    /// <p>You can optionally request server-side encryption where Amazon S3 encrypts your data as it
    /// writes it to disks in its data centers and decrypts it for you when you access it. You have
    /// the option of providing your own encryption key, or you can use the Amazon Web Services managed encryption
    /// keys. If you choose to provide your own encryption key, the request headers you provide in
    /// the request must match the headers you used in the request to initiate the upload by using
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>. For more information, go to <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingServerSideEncryption.html">Using Server-Side Encryption</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    ///
    /// <p>Server-side encryption is supported by the S3 Multipart Upload actions. Unless you are
    /// using a customer-provided encryption key, you don't need to specify the encryption
    /// parameters in each UploadPart request. Instead, you only need to specify the server-side
    /// encryption parameters in the initial Initiate Multipart request. For more information, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>.</p>
    ///
    /// <p>If you requested server-side encryption using a customer-provided encryption key in your
    /// initiate multipart upload request, you must provide identical encryption information in
    /// each part upload using the following headers.</p>
    ///
    ///
    /// <ul>
    /// <li>
    /// <p>x-amz-server-side-encryption-customer-algorithm</p>
    /// </li>
    /// <li>
    /// <p>x-amz-server-side-encryption-customer-key</p>
    /// </li>
    /// <li>
    /// <p>x-amz-server-side-encryption-customer-key-MD5</p>
    /// </li>
    /// </ul>
    ///
    /// <p class="title">
    /// <b>Special Errors</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code: NoSuchUpload</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: The specified multipart upload does not exist. The upload
    /// ID might be invalid, or the multipart upload might have been aborted or
    /// completed.</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i> HTTP Status Code: 404 Not Found </i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>SOAP Fault Code Prefix: Client</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    ///
    ///
    ///
    ///
    ///
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html">CompleteMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html">AbortMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListMultipartUploads.html">ListMultipartUploads</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn upload_part(&self, _input: UploadPartInput) -> S3Result<UploadPartOutput> {
        Err(s3_error!(NotImplemented, "UploadPart is not implemented yet"))
    }

    /// <p>Uploads a part by copying data from an existing object as data source. You specify the
    /// data source by adding the request header <code>x-amz-copy-source</code> in your request and
    /// a byte range by adding the request header <code>x-amz-copy-source-range</code> in your
    /// request. </p>
    /// <p>For information about maximum and minimum part sizes and other multipart upload specifications, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/qfacts.html">Multipart upload limits</a> in the <i>Amazon S3 User Guide</i>. </p>
    /// <note>
    /// <p>Instead of using an existing object as part data, you might use the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a>
    /// action and provide data in your request.</p>
    /// </note>
    ///
    /// <p>You must initiate a multipart upload before you can upload any part. In response to your
    /// initiate request. Amazon S3 returns a unique identifier, the upload ID, that you must include in
    /// your upload part request.</p>
    /// <p>For more information about using the <code>UploadPartCopy</code> operation, see the
    /// following:</p>
    ///
    /// <ul>
    /// <li>
    /// <p>For conceptual information about multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/uploadobjusingmpu.html">Uploading Objects Using Multipart
    /// Upload</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </li>
    /// <li>
    /// <p>For information about permissions required to use the multipart upload API, see
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuAndPermissions.html">Multipart Upload  and
    /// Permissions</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </li>
    /// <li>
    /// <p>For information about copying objects using a single atomic action vs. a multipart
    /// upload, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectOperations.html">Operations on Objects</a> in
    /// the <i>Amazon S3 User Guide</i>.</p>
    /// </li>
    /// <li>
    /// <p>For information about using server-side encryption with customer-provided
    /// encryption keys with the <code>UploadPartCopy</code> operation, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CopyObject.html">CopyObject</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a>.</p>
    /// </li>
    /// </ul>
    /// <p>Note the following additional considerations about the request headers
    /// <code>x-amz-copy-source-if-match</code>, <code>x-amz-copy-source-if-none-match</code>,
    /// <code>x-amz-copy-source-if-unmodified-since</code>, and
    /// <code>x-amz-copy-source-if-modified-since</code>:</p>
    /// <p> </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <b>Consideration 1</b> - If both of the
    /// <code>x-amz-copy-source-if-match</code> and
    /// <code>x-amz-copy-source-if-unmodified-since</code> headers are present in the
    /// request as follows:</p>
    /// <p>
    /// <code>x-amz-copy-source-if-match</code> condition evaluates to <code>true</code>,
    /// and;</p>
    /// <p>
    /// <code>x-amz-copy-source-if-unmodified-since</code> condition evaluates to
    /// <code>false</code>;</p>
    /// <p>Amazon S3 returns <code>200 OK</code> and copies the data.
    /// </p>
    ///
    /// </li>
    /// <li>
    /// <p>
    /// <b>Consideration 2</b> - If both of the
    /// <code>x-amz-copy-source-if-none-match</code> and
    /// <code>x-amz-copy-source-if-modified-since</code> headers are present in the
    /// request as follows:</p>
    /// <p>
    /// <code>x-amz-copy-source-if-none-match</code> condition evaluates to
    /// <code>false</code>, and;</p>
    /// <p>
    /// <code>x-amz-copy-source-if-modified-since</code> condition evaluates to
    /// <code>true</code>;</p>
    /// <p>Amazon S3 returns <code>412 Precondition Failed</code> response code.
    /// </p>
    /// </li>
    /// </ul>
    /// <p>
    /// <b>Versioning</b>
    /// </p>
    /// <p>If your bucket has versioning enabled, you could have multiple versions of the same
    /// object. By default, <code>x-amz-copy-source</code> identifies the current version of the
    /// object to copy. If the current version is a delete marker and you don't specify a versionId
    /// in the <code>x-amz-copy-source</code>, Amazon S3 returns a 404 error, because the object does
    /// not exist. If you specify versionId in the <code>x-amz-copy-source</code> and the versionId
    /// is a delete marker, Amazon S3 returns an HTTP 400 error, because you are not allowed to specify
    /// a delete marker as a version for the <code>x-amz-copy-source</code>. </p>
    /// <p>You can optionally specify a specific version of the source object to copy by adding the
    /// <code>versionId</code> subresource as shown in the following example:</p>
    /// <p>
    /// <code>x-amz-copy-source: /bucket/object?versionId=version id</code>
    /// </p>
    ///
    /// <p class="title">
    /// <b>Special Errors</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code: NoSuchUpload</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: The specified multipart upload does not exist. The upload
    /// ID might be invalid, or the multipart upload might have been aborted or
    /// completed.</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code: 404 Not Found</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// <li>
    /// <ul>
    /// <li>
    /// <p>
    /// <i>Code: InvalidRequest</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>Cause: The specified copy source is not supported as a byte-range
    /// copy source.</i>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <i>HTTP Status Code: 400 Bad Request</i>
    /// </p>
    /// </li>
    /// </ul>
    /// </li>
    /// </ul>
    ///
    ///
    ///
    ///
    ///
    ///
    /// <p class="title">
    /// <b>Related Resources</b>
    /// </p>
    /// <ul>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html">CompleteMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html">AbortMultipartUpload</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListMultipartUploads.html">ListMultipartUploads</a>
    /// </p>
    /// </li>
    /// </ul>
    async fn upload_part_copy(&self, _input: UploadPartCopyInput) -> S3Result<UploadPartCopyOutput> {
        Err(s3_error!(NotImplemented, "UploadPartCopy is not implemented yet"))
    }

    /// <p>Passes transformed
    /// objects to a <code>GetObject</code> operation when using Object Lambda access points. For information about
    /// Object Lambda access points, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/transforming-objects.html">Transforming objects with
    /// Object Lambda access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>This operation supports metadata that can be returned by <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>, in addition to
    /// <code>RequestRoute</code>, <code>RequestToken</code>, <code>StatusCode</code>,
    /// <code>ErrorCode</code>, and <code>ErrorMessage</code>. The <code>GetObject</code>
    /// response metadata is supported so that the <code>WriteGetObjectResponse</code> caller,
    /// typically an Lambda function, can provide the same metadata when it internally invokes
    /// <code>GetObject</code>. When <code>WriteGetObjectResponse</code> is called by a
    /// customer-owned Lambda function, the metadata returned to the end user
    /// <code>GetObject</code> call might differ from what Amazon S3 would normally return.</p>
    /// <p>You can include any number of metadata headers. When including a metadata header, it should be
    /// prefaced with <code>x-amz-meta</code>. For example, <code>x-amz-meta-my-custom-header: MyCustomValue</code>.
    /// The primary use case for this is to forward <code>GetObject</code> metadata.</p>
    /// <p>Amazon Web Services provides some prebuilt Lambda functions that you can use with S3 Object Lambda to detect and redact
    /// personally identifiable information (PII) and decompress S3 objects. These Lambda functions
    /// are available in the Amazon Web Services Serverless Application Repository, and can be selected through the Amazon Web Services Management Console when you create your
    /// Object Lambda access point.</p>
    /// <p>Example 1: PII Access Control - This Lambda function uses Amazon Comprehend, a natural language processing (NLP) service using machine learning to find insights and relationships in text. It automatically detects personally identifiable information (PII) such as names, addresses, dates, credit card numbers, and social security numbers from documents in your Amazon S3 bucket. </p>
    /// <p>Example 2: PII Redaction - This Lambda function uses Amazon Comprehend, a natural language processing (NLP) service using machine learning to find insights and relationships in text. It automatically redacts personally identifiable information (PII) such as names, addresses, dates, credit card numbers, and social security numbers from documents in your Amazon S3 bucket. </p>
    /// <p>Example 3: Decompression - The Lambda function S3ObjectLambdaDecompression, is equipped to decompress objects stored in S3 in one of six compressed file formats including bzip2, gzip, snappy, zlib, zstandard and ZIP. </p>
    /// <p>For information on how to view and use these functions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/olap-examples.html">Using Amazon Web Services built Lambda functions</a> in the <i>Amazon S3 User Guide</i>.</p>
    async fn write_get_object_response(&self, _input: WriteGetObjectResponseInput) -> S3Result {
        Err(s3_error!(NotImplemented, "WriteGetObjectResponse is not implemented yet"))
    }
}

pub struct AbortMultipartUpload;

pub struct CompleteMultipartUpload;

pub struct CopyObject;

pub struct CreateBucket;

pub struct CreateMultipartUpload;

pub struct DeleteBucket;

pub struct DeleteBucketAnalyticsConfiguration;

pub struct DeleteBucketCors;

pub struct DeleteBucketEncryption;

pub struct DeleteBucketIntelligentTieringConfiguration;

pub struct DeleteBucketInventoryConfiguration;

pub struct DeleteBucketLifecycle;

pub struct DeleteBucketMetricsConfiguration;

pub struct DeleteBucketOwnershipControls;

pub struct DeleteBucketPolicy;

pub struct DeleteBucketReplication;

pub struct DeleteBucketTagging;

pub struct DeleteBucketWebsite;

pub struct DeleteObject;

pub struct DeleteObjectTagging;

pub struct DeleteObjects;

pub struct DeletePublicAccessBlock;

pub struct GetBucketAccelerateConfiguration;

pub struct GetBucketAcl;

pub struct GetBucketAnalyticsConfiguration;

pub struct GetBucketCors;

pub struct GetBucketEncryption;

pub struct GetBucketIntelligentTieringConfiguration;

pub struct GetBucketInventoryConfiguration;

pub struct GetBucketLifecycleConfiguration;

pub struct GetBucketLocation;

pub struct GetBucketLogging;

pub struct GetBucketMetricsConfiguration;

pub struct GetBucketNotificationConfiguration;

pub struct GetBucketOwnershipControls;

pub struct GetBucketPolicy;

pub struct GetBucketPolicyStatus;

pub struct GetBucketReplication;

pub struct GetBucketRequestPayment;

pub struct GetBucketTagging;

pub struct GetBucketVersioning;

pub struct GetBucketWebsite;

pub struct GetObject;

pub struct GetObjectAcl;

pub struct GetObjectAttributes;

pub struct GetObjectLegalHold;

pub struct GetObjectLockConfiguration;

pub struct GetObjectRetention;

pub struct GetObjectTagging;

pub struct GetObjectTorrent;

pub struct GetPublicAccessBlock;

pub struct HeadBucket;

pub struct HeadObject;

pub struct ListBucketAnalyticsConfigurations;

pub struct ListBucketIntelligentTieringConfigurations;

pub struct ListBucketInventoryConfigurations;

pub struct ListBucketMetricsConfigurations;

pub struct ListBuckets;

pub struct ListMultipartUploads;

pub struct ListObjectVersions;

pub struct ListObjects;

pub struct ListObjectsV2;

pub struct ListParts;

pub struct PutBucketAccelerateConfiguration;

pub struct PutBucketAcl;

pub struct PutBucketAnalyticsConfiguration;

pub struct PutBucketCors;

pub struct PutBucketEncryption;

pub struct PutBucketIntelligentTieringConfiguration;

pub struct PutBucketInventoryConfiguration;

pub struct PutBucketLifecycleConfiguration;

pub struct PutBucketLogging;

pub struct PutBucketMetricsConfiguration;

pub struct PutBucketNotificationConfiguration;

pub struct PutBucketOwnershipControls;

pub struct PutBucketPolicy;

pub struct PutBucketReplication;

pub struct PutBucketRequestPayment;

pub struct PutBucketTagging;

pub struct PutBucketVersioning;

pub struct PutBucketWebsite;

pub struct PutObject;

pub struct PutObjectAcl;

pub struct PutObjectLegalHold;

pub struct PutObjectLockConfiguration;

pub struct PutObjectRetention;

pub struct PutObjectTagging;

pub struct PutPublicAccessBlock;

pub struct RestoreObject;

pub struct UploadPart;

pub struct UploadPartCopy;

pub struct WriteGetObjectResponse;

impl xml::SerializeContent for AbortIncompleteMultipartUpload {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("DaysAfterInitiation", &self.days_after_initiation)?;
        Ok(())
    }
}

impl xml::SerializeContent for AccessControlTranslation {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Owner", &self.owner)?;
        Ok(())
    }
}

impl xml::SerializeContent for AnalyticsAndOperator {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(iter) = &self.tags {
            s.flattened_list("Tag", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for AnalyticsConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        s.content("Id", &self.id)?;
        s.content("StorageClassAnalysis", &self.storage_class_analysis)?;
        Ok(())
    }
}

impl xml::SerializeContent for AnalyticsExportDestination {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("S3BucketDestination", &self.s3_bucket_destination)?;
        Ok(())
    }
}

impl xml::SerializeContent for AnalyticsFilter {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        match self {
            Self::And(x) => s.content("And", x),
            Self::Prefix(x) => s.content("Prefix", x),
            Self::Tag(x) => s.content("Tag", x),
        }
    }
}

impl xml::SerializeContent for AnalyticsS3BucketDestination {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Bucket", &self.bucket)?;
        if let Some(ref val) = self.bucket_account_id {
            s.content("BucketAccountId", val)?;
        }
        s.content("Format", &self.format)?;
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for AnalyticsS3ExportFileFormat {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for Bucket {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.creation_date {
            s.timestamp("CreationDate", val, TimestampFormat::DateTime)?;
        }
        if let Some(ref val) = self.name {
            s.content("Name", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for BucketAccelerateStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for BucketLocationConstraint {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for BucketLogsPermission {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for BucketVersioningStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for CORSRule {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.allowed_headers {
            s.flattened_list("AllowedHeader", iter)?;
        }
        {
            let iter = &self.allowed_methods;
            s.flattened_list("AllowedMethod", iter)?;
        }
        {
            let iter = &self.allowed_origins;
            s.flattened_list("AllowedOrigin", iter)?;
        }
        if let Some(iter) = &self.expose_headers {
            s.flattened_list("ExposeHeader", iter)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        s.content("MaxAgeSeconds", &self.max_age_seconds)?;
        Ok(())
    }
}

impl xml::SerializeContent for Checksum {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ChecksumAlgorithm {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for CommonPrefix {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for CompleteMultipartUploadOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.bucket {
            s.content("Bucket", val)?;
        }
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.location {
            s.content("Location", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for Condition {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.http_error_code_returned_equals {
            s.content("HttpErrorCodeReturnedEquals", val)?;
        }
        if let Some(ref val) = self.key_prefix_equals {
            s.content("KeyPrefixEquals", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for CopyObjectResult {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for CopyPartResult {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for CreateMultipartUploadOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.bucket {
            s.content("Bucket", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.upload_id {
            s.content("UploadId", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for DefaultRetention {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Days", &self.days)?;
        if let Some(ref val) = self.mode {
            s.content("Mode", val)?;
        }
        s.content("Years", &self.years)?;
        Ok(())
    }
}

impl xml::SerializeContent for DeleteMarkerEntry {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("IsLatest", &self.is_latest)?;
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        if let Some(ref val) = self.version_id {
            s.content("VersionId", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for DeleteMarkerReplication {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.status {
            s.content("Status", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for DeleteMarkerReplicationStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for DeleteObjectsOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.deleted {
            s.flattened_list("Deleted", iter)?;
        }
        if let Some(iter) = &self.errors {
            s.flattened_list("Error", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for DeletedObject {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("DeleteMarker", &self.delete_marker)?;
        if let Some(ref val) = self.delete_marker_version_id {
            s.content("DeleteMarkerVersionId", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.version_id {
            s.content("VersionId", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for Destination {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.access_control_translation {
            s.content("AccessControlTranslation", val)?;
        }
        if let Some(ref val) = self.account {
            s.content("Account", val)?;
        }
        s.content("Bucket", &self.bucket)?;
        if let Some(ref val) = self.encryption_configuration {
            s.content("EncryptionConfiguration", val)?;
        }
        if let Some(ref val) = self.metrics {
            s.content("Metrics", val)?;
        }
        if let Some(ref val) = self.replication_time {
            s.content("ReplicationTime", val)?;
        }
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for EncodingType {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for EncryptionConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.replica_kms_key_id {
            s.content("ReplicaKmsKeyID", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for Error {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.code {
            s.content("Code", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.message {
            s.content("Message", val)?;
        }
        if let Some(ref val) = self.version_id {
            s.content("VersionId", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ErrorDocument {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Key", &self.key)?;
        Ok(())
    }
}

impl xml::SerializeContent for EventBridgeConfiguration {
    fn serialize_content<W: Write>(&self, _: &mut xml::Serializer<W>) -> xml::SerResult {
        Ok(())
    }
}

impl xml::SerializeContent for ExistingObjectReplication {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Status", &self.status)?;
        Ok(())
    }
}

impl xml::SerializeContent for ExistingObjectReplicationStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for ExpirationStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for FilterRule {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.name {
            s.content("Name", val)?;
        }
        if let Some(ref val) = self.value {
            s.content("Value", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for FilterRuleName {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for GetBucketAccelerateConfigurationOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.status {
            s.content("Status", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetBucketAclOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.grants {
            s.list("AccessControlList", "Grant", iter)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetBucketCorsOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.cors_rules {
            s.flattened_list("CORSRule", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetBucketLifecycleConfigurationOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.rules {
            s.flattened_list("Rule", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetBucketLocationOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.location_constraint {
            s.content("LocationConstraint", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetBucketLoggingOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.logging_enabled {
            s.content("LoggingEnabled", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetBucketRequestPaymentOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.payer {
            s.content("Payer", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetBucketTaggingOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        {
            let iter = &self.tag_set;
            s.list("TagSet", "Tag", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetBucketVersioningOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.mfa_delete {
            s.content("MfaDelete", val)?;
        }
        if let Some(ref val) = self.status {
            s.content("Status", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetBucketWebsiteOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.error_document {
            s.content("ErrorDocument", val)?;
        }
        if let Some(ref val) = self.index_document {
            s.content("IndexDocument", val)?;
        }
        if let Some(ref val) = self.redirect_all_requests_to {
            s.content("RedirectAllRequestsTo", val)?;
        }
        if let Some(iter) = &self.routing_rules {
            s.list("RoutingRules", "RoutingRule", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetObjectAclOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.grants {
            s.list("AccessControlList", "Grant", iter)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetObjectAttributesOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.checksum {
            s.content("Checksum", val)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.object_parts {
            s.content("ObjectParts", val)?;
        }
        s.content("ObjectSize", &self.object_size)?;
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for GetObjectAttributesParts {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("IsTruncated", &self.is_truncated)?;
        s.content("MaxParts", &self.max_parts)?;
        if let Some(ref val) = self.next_part_number_marker {
            s.content("NextPartNumberMarker", val)?;
        }
        if let Some(ref val) = self.part_number_marker {
            s.content("PartNumberMarker", val)?;
        }
        if let Some(iter) = &self.parts {
            s.flattened_list("Part", iter)?;
        }
        s.content("PartsCount", &self.total_parts_count)?;
        Ok(())
    }
}

impl xml::SerializeContent for GetObjectTaggingOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        {
            let iter = &self.tag_set;
            s.list("TagSet", "Tag", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for Grant {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.grantee {
            s.content("Grantee", val)?;
        }
        if let Some(ref val) = self.permission {
            s.content("Permission", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for Grantee {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.display_name {
            s.content("DisplayName", val)?;
        }
        if let Some(ref val) = self.email_address {
            s.content("EmailAddress", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        s.content("xsi:type", &self.type_)?;
        if let Some(ref val) = self.uri {
            s.content("URI", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for IndexDocument {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Suffix", &self.suffix)?;
        Ok(())
    }
}

impl xml::SerializeContent for Initiator {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.display_name {
            s.content("DisplayName", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for IntelligentTieringAccessTier {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for IntelligentTieringAndOperator {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(iter) = &self.tags {
            s.flattened_list("Tag", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for IntelligentTieringConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        s.content("Id", &self.id)?;
        s.content("Status", &self.status)?;
        {
            let iter = &self.tierings;
            s.flattened_list("Tiering", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for IntelligentTieringFilter {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.and {
            s.content("And", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(ref val) = self.tag {
            s.content("Tag", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for IntelligentTieringStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for InventoryConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Destination", &self.destination)?;
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        s.content("Id", &self.id)?;
        s.content("IncludedObjectVersions", &self.included_object_versions)?;
        s.content("IsEnabled", &self.is_enabled)?;
        if let Some(iter) = &self.optional_fields {
            s.list("OptionalFields", "Field", iter)?;
        }
        s.content("Schedule", &self.schedule)?;
        Ok(())
    }
}

impl xml::SerializeContent for InventoryDestination {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("S3BucketDestination", &self.s3_bucket_destination)?;
        Ok(())
    }
}

impl xml::SerializeContent for InventoryEncryption {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.ssekms {
            s.content("SSE-KMS", val)?;
        }
        if let Some(ref val) = self.sses3 {
            s.content("SSE-S3", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for InventoryFilter {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Prefix", &self.prefix)?;
        Ok(())
    }
}

impl xml::SerializeContent for InventoryFormat {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for InventoryFrequency {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for InventoryIncludedObjectVersions {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for InventoryOptionalField {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for InventoryS3BucketDestination {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.account_id {
            s.content("AccountId", val)?;
        }
        s.content("Bucket", &self.bucket)?;
        if let Some(ref val) = self.encryption {
            s.content("Encryption", val)?;
        }
        s.content("Format", &self.format)?;
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for InventorySchedule {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Frequency", &self.frequency)?;
        Ok(())
    }
}

impl xml::SerializeContent for LambdaFunctionConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        {
            let iter = &self.events;
            s.flattened_list("Event", iter)?;
        }
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("Id", val)?;
        }
        s.content("CloudFunction", &self.lambda_function_arn)?;
        Ok(())
    }
}

impl xml::SerializeContent for LifecycleExpiration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.date {
            s.timestamp("Date", val, TimestampFormat::DateTime)?;
        }
        s.content("Days", &self.days)?;
        s.content("ExpiredObjectDeleteMarker", &self.expired_object_delete_marker)?;
        Ok(())
    }
}

impl xml::SerializeContent for LifecycleRule {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.abort_incomplete_multipart_upload {
            s.content("AbortIncompleteMultipartUpload", val)?;
        }
        if let Some(ref val) = self.expiration {
            s.content("Expiration", val)?;
        }
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        if let Some(ref val) = self.noncurrent_version_expiration {
            s.content("NoncurrentVersionExpiration", val)?;
        }
        if let Some(iter) = &self.noncurrent_version_transitions {
            s.flattened_list("NoncurrentVersionTransition", iter)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        s.content("Status", &self.status)?;
        if let Some(iter) = &self.transitions {
            s.flattened_list("Transition", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for LifecycleRuleAndOperator {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ObjectSizeGreaterThan", &self.object_size_greater_than)?;
        s.content("ObjectSizeLessThan", &self.object_size_less_than)?;
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(iter) = &self.tags {
            s.flattened_list("Tag", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for LifecycleRuleFilter {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        match self {
            Self::And(x) => s.content("And", x),
            Self::ObjectSizeGreaterThan(x) => s.content("ObjectSizeGreaterThan", x),
            Self::ObjectSizeLessThan(x) => s.content("ObjectSizeLessThan", x),
            Self::Prefix(x) => s.content("Prefix", x),
            Self::Tag(x) => s.content("Tag", x),
        }
    }
}

impl xml::SerializeContent for ListBucketAnalyticsConfigurationsOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.analytics_configuration_list {
            s.flattened_list("AnalyticsConfiguration", iter)?;
        }
        if let Some(ref val) = self.continuation_token {
            s.content("ContinuationToken", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.next_continuation_token {
            s.content("NextContinuationToken", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ListBucketIntelligentTieringConfigurationsOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.continuation_token {
            s.content("ContinuationToken", val)?;
        }
        if let Some(iter) = &self.intelligent_tiering_configuration_list {
            s.flattened_list("IntelligentTieringConfiguration", iter)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.next_continuation_token {
            s.content("NextContinuationToken", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ListBucketInventoryConfigurationsOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.continuation_token {
            s.content("ContinuationToken", val)?;
        }
        if let Some(iter) = &self.inventory_configuration_list {
            s.flattened_list("InventoryConfiguration", iter)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.next_continuation_token {
            s.content("NextContinuationToken", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ListBucketMetricsConfigurationsOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.continuation_token {
            s.content("ContinuationToken", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(iter) = &self.metrics_configuration_list {
            s.flattened_list("MetricsConfiguration", iter)?;
        }
        if let Some(ref val) = self.next_continuation_token {
            s.content("NextContinuationToken", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ListBucketsOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.buckets {
            s.list("Buckets", "Bucket", iter)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ListMultipartUploadsOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.bucket {
            s.content("Bucket", val)?;
        }
        if let Some(iter) = &self.common_prefixes {
            s.flattened_list("CommonPrefixes", iter)?;
        }
        if let Some(ref val) = self.delimiter {
            s.content("Delimiter", val)?;
        }
        if let Some(ref val) = self.encoding_type {
            s.content("EncodingType", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.key_marker {
            s.content("KeyMarker", val)?;
        }
        s.content("MaxUploads", &self.max_uploads)?;
        if let Some(ref val) = self.next_key_marker {
            s.content("NextKeyMarker", val)?;
        }
        if let Some(ref val) = self.next_upload_id_marker {
            s.content("NextUploadIdMarker", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(ref val) = self.upload_id_marker {
            s.content("UploadIdMarker", val)?;
        }
        if let Some(iter) = &self.uploads {
            s.flattened_list("Upload", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ListObjectVersionsOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.common_prefixes {
            s.flattened_list("CommonPrefixes", iter)?;
        }
        if let Some(iter) = &self.delete_markers {
            s.flattened_list("DeleteMarker", iter)?;
        }
        if let Some(ref val) = self.delimiter {
            s.content("Delimiter", val)?;
        }
        if let Some(ref val) = self.encoding_type {
            s.content("EncodingType", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.key_marker {
            s.content("KeyMarker", val)?;
        }
        s.content("MaxKeys", &self.max_keys)?;
        if let Some(ref val) = self.name {
            s.content("Name", val)?;
        }
        if let Some(ref val) = self.next_key_marker {
            s.content("NextKeyMarker", val)?;
        }
        if let Some(ref val) = self.next_version_id_marker {
            s.content("NextVersionIdMarker", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(ref val) = self.version_id_marker {
            s.content("VersionIdMarker", val)?;
        }
        if let Some(iter) = &self.versions {
            s.flattened_list("Version", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ListObjectsOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.common_prefixes {
            s.flattened_list("CommonPrefixes", iter)?;
        }
        if let Some(iter) = &self.contents {
            s.flattened_list("Contents", iter)?;
        }
        if let Some(ref val) = self.delimiter {
            s.content("Delimiter", val)?;
        }
        if let Some(ref val) = self.encoding_type {
            s.content("EncodingType", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.marker {
            s.content("Marker", val)?;
        }
        s.content("MaxKeys", &self.max_keys)?;
        if let Some(ref val) = self.name {
            s.content("Name", val)?;
        }
        if let Some(ref val) = self.next_marker {
            s.content("NextMarker", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ListObjectsV2Output {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.common_prefixes {
            s.flattened_list("CommonPrefixes", iter)?;
        }
        if let Some(iter) = &self.contents {
            s.flattened_list("Contents", iter)?;
        }
        if let Some(ref val) = self.continuation_token {
            s.content("ContinuationToken", val)?;
        }
        if let Some(ref val) = self.delimiter {
            s.content("Delimiter", val)?;
        }
        if let Some(ref val) = self.encoding_type {
            s.content("EncodingType", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        s.content("KeyCount", &self.key_count)?;
        s.content("MaxKeys", &self.max_keys)?;
        if let Some(ref val) = self.name {
            s.content("Name", val)?;
        }
        if let Some(ref val) = self.next_continuation_token {
            s.content("NextContinuationToken", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(ref val) = self.start_after {
            s.content("StartAfter", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ListPartsOutput {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.bucket {
            s.content("Bucket", val)?;
        }
        if let Some(ref val) = self.checksum_algorithm {
            s.content("ChecksumAlgorithm", val)?;
        }
        if let Some(ref val) = self.initiator {
            s.content("Initiator", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        s.content("MaxParts", &self.max_parts)?;
        if let Some(ref val) = self.next_part_number_marker {
            s.content("NextPartNumberMarker", val)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        if let Some(ref val) = self.part_number_marker {
            s.content("PartNumberMarker", val)?;
        }
        if let Some(iter) = &self.parts {
            s.flattened_list("Part", iter)?;
        }
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        if let Some(ref val) = self.upload_id {
            s.content("UploadId", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for LoggingEnabled {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("TargetBucket", &self.target_bucket)?;
        if let Some(iter) = &self.target_grants {
            s.list("TargetGrants", "Grant", iter)?;
        }
        s.content("TargetPrefix", &self.target_prefix)?;
        Ok(())
    }
}

impl xml::SerializeContent for MFADeleteStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for Metrics {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.event_threshold {
            s.content("EventThreshold", val)?;
        }
        s.content("Status", &self.status)?;
        Ok(())
    }
}

impl xml::SerializeContent for MetricsAndOperator {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.access_point_arn {
            s.content("AccessPointArn", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(iter) = &self.tags {
            s.flattened_list("Tag", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for MetricsConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        s.content("Id", &self.id)?;
        Ok(())
    }
}

impl xml::SerializeContent for MetricsFilter {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        match self {
            Self::AccessPointArn(x) => s.content("AccessPointArn", x),
            Self::And(x) => s.content("And", x),
            Self::Prefix(x) => s.content("Prefix", x),
            Self::Tag(x) => s.content("Tag", x),
        }
    }
}

impl xml::SerializeContent for MetricsStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for MultipartUpload {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.checksum_algorithm {
            s.content("ChecksumAlgorithm", val)?;
        }
        if let Some(ref val) = self.initiated {
            s.timestamp("Initiated", val, TimestampFormat::DateTime)?;
        }
        if let Some(ref val) = self.initiator {
            s.content("Initiator", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        if let Some(ref val) = self.upload_id {
            s.content("UploadId", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for NoncurrentVersionExpiration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("NewerNoncurrentVersions", &self.newer_noncurrent_versions)?;
        s.content("NoncurrentDays", &self.noncurrent_days)?;
        Ok(())
    }
}

impl xml::SerializeContent for NoncurrentVersionTransition {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("NewerNoncurrentVersions", &self.newer_noncurrent_versions)?;
        s.content("NoncurrentDays", &self.noncurrent_days)?;
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for NotificationConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.event_bridge_configuration {
            s.content("EventBridgeConfiguration", val)?;
        }
        if let Some(iter) = &self.lambda_function_configurations {
            s.flattened_list("CloudFunctionConfiguration", iter)?;
        }
        if let Some(iter) = &self.queue_configurations {
            s.flattened_list("QueueConfiguration", iter)?;
        }
        if let Some(iter) = &self.topic_configurations {
            s.flattened_list("TopicConfiguration", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for NotificationConfigurationFilter {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.key {
            s.content("S3Key", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for Object {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.checksum_algorithm {
            s.flattened_list("ChecksumAlgorithm", iter)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        s.content("Size", &self.size)?;
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ObjectLockConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.object_lock_enabled {
            s.content("ObjectLockEnabled", val)?;
        }
        if let Some(ref val) = self.rule {
            s.content("Rule", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ObjectLockEnabled {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for ObjectLockLegalHold {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.status {
            s.content("Status", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ObjectLockLegalHoldStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for ObjectLockRetention {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.mode {
            s.content("Mode", val)?;
        }
        if let Some(ref val) = self.retain_until_date {
            s.timestamp("RetainUntilDate", val, TimestampFormat::DateTime)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ObjectLockRetentionMode {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for ObjectLockRule {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.default_retention {
            s.content("DefaultRetention", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ObjectOwnership {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for ObjectPart {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        s.content("PartNumber", &self.part_number)?;
        s.content("Size", &self.size)?;
        Ok(())
    }
}

impl xml::SerializeContent for ObjectStorageClass {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for ObjectVersion {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.checksum_algorithm {
            s.flattened_list("ChecksumAlgorithm", iter)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        s.content("IsLatest", &self.is_latest)?;
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        s.content("Size", &self.size)?;
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        if let Some(ref val) = self.version_id {
            s.content("VersionId", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ObjectVersionStorageClass {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for Owner {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.display_name {
            s.content("DisplayName", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for OwnerOverride {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for OwnershipControls {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        {
            let iter = &self.rules;
            s.flattened_list("Rule", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for OwnershipControlsRule {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ObjectOwnership", &self.object_ownership)?;
        Ok(())
    }
}

impl xml::SerializeContent for Part {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        s.content("PartNumber", &self.part_number)?;
        s.content("Size", &self.size)?;
        Ok(())
    }
}

impl xml::SerializeContent for Payer {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for Permission {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for PolicyStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("IsPublic", &self.is_public)?;
        Ok(())
    }
}

impl xml::SerializeContent for Protocol {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for PublicAccessBlockConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("BlockPublicAcls", &self.block_public_acls)?;
        s.content("BlockPublicPolicy", &self.block_public_policy)?;
        s.content("IgnorePublicAcls", &self.ignore_public_acls)?;
        s.content("RestrictPublicBuckets", &self.restrict_public_buckets)?;
        Ok(())
    }
}

impl xml::SerializeContent for QueueConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        {
            let iter = &self.events;
            s.flattened_list("Event", iter)?;
        }
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("Id", val)?;
        }
        s.content("Queue", &self.queue_arn)?;
        Ok(())
    }
}

impl xml::SerializeContent for Redirect {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.host_name {
            s.content("HostName", val)?;
        }
        if let Some(ref val) = self.http_redirect_code {
            s.content("HttpRedirectCode", val)?;
        }
        if let Some(ref val) = self.protocol {
            s.content("Protocol", val)?;
        }
        if let Some(ref val) = self.replace_key_prefix_with {
            s.content("ReplaceKeyPrefixWith", val)?;
        }
        if let Some(ref val) = self.replace_key_with {
            s.content("ReplaceKeyWith", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for RedirectAllRequestsTo {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("HostName", &self.host_name)?;
        if let Some(ref val) = self.protocol {
            s.content("Protocol", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ReplicaModifications {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Status", &self.status)?;
        Ok(())
    }
}

impl xml::SerializeContent for ReplicaModificationsStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for ReplicationConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Role", &self.role)?;
        {
            let iter = &self.rules;
            s.flattened_list("Rule", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ReplicationRule {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.delete_marker_replication {
            s.content("DeleteMarkerReplication", val)?;
        }
        s.content("Destination", &self.destination)?;
        if let Some(ref val) = self.existing_object_replication {
            s.content("ExistingObjectReplication", val)?;
        }
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        s.content("Priority", &self.priority)?;
        if let Some(ref val) = self.source_selection_criteria {
            s.content("SourceSelectionCriteria", val)?;
        }
        s.content("Status", &self.status)?;
        Ok(())
    }
}

impl xml::SerializeContent for ReplicationRuleAndOperator {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(iter) = &self.tags {
            s.flattened_list("Tag", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ReplicationRuleFilter {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        match self {
            Self::And(x) => s.content("And", x),
            Self::Prefix(x) => s.content("Prefix", x),
            Self::Tag(x) => s.content("Tag", x),
        }
    }
}

impl xml::SerializeContent for ReplicationRuleStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for ReplicationTime {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Status", &self.status)?;
        s.content("Time", &self.time)?;
        Ok(())
    }
}

impl xml::SerializeContent for ReplicationTimeStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for ReplicationTimeValue {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Minutes", &self.minutes)?;
        Ok(())
    }
}

impl xml::SerializeContent for RoutingRule {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.condition {
            s.content("Condition", val)?;
        }
        s.content("Redirect", &self.redirect)?;
        Ok(())
    }
}

impl xml::SerializeContent for S3KeyFilter {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(iter) = &self.filter_rules {
            s.flattened_list("FilterRule", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for SSEKMS {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("KeyId", &self.key_id)?;
        Ok(())
    }
}

impl xml::SerializeContent for SSES3 {
    fn serialize_content<W: Write>(&self, _: &mut xml::Serializer<W>) -> xml::SerResult {
        Ok(())
    }
}

impl xml::SerializeContent for ServerSideEncryption {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for ServerSideEncryptionByDefault {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.kms_master_key_id {
            s.content("KMSMasterKeyID", val)?;
        }
        s.content("SSEAlgorithm", &self.sse_algorithm)?;
        Ok(())
    }
}

impl xml::SerializeContent for ServerSideEncryptionConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        {
            let iter = &self.rules;
            s.flattened_list("Rule", iter)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for ServerSideEncryptionRule {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.apply_server_side_encryption_by_default {
            s.content("ApplyServerSideEncryptionByDefault", val)?;
        }
        s.content("BucketKeyEnabled", &self.bucket_key_enabled)?;
        Ok(())
    }
}

impl xml::SerializeContent for SourceSelectionCriteria {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.replica_modifications {
            s.content("ReplicaModifications", val)?;
        }
        if let Some(ref val) = self.sse_kms_encrypted_objects {
            s.content("SseKmsEncryptedObjects", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for SseKmsEncryptedObjects {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Status", &self.status)?;
        Ok(())
    }
}

impl xml::SerializeContent for SseKmsEncryptedObjectsStatus {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for StorageClass {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for StorageClassAnalysis {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.data_export {
            s.content("DataExport", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for StorageClassAnalysisDataExport {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Destination", &self.destination)?;
        s.content("OutputSchemaVersion", &self.output_schema_version)?;
        Ok(())
    }
}

impl xml::SerializeContent for StorageClassAnalysisSchemaVersion {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for Tag {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Key", &self.key)?;
        s.content("Value", &self.value)?;
        Ok(())
    }
}

impl xml::SerializeContent for TargetGrant {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.grantee {
            s.content("Grantee", val)?;
        }
        if let Some(ref val) = self.permission {
            s.content("Permission", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for Tiering {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("AccessTier", &self.access_tier)?;
        s.content("Days", &self.days)?;
        Ok(())
    }
}

impl xml::SerializeContent for TopicConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        {
            let iter = &self.events;
            s.flattened_list("Event", iter)?;
        }
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("Id", val)?;
        }
        s.content("Topic", &self.topic_arn)?;
        Ok(())
    }
}

impl xml::SerializeContent for Transition {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        if let Some(ref val) = self.date {
            s.timestamp("Date", val, TimestampFormat::DateTime)?;
        }
        s.content("Days", &self.days)?;
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        Ok(())
    }
}

impl xml::SerializeContent for TransitionStorageClass {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::SerializeContent for Type {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        self.as_str().serialize_content(s)
    }
}

impl xml::Serialize for AnalyticsConfiguration {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("AnalyticsConfiguration", self)
    }
}

impl xml::Serialize for CompleteMultipartUploadOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("CompleteMultipartUploadResult", self)
    }
}

impl xml::Serialize for CopyObjectResult {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("CopyObjectResult", self)
    }
}

impl xml::Serialize for CopyPartResult {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("CopyPartResult", self)
    }
}

impl xml::Serialize for CreateMultipartUploadOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("InitiateMultipartUploadResult", self)
    }
}

impl xml::Serialize for DeleteObjectsOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("DeleteResult", self)
    }
}

impl xml::Serialize for GetBucketAccelerateConfigurationOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("AccelerateConfiguration", self)
    }
}

impl xml::Serialize for GetBucketAclOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("AccessControlPolicy", self)
    }
}

impl xml::Serialize for GetBucketCorsOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("CORSConfiguration", self)
    }
}

impl xml::Serialize for GetBucketLifecycleConfigurationOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("LifecycleConfiguration", self)
    }
}

impl xml::Serialize for GetBucketLocationOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("LocationConstraint", self)
    }
}

impl xml::Serialize for GetBucketLoggingOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("BucketLoggingStatus", self)
    }
}

impl xml::Serialize for GetBucketRequestPaymentOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("RequestPaymentConfiguration", self)
    }
}

impl xml::Serialize for GetBucketTaggingOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Tagging", self)
    }
}

impl xml::Serialize for GetBucketVersioningOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("VersioningConfiguration", self)
    }
}

impl xml::Serialize for GetBucketWebsiteOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("WebsiteConfiguration", self)
    }
}

impl xml::Serialize for GetObjectAclOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("AccessControlPolicy", self)
    }
}

impl xml::Serialize for GetObjectAttributesOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("GetObjectAttributesOutput", self)
    }
}

impl xml::Serialize for GetObjectTaggingOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Tagging", self)
    }
}

impl xml::Serialize for IntelligentTieringConfiguration {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("IntelligentTieringConfiguration", self)
    }
}

impl xml::Serialize for InventoryConfiguration {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("InventoryConfiguration", self)
    }
}

impl xml::Serialize for ListBucketAnalyticsConfigurationsOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ListBucketAnalyticsConfigurationResult", self)
    }
}

impl xml::Serialize for ListBucketIntelligentTieringConfigurationsOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ListBucketIntelligentTieringConfigurationsOutput", self)
    }
}

impl xml::Serialize for ListBucketInventoryConfigurationsOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ListInventoryConfigurationsResult", self)
    }
}

impl xml::Serialize for ListBucketMetricsConfigurationsOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ListMetricsConfigurationsResult", self)
    }
}

impl xml::Serialize for ListBucketsOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ListAllMyBucketsResult", self)
    }
}

impl xml::Serialize for ListMultipartUploadsOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ListMultipartUploadsResult", self)
    }
}

impl xml::Serialize for ListObjectVersionsOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ListVersionsResult", self)
    }
}

impl xml::Serialize for ListObjectsOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ListBucketResult", self)
    }
}

impl xml::Serialize for ListObjectsV2Output {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ListBucketResult", self)
    }
}

impl xml::Serialize for ListPartsOutput {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ListPartsResult", self)
    }
}

impl xml::Serialize for MetricsConfiguration {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("MetricsConfiguration", self)
    }
}

impl xml::Serialize for NotificationConfiguration {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("NotificationConfiguration", self)
    }
}

impl xml::Serialize for ObjectLockConfiguration {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ObjectLockConfiguration", self)
    }
}

impl xml::Serialize for ObjectLockLegalHold {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ObjectLockLegalHold", self)
    }
}

impl xml::Serialize for ObjectLockRetention {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ObjectLockRetention", self)
    }
}

impl xml::Serialize for OwnershipControls {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("OwnershipControls", self)
    }
}

impl xml::Serialize for PolicyStatus {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("PolicyStatus", self)
    }
}

impl xml::Serialize for PublicAccessBlockConfiguration {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("PublicAccessBlockConfiguration", self)
    }
}

impl xml::Serialize for ReplicationConfiguration {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ReplicationConfiguration", self)
    }
}

impl xml::Serialize for ServerSideEncryptionConfiguration {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("ServerSideEncryptionConfiguration", self)
    }
}

impl<'xml> xml::DeserializeContent<'xml> for AbortIncompleteMultipartUpload {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut days_after_initiation: Option<DaysAfterInitiation> = None;
        d.for_each_element(|d, x| match x {
            b"DaysAfterInitiation" => {
                if days_after_initiation.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                days_after_initiation = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            days_after_initiation: days_after_initiation.unwrap_or(0),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for AccelerateConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut status: Option<BucketAccelerateStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { status })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for AccessControlPolicy {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut grants: Option<Grants> = None;
        let mut owner: Option<Owner> = None;
        d.for_each_element(|d, x| match x {
            b"AccessControlList" => {
                if grants.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                grants = Some(d.list_content("member")?);
                Ok(())
            }
            b"Owner" => {
                if owner.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                owner = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { grants, owner })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for AccessControlTranslation {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut owner: Option<OwnerOverride> = None;
        d.for_each_element(|d, x| match x {
            b"Owner" => {
                if owner.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                owner = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            owner: owner.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for AnalyticsAndOperator {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut prefix: Option<Prefix> = None;
        let mut tags: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                let ans: Tag = d.content()?;
                tags.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { prefix, tags })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for AnalyticsConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut filter: Option<AnalyticsFilter> = None;
        let mut id: Option<AnalyticsId> = None;
        let mut storage_class_analysis: Option<StorageClassAnalysis> = None;
        d.for_each_element(|d, x| match x {
            b"Filter" => {
                if filter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"StorageClassAnalysis" => {
                if storage_class_analysis.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                storage_class_analysis = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            filter,
            id: id.ok_or(xml::DeError::MissingField)?,
            storage_class_analysis: storage_class_analysis.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for AnalyticsExportDestination {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut s3_bucket_destination: Option<AnalyticsS3BucketDestination> = None;
        d.for_each_element(|d, x| match x {
            b"S3BucketDestination" => {
                if s3_bucket_destination.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                s3_bucket_destination = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            s3_bucket_destination: s3_bucket_destination.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for AnalyticsFilter {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.element(|d, x| match x {
            b"And" => Ok(Self::And(d.content()?)),
            b"Prefix" => Ok(Self::Prefix(d.content()?)),
            b"Tag" => Ok(Self::Tag(d.content()?)),
            _ => Err(xml::DeError::UnexpectedTagName),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for AnalyticsS3BucketDestination {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut bucket: Option<BucketName> = None;
        let mut bucket_account_id: Option<AccountId> = None;
        let mut format: Option<AnalyticsS3ExportFileFormat> = None;
        let mut prefix: Option<Prefix> = None;
        d.for_each_element(|d, x| match x {
            b"Bucket" => {
                if bucket.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                bucket = Some(d.content()?);
                Ok(())
            }
            b"BucketAccountId" => {
                if bucket_account_id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                bucket_account_id = Some(d.content()?);
                Ok(())
            }
            b"Format" => {
                if format.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                format = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            bucket: bucket.ok_or(xml::DeError::MissingField)?,
            bucket_account_id,
            format: format.ok_or(xml::DeError::MissingField)?,
            prefix,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for AnalyticsS3ExportFileFormat {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for BucketAccelerateStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for BucketLifecycleConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut rules: Option<LifecycleRules> = None;
        d.for_each_element(|d, x| match x {
            b"Rule" => {
                let ans: LifecycleRule = d.content()?;
                rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            rules: rules.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for BucketLocationConstraint {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for BucketLoggingStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut logging_enabled: Option<LoggingEnabled> = None;
        d.for_each_element(|d, x| match x {
            b"LoggingEnabled" => {
                if logging_enabled.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                logging_enabled = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { logging_enabled })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for BucketLogsPermission {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for BucketVersioningStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for CORSConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut cors_rules: Option<CORSRules> = None;
        d.for_each_element(|d, x| match x {
            b"CORSRule" => {
                let ans: CORSRule = d.content()?;
                cors_rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            cors_rules: cors_rules.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for CORSRule {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut allowed_headers: Option<AllowedHeaders> = None;
        let mut allowed_methods: Option<AllowedMethods> = None;
        let mut allowed_origins: Option<AllowedOrigins> = None;
        let mut expose_headers: Option<ExposeHeaders> = None;
        let mut id: Option<ID> = None;
        let mut max_age_seconds: Option<MaxAgeSeconds> = None;
        d.for_each_element(|d, x| match x {
            b"AllowedHeader" => {
                let ans: AllowedHeader = d.content()?;
                allowed_headers.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"AllowedMethod" => {
                let ans: AllowedMethod = d.content()?;
                allowed_methods.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"AllowedOrigin" => {
                let ans: AllowedOrigin = d.content()?;
                allowed_origins.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"ExposeHeader" => {
                let ans: ExposeHeader = d.content()?;
                expose_headers.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"ID" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"MaxAgeSeconds" => {
                if max_age_seconds.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                max_age_seconds = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            allowed_headers,
            allowed_methods: allowed_methods.ok_or(xml::DeError::MissingField)?,
            allowed_origins: allowed_origins.ok_or(xml::DeError::MissingField)?,
            expose_headers,
            id,
            max_age_seconds: max_age_seconds.unwrap_or(0),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for CSVInput {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut allow_quoted_record_delimiter: Option<AllowQuotedRecordDelimiter> = None;
        let mut comments: Option<Comments> = None;
        let mut field_delimiter: Option<FieldDelimiter> = None;
        let mut file_header_info: Option<FileHeaderInfo> = None;
        let mut quote_character: Option<QuoteCharacter> = None;
        let mut quote_escape_character: Option<QuoteEscapeCharacter> = None;
        let mut record_delimiter: Option<RecordDelimiter> = None;
        d.for_each_element(|d, x| match x {
            b"AllowQuotedRecordDelimiter" => {
                if allow_quoted_record_delimiter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                allow_quoted_record_delimiter = Some(d.content()?);
                Ok(())
            }
            b"Comments" => {
                if comments.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                comments = Some(d.content()?);
                Ok(())
            }
            b"FieldDelimiter" => {
                if field_delimiter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                field_delimiter = Some(d.content()?);
                Ok(())
            }
            b"FileHeaderInfo" => {
                if file_header_info.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                file_header_info = Some(d.content()?);
                Ok(())
            }
            b"QuoteCharacter" => {
                if quote_character.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                quote_character = Some(d.content()?);
                Ok(())
            }
            b"QuoteEscapeCharacter" => {
                if quote_escape_character.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                quote_escape_character = Some(d.content()?);
                Ok(())
            }
            b"RecordDelimiter" => {
                if record_delimiter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                record_delimiter = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            allow_quoted_record_delimiter: allow_quoted_record_delimiter.unwrap_or(false),
            comments,
            field_delimiter,
            file_header_info,
            quote_character,
            quote_escape_character,
            record_delimiter,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for CSVOutput {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut field_delimiter: Option<FieldDelimiter> = None;
        let mut quote_character: Option<QuoteCharacter> = None;
        let mut quote_escape_character: Option<QuoteEscapeCharacter> = None;
        let mut quote_fields: Option<QuoteFields> = None;
        let mut record_delimiter: Option<RecordDelimiter> = None;
        d.for_each_element(|d, x| match x {
            b"FieldDelimiter" => {
                if field_delimiter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                field_delimiter = Some(d.content()?);
                Ok(())
            }
            b"QuoteCharacter" => {
                if quote_character.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                quote_character = Some(d.content()?);
                Ok(())
            }
            b"QuoteEscapeCharacter" => {
                if quote_escape_character.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                quote_escape_character = Some(d.content()?);
                Ok(())
            }
            b"QuoteFields" => {
                if quote_fields.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                quote_fields = Some(d.content()?);
                Ok(())
            }
            b"RecordDelimiter" => {
                if record_delimiter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                record_delimiter = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            field_delimiter,
            quote_character,
            quote_escape_character,
            quote_fields,
            record_delimiter,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for CompletedMultipartUpload {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut parts: Option<CompletedPartList> = None;
        d.for_each_element(|d, x| match x {
            b"Part" => {
                let ans: CompletedPart = d.content()?;
                parts.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { parts })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for CompletedPart {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut checksum_crc32: Option<ChecksumCRC32> = None;
        let mut checksum_crc32c: Option<ChecksumCRC32C> = None;
        let mut checksum_sha1: Option<ChecksumSHA1> = None;
        let mut checksum_sha256: Option<ChecksumSHA256> = None;
        let mut e_tag: Option<ETag> = None;
        let mut part_number: Option<PartNumber> = None;
        d.for_each_element(|d, x| match x {
            b"ChecksumCRC32" => {
                if checksum_crc32.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                checksum_crc32 = Some(d.content()?);
                Ok(())
            }
            b"ChecksumCRC32C" => {
                if checksum_crc32c.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                checksum_crc32c = Some(d.content()?);
                Ok(())
            }
            b"ChecksumSHA1" => {
                if checksum_sha1.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                checksum_sha1 = Some(d.content()?);
                Ok(())
            }
            b"ChecksumSHA256" => {
                if checksum_sha256.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                checksum_sha256 = Some(d.content()?);
                Ok(())
            }
            b"ETag" => {
                if e_tag.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                e_tag = Some(d.content()?);
                Ok(())
            }
            b"PartNumber" => {
                if part_number.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                part_number = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            e_tag,
            part_number: part_number.unwrap_or(0),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for CompressionType {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Condition {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut http_error_code_returned_equals: Option<HttpErrorCodeReturnedEquals> = None;
        let mut key_prefix_equals: Option<KeyPrefixEquals> = None;
        d.for_each_element(|d, x| match x {
            b"HttpErrorCodeReturnedEquals" => {
                if http_error_code_returned_equals.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                http_error_code_returned_equals = Some(d.content()?);
                Ok(())
            }
            b"KeyPrefixEquals" => {
                if key_prefix_equals.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                key_prefix_equals = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            http_error_code_returned_equals,
            key_prefix_equals,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for CreateBucketConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut location_constraint: Option<BucketLocationConstraint> = None;
        d.for_each_element(|d, x| match x {
            b"LocationConstraint" => {
                if location_constraint.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                location_constraint = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { location_constraint })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for DefaultRetention {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut days: Option<Days> = None;
        let mut mode: Option<ObjectLockRetentionMode> = None;
        let mut years: Option<Years> = None;
        d.for_each_element(|d, x| match x {
            b"Days" => {
                if days.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                days = Some(d.content()?);
                Ok(())
            }
            b"Mode" => {
                if mode.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                mode = Some(d.content()?);
                Ok(())
            }
            b"Years" => {
                if years.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                years = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            days: days.unwrap_or(0),
            mode,
            years: years.unwrap_or(0),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Delete {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut objects: Option<ObjectIdentifierList> = None;
        let mut quiet: Option<Quiet> = None;
        d.for_each_element(|d, x| match x {
            b"Object" => {
                let ans: ObjectIdentifier = d.content()?;
                objects.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"Quiet" => {
                if quiet.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                quiet = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            objects: objects.ok_or(xml::DeError::MissingField)?,
            quiet: quiet.unwrap_or(false),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for DeleteMarkerReplication {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut status: Option<DeleteMarkerReplicationStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { status })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for DeleteMarkerReplicationStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Destination {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut access_control_translation: Option<AccessControlTranslation> = None;
        let mut account: Option<AccountId> = None;
        let mut bucket: Option<BucketName> = None;
        let mut encryption_configuration: Option<EncryptionConfiguration> = None;
        let mut metrics: Option<Metrics> = None;
        let mut replication_time: Option<ReplicationTime> = None;
        let mut storage_class: Option<StorageClass> = None;
        d.for_each_element(|d, x| match x {
            b"AccessControlTranslation" => {
                if access_control_translation.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                access_control_translation = Some(d.content()?);
                Ok(())
            }
            b"Account" => {
                if account.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                account = Some(d.content()?);
                Ok(())
            }
            b"Bucket" => {
                if bucket.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                bucket = Some(d.content()?);
                Ok(())
            }
            b"EncryptionConfiguration" => {
                if encryption_configuration.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                encryption_configuration = Some(d.content()?);
                Ok(())
            }
            b"Metrics" => {
                if metrics.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                metrics = Some(d.content()?);
                Ok(())
            }
            b"ReplicationTime" => {
                if replication_time.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                replication_time = Some(d.content()?);
                Ok(())
            }
            b"StorageClass" => {
                if storage_class.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                storage_class = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            access_control_translation,
            account,
            bucket: bucket.ok_or(xml::DeError::MissingField)?,
            encryption_configuration,
            metrics,
            replication_time,
            storage_class,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Encryption {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut encryption_type: Option<ServerSideEncryption> = None;
        let mut kms_context: Option<KMSContext> = None;
        let mut kms_key_id: Option<SSEKMSKeyId> = None;
        d.for_each_element(|d, x| match x {
            b"EncryptionType" => {
                if encryption_type.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                encryption_type = Some(d.content()?);
                Ok(())
            }
            b"KMSContext" => {
                if kms_context.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                kms_context = Some(d.content()?);
                Ok(())
            }
            b"KMSKeyId" => {
                if kms_key_id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                kms_key_id = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            encryption_type: encryption_type.ok_or(xml::DeError::MissingField)?,
            kms_context,
            kms_key_id,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for EncryptionConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut replica_kms_key_id: Option<ReplicaKmsKeyID> = None;
        d.for_each_element(|d, x| match x {
            b"ReplicaKmsKeyID" => {
                if replica_kms_key_id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                replica_kms_key_id = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { replica_kms_key_id })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ErrorDocument {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut key: Option<ObjectKey> = None;
        d.for_each_element(|d, x| match x {
            b"Key" => {
                if key.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                key = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            key: key.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for EventBridgeConfiguration {
    fn deserialize_content(_: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        Ok(Self {})
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ExistingObjectReplication {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut status: Option<ExistingObjectReplicationStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            status: status.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ExistingObjectReplicationStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ExpirationStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ExpressionType {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for FileHeaderInfo {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for FilterRule {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut name: Option<FilterRuleName> = None;
        let mut value: Option<FilterRuleValue> = None;
        d.for_each_element(|d, x| match x {
            b"Name" => {
                if name.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                name = Some(d.content()?);
                Ok(())
            }
            b"Value" => {
                if value.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                value = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { name, value })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for FilterRuleName {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for GlacierJobParameters {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut tier: Option<Tier> = None;
        d.for_each_element(|d, x| match x {
            b"Tier" => {
                if tier.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                tier = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            tier: tier.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Grant {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut grantee: Option<Grantee> = None;
        let mut permission: Option<Permission> = None;
        d.for_each_element(|d, x| match x {
            b"Grantee" => {
                if grantee.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                grantee = Some(d.content()?);
                Ok(())
            }
            b"Permission" => {
                if permission.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                permission = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { grantee, permission })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Grantee {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut display_name: Option<DisplayName> = None;
        let mut email_address: Option<EmailAddress> = None;
        let mut id: Option<ID> = None;
        let mut type_: Option<Type> = None;
        let mut uri: Option<URI> = None;
        d.for_each_element(|d, x| match x {
            b"DisplayName" => {
                if display_name.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                display_name = Some(d.content()?);
                Ok(())
            }
            b"EmailAddress" => {
                if email_address.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                email_address = Some(d.content()?);
                Ok(())
            }
            b"ID" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"xsi:type" => {
                if type_.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                type_ = Some(d.content()?);
                Ok(())
            }
            b"URI" => {
                if uri.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                uri = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            display_name,
            email_address,
            id,
            type_: type_.ok_or(xml::DeError::MissingField)?,
            uri,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for IndexDocument {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut suffix: Option<Suffix> = None;
        d.for_each_element(|d, x| match x {
            b"Suffix" => {
                if suffix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                suffix = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            suffix: suffix.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for InputSerialization {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut csv: Option<CSVInput> = None;
        let mut compression_type: Option<CompressionType> = None;
        let mut json: Option<JSONInput> = None;
        let mut parquet: Option<ParquetInput> = None;
        d.for_each_element(|d, x| match x {
            b"CSV" => {
                if csv.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                csv = Some(d.content()?);
                Ok(())
            }
            b"CompressionType" => {
                if compression_type.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                compression_type = Some(d.content()?);
                Ok(())
            }
            b"JSON" => {
                if json.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                json = Some(d.content()?);
                Ok(())
            }
            b"Parquet" => {
                if parquet.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                parquet = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            csv,
            compression_type,
            json,
            parquet,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for IntelligentTieringAccessTier {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for IntelligentTieringAndOperator {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut prefix: Option<Prefix> = None;
        let mut tags: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                let ans: Tag = d.content()?;
                tags.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { prefix, tags })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for IntelligentTieringConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut filter: Option<IntelligentTieringFilter> = None;
        let mut id: Option<IntelligentTieringId> = None;
        let mut status: Option<IntelligentTieringStatus> = None;
        let mut tierings: Option<TieringList> = None;
        d.for_each_element(|d, x| match x {
            b"Filter" => {
                if filter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            b"Tiering" => {
                let ans: Tiering = d.content()?;
                tierings.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            filter,
            id: id.ok_or(xml::DeError::MissingField)?,
            status: status.ok_or(xml::DeError::MissingField)?,
            tierings: tierings.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for IntelligentTieringFilter {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut and: Option<IntelligentTieringAndOperator> = None;
        let mut prefix: Option<Prefix> = None;
        let mut tag: Option<Tag> = None;
        d.for_each_element(|d, x| match x {
            b"And" => {
                if and.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                and = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                if tag.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                tag = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { and, prefix, tag })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for IntelligentTieringStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for InventoryConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut destination: Option<InventoryDestination> = None;
        let mut filter: Option<InventoryFilter> = None;
        let mut id: Option<InventoryId> = None;
        let mut included_object_versions: Option<InventoryIncludedObjectVersions> = None;
        let mut is_enabled: Option<IsEnabled> = None;
        let mut optional_fields: Option<InventoryOptionalFields> = None;
        let mut schedule: Option<InventorySchedule> = None;
        d.for_each_element(|d, x| match x {
            b"Destination" => {
                if destination.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                destination = Some(d.content()?);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"IncludedObjectVersions" => {
                if included_object_versions.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                included_object_versions = Some(d.content()?);
                Ok(())
            }
            b"IsEnabled" => {
                if is_enabled.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                is_enabled = Some(d.content()?);
                Ok(())
            }
            b"OptionalFields" => {
                if optional_fields.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                optional_fields = Some(d.list_content("member")?);
                Ok(())
            }
            b"Schedule" => {
                if schedule.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                schedule = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            destination: destination.ok_or(xml::DeError::MissingField)?,
            filter,
            id: id.ok_or(xml::DeError::MissingField)?,
            included_object_versions: included_object_versions.ok_or(xml::DeError::MissingField)?,
            is_enabled: is_enabled.unwrap_or(false),
            optional_fields,
            schedule: schedule.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for InventoryDestination {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut s3_bucket_destination: Option<InventoryS3BucketDestination> = None;
        d.for_each_element(|d, x| match x {
            b"S3BucketDestination" => {
                if s3_bucket_destination.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                s3_bucket_destination = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            s3_bucket_destination: s3_bucket_destination.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for InventoryEncryption {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut ssekms: Option<SSEKMS> = None;
        let mut sses3: Option<SSES3> = None;
        d.for_each_element(|d, x| match x {
            b"SSE-KMS" => {
                if ssekms.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                ssekms = Some(d.content()?);
                Ok(())
            }
            b"SSE-S3" => {
                if sses3.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                sses3 = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { ssekms, sses3 })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for InventoryFilter {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut prefix: Option<Prefix> = None;
        d.for_each_element(|d, x| match x {
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            prefix: prefix.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for InventoryFormat {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for InventoryFrequency {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for InventoryIncludedObjectVersions {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for InventoryOptionalField {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for InventoryS3BucketDestination {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut account_id: Option<AccountId> = None;
        let mut bucket: Option<BucketName> = None;
        let mut encryption: Option<InventoryEncryption> = None;
        let mut format: Option<InventoryFormat> = None;
        let mut prefix: Option<Prefix> = None;
        d.for_each_element(|d, x| match x {
            b"AccountId" => {
                if account_id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                account_id = Some(d.content()?);
                Ok(())
            }
            b"Bucket" => {
                if bucket.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                bucket = Some(d.content()?);
                Ok(())
            }
            b"Encryption" => {
                if encryption.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                encryption = Some(d.content()?);
                Ok(())
            }
            b"Format" => {
                if format.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                format = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            account_id,
            bucket: bucket.ok_or(xml::DeError::MissingField)?,
            encryption,
            format: format.ok_or(xml::DeError::MissingField)?,
            prefix,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for InventorySchedule {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut frequency: Option<InventoryFrequency> = None;
        d.for_each_element(|d, x| match x {
            b"Frequency" => {
                if frequency.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                frequency = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            frequency: frequency.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for JSONInput {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut type_: Option<JSONType> = None;
        d.for_each_element(|d, x| match x {
            b"Type" => {
                if type_.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                type_ = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { type_ })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for JSONOutput {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut record_delimiter: Option<RecordDelimiter> = None;
        d.for_each_element(|d, x| match x {
            b"RecordDelimiter" => {
                if record_delimiter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                record_delimiter = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { record_delimiter })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for JSONType {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for LambdaFunctionConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut events: Option<EventList> = None;
        let mut filter: Option<NotificationConfigurationFilter> = None;
        let mut id: Option<NotificationId> = None;
        let mut lambda_function_arn: Option<LambdaFunctionArn> = None;
        d.for_each_element(|d, x| match x {
            b"Event" => {
                let ans: Event = d.content()?;
                events.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"CloudFunction" => {
                if lambda_function_arn.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                lambda_function_arn = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            events: events.ok_or(xml::DeError::MissingField)?,
            filter,
            id,
            lambda_function_arn: lambda_function_arn.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for LifecycleExpiration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut date: Option<Date> = None;
        let mut days: Option<Days> = None;
        let mut expired_object_delete_marker: Option<ExpiredObjectDeleteMarker> = None;
        d.for_each_element(|d, x| match x {
            b"Date" => {
                if date.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                date = Some(d.timestamp(TimestampFormat::DateTime)?);
                Ok(())
            }
            b"Days" => {
                if days.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                days = Some(d.content()?);
                Ok(())
            }
            b"ExpiredObjectDeleteMarker" => {
                if expired_object_delete_marker.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                expired_object_delete_marker = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            date,
            days: days.unwrap_or(0),
            expired_object_delete_marker: expired_object_delete_marker.unwrap_or(false),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for LifecycleRule {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload> = None;
        let mut expiration: Option<LifecycleExpiration> = None;
        let mut filter: Option<LifecycleRuleFilter> = None;
        let mut id: Option<ID> = None;
        let mut noncurrent_version_expiration: Option<NoncurrentVersionExpiration> = None;
        let mut noncurrent_version_transitions: Option<NoncurrentVersionTransitionList> = None;
        let mut prefix: Option<Prefix> = None;
        let mut status: Option<ExpirationStatus> = None;
        let mut transitions: Option<TransitionList> = None;
        d.for_each_element(|d, x| match x {
            b"AbortIncompleteMultipartUpload" => {
                if abort_incomplete_multipart_upload.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                abort_incomplete_multipart_upload = Some(d.content()?);
                Ok(())
            }
            b"Expiration" => {
                if expiration.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                expiration = Some(d.content()?);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"ID" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"NoncurrentVersionExpiration" => {
                if noncurrent_version_expiration.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                noncurrent_version_expiration = Some(d.content()?);
                Ok(())
            }
            b"NoncurrentVersionTransition" => {
                let ans: NoncurrentVersionTransition = d.content()?;
                noncurrent_version_transitions.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            b"Transition" => {
                let ans: Transition = d.content()?;
                transitions.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            abort_incomplete_multipart_upload,
            expiration,
            filter,
            id,
            noncurrent_version_expiration,
            noncurrent_version_transitions,
            prefix,
            status: status.ok_or(xml::DeError::MissingField)?,
            transitions,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for LifecycleRuleAndOperator {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut object_size_greater_than: Option<ObjectSizeGreaterThanBytes> = None;
        let mut object_size_less_than: Option<ObjectSizeLessThanBytes> = None;
        let mut prefix: Option<Prefix> = None;
        let mut tags: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"ObjectSizeGreaterThan" => {
                if object_size_greater_than.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                object_size_greater_than = Some(d.content()?);
                Ok(())
            }
            b"ObjectSizeLessThan" => {
                if object_size_less_than.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                object_size_less_than = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                let ans: Tag = d.content()?;
                tags.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            object_size_greater_than: object_size_greater_than.unwrap_or(0),
            object_size_less_than: object_size_less_than.unwrap_or(0),
            prefix,
            tags,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for LifecycleRuleFilter {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.element(|d, x| match x {
            b"And" => Ok(Self::And(d.content()?)),
            b"ObjectSizeGreaterThan" => Ok(Self::ObjectSizeGreaterThan(d.content()?)),
            b"ObjectSizeLessThan" => Ok(Self::ObjectSizeLessThan(d.content()?)),
            b"Prefix" => Ok(Self::Prefix(d.content()?)),
            b"Tag" => Ok(Self::Tag(d.content()?)),
            _ => Err(xml::DeError::UnexpectedTagName),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for LoggingEnabled {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut target_bucket: Option<TargetBucket> = None;
        let mut target_grants: Option<TargetGrants> = None;
        let mut target_prefix: Option<TargetPrefix> = None;
        d.for_each_element(|d, x| match x {
            b"TargetBucket" => {
                if target_bucket.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                target_bucket = Some(d.content()?);
                Ok(())
            }
            b"TargetGrants" => {
                if target_grants.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                target_grants = Some(d.list_content("member")?);
                Ok(())
            }
            b"TargetPrefix" => {
                if target_prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                target_prefix = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            target_bucket: target_bucket.ok_or(xml::DeError::MissingField)?,
            target_grants,
            target_prefix: target_prefix.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for MFADelete {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for MetadataEntry {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut name: Option<MetadataKey> = None;
        let mut value: Option<MetadataValue> = None;
        d.for_each_element(|d, x| match x {
            b"Name" => {
                if name.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                name = Some(d.content()?);
                Ok(())
            }
            b"Value" => {
                if value.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                value = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { name, value })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Metrics {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut event_threshold: Option<ReplicationTimeValue> = None;
        let mut status: Option<MetricsStatus> = None;
        d.for_each_element(|d, x| match x {
            b"EventThreshold" => {
                if event_threshold.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                event_threshold = Some(d.content()?);
                Ok(())
            }
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            event_threshold,
            status: status.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for MetricsAndOperator {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut access_point_arn: Option<AccessPointArn> = None;
        let mut prefix: Option<Prefix> = None;
        let mut tags: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"AccessPointArn" => {
                if access_point_arn.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                access_point_arn = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                let ans: Tag = d.content()?;
                tags.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            access_point_arn,
            prefix,
            tags,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for MetricsConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut filter: Option<MetricsFilter> = None;
        let mut id: Option<MetricsId> = None;
        d.for_each_element(|d, x| match x {
            b"Filter" => {
                if filter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            filter,
            id: id.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for MetricsFilter {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.element(|d, x| match x {
            b"AccessPointArn" => Ok(Self::AccessPointArn(d.content()?)),
            b"And" => Ok(Self::And(d.content()?)),
            b"Prefix" => Ok(Self::Prefix(d.content()?)),
            b"Tag" => Ok(Self::Tag(d.content()?)),
            _ => Err(xml::DeError::UnexpectedTagName),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for MetricsStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for NoncurrentVersionExpiration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut newer_noncurrent_versions: Option<VersionCount> = None;
        let mut noncurrent_days: Option<Days> = None;
        d.for_each_element(|d, x| match x {
            b"NewerNoncurrentVersions" => {
                if newer_noncurrent_versions.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                newer_noncurrent_versions = Some(d.content()?);
                Ok(())
            }
            b"NoncurrentDays" => {
                if noncurrent_days.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                noncurrent_days = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            newer_noncurrent_versions: newer_noncurrent_versions.unwrap_or(0),
            noncurrent_days: noncurrent_days.unwrap_or(0),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for NoncurrentVersionTransition {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut newer_noncurrent_versions: Option<VersionCount> = None;
        let mut noncurrent_days: Option<Days> = None;
        let mut storage_class: Option<TransitionStorageClass> = None;
        d.for_each_element(|d, x| match x {
            b"NewerNoncurrentVersions" => {
                if newer_noncurrent_versions.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                newer_noncurrent_versions = Some(d.content()?);
                Ok(())
            }
            b"NoncurrentDays" => {
                if noncurrent_days.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                noncurrent_days = Some(d.content()?);
                Ok(())
            }
            b"StorageClass" => {
                if storage_class.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                storage_class = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            newer_noncurrent_versions: newer_noncurrent_versions.unwrap_or(0),
            noncurrent_days: noncurrent_days.unwrap_or(0),
            storage_class,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for NotificationConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut event_bridge_configuration: Option<EventBridgeConfiguration> = None;
        let mut lambda_function_configurations: Option<LambdaFunctionConfigurationList> = None;
        let mut queue_configurations: Option<QueueConfigurationList> = None;
        let mut topic_configurations: Option<TopicConfigurationList> = None;
        d.for_each_element(|d, x| match x {
            b"EventBridgeConfiguration" => {
                if event_bridge_configuration.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                event_bridge_configuration = Some(d.content()?);
                Ok(())
            }
            b"CloudFunctionConfiguration" => {
                let ans: LambdaFunctionConfiguration = d.content()?;
                lambda_function_configurations.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"QueueConfiguration" => {
                let ans: QueueConfiguration = d.content()?;
                queue_configurations.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"TopicConfiguration" => {
                let ans: TopicConfiguration = d.content()?;
                topic_configurations.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            event_bridge_configuration,
            lambda_function_configurations,
            queue_configurations,
            topic_configurations,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for NotificationConfigurationFilter {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut key: Option<S3KeyFilter> = None;
        d.for_each_element(|d, x| match x {
            b"S3Key" => {
                if key.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                key = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { key })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ObjectCannedACL {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ObjectIdentifier {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut key: Option<ObjectKey> = None;
        let mut version_id: Option<ObjectVersionId> = None;
        d.for_each_element(|d, x| match x {
            b"Key" => {
                if key.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                key = Some(d.content()?);
                Ok(())
            }
            b"VersionId" => {
                if version_id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                version_id = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            key: key.ok_or(xml::DeError::MissingField)?,
            version_id,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ObjectLockConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut object_lock_enabled: Option<ObjectLockEnabled> = None;
        let mut rule: Option<ObjectLockRule> = None;
        d.for_each_element(|d, x| match x {
            b"ObjectLockEnabled" => {
                if object_lock_enabled.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                object_lock_enabled = Some(d.content()?);
                Ok(())
            }
            b"Rule" => {
                if rule.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                rule = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            object_lock_enabled,
            rule,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ObjectLockEnabled {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ObjectLockLegalHold {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut status: Option<ObjectLockLegalHoldStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { status })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ObjectLockLegalHoldStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ObjectLockRetention {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut mode: Option<ObjectLockRetentionMode> = None;
        let mut retain_until_date: Option<Date> = None;
        d.for_each_element(|d, x| match x {
            b"Mode" => {
                if mode.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                mode = Some(d.content()?);
                Ok(())
            }
            b"RetainUntilDate" => {
                if retain_until_date.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                retain_until_date = Some(d.timestamp(TimestampFormat::DateTime)?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { mode, retain_until_date })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ObjectLockRetentionMode {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ObjectLockRule {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut default_retention: Option<DefaultRetention> = None;
        d.for_each_element(|d, x| match x {
            b"DefaultRetention" => {
                if default_retention.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                default_retention = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { default_retention })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ObjectOwnership {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for OutputLocation {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut s3: Option<S3Location> = None;
        d.for_each_element(|d, x| match x {
            b"S3" => {
                if s3.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                s3 = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { s3 })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for OutputSerialization {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut csv: Option<CSVOutput> = None;
        let mut json: Option<JSONOutput> = None;
        d.for_each_element(|d, x| match x {
            b"CSV" => {
                if csv.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                csv = Some(d.content()?);
                Ok(())
            }
            b"JSON" => {
                if json.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                json = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { csv, json })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Owner {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut display_name: Option<DisplayName> = None;
        let mut id: Option<ID> = None;
        d.for_each_element(|d, x| match x {
            b"DisplayName" => {
                if display_name.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                display_name = Some(d.content()?);
                Ok(())
            }
            b"ID" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { display_name, id })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for OwnerOverride {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for OwnershipControls {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut rules: Option<OwnershipControlsRules> = None;
        d.for_each_element(|d, x| match x {
            b"Rule" => {
                let ans: OwnershipControlsRule = d.content()?;
                rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            rules: rules.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for OwnershipControlsRule {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut object_ownership: Option<ObjectOwnership> = None;
        d.for_each_element(|d, x| match x {
            b"ObjectOwnership" => {
                if object_ownership.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                object_ownership = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            object_ownership: object_ownership.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ParquetInput {
    fn deserialize_content(_: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        Ok(Self {})
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Payer {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Permission {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Protocol {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for PublicAccessBlockConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut block_public_acls: Option<Setting> = None;
        let mut block_public_policy: Option<Setting> = None;
        let mut ignore_public_acls: Option<Setting> = None;
        let mut restrict_public_buckets: Option<Setting> = None;
        d.for_each_element(|d, x| match x {
            b"BlockPublicAcls" => {
                if block_public_acls.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                block_public_acls = Some(d.content()?);
                Ok(())
            }
            b"BlockPublicPolicy" => {
                if block_public_policy.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                block_public_policy = Some(d.content()?);
                Ok(())
            }
            b"IgnorePublicAcls" => {
                if ignore_public_acls.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                ignore_public_acls = Some(d.content()?);
                Ok(())
            }
            b"RestrictPublicBuckets" => {
                if restrict_public_buckets.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                restrict_public_buckets = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            block_public_acls: block_public_acls.unwrap_or(false),
            block_public_policy: block_public_policy.unwrap_or(false),
            ignore_public_acls: ignore_public_acls.unwrap_or(false),
            restrict_public_buckets: restrict_public_buckets.unwrap_or(false),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for QueueConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut events: Option<EventList> = None;
        let mut filter: Option<NotificationConfigurationFilter> = None;
        let mut id: Option<NotificationId> = None;
        let mut queue_arn: Option<QueueArn> = None;
        d.for_each_element(|d, x| match x {
            b"Event" => {
                let ans: Event = d.content()?;
                events.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"Queue" => {
                if queue_arn.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                queue_arn = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            events: events.ok_or(xml::DeError::MissingField)?,
            filter,
            id,
            queue_arn: queue_arn.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for QuoteFields {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Redirect {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut host_name: Option<HostName> = None;
        let mut http_redirect_code: Option<HttpRedirectCode> = None;
        let mut protocol: Option<Protocol> = None;
        let mut replace_key_prefix_with: Option<ReplaceKeyPrefixWith> = None;
        let mut replace_key_with: Option<ReplaceKeyWith> = None;
        d.for_each_element(|d, x| match x {
            b"HostName" => {
                if host_name.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                host_name = Some(d.content()?);
                Ok(())
            }
            b"HttpRedirectCode" => {
                if http_redirect_code.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                http_redirect_code = Some(d.content()?);
                Ok(())
            }
            b"Protocol" => {
                if protocol.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                protocol = Some(d.content()?);
                Ok(())
            }
            b"ReplaceKeyPrefixWith" => {
                if replace_key_prefix_with.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                replace_key_prefix_with = Some(d.content()?);
                Ok(())
            }
            b"ReplaceKeyWith" => {
                if replace_key_with.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                replace_key_with = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            host_name,
            http_redirect_code,
            protocol,
            replace_key_prefix_with,
            replace_key_with,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for RedirectAllRequestsTo {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut host_name: Option<HostName> = None;
        let mut protocol: Option<Protocol> = None;
        d.for_each_element(|d, x| match x {
            b"HostName" => {
                if host_name.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                host_name = Some(d.content()?);
                Ok(())
            }
            b"Protocol" => {
                if protocol.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                protocol = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            host_name: host_name.ok_or(xml::DeError::MissingField)?,
            protocol,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ReplicaModifications {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut status: Option<ReplicaModificationsStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            status: status.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ReplicaModificationsStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ReplicationConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut role: Option<Role> = None;
        let mut rules: Option<ReplicationRules> = None;
        d.for_each_element(|d, x| match x {
            b"Role" => {
                if role.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                role = Some(d.content()?);
                Ok(())
            }
            b"Rule" => {
                let ans: ReplicationRule = d.content()?;
                rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            role: role.ok_or(xml::DeError::MissingField)?,
            rules: rules.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ReplicationRule {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut delete_marker_replication: Option<DeleteMarkerReplication> = None;
        let mut destination: Option<Destination> = None;
        let mut existing_object_replication: Option<ExistingObjectReplication> = None;
        let mut filter: Option<ReplicationRuleFilter> = None;
        let mut id: Option<ID> = None;
        let mut prefix: Option<Prefix> = None;
        let mut priority: Option<Priority> = None;
        let mut source_selection_criteria: Option<SourceSelectionCriteria> = None;
        let mut status: Option<ReplicationRuleStatus> = None;
        d.for_each_element(|d, x| match x {
            b"DeleteMarkerReplication" => {
                if delete_marker_replication.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                delete_marker_replication = Some(d.content()?);
                Ok(())
            }
            b"Destination" => {
                if destination.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                destination = Some(d.content()?);
                Ok(())
            }
            b"ExistingObjectReplication" => {
                if existing_object_replication.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                existing_object_replication = Some(d.content()?);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"ID" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Priority" => {
                if priority.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                priority = Some(d.content()?);
                Ok(())
            }
            b"SourceSelectionCriteria" => {
                if source_selection_criteria.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                source_selection_criteria = Some(d.content()?);
                Ok(())
            }
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            delete_marker_replication,
            destination: destination.ok_or(xml::DeError::MissingField)?,
            existing_object_replication,
            filter,
            id,
            prefix,
            priority: priority.unwrap_or(0),
            source_selection_criteria,
            status: status.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ReplicationRuleAndOperator {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut prefix: Option<Prefix> = None;
        let mut tags: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                let ans: Tag = d.content()?;
                tags.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { prefix, tags })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ReplicationRuleFilter {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.element(|d, x| match x {
            b"And" => Ok(Self::And(d.content()?)),
            b"Prefix" => Ok(Self::Prefix(d.content()?)),
            b"Tag" => Ok(Self::Tag(d.content()?)),
            _ => Err(xml::DeError::UnexpectedTagName),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ReplicationRuleStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ReplicationTime {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut status: Option<ReplicationTimeStatus> = None;
        let mut time: Option<ReplicationTimeValue> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            b"Time" => {
                if time.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                time = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            status: status.ok_or(xml::DeError::MissingField)?,
            time: time.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ReplicationTimeStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ReplicationTimeValue {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut minutes: Option<Minutes> = None;
        d.for_each_element(|d, x| match x {
            b"Minutes" => {
                if minutes.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                minutes = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            minutes: minutes.unwrap_or(0),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for RequestPaymentConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut payer: Option<Payer> = None;
        d.for_each_element(|d, x| match x {
            b"Payer" => {
                if payer.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                payer = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            payer: payer.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for RestoreRequest {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut days: Option<Days> = None;
        let mut description: Option<Description> = None;
        let mut glacier_job_parameters: Option<GlacierJobParameters> = None;
        let mut output_location: Option<OutputLocation> = None;
        let mut select_parameters: Option<SelectParameters> = None;
        let mut tier: Option<Tier> = None;
        let mut type_: Option<RestoreRequestType> = None;
        d.for_each_element(|d, x| match x {
            b"Days" => {
                if days.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                days = Some(d.content()?);
                Ok(())
            }
            b"Description" => {
                if description.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                description = Some(d.content()?);
                Ok(())
            }
            b"GlacierJobParameters" => {
                if glacier_job_parameters.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                glacier_job_parameters = Some(d.content()?);
                Ok(())
            }
            b"OutputLocation" => {
                if output_location.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                output_location = Some(d.content()?);
                Ok(())
            }
            b"SelectParameters" => {
                if select_parameters.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                select_parameters = Some(d.content()?);
                Ok(())
            }
            b"Tier" => {
                if tier.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                tier = Some(d.content()?);
                Ok(())
            }
            b"Type" => {
                if type_.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                type_ = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            days: days.unwrap_or(0),
            description,
            glacier_job_parameters,
            output_location,
            select_parameters,
            tier,
            type_,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for RestoreRequestType {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for RoutingRule {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut condition: Option<Condition> = None;
        let mut redirect: Option<Redirect> = None;
        d.for_each_element(|d, x| match x {
            b"Condition" => {
                if condition.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                condition = Some(d.content()?);
                Ok(())
            }
            b"Redirect" => {
                if redirect.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                redirect = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            condition,
            redirect: redirect.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for S3KeyFilter {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut filter_rules: Option<FilterRuleList> = None;
        d.for_each_element(|d, x| match x {
            b"FilterRule" => {
                let ans: FilterRule = d.content()?;
                filter_rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { filter_rules })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for S3Location {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut access_control_list: Option<Grants> = None;
        let mut bucket_name: Option<BucketName> = None;
        let mut canned_acl: Option<ObjectCannedACL> = None;
        let mut encryption: Option<Encryption> = None;
        let mut prefix: Option<LocationPrefix> = None;
        let mut storage_class: Option<StorageClass> = None;
        let mut tagging: Option<Tagging> = None;
        let mut user_metadata: Option<UserMetadata> = None;
        d.for_each_element(|d, x| match x {
            b"AccessControlList" => {
                if access_control_list.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                access_control_list = Some(d.list_content("member")?);
                Ok(())
            }
            b"BucketName" => {
                if bucket_name.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                bucket_name = Some(d.content()?);
                Ok(())
            }
            b"CannedACL" => {
                if canned_acl.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                canned_acl = Some(d.content()?);
                Ok(())
            }
            b"Encryption" => {
                if encryption.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                encryption = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"StorageClass" => {
                if storage_class.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                storage_class = Some(d.content()?);
                Ok(())
            }
            b"Tagging" => {
                if tagging.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                tagging = Some(d.content()?);
                Ok(())
            }
            b"UserMetadata" => {
                if user_metadata.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                user_metadata = Some(d.list_content("member")?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            access_control_list,
            bucket_name: bucket_name.ok_or(xml::DeError::MissingField)?,
            canned_acl,
            encryption,
            prefix: prefix.ok_or(xml::DeError::MissingField)?,
            storage_class,
            tagging,
            user_metadata,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for SSEKMS {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut key_id: Option<SSEKMSKeyId> = None;
        d.for_each_element(|d, x| match x {
            b"KeyId" => {
                if key_id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                key_id = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            key_id: key_id.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for SSES3 {
    fn deserialize_content(_: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        Ok(Self {})
    }
}

impl<'xml> xml::DeserializeContent<'xml> for SelectParameters {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut expression: Option<Expression> = None;
        let mut expression_type: Option<ExpressionType> = None;
        let mut input_serialization: Option<InputSerialization> = None;
        let mut output_serialization: Option<OutputSerialization> = None;
        d.for_each_element(|d, x| match x {
            b"Expression" => {
                if expression.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                expression = Some(d.content()?);
                Ok(())
            }
            b"ExpressionType" => {
                if expression_type.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                expression_type = Some(d.content()?);
                Ok(())
            }
            b"InputSerialization" => {
                if input_serialization.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                input_serialization = Some(d.content()?);
                Ok(())
            }
            b"OutputSerialization" => {
                if output_serialization.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                output_serialization = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            expression: expression.ok_or(xml::DeError::MissingField)?,
            expression_type: expression_type.ok_or(xml::DeError::MissingField)?,
            input_serialization: input_serialization.ok_or(xml::DeError::MissingField)?,
            output_serialization: output_serialization.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ServerSideEncryption {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ServerSideEncryptionByDefault {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut kms_master_key_id: Option<SSEKMSKeyId> = None;
        let mut sse_algorithm: Option<ServerSideEncryption> = None;
        d.for_each_element(|d, x| match x {
            b"KMSMasterKeyID" => {
                if kms_master_key_id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                kms_master_key_id = Some(d.content()?);
                Ok(())
            }
            b"SSEAlgorithm" => {
                if sse_algorithm.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                sse_algorithm = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            kms_master_key_id,
            sse_algorithm: sse_algorithm.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ServerSideEncryptionConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut rules: Option<ServerSideEncryptionRules> = None;
        d.for_each_element(|d, x| match x {
            b"Rule" => {
                let ans: ServerSideEncryptionRule = d.content()?;
                rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            rules: rules.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for ServerSideEncryptionRule {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault> = None;
        let mut bucket_key_enabled: Option<BucketKeyEnabled> = None;
        d.for_each_element(|d, x| match x {
            b"ApplyServerSideEncryptionByDefault" => {
                if apply_server_side_encryption_by_default.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                apply_server_side_encryption_by_default = Some(d.content()?);
                Ok(())
            }
            b"BucketKeyEnabled" => {
                if bucket_key_enabled.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                bucket_key_enabled = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            apply_server_side_encryption_by_default,
            bucket_key_enabled: bucket_key_enabled.unwrap_or(false),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for SourceSelectionCriteria {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut replica_modifications: Option<ReplicaModifications> = None;
        let mut sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects> = None;
        d.for_each_element(|d, x| match x {
            b"ReplicaModifications" => {
                if replica_modifications.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                replica_modifications = Some(d.content()?);
                Ok(())
            }
            b"SseKmsEncryptedObjects" => {
                if sse_kms_encrypted_objects.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                sse_kms_encrypted_objects = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            replica_modifications,
            sse_kms_encrypted_objects,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for SseKmsEncryptedObjects {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut status: Option<SseKmsEncryptedObjectsStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            status: status.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for SseKmsEncryptedObjectsStatus {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for StorageClass {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for StorageClassAnalysis {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut data_export: Option<StorageClassAnalysisDataExport> = None;
        d.for_each_element(|d, x| match x {
            b"DataExport" => {
                if data_export.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                data_export = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { data_export })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for StorageClassAnalysisDataExport {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut destination: Option<AnalyticsExportDestination> = None;
        let mut output_schema_version: Option<StorageClassAnalysisSchemaVersion> = None;
        d.for_each_element(|d, x| match x {
            b"Destination" => {
                if destination.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                destination = Some(d.content()?);
                Ok(())
            }
            b"OutputSchemaVersion" => {
                if output_schema_version.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                output_schema_version = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            destination: destination.ok_or(xml::DeError::MissingField)?,
            output_schema_version: output_schema_version.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for StorageClassAnalysisSchemaVersion {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Tag {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut key: Option<ObjectKey> = None;
        let mut value: Option<Value> = None;
        d.for_each_element(|d, x| match x {
            b"Key" => {
                if key.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                key = Some(d.content()?);
                Ok(())
            }
            b"Value" => {
                if value.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                value = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            key: key.ok_or(xml::DeError::MissingField)?,
            value: value.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Tagging {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut tag_set: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"TagSet" => {
                if tag_set.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                tag_set = Some(d.list_content("member")?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            tag_set: tag_set.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for TargetGrant {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut grantee: Option<Grantee> = None;
        let mut permission: Option<BucketLogsPermission> = None;
        d.for_each_element(|d, x| match x {
            b"Grantee" => {
                if grantee.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                grantee = Some(d.content()?);
                Ok(())
            }
            b"Permission" => {
                if permission.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                permission = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { grantee, permission })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Tier {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Tiering {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut access_tier: Option<IntelligentTieringAccessTier> = None;
        let mut days: Option<IntelligentTieringDays> = None;
        d.for_each_element(|d, x| match x {
            b"AccessTier" => {
                if access_tier.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                access_tier = Some(d.content()?);
                Ok(())
            }
            b"Days" => {
                if days.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                days = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            access_tier: access_tier.ok_or(xml::DeError::MissingField)?,
            days: days.unwrap_or(0),
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for TopicConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut events: Option<EventList> = None;
        let mut filter: Option<NotificationConfigurationFilter> = None;
        let mut id: Option<NotificationId> = None;
        let mut topic_arn: Option<TopicArn> = None;
        d.for_each_element(|d, x| match x {
            b"Event" => {
                let ans: Event = d.content()?;
                events.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"Topic" => {
                if topic_arn.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                topic_arn = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            events: events.ok_or(xml::DeError::MissingField)?,
            filter,
            id,
            topic_arn: topic_arn.ok_or(xml::DeError::MissingField)?,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Transition {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut date: Option<Date> = None;
        let mut days: Option<Days> = None;
        let mut storage_class: Option<TransitionStorageClass> = None;
        d.for_each_element(|d, x| match x {
            b"Date" => {
                if date.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                date = Some(d.timestamp(TimestampFormat::DateTime)?);
                Ok(())
            }
            b"Days" => {
                if days.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                days = Some(d.content()?);
                Ok(())
            }
            b"StorageClass" => {
                if storage_class.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                storage_class = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            date,
            days: days.unwrap_or(0),
            storage_class,
        })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for TransitionStorageClass {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for Type {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.text(|t| Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))
    }
}

impl<'xml> xml::DeserializeContent<'xml> for VersioningConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut mfa_delete: Option<MFADelete> = None;
        let mut status: Option<BucketVersioningStatus> = None;
        d.for_each_element(|d, x| match x {
            b"MfaDelete" => {
                if mfa_delete.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                mfa_delete = Some(d.content()?);
                Ok(())
            }
            b"Status" => {
                if status.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self { mfa_delete, status })
    }
}

impl<'xml> xml::DeserializeContent<'xml> for WebsiteConfiguration {
    fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        let mut error_document: Option<ErrorDocument> = None;
        let mut index_document: Option<IndexDocument> = None;
        let mut redirect_all_requests_to: Option<RedirectAllRequestsTo> = None;
        let mut routing_rules: Option<RoutingRules> = None;
        d.for_each_element(|d, x| match x {
            b"ErrorDocument" => {
                if error_document.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                error_document = Some(d.content()?);
                Ok(())
            }
            b"IndexDocument" => {
                if index_document.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                index_document = Some(d.content()?);
                Ok(())
            }
            b"RedirectAllRequestsTo" => {
                if redirect_all_requests_to.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                redirect_all_requests_to = Some(d.content()?);
                Ok(())
            }
            b"RoutingRules" => {
                if routing_rules.is_some() {
                    return Err(xml::DeError::DuplicateField);
                }
                routing_rules = Some(d.list_content("member")?);
                Ok(())
            }
            _ => Err(xml::DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            error_document,
            index_document,
            redirect_all_requests_to,
            routing_rules,
        })
    }
}

impl<'xml> xml::Deserialize<'xml> for AccelerateConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("AccelerateConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for AccessControlPolicy {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("AccessControlPolicy", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for AnalyticsConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("AnalyticsConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for BucketLifecycleConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("LifecycleConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for BucketLoggingStatus {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("BucketLoggingStatus", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for CORSConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("CORSConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for CompletedMultipartUpload {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("CompleteMultipartUpload", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for CreateBucketConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("CreateBucketConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for Delete {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("Delete", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for IntelligentTieringConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("IntelligentTieringConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for InventoryConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("InventoryConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for MetricsConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("MetricsConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for NotificationConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("NotificationConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for ObjectLockConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("ObjectLockConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for ObjectLockLegalHold {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("LegalHold", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for ObjectLockRetention {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("Retention", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for OwnershipControls {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("OwnershipControls", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for PublicAccessBlockConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("PublicAccessBlockConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for ReplicationConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("ReplicationConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for RequestPaymentConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("RequestPaymentConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for RestoreRequest {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("RestoreRequest", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for ServerSideEncryptionConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("ServerSideEncryptionConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for Tagging {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("Tagging", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for VersioningConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("VersioningConfiguration", |d| d.content())
    }
}

impl<'xml> xml::Deserialize<'xml> for WebsiteConfiguration {
    fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {
        d.named_element("WebsiteConfiguration", |d| d.content())
    }
}

impl http::TryIntoHeaderValue for ArchiveStatus {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::ArchiveAccess => Ok(http::HeaderValue::from_static("{}")),
            Self::DeepArchiveAccess => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for BucketCannedACL {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::AuthenticatedRead => Ok(http::HeaderValue::from_static("{}")),
            Self::Private => Ok(http::HeaderValue::from_static("{}")),
            Self::PublicRead => Ok(http::HeaderValue::from_static("{}")),
            Self::PublicReadWrite => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for ChecksumAlgorithm {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::Crc32 => Ok(http::HeaderValue::from_static("{}")),
            Self::Crc32c => Ok(http::HeaderValue::from_static("{}")),
            Self::Sha1 => Ok(http::HeaderValue::from_static("{}")),
            Self::Sha256 => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for ChecksumMode {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::Enabled => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for MetadataDirective {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::Copy => Ok(http::HeaderValue::from_static("{}")),
            Self::Replace => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for ObjectAttributes {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::Checksum => Ok(http::HeaderValue::from_static("{}")),
            Self::Etag => Ok(http::HeaderValue::from_static("{}")),
            Self::ObjectParts => Ok(http::HeaderValue::from_static("{}")),
            Self::ObjectSize => Ok(http::HeaderValue::from_static("{}")),
            Self::StorageClass => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for ObjectCannedACL {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::AuthenticatedRead => Ok(http::HeaderValue::from_static("{}")),
            Self::AwsExecRead => Ok(http::HeaderValue::from_static("{}")),
            Self::BucketOwnerFullControl => Ok(http::HeaderValue::from_static("{}")),
            Self::BucketOwnerRead => Ok(http::HeaderValue::from_static("{}")),
            Self::Private => Ok(http::HeaderValue::from_static("{}")),
            Self::PublicRead => Ok(http::HeaderValue::from_static("{}")),
            Self::PublicReadWrite => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for ObjectLockLegalHoldStatus {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::Off => Ok(http::HeaderValue::from_static("{}")),
            Self::On => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for ObjectLockMode {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::Compliance => Ok(http::HeaderValue::from_static("{}")),
            Self::Governance => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for ObjectOwnership {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::BucketOwnerEnforced => Ok(http::HeaderValue::from_static("{}")),
            Self::BucketOwnerPreferred => Ok(http::HeaderValue::from_static("{}")),
            Self::ObjectWriter => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for ReplicationStatus {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::Complete => Ok(http::HeaderValue::from_static("{}")),
            Self::Failed => Ok(http::HeaderValue::from_static("{}")),
            Self::Pending => Ok(http::HeaderValue::from_static("{}")),
            Self::Replica => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for RequestCharged {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::Requester => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for RequestPayer {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::Requester => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for ServerSideEncryption {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::Aes256 => Ok(http::HeaderValue::from_static("{}")),
            Self::AwsKms => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for StorageClass {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::DeepArchive => Ok(http::HeaderValue::from_static("{}")),
            Self::Glacier => Ok(http::HeaderValue::from_static("{}")),
            Self::GlacierIr => Ok(http::HeaderValue::from_static("{}")),
            Self::IntelligentTiering => Ok(http::HeaderValue::from_static("{}")),
            Self::OnezoneIa => Ok(http::HeaderValue::from_static("{}")),
            Self::Outposts => Ok(http::HeaderValue::from_static("{}")),
            Self::ReducedRedundancy => Ok(http::HeaderValue::from_static("{}")),
            Self::Standard => Ok(http::HeaderValue::from_static("{}")),
            Self::StandardIa => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryIntoHeaderValue for TaggingDirective {
    type Error = std::convert::Infallible;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match self {
            Self::Copy => Ok(http::HeaderValue::from_static("{}")),
            Self::Replace => Ok(http::HeaderValue::from_static("{}")),
        }
    }
}

impl http::TryFromHeaderValue for ArchiveStatus {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for BucketCannedACL {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for ChecksumAlgorithm {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for ChecksumMode {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for MetadataDirective {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for ObjectAttributes {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for ObjectCannedACL {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for ObjectLockLegalHoldStatus {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for ObjectLockMode {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for ObjectOwnership {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for ReplicationStatus {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for RequestCharged {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for RequestPayer {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for ServerSideEncryption {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for StorageClass {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl http::TryFromHeaderValue for TaggingDirective {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)
    }
}

impl AbortMultipartUpload {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<AbortMultipartUploadInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let upload_id: MultipartUploadId = http::parse_query(req, "uploadId")?;

        Ok(AbortMultipartUploadInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
            upload_id,
        })
    }

    pub fn serialize_http(x: AbortMultipartUploadOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

impl CompleteMultipartUpload {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<CompleteMultipartUploadInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_crc32: Option<ChecksumCRC32> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32)?;

        let checksum_crc32c: Option<ChecksumCRC32C> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32C)?;

        let checksum_sha1: Option<ChecksumSHA1> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA1)?;

        let checksum_sha256: Option<ChecksumSHA256> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA256)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let multipart_upload: Option<CompletedMultipartUpload> = http::take_opt_xml_body(req)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let upload_id: MultipartUploadId = http::parse_query(req, "uploadId")?;

        Ok(CompleteMultipartUploadInput {
            bucket,
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            expected_bucket_owner,
            key,
            multipart_upload,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            upload_id,
        })
    }

    pub fn serialize_http(x: CompleteMultipartUploadOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED,
            x.bucket_key_enabled,
        )?;
        http::add_opt_header(&mut res, X_AMZ_EXPIRATION, x.expiration)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

impl CopyObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<CopyObjectInput> {
        let (bucket, key) = http::unwrap_object(req);

        let acl: Option<ObjectCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let bucket_key_enabled: BucketKeyEnabled =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED)?.unwrap_or(false);

        let cache_control: Option<CacheControl> = http::parse_opt_header(req, &CACHE_CONTROL)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_ALGORITHM)?;

        let content_disposition: Option<ContentDisposition> = http::parse_opt_header(req, &CONTENT_DISPOSITION)?;

        let content_encoding: Option<ContentEncoding> = http::parse_opt_header(req, &CONTENT_ENCODING)?;

        let content_language: Option<ContentLanguage> = http::parse_opt_header(req, &CONTENT_LANGUAGE)?;

        let content_type: Option<ContentType> = http::parse_opt_header(req, &CONTENT_TYPE)?;

        let copy_source: CopySource = http::parse_header(req, &X_AMZ_COPY_SOURCE)?;

        let copy_source_if_match: Option<CopySourceIfMatch> = http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_IF_MATCH)?;

        let copy_source_if_modified_since: Option<CopySourceIfModifiedSince> =
            http::parse_opt_header_timestamp(req, &X_AMZ_COPY_SOURCE_IF_MODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let copy_source_if_none_match: Option<CopySourceIfNoneMatch> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_IF_NONE_MATCH)?;

        let copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince> =
            http::parse_opt_header_timestamp(req, &X_AMZ_COPY_SOURCE_IF_UNMODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let copy_source_sse_customer_key: Option<CopySourceSSECustomerKey> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let expected_source_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_SOURCE_EXPECTED_BUCKET_OWNER)?;

        let expires: Option<Expires> = http::parse_opt_header_timestamp(req, &EXPIRES, TimestampFormat::HttpDate)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        let metadata: Option<Metadata> = http::parse_opt_metadata(req)?;

        let metadata_directive: Option<MetadataDirective> = http::parse_opt_header(req, &X_AMZ_METADATA_DIRECTIVE)?;

        let object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus> =
            http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_LEGAL_HOLD)?;

        let object_lock_mode: Option<ObjectLockMode> = http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_MODE)?;

        let object_lock_retain_until_date: Option<ObjectLockRetainUntilDate> =
            http::parse_opt_header_timestamp(req, &X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE, TimestampFormat::DateTime)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let ssekms_encryption_context: Option<SSEKMSEncryptionContext> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT)?;

        let ssekms_key_id: Option<SSEKMSKeyId> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID)?;

        let server_side_encryption: Option<ServerSideEncryption> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION)?;

        let storage_class: Option<StorageClass> = http::parse_opt_header(req, &X_AMZ_STORAGE_CLASS)?;

        let tagging: Option<TaggingHeader> = http::parse_opt_header(req, &X_AMZ_TAGGING)?;

        let tagging_directive: Option<TaggingDirective> = http::parse_opt_header(req, &X_AMZ_TAGGING_DIRECTIVE)?;

        let website_redirect_location: Option<WebsiteRedirectLocation> =
            http::parse_opt_header(req, &X_AMZ_WEBSITE_REDIRECT_LOCATION)?;

        Ok(CopyObjectInput {
            acl,
            bucket,
            bucket_key_enabled,
            cache_control,
            checksum_algorithm,
            content_disposition,
            content_encoding,
            content_language,
            content_type,
            copy_source,
            copy_source_if_match,
            copy_source_if_modified_since,
            copy_source_if_none_match,
            copy_source_if_unmodified_since,
            copy_source_sse_customer_algorithm,
            copy_source_sse_customer_key,
            copy_source_sse_customer_key_md5,
            expected_bucket_owner,
            expected_source_bucket_owner,
            expires,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write_acp,
            key,
            metadata,
            metadata_directive,
            object_lock_legal_hold_status,
            object_lock_mode,
            object_lock_retain_until_date,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            ssekms_encryption_context,
            ssekms_key_id,
            server_side_encryption,
            storage_class,
            tagging,
            tagging_directive,
            website_redirect_location,
        })
    }

    pub fn serialize_http(x: CopyObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.copy_object_result {
            http::set_xml_body(&mut res, val)?;
        }
        http::add_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED,
            x.bucket_key_enabled,
        )?;
        http::add_opt_header(&mut res, X_AMZ_COPY_SOURCE_VERSION_ID, x.copy_source_version_id)?;
        http::add_opt_header(&mut res, X_AMZ_EXPIRATION, x.expiration)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM,
            x.sse_customer_algorithm,
        )?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5,
            x.sse_customer_key_md5,
        )?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT, x.ssekms_encryption_context)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

impl CreateBucket {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<CreateBucketInput> {
        let bucket = http::unwrap_bucket(req);

        let acl: Option<BucketCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let create_bucket_configuration: Option<CreateBucketConfiguration> = http::take_opt_xml_body(req)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write: Option<GrantWrite> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        let object_lock_enabled_for_bucket: ObjectLockEnabledForBucket =
            http::parse_opt_header(req, &X_AMZ_BUCKET_OBJECT_LOCK_ENABLED)?.unwrap_or(false);

        let object_ownership: Option<ObjectOwnership> = http::parse_opt_header(req, &X_AMZ_OBJECT_OWNERSHIP)?;

        Ok(CreateBucketInput {
            acl,
            bucket,
            create_bucket_configuration,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write,
            grant_write_acp,
            object_lock_enabled_for_bucket,
            object_ownership,
        })
    }

    pub fn serialize_http(x: CreateBucketOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, LOCATION, x.location)?;
        Ok(res)
    }
}

impl CreateMultipartUpload {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<CreateMultipartUploadInput> {
        let (bucket, key) = http::unwrap_object(req);

        let acl: Option<ObjectCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let bucket_key_enabled: BucketKeyEnabled =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED)?.unwrap_or(false);

        let cache_control: Option<CacheControl> = http::parse_opt_header(req, &CACHE_CONTROL)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_ALGORITHM)?;

        let content_disposition: Option<ContentDisposition> = http::parse_opt_header(req, &CONTENT_DISPOSITION)?;

        let content_encoding: Option<ContentEncoding> = http::parse_opt_header(req, &CONTENT_ENCODING)?;

        let content_language: Option<ContentLanguage> = http::parse_opt_header(req, &CONTENT_LANGUAGE)?;

        let content_type: Option<ContentType> = http::parse_opt_header(req, &CONTENT_TYPE)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let expires: Option<Expires> = http::parse_opt_header_timestamp(req, &EXPIRES, TimestampFormat::HttpDate)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        let metadata: Option<Metadata> = http::parse_opt_metadata(req)?;

        let object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus> =
            http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_LEGAL_HOLD)?;

        let object_lock_mode: Option<ObjectLockMode> = http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_MODE)?;

        let object_lock_retain_until_date: Option<ObjectLockRetainUntilDate> =
            http::parse_opt_header_timestamp(req, &X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE, TimestampFormat::DateTime)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let ssekms_encryption_context: Option<SSEKMSEncryptionContext> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT)?;

        let ssekms_key_id: Option<SSEKMSKeyId> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID)?;

        let server_side_encryption: Option<ServerSideEncryption> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION)?;

        let storage_class: Option<StorageClass> = http::parse_opt_header(req, &X_AMZ_STORAGE_CLASS)?;

        let tagging: Option<TaggingHeader> = http::parse_opt_header(req, &X_AMZ_TAGGING)?;

        let website_redirect_location: Option<WebsiteRedirectLocation> =
            http::parse_opt_header(req, &X_AMZ_WEBSITE_REDIRECT_LOCATION)?;

        Ok(CreateMultipartUploadInput {
            acl,
            bucket,
            bucket_key_enabled,
            cache_control,
            checksum_algorithm,
            content_disposition,
            content_encoding,
            content_language,
            content_type,
            expected_bucket_owner,
            expires,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write_acp,
            key,
            metadata,
            object_lock_legal_hold_status,
            object_lock_mode,
            object_lock_retain_until_date,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            ssekms_encryption_context,
            ssekms_key_id,
            server_side_encryption,
            storage_class,
            tagging,
            website_redirect_location,
        })
    }

    pub fn serialize_http(x: CreateMultipartUploadOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_opt_header_timestamp(&mut res, X_AMZ_ABORT_DATE, x.abort_date, TimestampFormat::HttpDate)?;
        http::add_opt_header(&mut res, X_AMZ_ABORT_RULE_ID, x.abort_rule_id)?;
        http::add_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED,
            x.bucket_key_enabled,
        )?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_ALGORITHM, x.checksum_algorithm)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM,
            x.sse_customer_algorithm,
        )?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5,
            x.sse_customer_key_md5,
        )?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT, x.ssekms_encryption_context)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        Ok(res)
    }
}

impl DeleteBucket {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketAnalyticsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketAnalyticsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: AnalyticsId = http::parse_query(req, "id")?;

        Ok(DeleteBucketAnalyticsConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketCors {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketCorsInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketCorsInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketEncryption {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketEncryptionInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketEncryptionInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketIntelligentTieringConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketIntelligentTieringConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let id: IntelligentTieringId = http::parse_query(req, "id")?;

        Ok(DeleteBucketIntelligentTieringConfigurationInput { bucket, id })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketInventoryConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketInventoryConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: InventoryId = http::parse_query(req, "id")?;

        Ok(DeleteBucketInventoryConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketLifecycle {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketLifecycleInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketLifecycleInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketMetricsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketMetricsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: MetricsId = http::parse_query(req, "id")?;

        Ok(DeleteBucketMetricsConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketOwnershipControls {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketOwnershipControlsInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketOwnershipControlsInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketPolicy {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketPolicyInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketPolicyInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketReplication {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketReplicationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketReplicationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketTaggingInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketTaggingInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteBucketWebsite {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketWebsiteInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketWebsiteInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl DeleteObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteObjectInput> {
        let (bucket, key) = http::unwrap_object(req);

        let bypass_governance_retention: BypassGovernanceRetention =
            http::parse_opt_header(req, &X_AMZ_BYPASS_GOVERNANCE_RETENTION)?.unwrap_or(false);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let mfa: Option<MFA> = http::parse_opt_header(req, &X_AMZ_MFA)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(DeleteObjectInput {
            bucket,
            bypass_governance_retention,
            expected_bucket_owner,
            key,
            mfa,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: DeleteObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        http::add_header(&mut res, X_AMZ_DELETE_MARKER, x.delete_marker)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

impl DeleteObjectTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteObjectTaggingInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(DeleteObjectTaggingInput {
            bucket,
            expected_bucket_owner,
            key,
            version_id,
        })
    }

    pub fn serialize_http(x: DeleteObjectTaggingOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

impl DeleteObjects {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteObjectsInput> {
        let bucket = http::unwrap_bucket(req);

        let bypass_governance_retention: BypassGovernanceRetention =
            http::parse_opt_header(req, &X_AMZ_BYPASS_GOVERNANCE_RETENTION)?.unwrap_or(false);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let delete: Delete = http::take_xml_body(req)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let mfa: Option<MFA> = http::parse_opt_header(req, &X_AMZ_MFA)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        Ok(DeleteObjectsInput {
            bucket,
            bypass_governance_retention,
            checksum_algorithm,
            delete,
            expected_bucket_owner,
            mfa,
            request_payer,
        })
    }

    pub fn serialize_http(x: DeleteObjectsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

impl DeletePublicAccessBlock {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeletePublicAccessBlockInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeletePublicAccessBlockInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        res
    }
}

impl GetBucketAccelerateConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketAccelerateConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketAccelerateConfigurationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketAccelerateConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl GetBucketAcl {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketAclInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketAclInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketAclOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl GetBucketAnalyticsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketAnalyticsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: AnalyticsId = http::parse_query(req, "id")?;

        Ok(GetBucketAnalyticsConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http(x: GetBucketAnalyticsConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.analytics_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl GetBucketCors {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketCorsInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketCorsInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketCorsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl GetBucketEncryption {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketEncryptionInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketEncryptionInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketEncryptionOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.server_side_encryption_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl GetBucketIntelligentTieringConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketIntelligentTieringConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let id: IntelligentTieringId = http::parse_query(req, "id")?;

        Ok(GetBucketIntelligentTieringConfigurationInput { bucket, id })
    }

    pub fn serialize_http(x: GetBucketIntelligentTieringConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.intelligent_tiering_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl GetBucketInventoryConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketInventoryConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: InventoryId = http::parse_query(req, "id")?;

        Ok(GetBucketInventoryConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http(x: GetBucketInventoryConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.inventory_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl GetBucketLifecycleConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketLifecycleConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketLifecycleConfigurationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketLifecycleConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl GetBucketLocation {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketLocationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketLocationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketLocationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl GetBucketLogging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketLoggingInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketLoggingInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketLoggingOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl GetBucketMetricsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketMetricsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: MetricsId = http::parse_query(req, "id")?;

        Ok(GetBucketMetricsConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http(x: GetBucketMetricsConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.metrics_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl GetBucketNotificationConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketNotificationConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketNotificationConfigurationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: NotificationConfiguration) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl GetBucketOwnershipControls {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketOwnershipControlsInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketOwnershipControlsInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketOwnershipControlsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.ownership_controls {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl GetBucketPolicy {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketPolicyInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketPolicyInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketPolicyOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(val) = x.policy {
            *res.body_mut() = http::Body::from(val);
        }
        Ok(res)
    }
}

impl GetBucketPolicyStatus {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketPolicyStatusInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketPolicyStatusInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketPolicyStatusOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.policy_status {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl GetBucketReplication {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketReplicationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketReplicationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketReplicationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.replication_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl GetBucketRequestPayment {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketRequestPaymentInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketRequestPaymentInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketRequestPaymentOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl GetBucketTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketTaggingInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketTaggingInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketTaggingOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl GetBucketVersioning {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketVersioningInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketVersioningInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketVersioningOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl GetBucketWebsite {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketWebsiteInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketWebsiteInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketWebsiteOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl GetObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_mode: Option<ChecksumMode> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_MODE)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let if_match: Option<IfMatch> = http::parse_opt_header(req, &IF_MATCH)?;

        let if_modified_since: Option<IfModifiedSince> =
            http::parse_opt_header_timestamp(req, &IF_MODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let if_none_match: Option<IfNoneMatch> = http::parse_opt_header(req, &IF_NONE_MATCH)?;

        let if_unmodified_since: Option<IfUnmodifiedSince> =
            http::parse_opt_header_timestamp(req, &IF_UNMODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let part_number: PartNumber = http::parse_opt_query(req, "partNumber")?.unwrap_or(0);

        let range: Option<Range> = http::parse_opt_header(req, &RANGE)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let response_cache_control: Option<ResponseCacheControl> = http::parse_opt_query(req, "response-cache-control")?;

        let response_content_disposition: Option<ResponseContentDisposition> =
            http::parse_opt_query(req, "response-content-disposition")?;

        let response_content_encoding: Option<ResponseContentEncoding> = http::parse_opt_query(req, "response-content-encoding")?;

        let response_content_language: Option<ResponseContentLanguage> = http::parse_opt_query(req, "response-content-language")?;

        let response_content_type: Option<ResponseContentType> = http::parse_opt_query(req, "response-content-type")?;

        let response_expires: Option<ResponseExpires> =
            http::parse_opt_query_timestamp(req, "response-expires", TimestampFormat::HttpDate)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectInput {
            bucket,
            checksum_mode,
            expected_bucket_owner,
            if_match,
            if_modified_since,
            if_none_match,
            if_unmodified_since,
            key,
            part_number,
            range,
            request_payer,
            response_cache_control,
            response_content_disposition,
            response_content_encoding,
            response_content_language,
            response_content_type,
            response_expires,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(val) = x.body {
            http::set_stream_body(&mut res, val);
        }
        http::add_opt_header(&mut res, ACCEPT_RANGES, x.accept_ranges)?;
        http::add_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED,
            x.bucket_key_enabled,
        )?;
        http::add_opt_header(&mut res, CACHE_CONTROL, x.cache_control)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32, x.checksum_crc32)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32C, x.checksum_crc32c)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA1, x.checksum_sha1)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA256, x.checksum_sha256)?;
        http::add_opt_header(&mut res, CONTENT_DISPOSITION, x.content_disposition)?;
        http::add_opt_header(&mut res, CONTENT_ENCODING, x.content_encoding)?;
        http::add_opt_header(&mut res, CONTENT_LANGUAGE, x.content_language)?;
        http::add_header(&mut res, CONTENT_LENGTH, x.content_length)?;
        http::add_opt_header(&mut res, CONTENT_RANGE, x.content_range)?;
        http::add_opt_header(&mut res, CONTENT_TYPE, x.content_type)?;
        http::add_header(&mut res, X_AMZ_DELETE_MARKER, x.delete_marker)?;
        http::add_opt_header(&mut res, ETAG, x.e_tag)?;
        http::add_opt_header(&mut res, X_AMZ_EXPIRATION, x.expiration)?;
        http::add_opt_header_timestamp(&mut res, EXPIRES, x.expires, TimestampFormat::HttpDate)?;
        http::add_opt_header_timestamp(&mut res, LAST_MODIFIED, x.last_modified, TimestampFormat::HttpDate)?;
        http::add_opt_metadata(&mut res, x.metadata)?;
        http::add_header(&mut res, X_AMZ_MISSING_META, x.missing_meta)?;
        http::add_opt_header(&mut res, X_AMZ_OBJECT_LOCK_LEGAL_HOLD, x.object_lock_legal_hold_status)?;
        http::add_opt_header(&mut res, X_AMZ_OBJECT_LOCK_MODE, x.object_lock_mode)?;
        http::add_opt_header_timestamp(
            &mut res,
            X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE,
            x.object_lock_retain_until_date,
            TimestampFormat::DateTime,
        )?;
        http::add_header(&mut res, X_AMZ_MP_PARTS_COUNT, x.parts_count)?;
        http::add_opt_header(&mut res, X_AMZ_REPLICATION_STATUS, x.replication_status)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_RESTORE, x.restore)?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM,
            x.sse_customer_algorithm,
        )?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5,
            x.sse_customer_key_md5,
        )?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        http::add_opt_header(&mut res, X_AMZ_STORAGE_CLASS, x.storage_class)?;
        http::add_header(&mut res, X_AMZ_TAGGING_COUNT, x.tag_count)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        http::add_opt_header(&mut res, X_AMZ_WEBSITE_REDIRECT_LOCATION, x.website_redirect_location)?;
        Ok(res)
    }
}

impl GetObjectAcl {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectAclInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectAclInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectAclOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

impl GetObjectAttributes {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectAttributesInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let max_parts: MaxParts = http::parse_opt_header(req, &X_AMZ_MAX_PARTS)?.unwrap_or(0);

        let object_attributes: ObjectAttributesList = http::parse_list_header(req, &X_AMZ_OBJECT_ATTRIBUTES)?;

        let part_number_marker: Option<PartNumberMarker> = http::parse_opt_header(req, &X_AMZ_PART_NUMBER_MARKER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectAttributesInput {
            bucket,
            expected_bucket_owner,
            key,
            max_parts,
            object_attributes,
            part_number_marker,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectAttributesOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_header(&mut res, X_AMZ_DELETE_MARKER, x.delete_marker)?;
        http::add_opt_header_timestamp(&mut res, LAST_MODIFIED, x.last_modified, TimestampFormat::HttpDate)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

impl GetObjectLegalHold {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectLegalHoldInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectLegalHoldInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectLegalHoldOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.legal_hold {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl GetObjectLockConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectLockConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetObjectLockConfigurationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetObjectLockConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.object_lock_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl GetObjectRetention {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectRetentionInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectRetentionInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectRetentionOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.retention {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl GetObjectTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectTaggingInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectTaggingInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectTaggingOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

impl GetObjectTorrent {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectTorrentInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        Ok(GetObjectTorrentInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
        })
    }

    pub fn serialize_http(x: GetObjectTorrentOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(val) = x.body {
            http::set_stream_body(&mut res, val);
        }
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

impl GetPublicAccessBlock {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetPublicAccessBlockInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetPublicAccessBlockInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetPublicAccessBlockOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.public_access_block_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

impl HeadBucket {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<HeadBucketInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(HeadBucketInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl HeadObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<HeadObjectInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_mode: Option<ChecksumMode> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_MODE)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let if_match: Option<IfMatch> = http::parse_opt_header(req, &IF_MATCH)?;

        let if_modified_since: Option<IfModifiedSince> =
            http::parse_opt_header_timestamp(req, &IF_MODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let if_none_match: Option<IfNoneMatch> = http::parse_opt_header(req, &IF_NONE_MATCH)?;

        let if_unmodified_since: Option<IfUnmodifiedSince> =
            http::parse_opt_header_timestamp(req, &IF_UNMODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let part_number: PartNumber = http::parse_opt_query(req, "partNumber")?.unwrap_or(0);

        let range: Option<Range> = http::parse_opt_header(req, &RANGE)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(HeadObjectInput {
            bucket,
            checksum_mode,
            expected_bucket_owner,
            if_match,
            if_modified_since,
            if_none_match,
            if_unmodified_since,
            key,
            part_number,
            range,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            version_id,
        })
    }

    pub fn serialize_http(x: HeadObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, ACCEPT_RANGES, x.accept_ranges)?;
        http::add_opt_header(&mut res, X_AMZ_ARCHIVE_STATUS, x.archive_status)?;
        http::add_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED,
            x.bucket_key_enabled,
        )?;
        http::add_opt_header(&mut res, CACHE_CONTROL, x.cache_control)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32, x.checksum_crc32)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32C, x.checksum_crc32c)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA1, x.checksum_sha1)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA256, x.checksum_sha256)?;
        http::add_opt_header(&mut res, CONTENT_DISPOSITION, x.content_disposition)?;
        http::add_opt_header(&mut res, CONTENT_ENCODING, x.content_encoding)?;
        http::add_opt_header(&mut res, CONTENT_LANGUAGE, x.content_language)?;
        http::add_header(&mut res, CONTENT_LENGTH, x.content_length)?;
        http::add_opt_header(&mut res, CONTENT_TYPE, x.content_type)?;
        http::add_header(&mut res, X_AMZ_DELETE_MARKER, x.delete_marker)?;
        http::add_opt_header(&mut res, ETAG, x.e_tag)?;
        http::add_opt_header(&mut res, X_AMZ_EXPIRATION, x.expiration)?;
        http::add_opt_header_timestamp(&mut res, EXPIRES, x.expires, TimestampFormat::HttpDate)?;
        http::add_opt_header_timestamp(&mut res, LAST_MODIFIED, x.last_modified, TimestampFormat::HttpDate)?;
        http::add_opt_metadata(&mut res, x.metadata)?;
        http::add_header(&mut res, X_AMZ_MISSING_META, x.missing_meta)?;
        http::add_opt_header(&mut res, X_AMZ_OBJECT_LOCK_LEGAL_HOLD, x.object_lock_legal_hold_status)?;
        http::add_opt_header(&mut res, X_AMZ_OBJECT_LOCK_MODE, x.object_lock_mode)?;
        http::add_opt_header_timestamp(
            &mut res,
            X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE,
            x.object_lock_retain_until_date,
            TimestampFormat::DateTime,
        )?;
        http::add_header(&mut res, X_AMZ_MP_PARTS_COUNT, x.parts_count)?;
        http::add_opt_header(&mut res, X_AMZ_REPLICATION_STATUS, x.replication_status)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_RESTORE, x.restore)?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM,
            x.sse_customer_algorithm,
        )?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5,
            x.sse_customer_key_md5,
        )?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        http::add_opt_header(&mut res, X_AMZ_STORAGE_CLASS, x.storage_class)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        http::add_opt_header(&mut res, X_AMZ_WEBSITE_REDIRECT_LOCATION, x.website_redirect_location)?;
        Ok(res)
    }
}

impl ListBucketAnalyticsConfigurations {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListBucketAnalyticsConfigurationsInput> {
        let bucket = http::unwrap_bucket(req);

        let continuation_token: Option<Token> = http::parse_opt_query(req, "continuation-token")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(ListBucketAnalyticsConfigurationsInput {
            bucket,
            continuation_token,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: ListBucketAnalyticsConfigurationsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl ListBucketIntelligentTieringConfigurations {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListBucketIntelligentTieringConfigurationsInput> {
        let bucket = http::unwrap_bucket(req);

        let continuation_token: Option<Token> = http::parse_opt_query(req, "continuation-token")?;

        Ok(ListBucketIntelligentTieringConfigurationsInput {
            bucket,
            continuation_token,
        })
    }

    pub fn serialize_http(x: ListBucketIntelligentTieringConfigurationsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl ListBucketInventoryConfigurations {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListBucketInventoryConfigurationsInput> {
        let bucket = http::unwrap_bucket(req);

        let continuation_token: Option<Token> = http::parse_opt_query(req, "continuation-token")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(ListBucketInventoryConfigurationsInput {
            bucket,
            continuation_token,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: ListBucketInventoryConfigurationsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl ListBucketMetricsConfigurations {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListBucketMetricsConfigurationsInput> {
        let bucket = http::unwrap_bucket(req);

        let continuation_token: Option<Token> = http::parse_opt_query(req, "continuation-token")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(ListBucketMetricsConfigurationsInput {
            bucket,
            continuation_token,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: ListBucketMetricsConfigurationsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl ListBuckets {
    pub fn serialize_http(x: ListBucketsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl ListMultipartUploads {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListMultipartUploadsInput> {
        let bucket = http::unwrap_bucket(req);

        let delimiter: Option<Delimiter> = http::parse_opt_query(req, "delimiter")?;

        let encoding_type: Option<EncodingType> = http::parse_opt_query(req, "encoding-type")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let key_marker: Option<KeyMarker> = http::parse_opt_query(req, "key-marker")?;

        let max_uploads: MaxUploads = http::parse_opt_query(req, "max-uploads")?.unwrap_or(0);

        let prefix: Option<Prefix> = http::parse_opt_query(req, "prefix")?;

        let upload_id_marker: Option<UploadIdMarker> = http::parse_opt_query(req, "upload-id-marker")?;

        Ok(ListMultipartUploadsInput {
            bucket,
            delimiter,
            encoding_type,
            expected_bucket_owner,
            key_marker,
            max_uploads,
            prefix,
            upload_id_marker,
        })
    }

    pub fn serialize_http(x: ListMultipartUploadsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl ListObjectVersions {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListObjectVersionsInput> {
        let bucket = http::unwrap_bucket(req);

        let delimiter: Option<Delimiter> = http::parse_opt_query(req, "delimiter")?;

        let encoding_type: Option<EncodingType> = http::parse_opt_query(req, "encoding-type")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let key_marker: Option<KeyMarker> = http::parse_opt_query(req, "key-marker")?;

        let max_keys: MaxKeys = http::parse_opt_query(req, "max-keys")?.unwrap_or(0);

        let prefix: Option<Prefix> = http::parse_opt_query(req, "prefix")?;

        let version_id_marker: Option<VersionIdMarker> = http::parse_opt_query(req, "version-id-marker")?;

        Ok(ListObjectVersionsInput {
            bucket,
            delimiter,
            encoding_type,
            expected_bucket_owner,
            key_marker,
            max_keys,
            prefix,
            version_id_marker,
        })
    }

    pub fn serialize_http(x: ListObjectVersionsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl ListObjects {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListObjectsInput> {
        let bucket = http::unwrap_bucket(req);

        let delimiter: Option<Delimiter> = http::parse_opt_query(req, "delimiter")?;

        let encoding_type: Option<EncodingType> = http::parse_opt_query(req, "encoding-type")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let marker: Option<Marker> = http::parse_opt_query(req, "marker")?;

        let max_keys: MaxKeys = http::parse_opt_query(req, "max-keys")?.unwrap_or(0);

        let prefix: Option<Prefix> = http::parse_opt_query(req, "prefix")?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        Ok(ListObjectsInput {
            bucket,
            delimiter,
            encoding_type,
            expected_bucket_owner,
            marker,
            max_keys,
            prefix,
            request_payer,
        })
    }

    pub fn serialize_http(x: ListObjectsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl ListObjectsV2 {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListObjectsV2Input> {
        let bucket = http::unwrap_bucket(req);

        let continuation_token: Option<Token> = http::parse_opt_query(req, "continuation-token")?;

        let delimiter: Option<Delimiter> = http::parse_opt_query(req, "delimiter")?;

        let encoding_type: Option<EncodingType> = http::parse_opt_query(req, "encoding-type")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let fetch_owner: FetchOwner = http::parse_opt_query(req, "fetch-owner")?.unwrap_or(false);

        let max_keys: MaxKeys = http::parse_opt_query(req, "max-keys")?.unwrap_or(0);

        let prefix: Option<Prefix> = http::parse_opt_query(req, "prefix")?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let start_after: Option<StartAfter> = http::parse_opt_query(req, "start-after")?;

        Ok(ListObjectsV2Input {
            bucket,
            continuation_token,
            delimiter,
            encoding_type,
            expected_bucket_owner,
            fetch_owner,
            max_keys,
            prefix,
            request_payer,
            start_after,
        })
    }

    pub fn serialize_http(x: ListObjectsV2Output) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

impl ListParts {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListPartsInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let max_parts: MaxParts = http::parse_opt_query(req, "max-parts")?.unwrap_or(0);

        let part_number_marker: Option<PartNumberMarker> = http::parse_opt_query(req, "part-number-marker")?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let upload_id: MultipartUploadId = http::parse_query(req, "uploadId")?;

        Ok(ListPartsInput {
            bucket,
            expected_bucket_owner,
            key,
            max_parts,
            part_number_marker,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            upload_id,
        })
    }

    pub fn serialize_http(x: ListPartsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_opt_header_timestamp(&mut res, X_AMZ_ABORT_DATE, x.abort_date, TimestampFormat::HttpDate)?;
        http::add_opt_header(&mut res, X_AMZ_ABORT_RULE_ID, x.abort_rule_id)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

impl PutBucketAccelerateConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketAccelerateConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let accelerate_configuration: AccelerateConfiguration = http::take_xml_body(req)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(PutBucketAccelerateConfigurationInput {
            accelerate_configuration,
            bucket,
            checksum_algorithm,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketAcl {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketAclInput> {
        let bucket = http::unwrap_bucket(req);

        let acl: Option<BucketCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let access_control_policy: Option<AccessControlPolicy> = http::take_opt_xml_body(req)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write: Option<GrantWrite> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        Ok(PutBucketAclInput {
            acl,
            access_control_policy,
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write,
            grant_write_acp,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketAnalyticsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketAnalyticsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let analytics_configuration: AnalyticsConfiguration = http::take_xml_body(req)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: AnalyticsId = http::parse_query(req, "id")?;

        Ok(PutBucketAnalyticsConfigurationInput {
            analytics_configuration,
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketCors {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketCorsInput> {
        let bucket = http::unwrap_bucket(req);

        let cors_configuration: CORSConfiguration = http::take_xml_body(req)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(PutBucketCorsInput {
            bucket,
            cors_configuration,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketEncryption {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketEncryptionInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let server_side_encryption_configuration: ServerSideEncryptionConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketEncryptionInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            server_side_encryption_configuration,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketIntelligentTieringConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketIntelligentTieringConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let id: IntelligentTieringId = http::parse_query(req, "id")?;

        let intelligent_tiering_configuration: IntelligentTieringConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketIntelligentTieringConfigurationInput {
            bucket,
            id,
            intelligent_tiering_configuration,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketInventoryConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketInventoryConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: InventoryId = http::parse_query(req, "id")?;

        let inventory_configuration: InventoryConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketInventoryConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
            inventory_configuration,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketLifecycleConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketLifecycleConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let lifecycle_configuration: Option<BucketLifecycleConfiguration> = http::take_opt_xml_body(req)?;

        Ok(PutBucketLifecycleConfigurationInput {
            bucket,
            checksum_algorithm,
            expected_bucket_owner,
            lifecycle_configuration,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketLogging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketLoggingInput> {
        let bucket = http::unwrap_bucket(req);

        let bucket_logging_status: BucketLoggingStatus = http::take_xml_body(req)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(PutBucketLoggingInput {
            bucket,
            bucket_logging_status,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketMetricsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketMetricsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: MetricsId = http::parse_query(req, "id")?;

        let metrics_configuration: MetricsConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketMetricsConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
            metrics_configuration,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketNotificationConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketNotificationConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let notification_configuration: NotificationConfiguration = http::take_xml_body(req)?;

        let skip_destination_validation: SkipValidation =
            http::parse_opt_header(req, &X_AMZ_SKIP_DESTINATION_VALIDATION)?.unwrap_or(false);

        Ok(PutBucketNotificationConfigurationInput {
            bucket,
            expected_bucket_owner,
            notification_configuration,
            skip_destination_validation,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketOwnershipControls {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketOwnershipControlsInput> {
        let bucket = http::unwrap_bucket(req);

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let ownership_controls: OwnershipControls = http::take_xml_body(req)?;

        Ok(PutBucketOwnershipControlsInput {
            bucket,
            content_md5,
            expected_bucket_owner,
            ownership_controls,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketPolicy {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketPolicyInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let confirm_remove_self_bucket_access: ConfirmRemoveSelfBucketAccess =
            http::parse_opt_header(req, &X_AMZ_CONFIRM_REMOVE_SELF_BUCKET_ACCESS)?.unwrap_or(false);

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let policy: Policy = http::take_string_body(req)?;

        Ok(PutBucketPolicyInput {
            bucket,
            checksum_algorithm,
            confirm_remove_self_bucket_access,
            content_md5,
            expected_bucket_owner,
            policy,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketReplication {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketReplicationInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let replication_configuration: ReplicationConfiguration = http::take_xml_body(req)?;

        let token: Option<ObjectLockToken> = http::parse_opt_header(req, &X_AMZ_BUCKET_OBJECT_LOCK_TOKEN)?;

        Ok(PutBucketReplicationInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            replication_configuration,
            token,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketRequestPayment {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketRequestPaymentInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payment_configuration: RequestPaymentConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketRequestPaymentInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            request_payment_configuration,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketTaggingInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let tagging: Tagging = http::take_xml_body(req)?;

        Ok(PutBucketTaggingInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            tagging,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketVersioning {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketVersioningInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let mfa: Option<MFA> = http::parse_opt_header(req, &X_AMZ_MFA)?;

        let versioning_configuration: VersioningConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketVersioningInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            mfa,
            versioning_configuration,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutBucketWebsite {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketWebsiteInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let website_configuration: WebsiteConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketWebsiteInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            website_configuration,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl PutObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectInput> {
        if let Some(m) = req.extensions_mut().remove::<http::Multipart>() {
            return Self::deserialize_http_multipart(req, m);
        }

        let (bucket, key) = http::unwrap_object(req);

        let acl: Option<ObjectCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let body: Option<StreamingBlob> = Some(http::take_stream_body(req));

        let bucket_key_enabled: BucketKeyEnabled =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED)?.unwrap_or(false);

        let cache_control: Option<CacheControl> = http::parse_opt_header(req, &CACHE_CONTROL)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let checksum_crc32: Option<ChecksumCRC32> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32)?;

        let checksum_crc32c: Option<ChecksumCRC32C> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32C)?;

        let checksum_sha1: Option<ChecksumSHA1> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA1)?;

        let checksum_sha256: Option<ChecksumSHA256> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA256)?;

        let content_disposition: Option<ContentDisposition> = http::parse_opt_header(req, &CONTENT_DISPOSITION)?;

        let content_encoding: Option<ContentEncoding> = http::parse_opt_header(req, &CONTENT_ENCODING)?;

        let content_language: Option<ContentLanguage> = http::parse_opt_header(req, &CONTENT_LANGUAGE)?;

        let content_length: ContentLength = http::parse_opt_header(req, &CONTENT_LENGTH)?.unwrap_or(0);

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let content_type: Option<ContentType> = http::parse_opt_header(req, &CONTENT_TYPE)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let expires: Option<Expires> = http::parse_opt_header_timestamp(req, &EXPIRES, TimestampFormat::HttpDate)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        let metadata: Option<Metadata> = http::parse_opt_metadata(req)?;

        let object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus> =
            http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_LEGAL_HOLD)?;

        let object_lock_mode: Option<ObjectLockMode> = http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_MODE)?;

        let object_lock_retain_until_date: Option<ObjectLockRetainUntilDate> =
            http::parse_opt_header_timestamp(req, &X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE, TimestampFormat::DateTime)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let ssekms_encryption_context: Option<SSEKMSEncryptionContext> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT)?;

        let ssekms_key_id: Option<SSEKMSKeyId> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID)?;

        let server_side_encryption: Option<ServerSideEncryption> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION)?;

        let storage_class: Option<StorageClass> = http::parse_opt_header(req, &X_AMZ_STORAGE_CLASS)?;

        let tagging: Option<TaggingHeader> = http::parse_opt_header(req, &X_AMZ_TAGGING)?;

        let website_redirect_location: Option<WebsiteRedirectLocation> =
            http::parse_opt_header(req, &X_AMZ_WEBSITE_REDIRECT_LOCATION)?;

        Ok(PutObjectInput {
            acl,
            body,
            bucket,
            bucket_key_enabled,
            cache_control,
            checksum_algorithm,
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            content_disposition,
            content_encoding,
            content_language,
            content_length,
            content_md5,
            content_type,
            expected_bucket_owner,
            expires,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write_acp,
            key,
            metadata,
            object_lock_legal_hold_status,
            object_lock_mode,
            object_lock_retain_until_date,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            ssekms_encryption_context,
            ssekms_key_id,
            server_side_encryption,
            storage_class,
            tagging,
            website_redirect_location,
        })
    }

    pub fn deserialize_http_multipart(req: &mut http::Request, mut m: http::Multipart) -> S3Result<PutObjectInput> {
        let (bucket, key) = http::unwrap_object(req);

        let body: Option<StreamingBlob> = m.take_file_stream().map(StreamingBlob::wrap);

        let acl: Option<ObjectCannedACL> = http::parse_field_value(&m, "x-amz-acl")?;

        let bucket_key_enabled: BucketKeyEnabled =
            http::parse_field_value(&m, "x-amz-server-side-encryption-bucket-key-enabled")?.unwrap_or(false);

        let cache_control: Option<CacheControl> = http::parse_field_value(&m, "cache-control")?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_field_value(&m, "x-amz-sdk-checksum-algorithm")?;

        let checksum_crc32: Option<ChecksumCRC32> = http::parse_field_value(&m, "x-amz-checksum-crc32")?;

        let checksum_crc32c: Option<ChecksumCRC32C> = http::parse_field_value(&m, "x-amz-checksum-crc32c")?;

        let checksum_sha1: Option<ChecksumSHA1> = http::parse_field_value(&m, "x-amz-checksum-sha1")?;

        let checksum_sha256: Option<ChecksumSHA256> = http::parse_field_value(&m, "x-amz-checksum-sha256")?;

        let content_disposition: Option<ContentDisposition> = http::parse_field_value(&m, "content-disposition")?;

        let content_encoding: Option<ContentEncoding> = http::parse_field_value(&m, "content-encoding")?;

        let content_language: Option<ContentLanguage> = http::parse_field_value(&m, "content-language")?;

        let content_length: ContentLength = http::parse_field_value(&m, "content-length")?.unwrap_or(0);

        let content_md5: Option<ContentMD5> = http::parse_field_value(&m, "content-md5")?;

        let content_type: Option<ContentType> = http::parse_field_value(&m, "content-type")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_field_value(&m, "x-amz-expected-bucket-owner")?;

        let expires: Option<Expires> = http::parse_field_value_timestamp(&m, "expires", TimestampFormat::HttpDate)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_field_value(&m, "x-amz-grant-full-control")?;

        let grant_read: Option<GrantRead> = http::parse_field_value(&m, "x-amz-grant-read")?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_field_value(&m, "x-amz-grant-read-acp")?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_field_value(&m, "x-amz-grant-write-acp")?;

        let metadata: Option<Metadata> = {
            let mut metadata: Metadata = Default::default();
            for (name, value) in m.fields() {
                if let Some(key) = name.strip_prefix("x-amz-meta-") {
                    if key.is_empty() {
                        continue;
                    }
                    metadata.insert(key.to_owned(), value.to_owned());
                }
            }
            if metadata.is_empty() {
                None
            } else {
                Some(metadata)
            }
        };

        let object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus> =
            http::parse_field_value(&m, "x-amz-object-lock-legal-hold")?;

        let object_lock_mode: Option<ObjectLockMode> = http::parse_field_value(&m, "x-amz-object-lock-mode")?;

        let object_lock_retain_until_date: Option<ObjectLockRetainUntilDate> =
            http::parse_field_value_timestamp(&m, "x-amz-object-lock-retain-until-date", TimestampFormat::DateTime)?;

        let request_payer: Option<RequestPayer> = http::parse_field_value(&m, "x-amz-request-payer")?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_field_value(&m, "x-amz-server-side-encryption-customer-algorithm")?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_field_value(&m, "x-amz-server-side-encryption-customer-key")?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_field_value(&m, "x-amz-server-side-encryption-customer-key-md5")?;

        let ssekms_encryption_context: Option<SSEKMSEncryptionContext> =
            http::parse_field_value(&m, "x-amz-server-side-encryption-context")?;

        let ssekms_key_id: Option<SSEKMSKeyId> = http::parse_field_value(&m, "x-amz-server-side-encryption-aws-kms-key-id")?;

        let server_side_encryption: Option<ServerSideEncryption> = http::parse_field_value(&m, "x-amz-server-side-encryption")?;

        let storage_class: Option<StorageClass> = http::parse_field_value(&m, "x-amz-storage-class")?;

        let tagging: Option<TaggingHeader> = http::parse_field_value(&m, "x-amz-tagging")?;

        let website_redirect_location: Option<WebsiteRedirectLocation> =
            http::parse_field_value(&m, "x-amz-website-redirect-location")?;

        Ok(PutObjectInput {
            acl,
            body,
            bucket,
            bucket_key_enabled,
            cache_control,
            checksum_algorithm,
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            content_disposition,
            content_encoding,
            content_language,
            content_length,
            content_md5,
            content_type,
            expected_bucket_owner,
            expires,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write_acp,
            key,
            metadata,
            object_lock_legal_hold_status,
            object_lock_mode,
            object_lock_retain_until_date,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            ssekms_encryption_context,
            ssekms_key_id,
            server_side_encryption,
            storage_class,
            tagging,
            website_redirect_location,
        })
    }

    pub fn serialize_http(x: PutObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED,
            x.bucket_key_enabled,
        )?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32, x.checksum_crc32)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32C, x.checksum_crc32c)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA1, x.checksum_sha1)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA256, x.checksum_sha256)?;
        http::add_opt_header(&mut res, ETAG, x.e_tag)?;
        http::add_opt_header(&mut res, X_AMZ_EXPIRATION, x.expiration)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM,
            x.sse_customer_algorithm,
        )?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5,
            x.sse_customer_key_md5,
        )?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT, x.ssekms_encryption_context)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

impl PutObjectAcl {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectAclInput> {
        let (bucket, key) = http::unwrap_object(req);

        let acl: Option<ObjectCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let access_control_policy: Option<AccessControlPolicy> = http::take_opt_xml_body(req)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write: Option<GrantWrite> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(PutObjectAclInput {
            acl,
            access_control_policy,
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write,
            grant_write_acp,
            key,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: PutObjectAclOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

impl PutObjectLegalHold {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectLegalHoldInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let legal_hold: Option<ObjectLockLegalHold> = http::take_opt_xml_body(req)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(PutObjectLegalHoldInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            key,
            legal_hold,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: PutObjectLegalHoldOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

impl PutObjectLockConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectLockConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let object_lock_configuration: Option<ObjectLockConfiguration> = http::take_opt_xml_body(req)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let token: Option<ObjectLockToken> = http::parse_opt_header(req, &X_AMZ_BUCKET_OBJECT_LOCK_TOKEN)?;

        Ok(PutObjectLockConfigurationInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            object_lock_configuration,
            request_payer,
            token,
        })
    }

    pub fn serialize_http(x: PutObjectLockConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

impl PutObjectRetention {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectRetentionInput> {
        let (bucket, key) = http::unwrap_object(req);

        let bypass_governance_retention: BypassGovernanceRetention =
            http::parse_opt_header(req, &X_AMZ_BYPASS_GOVERNANCE_RETENTION)?.unwrap_or(false);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let retention: Option<ObjectLockRetention> = http::take_opt_xml_body(req)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(PutObjectRetentionInput {
            bucket,
            bypass_governance_retention,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            key,
            request_payer,
            retention,
            version_id,
        })
    }

    pub fn serialize_http(x: PutObjectRetentionOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

impl PutObjectTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectTaggingInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let tagging: Tagging = http::take_xml_body(req)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(PutObjectTaggingInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            key,
            request_payer,
            tagging,
            version_id,
        })
    }

    pub fn serialize_http(x: PutObjectTaggingOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

impl PutPublicAccessBlock {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutPublicAccessBlockInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let public_access_block_configuration: PublicAccessBlockConfiguration = http::take_xml_body(req)?;

        Ok(PutPublicAccessBlockInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            public_access_block_configuration,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

impl RestoreObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<RestoreObjectInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let restore_request: Option<RestoreRequest> = http::take_opt_xml_body(req)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(RestoreObjectInput {
            bucket,
            checksum_algorithm,
            expected_bucket_owner,
            key,
            request_payer,
            restore_request,
            version_id,
        })
    }

    pub fn serialize_http(x: RestoreObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_RESTORE_OUTPUT_PATH, x.restore_output_path)?;
        Ok(res)
    }
}

impl UploadPart {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<UploadPartInput> {
        let (bucket, key) = http::unwrap_object(req);

        let body: Option<StreamingBlob> = Some(http::take_stream_body(req));

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let checksum_crc32: Option<ChecksumCRC32> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32)?;

        let checksum_crc32c: Option<ChecksumCRC32C> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32C)?;

        let checksum_sha1: Option<ChecksumSHA1> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA1)?;

        let checksum_sha256: Option<ChecksumSHA256> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA256)?;

        let content_length: ContentLength = http::parse_opt_header(req, &CONTENT_LENGTH)?.unwrap_or(0);

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let part_number: PartNumber = http::parse_opt_query(req, "partNumber")?.unwrap_or(0);

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let upload_id: MultipartUploadId = http::parse_query(req, "uploadId")?;

        Ok(UploadPartInput {
            body,
            bucket,
            checksum_algorithm,
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            content_length,
            content_md5,
            expected_bucket_owner,
            key,
            part_number,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            upload_id,
        })
    }

    pub fn serialize_http(x: UploadPartOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED,
            x.bucket_key_enabled,
        )?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32, x.checksum_crc32)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32C, x.checksum_crc32c)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA1, x.checksum_sha1)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA256, x.checksum_sha256)?;
        http::add_opt_header(&mut res, ETAG, x.e_tag)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM,
            x.sse_customer_algorithm,
        )?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5,
            x.sse_customer_key_md5,
        )?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        Ok(res)
    }
}

impl UploadPartCopy {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<UploadPartCopyInput> {
        let (bucket, key) = http::unwrap_object(req);

        let copy_source: CopySource = http::parse_header(req, &X_AMZ_COPY_SOURCE)?;

        let copy_source_if_match: Option<CopySourceIfMatch> = http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_IF_MATCH)?;

        let copy_source_if_modified_since: Option<CopySourceIfModifiedSince> =
            http::parse_opt_header_timestamp(req, &X_AMZ_COPY_SOURCE_IF_MODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let copy_source_if_none_match: Option<CopySourceIfNoneMatch> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_IF_NONE_MATCH)?;

        let copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince> =
            http::parse_opt_header_timestamp(req, &X_AMZ_COPY_SOURCE_IF_UNMODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let copy_source_range: Option<CopySourceRange> = http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_RANGE)?;

        let copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let copy_source_sse_customer_key: Option<CopySourceSSECustomerKey> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let expected_source_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_SOURCE_EXPECTED_BUCKET_OWNER)?;

        let part_number: PartNumber = http::parse_opt_query(req, "partNumber")?.unwrap_or(0);

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let upload_id: MultipartUploadId = http::parse_query(req, "uploadId")?;

        Ok(UploadPartCopyInput {
            bucket,
            copy_source,
            copy_source_if_match,
            copy_source_if_modified_since,
            copy_source_if_none_match,
            copy_source_if_unmodified_since,
            copy_source_range,
            copy_source_sse_customer_algorithm,
            copy_source_sse_customer_key,
            copy_source_sse_customer_key_md5,
            expected_bucket_owner,
            expected_source_bucket_owner,
            key,
            part_number,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            upload_id,
        })
    }

    pub fn serialize_http(x: UploadPartCopyOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.copy_part_result {
            http::set_xml_body(&mut res, val)?;
        }
        http::add_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED,
            x.bucket_key_enabled,
        )?;
        http::add_opt_header(&mut res, X_AMZ_COPY_SOURCE_VERSION_ID, x.copy_source_version_id)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM,
            x.sse_customer_algorithm,
        )?;
        http::add_opt_header(
            &mut res,
            X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5,
            x.sse_customer_key_md5,
        )?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        Ok(res)
    }
}

impl WriteGetObjectResponse {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<WriteGetObjectResponseInput> {
        let accept_ranges: Option<AcceptRanges> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_ACCEPT_RANGES)?;

        let body: Option<StreamingBlob> = Some(http::take_stream_body(req));

        let bucket_key_enabled: BucketKeyEnabled =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED)?.unwrap_or(false);

        let cache_control: Option<CacheControl> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CACHE_CONTROL)?;

        let checksum_crc32: Option<ChecksumCRC32> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_CRC32)?;

        let checksum_crc32c: Option<ChecksumCRC32C> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_CRC32C)?;

        let checksum_sha1: Option<ChecksumSHA1> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_SHA1)?;

        let checksum_sha256: Option<ChecksumSHA256> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_SHA256)?;

        let content_disposition: Option<ContentDisposition> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CONTENT_DISPOSITION)?;

        let content_encoding: Option<ContentEncoding> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CONTENT_ENCODING)?;

        let content_language: Option<ContentLanguage> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CONTENT_LANGUAGE)?;

        let content_length: ContentLength = http::parse_opt_header(req, &CONTENT_LENGTH)?.unwrap_or(0);

        let content_range: Option<ContentRange> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CONTENT_RANGE)?;

        let content_type: Option<ContentType> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CONTENT_TYPE)?;

        let delete_marker: DeleteMarker = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_DELETE_MARKER)?.unwrap_or(false);

        let e_tag: Option<ETag> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_E_TAG)?;

        let error_code: Option<ErrorCode> = http::parse_opt_header(req, &X_AMZ_FWD_ERROR_CODE)?;

        let error_message: Option<ErrorMessage> = http::parse_opt_header(req, &X_AMZ_FWD_ERROR_MESSAGE)?;

        let expiration: Option<Expiration> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_EXPIRATION)?;

        let expires: Option<Expires> =
            http::parse_opt_header_timestamp(req, &X_AMZ_FWD_HEADER_EXPIRES, TimestampFormat::HttpDate)?;

        let last_modified: Option<LastModified> =
            http::parse_opt_header_timestamp(req, &X_AMZ_FWD_HEADER_LAST_MODIFIED, TimestampFormat::HttpDate)?;

        let metadata: Option<Metadata> = http::parse_opt_metadata(req)?;

        let missing_meta: MissingMeta = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_MISSING_META)?.unwrap_or(0);

        let object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_OBJECT_LOCK_LEGAL_HOLD)?;

        let object_lock_mode: Option<ObjectLockMode> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_OBJECT_LOCK_MODE)?;

        let object_lock_retain_until_date: Option<ObjectLockRetainUntilDate> = http::parse_opt_header_timestamp(
            req,
            &X_AMZ_FWD_HEADER_X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE,
            TimestampFormat::DateTime,
        )?;

        let parts_count: PartsCount = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_MP_PARTS_COUNT)?.unwrap_or(0);

        let replication_status: Option<ReplicationStatus> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_REPLICATION_STATUS)?;

        let request_charged: Option<RequestCharged> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_REQUEST_CHARGED)?;

        let request_route: RequestRoute = http::parse_header(req, &X_AMZ_REQUEST_ROUTE)?;

        let request_token: RequestToken = http::parse_header(req, &X_AMZ_REQUEST_TOKEN)?;

        let restore: Option<Restore> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_RESTORE)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let ssekms_key_id: Option<SSEKMSKeyId> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID)?;

        let server_side_encryption: Option<ServerSideEncryption> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION)?;

        let status_code: GetObjectResponseStatusCode = http::parse_opt_header(req, &X_AMZ_FWD_STATUS)?.unwrap_or(0);

        let storage_class: Option<StorageClass> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_STORAGE_CLASS)?;

        let tag_count: TagCount = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_TAGGING_COUNT)?.unwrap_or(0);

        let version_id: Option<ObjectVersionId> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_VERSION_ID)?;

        Ok(WriteGetObjectResponseInput {
            accept_ranges,
            body,
            bucket_key_enabled,
            cache_control,
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            content_disposition,
            content_encoding,
            content_language,
            content_length,
            content_range,
            content_type,
            delete_marker,
            e_tag,
            error_code,
            error_message,
            expiration,
            expires,
            last_modified,
            metadata,
            missing_meta,
            object_lock_legal_hold_status,
            object_lock_mode,
            object_lock_retain_until_date,
            parts_count,
            replication_status,
            request_charged,
            request_route,
            request_token,
            restore,
            sse_customer_algorithm,
            sse_customer_key_md5,
            ssekms_key_id,
            server_side_encryption,
            status_code,
            storage_class,
            tag_count,
            version_id,
        })
    }

    pub fn serialize_http() -> http::Response {
        http::Response::default()
    }
}

#[async_trait::async_trait]
impl super::Operation for AbortMultipartUpload {
    fn name(&self) -> &'static str {
        "AbortMultipartUpload"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.abort_multipart_upload(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for CompleteMultipartUpload {
    fn name(&self) -> &'static str {
        "CompleteMultipartUpload"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.complete_multipart_upload(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for CopyObject {
    fn name(&self) -> &'static str {
        "CopyObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.copy_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for CreateBucket {
    fn name(&self) -> &'static str {
        "CreateBucket"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.create_bucket(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for CreateMultipartUpload {
    fn name(&self) -> &'static str {
        "CreateMultipartUpload"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.create_multipart_upload(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucket {
    fn name(&self) -> &'static str {
        "DeleteBucket"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketAnalyticsConfiguration {
    fn name(&self) -> &'static str {
        "DeleteBucketAnalyticsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_analytics_configuration(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketCors {
    fn name(&self) -> &'static str {
        "DeleteBucketCors"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_cors(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketEncryption {
    fn name(&self) -> &'static str {
        "DeleteBucketEncryption"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_encryption(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketIntelligentTieringConfiguration {
    fn name(&self) -> &'static str {
        "DeleteBucketIntelligentTieringConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_intelligent_tiering_configuration(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketInventoryConfiguration {
    fn name(&self) -> &'static str {
        "DeleteBucketInventoryConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_inventory_configuration(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketLifecycle {
    fn name(&self) -> &'static str {
        "DeleteBucketLifecycle"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_lifecycle(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketMetricsConfiguration {
    fn name(&self) -> &'static str {
        "DeleteBucketMetricsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_metrics_configuration(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketOwnershipControls {
    fn name(&self) -> &'static str {
        "DeleteBucketOwnershipControls"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_ownership_controls(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketPolicy {
    fn name(&self) -> &'static str {
        "DeleteBucketPolicy"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_policy(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketReplication {
    fn name(&self) -> &'static str {
        "DeleteBucketReplication"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_replication(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketTagging {
    fn name(&self) -> &'static str {
        "DeleteBucketTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_tagging(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketWebsite {
    fn name(&self) -> &'static str {
        "DeleteBucketWebsite"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_website(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteObject {
    fn name(&self) -> &'static str {
        "DeleteObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteObjectTagging {
    fn name(&self) -> &'static str {
        "DeleteObjectTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_object_tagging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteObjects {
    fn name(&self) -> &'static str {
        "DeleteObjects"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_objects(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeletePublicAccessBlock {
    fn name(&self) -> &'static str {
        "DeletePublicAccessBlock"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_public_access_block(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketAccelerateConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketAccelerateConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_accelerate_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketAcl {
    fn name(&self) -> &'static str {
        "GetBucketAcl"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_acl(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketAnalyticsConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketAnalyticsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_analytics_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketCors {
    fn name(&self) -> &'static str {
        "GetBucketCors"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_cors(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketEncryption {
    fn name(&self) -> &'static str {
        "GetBucketEncryption"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_encryption(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketIntelligentTieringConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketIntelligentTieringConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_intelligent_tiering_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketInventoryConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketInventoryConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_inventory_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketLifecycleConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketLifecycleConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_lifecycle_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketLocation {
    fn name(&self) -> &'static str {
        "GetBucketLocation"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_location(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketLogging {
    fn name(&self) -> &'static str {
        "GetBucketLogging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_logging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketMetricsConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketMetricsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_metrics_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketNotificationConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketNotificationConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_notification_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketOwnershipControls {
    fn name(&self) -> &'static str {
        "GetBucketOwnershipControls"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_ownership_controls(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketPolicy {
    fn name(&self) -> &'static str {
        "GetBucketPolicy"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_policy(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketPolicyStatus {
    fn name(&self) -> &'static str {
        "GetBucketPolicyStatus"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_policy_status(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketReplication {
    fn name(&self) -> &'static str {
        "GetBucketReplication"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_replication(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketRequestPayment {
    fn name(&self) -> &'static str {
        "GetBucketRequestPayment"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_request_payment(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketTagging {
    fn name(&self) -> &'static str {
        "GetBucketTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_tagging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketVersioning {
    fn name(&self) -> &'static str {
        "GetBucketVersioning"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_versioning(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketWebsite {
    fn name(&self) -> &'static str {
        "GetBucketWebsite"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_website(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObject {
    fn name(&self) -> &'static str {
        "GetObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectAcl {
    fn name(&self) -> &'static str {
        "GetObjectAcl"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_acl(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectAttributes {
    fn name(&self) -> &'static str {
        "GetObjectAttributes"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_attributes(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectLegalHold {
    fn name(&self) -> &'static str {
        "GetObjectLegalHold"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_legal_hold(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectLockConfiguration {
    fn name(&self) -> &'static str {
        "GetObjectLockConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_lock_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectRetention {
    fn name(&self) -> &'static str {
        "GetObjectRetention"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_retention(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectTagging {
    fn name(&self) -> &'static str {
        "GetObjectTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_tagging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectTorrent {
    fn name(&self) -> &'static str {
        "GetObjectTorrent"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_torrent(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetPublicAccessBlock {
    fn name(&self) -> &'static str {
        "GetPublicAccessBlock"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_public_access_block(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for HeadBucket {
    fn name(&self) -> &'static str {
        "HeadBucket"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.head_bucket(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for HeadObject {
    fn name(&self) -> &'static str {
        "HeadObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.head_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListBucketAnalyticsConfigurations {
    fn name(&self) -> &'static str {
        "ListBucketAnalyticsConfigurations"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_bucket_analytics_configurations(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListBucketIntelligentTieringConfigurations {
    fn name(&self) -> &'static str {
        "ListBucketIntelligentTieringConfigurations"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_bucket_intelligent_tiering_configurations(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListBucketInventoryConfigurations {
    fn name(&self) -> &'static str {
        "ListBucketInventoryConfigurations"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_bucket_inventory_configurations(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListBucketMetricsConfigurations {
    fn name(&self) -> &'static str {
        "ListBucketMetricsConfigurations"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_bucket_metrics_configurations(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListBuckets {
    fn name(&self) -> &'static str {
        "ListBuckets"
    }

    async fn call(&self, s3: &dyn S3, _: &mut http::Request) -> S3Result<http::Response> {
        let result = s3.list_buckets().await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListMultipartUploads {
    fn name(&self) -> &'static str {
        "ListMultipartUploads"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_multipart_uploads(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListObjectVersions {
    fn name(&self) -> &'static str {
        "ListObjectVersions"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_object_versions(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListObjects {
    fn name(&self) -> &'static str {
        "ListObjects"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_objects(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListObjectsV2 {
    fn name(&self) -> &'static str {
        "ListObjectsV2"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_objects_v2(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListParts {
    fn name(&self) -> &'static str {
        "ListParts"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_parts(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketAccelerateConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketAccelerateConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_accelerate_configuration(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketAcl {
    fn name(&self) -> &'static str {
        "PutBucketAcl"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_acl(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketAnalyticsConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketAnalyticsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_analytics_configuration(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketCors {
    fn name(&self) -> &'static str {
        "PutBucketCors"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_cors(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketEncryption {
    fn name(&self) -> &'static str {
        "PutBucketEncryption"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_encryption(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketIntelligentTieringConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketIntelligentTieringConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_intelligent_tiering_configuration(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketInventoryConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketInventoryConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_inventory_configuration(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketLifecycleConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketLifecycleConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_lifecycle_configuration(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketLogging {
    fn name(&self) -> &'static str {
        "PutBucketLogging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_logging(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketMetricsConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketMetricsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_metrics_configuration(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketNotificationConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketNotificationConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_notification_configuration(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketOwnershipControls {
    fn name(&self) -> &'static str {
        "PutBucketOwnershipControls"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_ownership_controls(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketPolicy {
    fn name(&self) -> &'static str {
        "PutBucketPolicy"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_policy(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketReplication {
    fn name(&self) -> &'static str {
        "PutBucketReplication"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_replication(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketRequestPayment {
    fn name(&self) -> &'static str {
        "PutBucketRequestPayment"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_request_payment(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketTagging {
    fn name(&self) -> &'static str {
        "PutBucketTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_tagging(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketVersioning {
    fn name(&self) -> &'static str {
        "PutBucketVersioning"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_versioning(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketWebsite {
    fn name(&self) -> &'static str {
        "PutBucketWebsite"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_website(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObject {
    fn name(&self) -> &'static str {
        "PutObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObjectAcl {
    fn name(&self) -> &'static str {
        "PutObjectAcl"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object_acl(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObjectLegalHold {
    fn name(&self) -> &'static str {
        "PutObjectLegalHold"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object_legal_hold(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObjectLockConfiguration {
    fn name(&self) -> &'static str {
        "PutObjectLockConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object_lock_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObjectRetention {
    fn name(&self) -> &'static str {
        "PutObjectRetention"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object_retention(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObjectTagging {
    fn name(&self) -> &'static str {
        "PutObjectTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object_tagging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutPublicAccessBlock {
    fn name(&self) -> &'static str {
        "PutPublicAccessBlock"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_public_access_block(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for RestoreObject {
    fn name(&self) -> &'static str {
        "RestoreObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.restore_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for UploadPart {
    fn name(&self) -> &'static str {
        "UploadPart"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.upload_part(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for UploadPartCopy {
    fn name(&self) -> &'static str {
        "UploadPartCopy"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.upload_part_copy(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for WriteGetObjectResponse {
    fn name(&self) -> &'static str {
        "WriteGetObjectResponse"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.write_get_object_response(input).await;
        let res = match result {
            Ok(()) => Self::serialize_http(),
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub fn resolve_route(
    req: &http::Request,
    s3_path: &S3Path,
    qs: Option<&http::OrderedQs>,
) -> S3Result<(&'static dyn super::Operation, bool)> {
    match req.method().clone() {
        hyper::Method::HEAD => match s3_path {
            S3Path::Root => Err(super::unknown_operation()),
            S3Path::Bucket { .. } => Ok((&HeadBucket as &'static dyn super::Operation, false)),
            S3Path::Object { .. } => Ok((&HeadObject as &'static dyn super::Operation, false)),
        },
        hyper::Method::GET => match s3_path {
            S3Path::Root => Ok((&ListBuckets as &'static dyn super::Operation, false)),
            S3Path::Bucket { .. } => {
                if let Some(qs) = qs {
                    if qs.has("analytics") {
                        return Ok((&GetBucketAnalyticsConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("intelligent-tiering") {
                        return Ok((
                            &GetBucketIntelligentTieringConfiguration as &'static dyn super::Operation,
                            false,
                        ));
                    }
                    if qs.has("inventory") {
                        return Ok((&GetBucketInventoryConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("metrics") {
                        return Ok((&GetBucketMetricsConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("accelerate") {
                        return Ok((&GetBucketAccelerateConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("acl") {
                        return Ok((&GetBucketAcl as &'static dyn super::Operation, false));
                    }
                    if qs.has("cors") {
                        return Ok((&GetBucketCors as &'static dyn super::Operation, false));
                    }
                    if qs.has("encryption") {
                        return Ok((&GetBucketEncryption as &'static dyn super::Operation, false));
                    }
                    if qs.has("lifecycle") {
                        return Ok((&GetBucketLifecycleConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("location") {
                        return Ok((&GetBucketLocation as &'static dyn super::Operation, false));
                    }
                    if qs.has("logging") {
                        return Ok((&GetBucketLogging as &'static dyn super::Operation, false));
                    }
                    if qs.has("notification") {
                        return Ok((&GetBucketNotificationConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("ownershipControls") {
                        return Ok((&GetBucketOwnershipControls as &'static dyn super::Operation, false));
                    }
                    if qs.has("policy") {
                        return Ok((&GetBucketPolicy as &'static dyn super::Operation, false));
                    }
                    if qs.has("policyStatus") {
                        return Ok((&GetBucketPolicyStatus as &'static dyn super::Operation, false));
                    }
                    if qs.has("replication") {
                        return Ok((&GetBucketReplication as &'static dyn super::Operation, false));
                    }
                    if qs.has("requestPayment") {
                        return Ok((&GetBucketRequestPayment as &'static dyn super::Operation, false));
                    }
                    if qs.has("tagging") {
                        return Ok((&GetBucketTagging as &'static dyn super::Operation, false));
                    }
                    if qs.has("versioning") {
                        return Ok((&GetBucketVersioning as &'static dyn super::Operation, false));
                    }
                    if qs.has("website") {
                        return Ok((&GetBucketWebsite as &'static dyn super::Operation, false));
                    }
                    if qs.has("object-lock") {
                        return Ok((&GetObjectLockConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("publicAccessBlock") {
                        return Ok((&GetPublicAccessBlock as &'static dyn super::Operation, false));
                    }
                    if qs.has("analytics") {
                        return Ok((&ListBucketAnalyticsConfigurations as &'static dyn super::Operation, false));
                    }
                    if qs.has("intelligent-tiering") {
                        return Ok((
                            &ListBucketIntelligentTieringConfigurations as &'static dyn super::Operation,
                            false,
                        ));
                    }
                    if qs.has("inventory") {
                        return Ok((&ListBucketInventoryConfigurations as &'static dyn super::Operation, false));
                    }
                    if qs.has("metrics") {
                        return Ok((&ListBucketMetricsConfigurations as &'static dyn super::Operation, false));
                    }
                    if qs.has("uploads") {
                        return Ok((&ListMultipartUploads as &'static dyn super::Operation, false));
                    }
                    if qs.has("versions") {
                        return Ok((&ListObjectVersions as &'static dyn super::Operation, false));
                    }
                    if super::check_query_pattern(qs, "list-type", "2") {
                        return Ok((&ListObjectsV2 as &'static dyn super::Operation, false));
                    }
                }
                Ok((&ListObjects as &'static dyn super::Operation, false))
            }
            S3Path::Object { .. } => {
                if let Some(qs) = qs {
                    if qs.has("attributes") {
                        return Ok((&GetObjectAttributes as &'static dyn super::Operation, false));
                    }
                    if qs.has("acl") {
                        return Ok((&GetObjectAcl as &'static dyn super::Operation, false));
                    }
                    if qs.has("legal-hold") {
                        return Ok((&GetObjectLegalHold as &'static dyn super::Operation, false));
                    }
                    if qs.has("retention") {
                        return Ok((&GetObjectRetention as &'static dyn super::Operation, false));
                    }
                    if qs.has("tagging") {
                        return Ok((&GetObjectTagging as &'static dyn super::Operation, false));
                    }
                    if qs.has("torrent") {
                        return Ok((&GetObjectTorrent as &'static dyn super::Operation, false));
                    }
                }
                if let Some(qs) = qs {
                    if qs.has("uploadId") {
                        return Ok((&ListParts as &'static dyn super::Operation, false));
                    }
                }
                Ok((&GetObject as &'static dyn super::Operation, false))
            }
        },
        hyper::Method::POST => match s3_path {
            S3Path::Root => Err(super::unknown_operation()),
            S3Path::Bucket { .. } => {
                if let Some(qs) = qs {
                    if qs.has("delete") {
                        return Ok((&DeleteObjects as &'static dyn super::Operation, true));
                    }
                }
                if req.headers().contains_key("x-amz-request-route") && req.headers().contains_key("x-amz-request-token") {
                    return Ok((&WriteGetObjectResponse as &'static dyn super::Operation, false));
                }
                Err(super::unknown_operation())
            }
            S3Path::Object { .. } => {
                if let Some(qs) = qs {
                    if qs.has("uploads") {
                        return Ok((&CreateMultipartUpload as &'static dyn super::Operation, false));
                    }
                    if qs.has("restore") {
                        return Ok((&RestoreObject as &'static dyn super::Operation, true));
                    }
                }
                if let Some(qs) = qs {
                    if qs.has("uploadId") {
                        return Ok((&CompleteMultipartUpload as &'static dyn super::Operation, true));
                    }
                }
                Err(super::unknown_operation())
            }
        },
        hyper::Method::PUT => match s3_path {
            S3Path::Root => Err(super::unknown_operation()),
            S3Path::Bucket { .. } => {
                if let Some(qs) = qs {
                    if qs.has("analytics") {
                        return Ok((&PutBucketAnalyticsConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("intelligent-tiering") {
                        return Ok((
                            &PutBucketIntelligentTieringConfiguration as &'static dyn super::Operation,
                            true,
                        ));
                    }
                    if qs.has("inventory") {
                        return Ok((&PutBucketInventoryConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("metrics") {
                        return Ok((&PutBucketMetricsConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("accelerate") {
                        return Ok((&PutBucketAccelerateConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("acl") {
                        return Ok((&PutBucketAcl as &'static dyn super::Operation, true));
                    }
                    if qs.has("cors") {
                        return Ok((&PutBucketCors as &'static dyn super::Operation, true));
                    }
                    if qs.has("encryption") {
                        return Ok((&PutBucketEncryption as &'static dyn super::Operation, true));
                    }
                    if qs.has("lifecycle") {
                        return Ok((&PutBucketLifecycleConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("logging") {
                        return Ok((&PutBucketLogging as &'static dyn super::Operation, true));
                    }
                    if qs.has("notification") {
                        return Ok((&PutBucketNotificationConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("ownershipControls") {
                        return Ok((&PutBucketOwnershipControls as &'static dyn super::Operation, true));
                    }
                    if qs.has("policy") {
                        return Ok((&PutBucketPolicy as &'static dyn super::Operation, true));
                    }
                    if qs.has("replication") {
                        return Ok((&PutBucketReplication as &'static dyn super::Operation, true));
                    }
                    if qs.has("requestPayment") {
                        return Ok((&PutBucketRequestPayment as &'static dyn super::Operation, true));
                    }
                    if qs.has("tagging") {
                        return Ok((&PutBucketTagging as &'static dyn super::Operation, true));
                    }
                    if qs.has("versioning") {
                        return Ok((&PutBucketVersioning as &'static dyn super::Operation, true));
                    }
                    if qs.has("website") {
                        return Ok((&PutBucketWebsite as &'static dyn super::Operation, true));
                    }
                    if qs.has("object-lock") {
                        return Ok((&PutObjectLockConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("publicAccessBlock") {
                        return Ok((&PutPublicAccessBlock as &'static dyn super::Operation, true));
                    }
                }
                Ok((&CreateBucket as &'static dyn super::Operation, true))
            }
            S3Path::Object { .. } => {
                if let Some(qs) = qs {
                    if qs.has("acl") {
                        return Ok((&PutObjectAcl as &'static dyn super::Operation, true));
                    }
                    if qs.has("legal-hold") {
                        return Ok((&PutObjectLegalHold as &'static dyn super::Operation, true));
                    }
                    if qs.has("retention") {
                        return Ok((&PutObjectRetention as &'static dyn super::Operation, true));
                    }
                    if qs.has("tagging") {
                        return Ok((&PutObjectTagging as &'static dyn super::Operation, true));
                    }
                }
                if let Some(qs) = qs {
                    if qs.has("uploadId") && req.headers().contains_key("x-amz-copy-source") {
                        return Ok((&UploadPartCopy as &'static dyn super::Operation, false));
                    }
                }
                if let Some(qs) = qs {
                    if qs.has("uploadId") {
                        return Ok((&UploadPart as &'static dyn super::Operation, false));
                    }
                }
                if req.headers().contains_key("x-amz-copy-source") {
                    return Ok((&CopyObject as &'static dyn super::Operation, false));
                }
                Ok((&PutObject as &'static dyn super::Operation, false))
            }
        },
        hyper::Method::DELETE => match s3_path {
            S3Path::Root => Err(super::unknown_operation()),
            S3Path::Bucket { .. } => {
                if let Some(qs) = qs {
                    if qs.has("analytics") {
                        return Ok((&DeleteBucketAnalyticsConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("intelligent-tiering") {
                        return Ok((
                            &DeleteBucketIntelligentTieringConfiguration as &'static dyn super::Operation,
                            false,
                        ));
                    }
                    if qs.has("inventory") {
                        return Ok((&DeleteBucketInventoryConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("metrics") {
                        return Ok((&DeleteBucketMetricsConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("cors") {
                        return Ok((&DeleteBucketCors as &'static dyn super::Operation, false));
                    }
                    if qs.has("encryption") {
                        return Ok((&DeleteBucketEncryption as &'static dyn super::Operation, false));
                    }
                    if qs.has("lifecycle") {
                        return Ok((&DeleteBucketLifecycle as &'static dyn super::Operation, false));
                    }
                    if qs.has("ownershipControls") {
                        return Ok((&DeleteBucketOwnershipControls as &'static dyn super::Operation, false));
                    }
                    if qs.has("policy") {
                        return Ok((&DeleteBucketPolicy as &'static dyn super::Operation, false));
                    }
                    if qs.has("replication") {
                        return Ok((&DeleteBucketReplication as &'static dyn super::Operation, false));
                    }
                    if qs.has("tagging") {
                        return Ok((&DeleteBucketTagging as &'static dyn super::Operation, false));
                    }
                    if qs.has("website") {
                        return Ok((&DeleteBucketWebsite as &'static dyn super::Operation, false));
                    }
                    if qs.has("publicAccessBlock") {
                        return Ok((&DeletePublicAccessBlock as &'static dyn super::Operation, false));
                    }
                }
                Ok((&DeleteBucket as &'static dyn super::Operation, false))
            }
            S3Path::Object { .. } => {
                if let Some(qs) = qs {
                    if qs.has("tagging") {
                        return Ok((&DeleteObjectTagging as &'static dyn super::Operation, false));
                    }
                }
                if let Some(qs) = qs {
                    if qs.has("uploadId") {
                        return Ok((&AbortMultipartUpload as &'static dyn super::Operation, false));
                    }
                }
                Ok((&DeleteObject as &'static dyn super::Operation, false))
            }
        },
        _ => Err(super::unknown_operation()),
    }
}
