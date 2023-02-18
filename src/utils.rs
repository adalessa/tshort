pub mod config {
    use serde::{Deserialize, Serialize};
    use std::{fs, io::Error};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ProjectConfig {
        pub name: String,
        pub directory: String,
        pub icon: Option<String>,
        pub color: Option<String>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Config {
        pub directories: Vec<ProjectConfig>,
        pub projects: Vec<ProjectConfig>,
    }

    impl Config {
        pub fn load(path: &str) -> Self {
            let data = find_or_new(
                shellexpand::tilde(path).into_owned().as_str(),
                "{\"projects\": []}",
            )
            .expect("Cant read nor create the file");
            serde_json::from_str(&data).expect("JSON does not have correct format.")
        }
    }

    fn find_or_new(file_path: &str, initial_contents: &str) -> Result<String, Error> {
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
