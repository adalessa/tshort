extern crate skim;

mod project;
mod tmux;
mod utils;

use crate::tmux::session::{connect, create_or_connect};
use crate::utils::config::{load, Config};
use std::collections::HashMap;
use std::fs;
use std::fs::File;

fn main() {
    let config: Config = load("~/.config/projects.json");

    let projects_dir = shellexpand::tilde("~/.cache/tshort.json").to_string();

    let mut projects: HashMap<String, project::selector::Project> = match fs::read_to_string(projects_dir.to_owned()) {
        Ok(data) => serde_json::from_str(&data).expect("JSON does not have correct format."),
        Err(_) => HashMap::new(),
    };


    match std::env::args().nth(1).unwrap().as_str() {
        "bind" => {
            let key = std::env::args()
                .nth(2)
                .expect("To use bind needs to provide a key");

            match projects.get(&key) {
                Some(item) => {
                    let success = connect(&item.session_name().to_string());
                    if !success {
                        let item = match project::selector::run(config) {
                            Ok(item) => item,
                            Err(_e) => return,
                        };

                        let success = create_or_connect(item.to_owned());
                        if !success {
                            panic!("Error creating tmux session")
                        }

                        projects.insert(key, item);
                    }
                },
                _ => {
                    let item = match project::selector::run(config) {
                        Ok(item) => item,
                        Err(_e) => return,
                    };

                    let success = create_or_connect(item.to_owned());
                    if !success {
                        panic!("Error creating tmux session")
                    }

                    projects.insert(key, item);
                }
            };

            serde_json::to_writer(&File::create(projects_dir).unwrap(), &projects).unwrap();
        },
        "forget" => {
            let key = std::env::args()
                .nth(2)
                .expect("To use bind needs to provide a key");

            projects.remove(&key);

            serde_json::to_writer(&File::create(projects_dir).unwrap(), &projects).unwrap();
        },

        "list" => {
            let mut projects_names: Vec<String> = Vec::<String>::new();
            for val in projects.values() {
                projects_names.push(val.session_name().to_string());
            }

            println!("{}", projects_names.join("|"));
        },
        _ => {
            let item = match project::selector::run(config) {
                Ok(item) => item,
                Err(_e) => return,
            };

            create_or_connect(item);
        },
    };
}
