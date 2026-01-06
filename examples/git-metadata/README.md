# Git Metadata Example

Demonstrates using build script parameters to conditionally include git information.

## Usage

Since Cargo integration has not been done `[package.build.parameters]` yet, we manually set environment variables:

### Include git hash only:
```bash
BUILD_PARAM_INCLUDE_GIT_HASH=true cargo run
```

### Include everything:
```bash
BUILD_PARAM_INCLUDE_GIT_HASH=true \
BUILD_PARAM_INCLUDE_BUILD_TIME=true \
BUILD_PARAM_INCLUDE_BRANCH=true \
cargo run
```

### Include nothing (defaults):
```bash
cargo run
```