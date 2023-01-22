use std::{borrow::Cow, path::Path};

use raster::Color;
use serde::{Deserialize, Serialize};
use skim::{AnsiString, DisplayContext, SkimItem};

use crate::utils::config::ProjectConfig;

#[derive(Clone, Serialize, Deserialize)]
pub struct Project {
    path: String,
    group: String,
    icon: String,
    color: Option<String>,
}

impl Project {
    pub fn new(path: String, config: &ProjectConfig) -> Self {
        Self {
            path,
            group: config.name.clone(),
            icon: config.icon.clone().unwrap_or("".to_string()),
            color: config.color.clone(),
        }
    }

    pub fn session_name(&self) -> Cow<str> {
        let path = Path::new(&self.path);
        let path = path
            .file_name()
            .expect("Is not a directory")
            .to_str()
            .unwrap();

        Cow::from(format!(
            "[{}] {}",
            &self.group,
            str::replace(path, ".", "_")
        ))
    }

    pub fn tmux_display(&self) -> Cow<str> {
        match &self.color {
            Some(color) => Cow::from(format!(
                "#[fg={}]{}#[fg=default] {}",
                color,
                &self.icon,
                str::replace(self.get_path(), ".", "_")
            )),
            None => Cow::from(format!(
                "{}  {}",
                &self.icon,
                str::replace(self.get_path(), ".", "_")
            )),
        }
    }

    pub fn skim_display(&self) -> Cow<str> {
        match &self.color {
            Some(color) => {
                let color = Color::hex(color).unwrap();
                Cow::from(format!(
                    "\x1b[38;2;{};{};{}m{}\x1b[m {}",
                    &color.r.to_string(),
                    &color.g.to_string(),
                    &color.b.to_string(),
                    &self.icon,
                    str::replace(self.get_path(), ".", "_")
                ))
            }
            None => Cow::from(format!(
                "{}  {}",
                &self.icon,
                str::replace(self.get_path(), ".", "_")
            )),
        }
    }

    pub fn skim_text(&self) -> Cow<str> {
        Cow::from(format!("{}", self.session_name()))
    }

    pub fn path(&self) -> &Path {
        Path::new(&self.path)
    }

    fn get_path(&self) -> &str {
        let path = Path::new(&self.path);
        let path = path
            .file_name()
            .expect("Is not a directory")
            .to_str()
            .unwrap();
        return path;
    }
}

impl SkimItem for Project {
    fn text(&self) -> Cow<str> {
        Cow::from(self.skim_text())
    }

    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
        // AnsiString::new_string("\x1b[31mhello:\x1b[m\n".to_string(), vec!())
        AnsiString::parse(&self.skim_display())
    }
}
