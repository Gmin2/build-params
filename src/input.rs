//! Reading build script parameters from environment variables.
//!
//! Cargo sets build parameters as environment variables with the prefix
//! `BUILD_PARAM_`. This module provides typed access to those values.
//!

use std::env;

/// Error macros for consistent error messages
macro_rules! missing {
    ($key:expr) => {
        panic!("build parameter `{}` is missing", $key)
    };
}

macro_rules! invalid {
    ($key:expr, $expected:expr) => {
        panic!(
            "build parameter `{}` is invalid: expected {}",
            $key, $expected
        )
    };
}

/// Helper function to get an optional string parameter
///
/// Parameters are set by Cargo as `BUILD_PARAM_<KEY>` where `<KEY>` is the
/// parameter name converted to uppercase with hyphens replaced by underscores.
#[track_caller]
fn get_opt_str(key: &str) -> Option<String> {
    let env_key = format!("BUILD_PARAM_{}", key.to_uppercase().replace('-', "_"));
    env::var(&env_key).ok()
}

/// Helper function to get a required string parameter
#[track_caller]
fn get_str(key: &str) -> String {
    get_opt_str(key).unwrap_or_else(|| missing!(key))
}

/// Read an optional string parameter from build script configuration.
///
/// Returns `None` if the parameter was not set in `[package.build.parameters]`.
///
/// # Example
///
/// ```rust,ignore
/// // Cargo.toml:
/// // [package.build.parameters]
/// // output_dir = "generated"
///
/// let dir = build_params::input::param("output_dir");
/// assert_eq!(dir, Some("generated".to_string()));
/// ```
#[track_caller]
pub fn param(key: &str) -> Option<String> {
    get_opt_str(key)
}

/// Read a required string parameter from build script configuration.
///
/// # Panics
///
/// Panics if the parameter was not set in `[package.build.parameters]`.
///
/// # Example
///
/// ```rust,ignore
/// let dir = build_params::input::param_required("output_dir");
/// ```
#[track_caller]
pub fn param_required(key: &str) -> String {
    get_str(key)
}

/// Read a boolean parameter from build script configuration.
///
/// Accepts `"true"`, `"1"` for true, and `"false"`, `"0"` for false.
/// Returns `None` if the parameter was not set.
///
/// # Panics
///
/// Panics if the parameter value is not a valid boolean.
///
/// # Example
///
/// ```rust,ignore
/// // [package.build.parameters]
/// // enable_simd = true
///
/// let simd = build_params::input::param_bool("enable_simd");
/// assert_eq!(simd, Some(true));
/// ```
#[track_caller]
pub fn param_bool(key: &str) -> Option<bool> {
    let value = get_opt_str(key)?;
    match value.as_str() {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => invalid!(key, "boolean (true, false, 1, or 0)"),
    }
}

/// Read a required boolean parameter from build script configuration.
///
/// # Panics
///
/// Panics if the parameter was not set or is not a valid boolean.
#[track_caller]
pub fn param_bool_required(key: &str) -> bool {
    param_bool(key).unwrap_or_else(|| missing!(key))
}

/// Read an integer parameter from build script configuration.
///
/// Returns `None` if the parameter was not set.
///
/// # Panics
///
/// Panics if the parameter value is not a valid i64 integer.
///
/// # Example
///
/// ```rust,ignore
/// // [package.build.parameters]
/// // optimization_level = 3
///
/// let opt = build_params::input::param_i64("optimization_level");
/// assert_eq!(opt, Some(3));
/// ```
#[track_caller]
pub fn param_i64(key: &str) -> Option<i64> {
    let value = get_opt_str(key)?;
    match value.parse::<i64>() {
        Ok(n) => Some(n),
        Err(_) => invalid!(key, "integer"),
    }
}

/// Read a required integer parameter from build script configuration.
///
/// # Panics
///
/// Panics if the parameter was not set or is not a valid i64 integer.
#[track_caller]
pub fn param_i64_required(key: &str) -> i64 {
    param_i64(key).unwrap_or_else(|| missing!(key))
}

/// Read an unsigned integer parameter from build script configuration.
///
/// Returns `None` if the parameter was not set.
///
/// # Panics
///
/// Panics if the parameter value is not a valid u64 integer.
#[track_caller]
pub fn param_u64(key: &str) -> Option<u64> {
    let value = get_opt_str(key)?;
    match value.parse::<u64>() {
        Ok(n) => Some(n),
        Err(_) => invalid!(key, "unsigned integer"),
    }
}

/// Read a required unsigned integer parameter from build script configuration.
///
/// # Panics
///
/// Panics if the parameter was not set or is not a valid u64 integer.
#[track_caller]
pub fn param_u64_required(key: &str) -> u64 {
    param_u64(key).unwrap_or_else(|| missing!(key))
}
