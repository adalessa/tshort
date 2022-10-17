pub mod selector {
    use skim::prelude::*;
    use std::{
        borrow::Cow,
        fs,
        io::{self, Error, ErrorKind},
        path::Path,
    };

    use crate::utils::config::Config;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    pub struct Project {
        path: String,
        group: String,
    }

    impl Project {
        pub fn new(path: String, group: String) -> Self {
            Self { path, group }
        }

        pub fn session_name(&self) -> Cow<str> {
            let path = Path::new(&self.path);
            let path = path
                .file_name()
                .expect("Is not a directory")
                .to_str()
                .unwrap();

            Cow::from(format!("[{}] {}", &self.group, str::replace(path, ".", "_")))
        }

        pub fn path(&self) -> &Path {
            Path::new(&self.path)
        }
    }

    impl SkimItem for Project {
        fn text(&self) -> Cow<str> {
            self.session_name()
        }

        fn preview(&self, _context: PreviewContext) -> ItemPreview {
            ItemPreview::Text(self.path.to_owned())
        }
    }

    pub fn run(config: Config) -> Result<Project, io::Error> {
        let options = SkimOptionsBuilder::default()
            .height(Some("100%"))
            .multi(false)
            .build()
            .unwrap();

        let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

        for project in config.projects().iter() {
            let expanded_dir = shellexpand::tilde(project.directory());
            for file in fs::read_dir(expanded_dir.to_string()).unwrap() {
                let file = file.unwrap();
                if file.metadata().unwrap().is_dir() {
                    tx.send(Arc::new(Project::new(
                        file.path().display().to_string(),
                        project.name().to_string(),
                    )))
                    .unwrap()
                }
            }
        }
        drop(tx);

        let selected_items = Skim::run_with(&options, Some(rx))
            .map(|out| match out.final_key {
                Key::ESC => Vec::new(),
                Key::Enter => out.selected_items,
                _ => Vec::new(),
            })
            .unwrap_or_else(|| Vec::new())
            .iter()
            .map(|selected_item| {
                (**selected_item)
                    .as_any()
                    .downcast_ref::<Project>()
                    .unwrap()
                    .to_owned()
            })
            .collect::<Vec<Project>>();

        let selected = selected_items.iter().next();
        if !selected.is_some() {
            let error = Error::new(ErrorKind::Other, "Nothing selected");
            return Err(error);
        }

        Ok(selected.unwrap().clone())
    }
}
