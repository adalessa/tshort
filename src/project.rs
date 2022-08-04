pub mod selector {
    use skim::prelude::*;
    use std::{
        borrow::Cow,
        fs,
        io::{self, Error, ErrorKind},
        path::Path,
    };

    use crate::utils::config::Config;

    #[derive(Clone)]
    pub struct Project {
        path: String,
        group: String,
    }

    impl Project {
        pub fn new(path: String, group: String) -> Self {
            Self {
                path,
                group,
            }
        }

        pub fn session_name(&self) -> Cow<str> {
            let path = Path::new(&self.path);
            Cow::from(path.file_name().expect("Is not a directory").to_str().unwrap())
        }
    }

    impl SkimItem for Project {
        fn text(&self) -> Cow<str> {
            Cow::from(format!("[{}] {}", &self.group, &self.session_name()))
        }

        fn preview(&self, _context: PreviewContext) -> ItemPreview {
            ItemPreview::Text(self.path.to_owned())
        }
    }

    pub fn run(config: Config) -> Result<Project, io::Error> {
        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .multi(false)
            .build()
            .unwrap();

        let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

        for (group, dir) in config.directories().iter() {
            let expanded_dir = shellexpand::tilde(dir);
            for file in fs::read_dir(expanded_dir.to_string()).unwrap() {
                tx.send(Arc::new(Project::new(
                    file.unwrap().path().display().to_string(),
                    group.to_owned(),
                )))
                .unwrap()
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
