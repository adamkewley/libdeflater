# libdeflate-sys

Rust bindings to [libdeflate](https://github.com/ebiggers/libdeflate)

Not intended to be used directly.


# Runtime Requirements

- `libdeflate` installed system-wide (e.g. with `apt-get install libdeflate0`)


# Build Requirements

- `rust-bindgen` requirements installed (e.g. `apt-get install llvm-3.9-dev libclang-3.9-dev clang-3.9`)
- `libdeflate-dev` installed system-wide


# ToDo

- High-level integration tests to ensure it exports the correct API
- CI
- Put onto crates.io
- Write rust-level wrapper (with appropriate type-safety etc.)
