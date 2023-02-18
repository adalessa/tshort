extern crate skim;

mod actions;
mod application;
mod project;
mod projects;
mod selector;
mod tmux;
mod utils;

use clap::Parser;
use utils::config::Config;

use crate::application::{Args, Commands};

fn main() {
    let args = Args::parse();
    let config = Config::load(&args.config_file);

    match &args.command {
        Some(c) => match c {
            Commands::Bind { key } => actions::bind(key, config),
            Commands::Forget { key } => actions::forget(key),
            Commands::List => actions::list(),
            Commands::RemoveCache => actions::remove_cache(),
        },
        None => actions::default(config),
    }
}
