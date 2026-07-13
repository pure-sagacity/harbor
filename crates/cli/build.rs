use std::process::Command;

fn main() {
    let git_version = if let Ok(v) = std::env::var("CARGO_ENV_GIT_VERSION") {
        v
    } else {
        Command::new("git")
            .args(["describe", "--always", "--dirty"])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string())
    };

    println!("cargo:rustc-env=GIT_VERSION={}", git_version);

    println!("cargo:rerun-if-changed=.git/HEAD");
}
