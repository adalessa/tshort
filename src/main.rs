extern crate skim;

mod project;
mod utils;
mod tmux;

use crate::utils::config::{Config, load};
use crate::tmux::session::create_or_connect;

fn main() {
    let config: Config = load("./config.json");

    let item = match project::selector::run(config) {
        Ok(item) => item,
        Err(_e) => return,
    };

    create_or_connect(&item.session_name().to_string());
}

// TODO
// differente from having session number
// problem same project sessions names
