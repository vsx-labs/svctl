use svctl_proto::dev::vsx::svctl::v1::UserConfig;
use svctl_template::must_get_template_file;
use thiserror::Error;

use crate::exec::{execute_script, ExecError, ExecOptions};

#[derive(Error, Debug)]
pub enum UserError {
    #[error("{0:?} user command failed")]
    ComandFailed(UserConfig),
}

pub fn install(options: &ExecOptions, user: Option<UserConfig>) -> Result<(), ExecError> {
    print!("installing {:?}!", user);
    let scriptname = String::from("user/install/install.sh.tmpl");
    let filename = scriptname.clone();
    let file = must_get_template_file(scriptname);
    let script = String::from_utf8(file.content).unwrap(); // FIXME

    execute_script(options, filename, script).expect("install failed");

    Ok(())
}
