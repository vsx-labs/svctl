use std::{
    env,
    error::Error,
    process::{exit, Command},
};

fn main() -> Result<(), Box<dyn Error>> {
    let status = Command::new("buf")
        .arg("generate")
        // .arg("--debug")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .unwrap();

    if !status.success() {
        exit(status.code().unwrap_or(-1))
    }

    Ok(())
}
