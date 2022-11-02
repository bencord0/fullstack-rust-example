use std::{
    env,
    fs::remove_dir_all,
    path::Path,
    process::{Command, Stdio},
};

fn main() {
    // No further processing for self-build
    // This prevents recursively building ourself
    if env::var("TARGET").unwrap() == "wasm32-unknown-unknown" {
        return;
    }

    // When to rerun the build
    println!("cargo:run-if-changed=src");
    println!("cargo:run-if-changed=build.rs");

    // Tools we use
    let cargo = env::var("CARGO").unwrap();
    let bindgen = "wasm-bindgen";

    // Where are we, where are we building
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_dir);
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    // Since cargo holds a lock on the `--target-dir`, we make a new one
    let target_dir = Path::new(&out_dir).join("wasm-target");
    //let _ = remove_dir_all(&target_dir);

    // $ cargo build -p client --target wasm32-unknown-unknown
    let status = Command::new(cargo)
        .current_dir(manifest_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(&[
            "build", "-vv",
            "--target", "wasm32-unknown-unknown",
            "--target-dir", &target_dir.to_string_lossy(),
            "--release",
        ])
        .status()
        .expect("cargo build -p client --target wasm32-unknown-unknown --release");

    assert!(status.success());

    // $ wasm-bindgen --target=web client.wasm
    let client_wasm = target_dir.join("wasm32-unknown-unknown/release/client.wasm");
    let client_pkg = out_dir.join("pkg");
    let _ = remove_dir_all(&client_pkg);

    let status = Command::new(bindgen)
        .current_dir(target_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(&[
            "--target", "web",
            "--out-dir", &client_pkg.to_string_lossy(),
            &client_wasm.to_string_lossy(),
        ])
        .status()
        .expect("wasm-bindgen --target=web client.wasm");

    assert!(status.success());
}
