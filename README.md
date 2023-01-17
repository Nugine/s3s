# s3s

[![Apache 2.0 licensed][license-badge]][license-url]
[![Unsafe Forbidden][unsafe-forbidden-badge]][unsafe-forbidden-url]

[license-badge]: https://img.shields.io/badge/license-Apache--2.0-blue.svg
[license-url]: ./LICENSE
[unsafe-forbidden-badge]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[unsafe-forbidden-url]: https://github.com/rust-secure-code/safety-dance/

S3 Service Adapter


| crate                      |                                           version                                           |                                 docs                                 |
| :------------------------- | :-----------------------------------------------------------------------------------------: | :------------------------------------------------------------------: |
| [s3s](./crates/s3s/)       |    [![Crates.io](https://img.shields.io/crates/v/s3s.svg)](https://crates.io/crates/s3s)    |    [![Docs](https://docs.rs/s3s/badge.svg)](https://docs.rs/s3s/)    |
| [s3s-fs](./crates/s3s-fs/) | [![Crates.io](https://img.shields.io/crates/v/s3s-fs.svg)](https://crates.io/crates/s3s-fs) | [![Docs](https://docs.rs/s3s-fs/badge.svg)](https://docs.rs/s3s-fs/) |

This experimental project intends to offer an ergonomic adapter for building S3-compatible services.

`s3s` implements Amazon S3 REST API in the form of a generic [hyper](https://github.com/hyperium/hyper) service. S3-compatible services can focus on the S3 API itself and don't have to care about the HTTP layer.

`s3s-fs` implements the S3 API based on file system. It is designed for integration testing. DO NOT USE IT IN PRODUCTION.
