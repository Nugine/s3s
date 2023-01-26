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

`s3s-fs` implements the S3 API based on file system, as a sample implementation. It is designed for integration testing. DO NOT USE IT IN PRODUCTION.

## How it works

![architecture diagram](docs/arch/arch.svg)

The diagram above shows how `s3s` works. 

`s3s` converts HTTP requests to operation inputs before calling the user-defined service. 

`s3s` converts operation outputs or errors to HTTP responses after calling the user-defined service.

The data types, serialization and deserialization are generated from the smithy model in [aws-sdk-rust](https://github.com/awslabs/aws-sdk-rust) repository, with some manual hacks.

## Develop

Toolchain

+ Rust 1.66.1 or newer
+ [just](https://github.com/casey/just)

Get the source code

```bash
git clone https://github.com/Nugine/s3s.git
cd s3s
```

#### Run basic checks and tests

```bash
just dev
```

#### Play the test server

Install `s3s-fs`

```bash
just install
```

Run `s3s-fs` with example configuration

```bash
./scripts/s3s-fs.sh
```

```
Access Key: AKEXAMPLES3S
Secret Key: SKEXAMPLES3S
```

Then you can explore it with your favorite S3 client!

#### Run the codegen

```bash
just download-model
just codegen
```

It should change nothing if you are running the latest code.
