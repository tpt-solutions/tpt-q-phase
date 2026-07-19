# Contributing to TPT Q Phase

Thank you for your interest in contributing! This document describes how to get
started.

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating,
you are expected to uphold it.

## License

By contributing, you agree that your contributions are dual-licensed under
MIT OR Apache-2.0, at the option of the user, consistent with the rest of the
project (`SPDX-License-Identifier: MIT OR Apache-2.0`).

## Development Setup

- Install the [Rust toolchain](https://rustup.rs/).
- Build the workspace: `cargo build --workspace`
- Run tests: `cargo test --workspace`
- Run clippy: `cargo clippy --workspace --all-targets`
- Check formatting: `cargo fmt --all -- --check`

## SPDX Headers

Every source file must begin with:

```rust
// SPDX-License-Identifier: MIT OR Apache-2.0
```

## Submitting Changes

1. Fork and create a feature branch.
2. Make your change with tests.
3. Ensure `cargo test`, `cargo clippy`, and `cargo fmt` pass.
4. Open a pull request describing the change and referencing any related issues.
