extern crate skim;
use tmux_interface::TmuxCommand;

mod project;
mod utils;

use crate::utils::config::{Config, load};

fn main() {
    let config: Config = load("./config.json");

    let item = match project::selector::run(config) {
        Ok(item) => item,
        Err(_e) => return,
    };

    let tmux = TmuxCommand::new();

    let has_session = tmux
        .has_session()
        .target_session(item.session_name())
        .output()
        .unwrap()
        .success();

    if !has_session {
        tmux.new_session()
            .detached()
            .session_name(item.session_name())
            .output()
            .unwrap();
    }

    tmux.switch_client()
        .target_session(item.session_name())
        .output()
        .unwrap();
}

// TEST
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
