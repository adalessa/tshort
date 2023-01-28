use itertools::Itertools;

use crate::{projects, selector, tmux::session, utils::config::Config};

pub fn bind(key: &Option<String>, config: Config) {
    let mut projects = projects::get();
    let key = match key {
        Some(key) => key,
        None => panic!("No key provided"),
    };
    match projects.get(key.as_str()) {
        Some(item) => {
            let success = session::connect(&item.tmux_display());
            if !success {
                let item = match selector::run(config) {
                    Ok(item) => item,
                    Err(_e) => return,
                };

                let success = session::connect_or_create(item.to_owned());
                if !success {
                    panic!("Error creating tmux session")
                }

                projects.insert(key.to_string(), item);
            }
        }
        _ => {
            let item = match selector::run(config) {
                Ok(item) => item,
                Err(_e) => return,
            };

            let success = session::connect_or_create(item.to_owned());
            if !success {
                panic!("Error creating tmux session")
            }

            projects.insert(key.to_string(), item);
        }
    };

    projects::save(projects)
}

pub fn forget(key: &Option<String>) {
    let key = match key {
        Some(key) => key,
        None => panic!("No key provided"),
    };
    let mut projects = projects::get();
    projects.remove(key.as_str());
    projects::save(projects)
}

pub fn list() {
    let projects = projects::get();
    let mut projects_names: Vec<String> = Vec::<String>::new();

    projects.keys().sorted().for_each(|key| {
        projects_names.push(format!(
            "{} <{}>",
            projects[key].tmux_display(),
            key
        ));
    });

    println!("{}", projects_names.join(" "));
}

pub fn default(config: Config) {
    let item = match selector::run(config) {
        Ok(item) => item,
        Err(_e) => return,
    };

    session::connect_or_create(item);
}

pub fn remove_cache() {
    match projects::remove() {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
}
