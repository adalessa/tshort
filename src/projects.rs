use std::{collections::HashMap, fs::{self, File}};

use crate::{project::Project, tmux::session};

const CACHE_FILE: &str = "~/.cache/tshort.json";

pub fn get() -> HashMap<String, Project> {
    let mut projects: HashMap<String, Project> = match fs::read_to_string(get_cache_file_path()) {
        Ok(data) => serde_json::from_str(&data).expect("JSON does not have correct format."),
        Err(_) => HashMap::new(),
    };
    projects.retain(|_k, v| session::exists(&v.tmux_display()));
    projects
}

pub fn save(projects: HashMap<String, Project>) {
    serde_json::to_writer(&File::create(get_cache_file_path()).unwrap(), &projects).unwrap();
}

pub fn remove() -> std::io::Result<()>{
    fs::remove_file(get_cache_file_path())?;
    Ok(())
}

fn get_cache_file_path() -> String {
    shellexpand::tilde(CACHE_FILE).to_string()
}
