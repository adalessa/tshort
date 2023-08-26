use std::{
    collections::HashMap,
    fs::{self, File},
};

use crate::{project::Project, tmux::session};

const CACHE_FILE: &str = "~/.cache/tshort.json";

pub fn get() -> HashMap<String, Project> {
    let mut projects: HashMap<String, Project> = match fs::read_to_string(get_cache_file_path()) {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(data) => data,
            Err(_) => HashMap::new(),
        },
        Err(_) => HashMap::new(),
    };
    projects.retain(|_k, v| session::exists(&v.tmux_display()));
    projects
}

pub fn save(projects: HashMap<String, Project>) {
    serde_json::to_writer(&File::create(get_cache_file_path()).unwrap(), &projects).unwrap();
}

pub fn remove_item(key: &str) {
    let mut projects = get();
    projects.remove(key);
    save(projects)
}

pub fn remove() -> std::io::Result<()> {
    fs::remove_file(get_cache_file_path())
}

fn get_cache_file_path() -> String {
    shellexpand::tilde(CACHE_FILE).to_string()
}
