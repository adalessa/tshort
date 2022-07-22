extern crate skim;
use serde::{Deserialize, Serialize};
use skim::prelude::*;
use std::collections::HashMap;
use std::fs;
use tmux_interface::TmuxCommand;

#[derive(Serialize, Deserialize)]
struct Config {
    directories: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct Project {
    path: String,
    group: String,
    group_dir: String,
}

impl Project {
    fn session_name(&self) -> Cow<str> {
        Cow::Borrowed(&self.path)
    }
}

impl SkimItem for Project {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.path)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(self.path.to_owned())
    }
}

fn main() {
    let data = fs::read_to_string("./config.json").expect("Unable to read file");

    let config: Config = serde_json::from_str(&data).expect("JSON does not have correct format.");

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for (group, dir) in config.directories.iter() {
        let expanded_dir = shellexpand::tilde(dir);
        for file in fs::read_dir(expanded_dir.to_string()).unwrap() {
            tx.send(Arc::new(Project {
                path: file.unwrap().path().display().to_string(),
                group: group.to_owned(),
                group_dir: dir.to_owned(),
            }))
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
        println!("Nothing selected");
        return;
    }

    let item = selected.unwrap();
    println!(
        "Group {}, Group Dir {}, Path {}",
        item.group, item.group_dir, item.path
    );

    let tmux = TmuxCommand::new();

    let has_session = tmux
        .has_session()
        .target_session(item.session_name())
        .output()
        .unwrap()
        .success();

    if !has_session {
        tmux.new_session()
            .detached()
            .session_name(item.session_name())
            .output()
            .unwrap();
    }

    tmux.switch_client()
        .target_session(item.session_name())
        .output()
        .unwrap();
}
