use std::path::PathBuf;
// use svctl_proto::dev::vsx::svctl::v1;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct App {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", env = "SVCTL_CONFIG")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<TopCommands>,
}

#[derive(Subcommand)]
pub enum TopCommands {
    /// configuration-related subcommands
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// show the current config
    Show {},
}
