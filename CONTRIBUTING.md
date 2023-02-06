# Development Guide

Toolchain

+ [Rust 1.66.1 or newer](https://rustup.rs/)
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

#### Run the codegen

```bash
just download-model
just codegen
```

It should change nothing if you are running the latest code.

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

#### Run E2E tests

Install `s3s-proxy`

```bash
just install
```

Run the combined server and save logs

```bash
./scripts/s3s-proxy.sh | tee target/s3s-proxy.ansi
```

Open a new terminal, then run the test suite

```bash
./scripts/mint.sh
```
