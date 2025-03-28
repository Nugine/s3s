# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/Nugine/s3s/compare/v0.11.0...HEAD

## [v0.11.0] - 2025-03-28

[v0.11.0]: https://github.com/Nugine/s3s/compare/v0.10.1...v0.11.0

Tracking in [#267](https://github.com/Nugine/s3s/issues/267).

MSRV of this minor version: 1.85.0

### s3s

**BREAKING**: Following the latest model definitions in [aws-sdk-rust](https://github.com/awslabs/aws-sdk-rust), `s3s::dto` is updated.
+ You may come across some type changes reported by rustc.
+ The migration is not hard but requires some time.

**BREAKING**: More request parameters are accepted via upgrading model definitions.
+ S3 preconditions ([#241](https://github.com/Nugine/s3s/issues/241))
+ PutObject write_offset_bytes ([#249](https://github.com/Nugine/s3s/issues/249))

**BREAKING**: Policy-based access control is supported in `s3s::access` ([#161](https://github.com/Nugine/s3s/issues/161))
+ Add `S3Access` trait for access control.
+ Add `S3ServiceBuilder::set_access`.
+ Move `S3Auth::check_access` to `S3Access::check`.

**BREAKING**: Multi-domain is supported in `s3s::host`. ([#175](https://github.com/Nugine/s3s/issues/175))
+ Add `S3Host` trait for parsing host header.
+ Change `S3ServiceBuilder::set_base_domain` to `S3ServiceBuilder::set_host`.
+ Add `SingleDomain` parser.
+ Add `MultiDomain` parser.

Custom route is supported in `s3s::route` ([#195](https://github.com/Nugine/s3s/issues/195))
+ Add `S3Route` trait for custom route protected by signature verification.
+ Add `S3ServiceBuilder::set_route`.
+ Signature v4 supports AWS STS requests ([#208](https://github.com/Nugine/s3s/pull/208))
+ Add example using [axum](https://github.com/tokio-rs/axum) web framework ([#263](https://github.com/Nugine/s3s/pull/263))

Unstable `minio` branch:
+ Add `minio` branch for MinIO compatibility.
+ This branch is automatically force-rebased to the latest `main` branch.

Other notable changes
+ feat(s3s): export xml module ([#189](https://github.com/Nugine/s3s/pull/189))
+ fix(s3s/ops): allow presigned url requests with up to 15 minutes clock skew ([#216](https://github.com/Nugine/s3s/pull/216))
+ handle fmt message with implicit arguments in s3_error macro ([#228](https://github.com/Nugine/s3s/pull/228))
+ feat(s3s/dto): ignore empty strings ([#244](https://github.com/Nugine/s3s/pull/244))
+ feat(model): extra error codes ([#255](https://github.com/Nugine/s3s/pull/255))
+ feat(s3s/checksum): add crc64nvme ([#256](https://github.com/Nugine/s3s/pull/256))
+ feat(s3s/xml): support xmlns ([#265](https://github.com/Nugine/s3s/pull/265))

### s3s-model

+ Add crate `s3s-model` for S3 model definitions.

### s3s-policy

+ Add crate `s3s-policy` for S3 policy language.
+ Add grammar model types for serialization and deserialization in `s3s_policy::model`.
+ Add `PatternSet` for matching multiple patterns in `s3s_policy::pattern`.

### s3s-test

+ Add crate `s3s-test` for custom test framework.

### s3s-e2e

+ Add crate `s3s-e2e` for S3 compatibility tests.
