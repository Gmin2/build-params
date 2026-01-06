use std::process::Command;

fn main() {
    // Read parameters (or use defaults)
    let include_hash = build_params::input::param_bool("include_git_hash").unwrap_or(false);
    let include_time = build_params::input::param_bool("include_build_time").unwrap_or(false);
    let include_branch = build_params::input::param_bool("include_branch").unwrap_or(false);

    // Rerun if parameters change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=BUILD_PARAM_INCLUDE_GIT_HASH");
    println!("cargo:rerun-if-env-changed=BUILD_PARAM_INCLUDE_BUILD_TIME");
    println!("cargo:rerun-if-env-changed=BUILD_PARAM_INCLUDE_BRANCH");

    // Include git hash
    if include_hash {
        let hash = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .unwrap_or_else(|| "unknown".to_string());

        println!("cargo:rustc-env=GIT_HASH={}", hash.trim());
    }

    // Include build timestamp
    if include_time {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        println!("cargo:rustc-env=BUILD_TIMESTAMP={}", now);
    }

    // Include git branch
    if include_branch {
        let branch = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .unwrap_or_else(|| "unknown".to_string());

        println!("cargo:rustc-env=GIT_BRANCH={}", branch.trim());
    }
}
