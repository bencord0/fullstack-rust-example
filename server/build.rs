use std::{
    env,
    fs::remove_dir_all,
    io::{self, Write},
    path::Path,
    process::Command,
};

fn main() {
    println!("cargo:rerun-if-changed=../client/");
    println!("cargo:rerun-if-changed=build.rs");
    let cargo = env::var("CARGO").unwrap();

    // cargo build client
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let client_dir = Path::new(&manifest_dir)
        .parent().expect("top dir")
        .join("client");

    let out_dir = env::var("OUT_DIR").unwrap();
    let wasm_target = Path::new(&out_dir).join("wasm-target");

    let _ = remove_dir_all(&wasm_target);
    let output = Command::new(cargo)
        .current_dir(client_dir.canonicalize().unwrap())
        .args(&[
            "build",
            "--target", "wasm32-unknown-unknown",
            "--target-dir", &wasm_target.to_str().unwrap(),
            "--release",
        ])
        .output()
        .expect("cargo build -p client --target wasm32-unknown-unknown");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());

    // wasm-bindgen
    let client_wasm = wasm_target.join("wasm32-unknown-unknown/release/client.wasm");
    let client_pkg = Path::new(&out_dir).join("pkg");
    let _ = remove_dir_all(&client_pkg);
    let output = Command::new("wasm-bindgen")
        .args(&[
            "--target", "web",
            "--out-dir", &client_pkg.to_str().unwrap(),
            &client_wasm.to_str().unwrap(),
        ])
        .output()
        .expect("wasm-bindgen client.wasm");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}
