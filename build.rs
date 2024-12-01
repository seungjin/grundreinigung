use chrono::{Local, Utc};
use std::process::Command;

fn main() {
    // Get git information
    let hash = get_git_hash();
    let branch = get_git_branch();
    let commit_date = get_git_commit_date();

    let now = Utc::now().with_timezone(&Local);

    // Pass to compiler
    println!("cargo:rustc-env=GIT_HASH={}", hash);
    println!("cargo:rustc-env=GIT_BRANCH={}", branch);
    println!("cargo:rustc-env=GIT_COMMIT_DATE={}", commit_date);
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", now.to_rfc2822());

    // Force rebuild if git state changes
    println!("cargo:rerun-if-changed=.git/HEAD");
}

fn get_git_hash() -> String {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to get git hash");
    String::from_utf8(output.stdout)
        .expect("Failed to convert git hash to string")
        .trim()
        .to_string()
}

fn get_git_branch() -> String {
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Failed to get git branch");
    String::from_utf8(output.stdout)
        .expect("Failed to convert git branch to string")
        .trim()
        .to_string()
}

fn get_git_commit_date() -> String {
    let output = Command::new("git")
        .args(&["log", "-1", "--format=%cD"])
        .output()
        .expect("Failed to get commit date");
    String::from_utf8(output.stdout)
        .expect("Failed to convert commit date to string")
        .trim()
        .to_string()
}
