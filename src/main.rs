extern crate skim;

mod gui;
mod project;
mod tmux;
mod utils;

use crate::gui::rofi;
use crate::tmux::session::SessionManager;
use crate::utils::config::Config;
use clap::{Parser, Subcommand};
use itertools::Itertools;
use project::selector::Project;
use std::collections::HashMap;
use std::fs;
use std::fs::File;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("~/.config/projects.json"))]
    config: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Gui,
    Bind { key: Option<String> },
    Forget { key: Option<String> },
    List,
}

fn main() {
    let args = Args::parse();

    let config: Config = Config::load(&args.config);
    let editor = config.cli().editor().to_string();
    let session = SessionManager::new(editor);

    let cache_path = match config.cache() {
        Some(path) => path,
        None => "~/.cache/tshort.json",
    };

    let projects_dir = shellexpand::tilde(cache_path).to_string();

    let mut projects = get_projects(&projects_dir, &session);

    match &args.command {
        Some(Commands::Gui) => {
            rofi::run(config);
        },
        Some(Commands::Bind { key }) => {
            let key = match key {
                Some(key) => key.as_str(),
                None => panic!("No key provided"),
            };
            match projects.get(key) {
                Some(item) => {
                    let success = session.connect(&item.session_name().to_string());
                    if !success {
                        let item = match project::selector::run(config) {
                            Ok(item) => item,
                            Err(_e) => return,
                        };

                        let success = session.create_or_connect(item.to_owned());
                        if !success {
                            panic!("Error creating tmux session")
                        }

                        projects.insert(key.to_string(), item);
                    }
                }
                _ => {
                    let item = match project::selector::run(config) {
                        Ok(item) => item,
                        Err(_e) => return,
                    };

                    let success = session.create_or_connect(item.to_owned());
                    if !success {
                        panic!("Error creating tmux session")
                    }

                    projects.insert(key.to_string(), item);
                }
            };

            serde_json::to_writer(&File::create(projects_dir).unwrap(), &projects).unwrap();
        },
        Some(Commands::Forget { key }) => {
            let key = match key {
                Some(key) => key.as_str(),
                None => panic!("No key provided"),
            };
            projects.remove(key);

            serde_json::to_writer(&File::create(projects_dir).unwrap(), &projects).unwrap();
        },
        Some(Commands::List) => {
            let mut projects_names: Vec<String> = Vec::<String>::new();

            projects.keys().sorted().for_each(|key| {
                projects_names.push(format!(
                    "{} [{}]",
                    projects[key].display_name().to_string(),
                    key
                ));
            });

            println!("{}", projects_names.join("|"));
        },
        None => {
            let item = match project::selector::run(config) {
                Ok(item) => item,
                Err(_e) => return,
            };

            session.create_or_connect(item);
        },
    };
}

fn get_projects(projects_dir: &String, session: &SessionManager) -> HashMap<String, Project> {
    let mut projects: HashMap<String, Project> = match fs::read_to_string(projects_dir.to_owned()) {
        Ok(data) => serde_json::from_str(&data).expect("JSON does not have correct format."),
        Err(_) => HashMap::new(),
    };
    projects.retain(|_k, v| session.session_exists(&v.session_name().to_string()));
    projects
}
