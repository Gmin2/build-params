//! build-params provides a strongly typed interface for reading build script
//! parameters from `Cargo.toml`.
//!
//! Build scripts traditionally receive configuration through feature flags or
//! environment variables. This crate enables a declarative parameter syntax in
//! `Cargo.toml` that Cargo will pass to build scripts via environment variables.
//!
//! # Example
//!
//! In your `Cargo.toml`:
//! ```toml
//! cargo-features = ["build-script-parameters"]
//!
//! [package]
//! name = "hello"
//! build = "build.rs"
//!
//! [package.build.parameters]
//! optimization = "3"
//! enable_simd = true
//! ```
//!
//! In your `build.rs`:
//! ```rust,ignore
//! fn main() {
//!     let opt_level = build_params::param("optimization").unwrap();
//!     let simd = build_params::param_bool("enable_simd").unwrap();
//!
//!     println!("cargo::rustc-env=OPT={}", opt_level);
//!     println!("cargo::rustc-env=SIMD={}", simd);
//! }
//! ```
//!

#![cfg_attr(all(doc, feature = "unstable"), feature(doc_auto_cfg, doc_cfg))]

/// Module for reading build script parameters
pub mod input;
