use clap::Parser;
use svctl_cli::commands::{App, ConfigCommands, TopCommands};
use svctl_proto::dev::vsx::svctl::v1;

fn main() {
    let app = App::parse();
    let config = v1::Config::default();
    println!("Hello, world!: {:#?}", config);

    if let Some(config_path) = app.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &app.command {
        Some(TopCommands::Config { command }) => match &command {
            Some(ConfigCommands::Show {}) => {
                println!("config! {:#?}", config);
                println!("{:?}", config);
            }
            None => {}
        },
        None => {}
    }
}
