# rust-fourcc

`fourcc` is an API for working with FourCC values. It provides two types, `TypeId` and `FourCC`.

## `Usage`

`TypeId` is an alias for `[u8;4]`, this type works well with existing Rust traits for byte arrays and should be preferred.

```rust
use fourcc::TypeId;
let typeid: &TypeId = b"FORM";
```

`FourCC` encapsulates `TypeId` and decorates it with traits for converting to and from different POD types.
