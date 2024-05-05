# Development Guide

|               Toolchain               | Version |
| :-----------------------------------: | :-----: |
|      [Rust](https://rustup.rs/)       | ^1.75.0 |
| [just](https://github.com/casey/just) |    -    |
|                Python                 |  ^3.10  |
|                Docker                 |    -    |

Get the source code

```bash
git clone https://github.com/Nugine/s3s.git
cd s3s
```

#### Run basic checks and tests

```bash
just dev
```

#### Open documentation

```bash
just doc
```

#### Run the codegen

```bash
just download-model
just codegen
```

It should change nothing if you are running the latest code.

#### Play the test server

Install `s3s-fs` from source

```bash
cargo install --path crates/s3s-fs --features binary
```

You can also use the shortcut

```bash
just install-s3s-fs
```

Or install from crates.io

```bash
cargo install s3s-fs --features binary
```

Run `s3s-fs` with [example configuration](./scripts/s3s-fs.sh)

```bash
./scripts/s3s-fs.sh
```

Credentials used in the example configuration:

```
Access Key: AKEXAMPLES3S
Secret Key: SKEXAMPLES3S
```

Then you can explore it with your favorite S3 client!

#### Run E2E tests

Install `s3s-proxy`

```bash
just install-s3s-proxy
```

Run the combined server and save logs

```bash
./scripts/s3s-proxy.sh | tee target/s3s-proxy.log
```

Open a new terminal, then run the test suite

```bash
./scripts/mint.sh | tee target/mint.log
```
