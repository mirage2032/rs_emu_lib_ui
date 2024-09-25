use std::env;
use std::process::Command;

fn main() {
    let _output = Command::new("stylance")
        .arg(".")
        .current_dir(env::var("CARGO_MANIFEST_DIR").unwrap())
        .output()
        .expect("Failed to execute stylance");
    // println!("cargo:rerun-if-changed=static/css/stylance");
}
