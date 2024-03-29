use skim::prelude::*;
use std::fs;

use crate::{project::Project, utils::config::Config};

pub fn run(config: &Config) -> Option<Project> {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(false)
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for project in config.directories.iter() {
        let expanded_dir = shellexpand::tilde(&project.directory);
        for file in fs::read_dir(expanded_dir.to_string()).unwrap() {
            let file = file.unwrap();
            if file.metadata().unwrap().is_dir() {
                tx.send(Arc::new(Project {
                    path: file.path().display().to_string(),
                    name: file.file_name().into_string().unwrap(),
                    group: Some(project.name.clone()),
                    icon: project.icon.clone(),
                    color: project.color.clone(),
                }))
                .unwrap()
            }
        }
    }
    for project in config.projects.iter() {
        let dir = shellexpand::tilde(&project.directory);
        tx.send(Arc::new(Project {
            path: dir.into_owned(),
            name: project.name.clone(),
            group: None,
            icon: project.icon.clone(),
            color: project.color.clone(),
        }))
        .unwrap();
    }
    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| match out.final_key {
            Key::ESC => Vec::new(),
            Key::Enter => out.selected_items,
            _ => Vec::new(),
        })
        .unwrap_or_default()
        .iter()
        .map(|selected_item| {
            (**selected_item)
                .as_any()
                .downcast_ref::<Project>()
                .unwrap()
                .to_owned()
        })
        .collect::<Vec<Project>>();

    let selected = selected_items.first();
    selected?;

    Some(selected.unwrap().clone())
}
