use {
    crate::exec::{execute_script, ExecError, ExecOptions},
    askama::Template,
    svctl_proto::dev::vsx::svctl::v1::MachineConfig,
};

// @see https://github.com/rinja-rs/askama/blob/main/book/src/configuration.md#custom-syntaxes
#[derive(Template)]
#[template(path = "watchtower.install.sh", escape = "none")]

struct InstallTemplate<'a> {
    username: &'a str,
}

pub fn install(options: &ExecOptions, _machine: Option<&MachineConfig>) -> Result<(), ExecError> {
    let ctx = InstallTemplate { username: "foo" };
    let script = ctx.render().expect("script rendering failed");
    let filename = "script.sh".to_string();
    let result = execute_script(options, filename, script);

    result.expect("script execution failed");

    Ok(())
}
