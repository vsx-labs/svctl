use clap::{Parser, Subcommand};
use cli::Cli;
use std::path::PathBuf;
use svctl::dev::vsx::svctl::v1;

fn main() {
    let cli = Cli::parse();
    let config = v1::Config::default();

    // // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {name}");
    // }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Config { command }) => match &command {
            Some(ConfigCommands::Show {}) => {
                println!("config! {:#?}", config);
                println!("{:?}", config);
            }
            None => {}
        },
        None => {}
    }
}
