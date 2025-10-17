use std::process::Command;

fn main() {
    let output = Command::new("rustc").arg("--version").output().unwrap();
    let version = String::from_utf8(output.stdout).unwrap();
    let date = version
        .split(" ")
        .last()
        .unwrap()
        .replace("(", "")
        .replace(")", "");
    println!("cargo:rustc-env=RUSTC_BUILD_DATE={}", date);

    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
