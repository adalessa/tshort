pub mod rofi {
    use std::{fs, path::Path, process::Command};

    use crate::utils::config::Config;

    use rofi;

    struct Option {
        directory: String,
        icon: String,
    }

    impl Option {
        fn new(directory: String, icon: String) -> Self {
            Self { directory, icon }
        }

        fn label(&self) -> String {
            build_label(self.directory.to_owned(), self.icon.to_owned())
        }
    }

    fn base_name(dir: &String) -> &str {
        Path::new(dir)
            .file_name()
            .expect("Is not a directory")
            .to_str()
            .unwrap()
    }

    fn build_label(dir: String, icon: String) -> String {
        format!("{}\0icon\x1f{}", base_name(&dir), icon)
    }

    pub fn run(config: Config) {
        let mut project_list = Vec::new();
        for project in config.projects().iter() {
            let expended_dir = shellexpand::tilde(project.directory());
            for file in fs::read_dir(expended_dir.to_string()).unwrap() {
                let file = file.unwrap();
                if file.metadata().unwrap().is_dir() {
                    project_list.push(Option::new(
                        file.path().display().to_string(),
                        project.icon().to_owned(),
                    ))
                }
            }
        }

        let rofi_list: Vec<String> = project_list.iter().map(|item| item.label()).collect();

        match rofi::Rofi::new(&rofi_list)
            .theme(Some(shellexpand::tilde(config.gui().rofi_menu())))
            .set_sort()
            .lines(15)
            .prompt("Projects")
            .run_index()
        {
            Ok(choice) => {
                println!("Choice: {}", project_list[choice].directory);
                Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        "cd {} && {}",
                        project_list[choice].directory,
                        config.gui().editor(),
                    ))
                    .output()
                    .expect("failed to run");
            }
            Err(rofi::Error::Interrupted) => println!("Interrupted"),
            Err(e) => println!("Error: {}", e),
        }
    }
}
