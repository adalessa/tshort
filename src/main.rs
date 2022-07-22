extern crate skim;
use serde::{Deserialize, Serialize};
use skim::prelude::*;
use std::collections::HashMap;
use std::fs;

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
            // vec.push(format!("[{}] {}", name, file.unwrap().path().display()));
            // println!("in directory {} is the folder {}", dir, file.unwrap().path().display());
        }
    }
    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
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

    for item in selected_items {
        println!("Group {}, Group Dir {}, Path {}", item.group, item.group_dir, item.path);
        dbg!(item);
    }

    // if selected_items.len() > 1 {
    //     panic!("expected only one result")
    // } else if selected_items.len() == 0 {
    //     println!("No item selected");
    // } else {
    //     println!("Group {}, Group Dir {}, Path {}", selected_items[0].group, selected_items[0].group_dir, selected_items[0].path)
    // }
}
