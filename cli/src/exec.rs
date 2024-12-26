// use run_script::run_script;
use std::{
    ffi::OsStr,
    fmt,
    io::{self, Error},
    process::{self, Command},
};

use crate::highlighter::SyntaxHighlighter;

const IS_QUIET: bool = false;

/// A command execution error.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecError {
    exit_code: i32,
    output: String,
    error: String,
}

impl ExecError {
    // /// Creates a new `ExecError`.
    // pub(crate) fn new(exit_code: i32, output: String, error: String) -> ExecError {
    //     ExecError {
    //         exit_code,
    //         output,
    //         error,
    //     }
    // }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    pub fn output(&self) -> String {
        return self.output.clone(); // FIXME
    }

    pub fn error(&self) -> String {
        return self.error.clone(); // FIXME
    }
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to execute (exit_code: {}, output: {}, error: {})",
            self.exit_code, self.output, self.error,
        )
    }
}

impl std::error::Error for ExecError {}

impl From<ExecError> for std::io::Error {
    fn from(error: ExecError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, error)
    }
}

// pub fn execute_shell() -> Result<(), ExecError> {
//     return Ok(());
// }

pub struct ExecOptions {
    pub highlight: bool,
    pub dry_run: bool,
    pub silent: bool,
}

pub fn execute_script(
    options: &ExecOptions,
    _filename: String,
    script: String,
) -> Result<(), Error> {
    if !options.silent {
        if options.highlight {
            let hl = SyntaxHighlighter::new();
            print!("{}\n", hl.format("sh", &script));
        } else {
            print!("{}\n", script);
        }
    }

    if options.dry_run {
        print!("\nNOTE: --dry_run is ON, skipping script execution");
        return Ok(());
    }

    let output = Command::new("bash")
        .arg("-c")
        .arg(script)
        .stdout(std::process::Stdio::inherit())
        // .stdout(std::process::Stdio::piped())
        // .stderr(std::process::Stdio::piped())
        .spawn()?;

    let _output = output.wait_with_output()?;

    // if true {
    //     io::stdout().write_all(&output.stdout)?;
    //     io::stderr().write_all(&output.stderr)?;
    // }

    Ok(())

    // let (exit_code, output, error) = run_script!(script).unwrap();
    // if exit_code == 0 {
    //     return Ok(output);
    // }
    // return Err(ExecError {
    //     exit_code,
    //     output,
    //     error,
    // });
}

// from https://github.com/cosmos/cosmos-rust/blob/4604ae9ba67f50b8bf5b8e3e9f775a4c81943644/proto-build/src/main.rs#L113
#[allow(dead_code)]
fn run_cmd(cmd: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if IS_QUIET {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = process::Command::new(&cmd)
        .args(args)
        .stdout(stdout)
        .status()
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => panic!(
                "error running '{:?}': command not found. Is it installed?",
                cmd.as_ref()
            ),
            _ => panic!("error running '{:?}': {:?}", cmd.as_ref(), e),
        });

    if !exit_status.success() {
        match exit_status.code() {
            Some(code) => panic!("{:?} exited with error code: {:?}", cmd.as_ref(), code),
            None => panic!("{:?} exited without error code", cmd.as_ref()),
        }
    }
}
