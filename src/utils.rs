pub mod config {
    use serde::{Deserialize, Serialize};
    use std::fs;

    #[derive(Serialize, Deserialize, Clone)]
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
    }

    impl Gui {
        pub fn rofi_menu(&self) -> &str {
            self.rofi_menu.as_ref()
        }

        pub fn editor(&self) -> &str {
            self.editor.as_ref()
        }
    }

    pub fn load(path: &str) -> Config {
        let path = shellexpand::tilde(path).to_string();
        let data = fs::read_to_string(path).expect("Unable to read file");
        serde_json::from_str(&data).expect("JSON does not have correct format.")
    }
}
