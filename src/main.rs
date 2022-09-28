extern crate skim;

mod project;
mod tmux;
mod utils;

use crate::tmux::session::{connect, create_or_connect, session_exists};
use crate::utils::config::{load, Config};
use itertools::Itertools;
use rofi;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::process::Command;

fn main() {
    let config: Config = load("~/.config/projects.json");

    let projects_dir = shellexpand::tilde("~/.cache/tshort.json").to_string();

    let mut projects: HashMap<String, project::selector::Project> =
        match fs::read_to_string(projects_dir.to_owned()) {
            Ok(data) => serde_json::from_str(&data).expect("JSON does not have correct format."),
            Err(_) => HashMap::new(),
        };

    projects.retain(|_k, v| session_exists(&v.session_name().to_string()));

    if std::env::args().len() == 1 {
        let item = match project::selector::run(config) {
            Ok(item) => item,
            Err(_e) => return,
        };

        create_or_connect(item);
        return;
    }

    match std::env::args().nth(1).unwrap().as_str() {
        "gui" => {
            let mut project_list = Vec::new();
            for (group, dir) in config.directories().iter() {
                let expended_dir = shellexpand::tilde(dir);
                for file in fs::read_dir(expended_dir.to_string()).unwrap() {
                    project_list.push(
                        project::selector::Project::new(
                            file.unwrap().path().display().to_string(),
                            group.to_owned()
                        )
                    )
                }
            }
            let rofi_list = project_list.iter().map(|item| item.session_name().to_string()).collect::<Vec<String>>();
            match rofi::Rofi::new(&rofi_list)
                .theme(Some(String::from(
                    "/home/alpha/.config/rofi/launchers/type-1/style-1.rasi",
                )))
                .lines(15)
                .prompt("Projects")
                .run_index()
            {
                Ok(choice) => {
                    println!("Choice: {}", project_list[choice].path().display().to_string());
                    Command::new("sh")
                        .arg("-c")
                        .arg(format!("cd {} && neovide .", project_list[choice].path().display().to_string()))
                        .output()
                        .expect("failed to run");
                }
                Err(rofi::Error::Interrupted) => println!("Interrupted"),
                Err(e) => println!("Error: {}", e),
            }
        }
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
                }
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
        }
        "forget" => {
            let key = std::env::args()
                .nth(2)
                .expect("To use bind needs to provide a key");

            projects.remove(&key);

            serde_json::to_writer(&File::create(projects_dir).unwrap(), &projects).unwrap();
        }

        "list" => {
            let mut projects_names: Vec<String> = Vec::<String>::new();

            projects.keys().sorted().for_each(|key| {
                projects_names.push(format!(
                    "{} [{}]",
                    projects[key].session_name().to_string(),
                    key
                ));
            });

            println!("{}", projects_names.join("|"));
        }
        _ => {
            panic!("Parameter provided not valid");
        }
    };
}
