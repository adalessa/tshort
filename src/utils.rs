pub mod config {
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, fs};

    #[derive(Serialize, Deserialize)]
    pub struct Config {
        directories: HashMap<String, String>,
    }

    impl Config {
        pub fn directories(&self) -> &HashMap<String, String> {
            &self.directories
        }
    }

    pub fn load(path: &str) -> Config {
        let path = shellexpand::tilde(path).to_string();
        let data = fs::read_to_string(path).expect("Unable to read file");
        serde_json::from_str(&data).expect("JSON does not have correct format.")
    }
}
