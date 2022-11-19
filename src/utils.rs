pub mod config {
    use serde::{Deserialize, Serialize};
    use std::fs;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ProjectConfig {
        name: String,
        directory: String,
        icon: String,
        term_icon: String,
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

        pub fn term_icon(&self) -> &str {
            self.term_icon.as_ref()
        }
}

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Gui {
        rofi_menu: String,
        editor: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Cli {
        editor: String,
    }

    impl Cli {
        pub fn editor(&self) -> &str {
            self.editor.as_ref()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Config {
        projects: Vec<ProjectConfig>,
        gui: Gui,
        cli: Cli,
        cache: Option<String>,
    }

    impl Config {
        pub fn projects(&self) -> &[ProjectConfig] {
            self.projects.as_ref()
        }

        pub fn gui(&self) -> &Gui {
            &self.gui
        }

        pub fn cli(&self) -> &Cli {
            &self.cli
        }

        pub fn cache(&self) -> Option<&String> {
            self.cache.as_ref()
        }

        pub fn load(path: &str) -> Self {
            let data = fs::read_to_string(shellexpand::tilde(path).to_string())
                .expect("Unable to read file");
            serde_json::from_str(&data).expect("JSON does not have correct format.")
        }
    }

    impl Gui {
        pub fn rofi_menu(&self) -> &str {
            self.rofi_menu.as_ref()
        }

        pub fn editor(&self) -> &str {
            self.editor.as_ref()
        }
    }
}
