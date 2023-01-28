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
    let config = Config::load(args.config_file());

    match &args.command() {
        Some(Commands::Bind { key }) => actions::bind(key, config),
        Some(Commands::Forget { key }) => actions::forget(key),
        Some(Commands::List) => actions::list(),
        Some(Commands::RemoveCache) => actions::remove_cache(),
        None => actions::default(config),
    }
}
