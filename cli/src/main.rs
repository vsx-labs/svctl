use std::{ffi::OsString, io::IsTerminal};

use clap::Parser;
use svctl_cli::{
    commands::{App, ConfigCommands, TopCommands, UserCommands, WatchtowerCommands},
    exec::ExecOptions,
    user, watchtower,
};
use svctl_proto::{get_default_config, read_config};

fn main() {
    let args = vec![
        OsString::from("svctl"),
        OsString::from("watchtower"),
        OsString::from("install"),
    ];

    let app = App::parse_from(args);
    let mut config = get_default_config();

    if let Some(config_path) = app.config.as_deref() {
        config = read_config(config_path).unwrap_or_else(|err| panic!("invalid config: {err}"));
    }

    let exec_options = ExecOptions {
        dry_run: false,
        highlight: std::io::stdout().is_terminal(),
        silent: false,
    };

    let _ = match &app.command {
        Some(TopCommands::Config { command }) => match &command {
            Some(ConfigCommands::Show {}) => {
                println!("config! {:#?}", config);
                println!("{:?}", config);
            }
            None => {}
        },
        Some(TopCommands::User { command }) => match &command {
            Some(UserCommands::Show {}) => {
                println!("config! {:#?}", config);
                println!("{:?}", config);
            }
            Some(UserCommands::Install {}) => {
                let _ = user::install(&exec_options, config.user);
            }
            None => {}
        },
        Some(TopCommands::Watchtower { command }) => match &command {
            Some(WatchtowerCommands::Install {}) => {
                watchtower::install(&exec_options, config.machines.get(0))
                    .unwrap_or_else(|err| panic!("failed: {:#?}", err));
            }
            None => {}
        },
        None => {}
    };
}
