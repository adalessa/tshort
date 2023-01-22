use skim::prelude::*;
use std::{
    fs,
    io::{self, Error, ErrorKind},
};

use crate::{utils::config::Config, project::Project};

pub fn run(config: Config) -> Result<Project, io::Error> {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(false)
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for project in config.projects.iter() {
        let expanded_dir = shellexpand::tilde(project.directory());
        for file in fs::read_dir(expanded_dir.to_string()).unwrap() {
            let file = file.unwrap();
            if file.metadata().unwrap().is_dir() {
                tx.send(Arc::new(Project::new(
                    file.path().display().to_string(),
                    project,
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
