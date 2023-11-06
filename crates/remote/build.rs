use std::process::Command;

fn main() {
    Command::new("cargo")
        .args(&[
            "build",
            "--package",
            "renderer",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .status()
        .unwrap();
    Command::new("cmd").args(&["pwd"]).status().unwrap();
    println!("was");
    Command::new("wasm-bindgen")
        .args(&[
            "--no-typescript",
            "--out-dir",
            "./web/renderer",
            "--target",
            "web",
            "../../target/wasm32-unknown-unknown/release/renderer.wasm",
        ])
        .status()
        .unwrap();
    println!("cargo:rerun-if-changed=../renderer/src");
    println!("cargo:rerun-if-changed=./web/renderer");
}
