# fourcc

![GitHub Package Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FStealthOfKing%2Frust-fourcc%2Frefs%2Fheads%2Fmaster%2FCargo.toml&query=%24.package.version&prefix=v&label=Rust)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/StealthOfKing/rust-fourcc/rust.yml)
![GitHub License](https://img.shields.io/github/license/StealthOfKing/rust-fourcc)

`fourcc` is an API for working with FourCC values. It provides two types, `TypeId` and `FourCC`.

## Usage

`TypeId` is an alias for `[u8;4]`, this type works well with existing Rust traits for byte arrays and should be preferred.

```rust
use fourcc::TypeId;
let typeid: &TypeId = b"FORM";
```

`FourCC` encapsulates `TypeId` and decorates it with traits for converting to and from different POD types.
