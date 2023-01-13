pub mod config {
    use serde::{Deserialize, Serialize};
    use std::{fs, io::Error};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ProjectConfig {
        name: String,
        directory: String,
        icon: Option<String>,
    }

    impl ProjectConfig {
        pub fn name(&self) -> &str {
            self.name.as_ref()
        }

        pub fn directory(&self) -> &str {
            self.directory.as_ref()
        }

        pub fn icon(&self) -> Option<&String> {
            self.icon.as_ref()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Config {
        pub projects: Vec<ProjectConfig>,
    }

    impl Config {
        pub fn load(path: &str) -> Self {
            let data = read_or_create_file(shellexpand::tilde(path).to_string().as_str(), "{\"projects\": []}").expect("Cant read nor create the file");
            serde_json::from_str(&data).expect("JSON does not have correct format.")
        }
    }

    fn read_or_create_file(file_path: &str, initial_contents: &str) -> Result<String, Error> {
        match fs::read_to_string(file_path) {
            Ok(contents) => Ok(contents),
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    fs::write(file_path, initial_contents)?;

                    fs::read_to_string(file_path)
                } else {
                    Err(err)
                }
            }
        }
    }
}
