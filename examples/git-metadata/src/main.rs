fn main() {
    println!("Build Information:");

    // These will only be set if the build parameters enabled them
    if let Some(hash) = option_env!("GIT_HASH") {
        println!("Git Hash:       {}", hash);
    } else {
        println!("Git Hash:       [not included]");
    }

    if let Some(timestamp) = option_env!("BUILD_TIMESTAMP") {
        if let Ok(ts) = timestamp.parse::<u64>() {
            println!("Build Time:     {} (unix timestamp)", ts);
        }
    } else {
        println!("Build Time:     [not included]");
    }

    if let Some(branch) = option_env!("GIT_BRANCH") {
        println!("Git Branch:     {}", branch);
    } else {
        println!("Git Branch:     [not included]");
    }
}
