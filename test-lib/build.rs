fn main() {
    // Smoke test all parameter reading functions
    smoke_test_inputs();

    // Set a cfg to verify this build script ran
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-cfg=did_run_build_script");
}

fn smoke_test_inputs() {
    use build_params::input::*;

    // Test optional string parameter
    dbg!(param("test_string"));

    // Test optional boolean parameter
    dbg!(param_bool("test_bool"));

    // Test optional integer parameters
    dbg!(param_i64("test_i64"));
    dbg!(param_u64("test_u64"));

    // Test with actual environment variables if set
    std::env::set_var("BUILD_PARAM_SAMPLE_STRING", "hello");
    std::env::set_var("BUILD_PARAM_SAMPLE_BOOL", "true");
    std::env::set_var("BUILD_PARAM_SAMPLE_INT", "42");

    assert_eq!(param("sample_string"), Some("hello".to_string()));
    assert_eq!(param_bool("sample_bool"), Some(true));
    assert_eq!(param_i64("sample_int"), Some(42));

    // Test required variants would panic if missing, so we only test with set values
    assert_eq!(param_required("sample_string"), "hello");
    assert_eq!(param_bool_required("sample_bool"), true);
    assert_eq!(param_i64_required("sample_int"), 42);

    println!("cargo:warning=All parameter smoke tests passed!");
}
