use std::path::PathBuf;
// use svctl_proto::dev::vsx::svctl::v1;

use clap::{Parser, Subcommand};

#[derive(Default, Clone, Debug)]
pub struct ComputedValues {
    // put derived configuration here
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct App {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", env = "SVCTL_CONFIG")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<TopCommands>,

    #[arg(skip)]
    pub computed: ComputedValues,
}

#[derive(Subcommand)]
pub enum TopCommands {
    /// configuration-related subcommands
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },
    User {
        #[command(subcommand)]
        command: Option<UserCommands>,
    },
    Watchtower {
        #[command(subcommand)]
        command: Option<WatchtowerCommands>,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// show the current config
    Show {},
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// show the current user
    Show {},
    /// install as sudo
    Install {},
}

#[derive(Subcommand)]
pub enum WatchtowerCommands {
    /// install watchtower
    Install {},
}
