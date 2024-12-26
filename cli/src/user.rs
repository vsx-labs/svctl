use {
    super::exec::{execute_script, ExecError, ExecOptions},
    askama::Template,
    svctl_proto::dev::vsx::svctl::v1::UserConfig,
};

// @see https://github.com/rinja-rs/askama/blob/main/book/src/configuration.md#custom-syntaxes
#[derive(Template)]
#[template(path = "user.install.sh", escape = "none")]

struct InstallTemplate<'a> {
    username: &'a str,
}

pub fn install(options: &ExecOptions, user: &UserConfig) -> Result<(), ExecError> {
    let template = InstallTemplate {
        username: &user.name,
    };
    let script = template.render().expect("template render failed");
    let result = execute_script(options, script);

    result.expect("script execution failed");

    Ok(())
}
