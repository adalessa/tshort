extern crate skim;

mod commands;
mod application;
mod project;
mod projects;
mod selector;
mod tmux;
mod utils;

use clap::Parser;
use utils::config::Config;
use std::process;

use crate::application::{Args, Commands};
use commands::*;

fn main() {
    let args = Args::parse();
    let config = Config::load(&args.config_file);

    let exit_code = match args.command {
        Some(c) => match c {
            Commands::Bind { key } => BindCommand::new(key, config).handle(),
            Commands::Forget { key } => ForgetCommand::new(key).handle(),
            Commands::List => ListCommand::new().handle(),
            Commands::RemoveCache => RemoveCacheCommand::new().handle(),
        },
        None => DefaultCommand::new(config).handle(),
    };

    process::exit(exit_code);
}
