use std::process::Command;

fn main() {
    if !Command::new("cargo")
        .args(&[
            "build",
            "--package",
            "renderer",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .status()
        .unwrap()
        .success()
    {
        panic!();
    };
    if !Command::new("wasm-bindgen")
        .args(&[
            "--no-typescript",
            "--out-dir",
            "./web/renderer",
            "--target",
            "web",
            "../../target/wasm32-unknown-unknown/release/renderer.wasm",
        ])
        .status()
        .unwrap()
        .success()
    {
        panic!();
    };
    println!("cargo:rerun-if-changed=../renderer/src");
    println!("cargo:rerun-if-changed=./web/renderer");
}
