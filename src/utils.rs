pub mod config {
    use serde::{Deserialize, Serialize};
    use std::fs;

    #[derive(Serialize, Deserialize)]
    pub struct ProjectConfig {
        name: String,
        directory: String,
        icon: String,
    }

    impl ProjectConfig {
        pub fn name(&self) -> &str {
            self.name.as_ref()
        }

        pub fn directory(&self) -> &str {
            self.directory.as_ref()
        }

        pub fn icon(&self) -> &str {
            self.icon.as_ref()
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct Config {
        projects: Vec<ProjectConfig>,
        menu: String,
    }

    impl Config {
        pub fn menu(&self) -> &str {
            self.menu.as_ref()
        }

        pub fn projects(&self) -> &[ProjectConfig] {
            self.projects.as_ref()
        }
    }

    pub fn load(path: &str) -> Config {
        let path = shellexpand::tilde(path).to_string();
        let data = fs::read_to_string(path).expect("Unable to read file");
        serde_json::from_str(&data).expect("JSON does not have correct format.")
    }
}
