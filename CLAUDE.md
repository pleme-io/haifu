# haifu — OTA Update Server

Self-hosted OTA server for custom ROMs and enterprise APK distribution. Consumes ArchiveReader, ApkSignatureVerifier, VbmetaParser traits.

## Build & Test

```bash
cargo build
cargo test
cargo run -- serve
```

## Conventions

- Edition 2024, Rust 1.91.0+, MIT, clippy pedantic
- Release: codegen-units=1, lto=true, opt-level="z", strip=true
