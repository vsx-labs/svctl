// use std::io::Result;
// use protobuf_src;

// fn main() -> Result<()> {
//     prost_build::compile_protos(&["src/config.proto"], &["src/"])?;
//     Ok(())
// }

use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::{exit, Command};

fn main() -> Result<(), Box<dyn Error>> {
    // let protoc_path = protobuf_src::protoc();
    // env::set_var("PROTOC", protoc_path);

    // Get the path to the protoc-gen-prost binary in the cargo target directory.
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // let cargo_pkg_name = env::var("CARGO_PKG_NAME").unwrap();

    // Assuming a debug build for simplicity, adjust if needed for release builds
    let protoc_gen_prost_path = PathBuf::from(cargo_manifest_dir)
        .join("..")
        .join("target")
        .join("debug") // Change to "release" if doing a release build
        .join("protoc-gen-prost");

    // Add the directory containing protoc-gen-prost to PATH
    let path_var = env::var("PATH").unwrap_or_default();
    let new_path = format!(
        "{}{}{}",
        protoc_gen_prost_path.parent().unwrap().to_str().unwrap(),
        if cfg!(windows) { ";" } else { ":" },
        path_var
    );
    // env::set_var("PATH", new_path);
    // let buf_path = new_path.clone();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto/dev/vsx/svctl/v1/config.proto");
    let status = Command::new("buf")
        .arg("generate")
        .arg("--debug")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("PATH", new_path)
        .status()
        .unwrap();

    if !status.success() {
        exit(status.code().unwrap_or(-1))
    }

    Ok(())
}
