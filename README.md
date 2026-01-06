# build-params

A strongly typed interface for reading build script parameters from Cargo.toml.

## Overview

This library provides a type-safe API for build scripts to read configuration parameters that will eventually be specified in Cargo.toml under `[package.build.parameters]`. Currently, parameters are passed via environment variables with the `BUILD_PARAM_` prefix.

## Usage

In your build.rs:

```rust
fn main() {
    let database = build_params::input::param("database").unwrap_or_else(|| "sqlite".to_string());
    let debug = build_params::input::param_bool("debug_mode").unwrap_or(false);
    let max_conn = build_params::input::param_i64("max_connections").unwrap_or(10);
}
```

Currently, set parameters via environment variables:

```bash
BUILD_PARAM_DATABASE=postgres BUILD_PARAM_DEBUG_MODE=true cargo build
```

