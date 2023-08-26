use itertools::Itertools;

use crate::{projects, selector, tmux::session, utils::config::Config};

pub trait Command {
    fn handle(&self) -> i32;
}

pub struct BindCommand {
    key: Option<String>,
    config: Config,
}

impl BindCommand {
    pub fn new(key: Option<String>, config: Config) -> Self {
        Self { key, config }
    }
}

impl Command for BindCommand {
    fn handle(&self) -> i32 {
        let key = match &self.key {
            Some(key) => key,
            None => {
                println!("No key provided");
                return 1
            },
        };

        let mut projects = projects::get();
        match projects.get(key) {
            Some(item) => {
                let success = session::connect(&item.tmux_display());
                if !success {
                    let item = match selector::run(&self.config) {
                        Ok(item) => item,
                        Err(_e) => return 1,
                    };

                    let success = session::connect_or_create(item.to_owned());
                    if !success {
                        panic!("Error creating tmux session")
                    }
                    projects.insert(key.clone(), item);
                }
            }
            None => {
                let item = match selector::run(&self.config) {
                    Ok(item) => item,
                    Err(_e) => return 1,
                };

                let success = session::connect_or_create(item.to_owned());
                if !success {
                    panic!("Error creating tmux session")
                }

                projects.insert(key.clone(), item);
            }
        };

        projects::save(projects);

        0
    }
}

pub struct ForgetCommand {
    key: Option<String>,
}

impl ForgetCommand {
    pub fn new(key: Option<String>) -> Self {
        Self { key }
    }
}

impl Command for ForgetCommand {
    fn handle(&self) -> i32 {
        let key = match &self.key {
            Some(key) => key,
            None => {
                println!("No key provided");
                return 1
            },
        };
        projects::remove_item(key);

        0
    }
}

pub struct ListCommand {}

impl ListCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for ListCommand {
    fn handle(&self) -> i32 {
        let projects = projects::get();
        let mut projects_names: Vec<String> = Vec::<String>::new();

        projects.keys().sorted().for_each(|key| {
            projects_names.push(format!(
                "{} #[fg=#212E3E]#{}#[fg=default]",
                projects[key].tmux_display(),
                key
            ));
        });

        println!("{}", projects_names.join(" "));

        0
    }
}

pub struct RemoveCacheCommand {}

impl RemoveCacheCommand {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for RemoveCacheCommand {
    fn handle(&self) -> i32 {
        match projects::remove() {
            Ok(_) => 0,
            Err(err) => panic!("{}", err),
        }
    }
}

pub struct DefaultCommand {
    config: Config,
}

impl DefaultCommand {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Command for DefaultCommand {
    fn handle(&self) -> i32 {
        let item = match selector::run(&self.config) {
            Ok(item) => item,
            Err(_e) => return 1,
        };

        session::connect_or_create(item);

        0
    }
}
