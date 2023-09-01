use std::{borrow::Cow, path::Path};

use raster::Color;
use serde::{Deserialize, Serialize};
use skim::prelude::*;
use skim::{AnsiString, DisplayContext, SkimItem};

#[derive(Clone, Serialize, Deserialize)]
pub struct Project {
    pub path: String,
    pub name: String,
    pub group: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

impl Project {
    pub fn session_name(&self) -> Cow<str> {
        Cow::from(format!(
            "[{}] {}",
            self.group.as_ref().unwrap_or(&"".to_string()),
            str::replace(&self.name, ".", "_")
        ))
    }

    pub fn tmux_display(&self) -> Cow<str> {
        match &self.color {
            Some(color) => Cow::from(format!(
                "#[fg={}]{}#[fg=default] {}",
                color,
                self.icon.as_ref().unwrap(),
                str::replace(&self.name, ".", "_")
            )),
            None => Cow::from(format!(
                "{}  {}",
                self.icon.as_ref().unwrap(),
                str::replace(&self.name, ".", "_")
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
                    self.icon.as_ref().unwrap_or(&"".to_string()),
                    str::replace(&self.name, ".", "_")
                ))
            }
            None => Cow::from(format!(
                "{}  {}",
                self.icon.as_ref().unwrap_or(&"".to_string()),
                str::replace(&self.name, ".", "_")
            )),
        }
    }

    pub fn skim_text(&self) -> Cow<str> {
        Cow::from(format!("{}", self.session_name()))
    }

    pub fn path(&self) -> &Path {
        Path::new(&self.path)
    }
}

impl SkimItem for Project {
    fn text(&self) -> Cow<str> {
        self.skim_text()
    }

    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::parse(&self.skim_display())
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        let possible_files = vec![
            "README.md",
            "readme.md",
            "Readme.md",
            "README.MD",
            "Readme.MD",
            "ReadMe.md",
            "ReadMe.MD",
        ];

        for file in &possible_files {
            let readme_path = self.path().join(file);
            if readme_path.exists() {
                return ItemPreview::Command(
                    "bat --color always ".to_string() + readme_path.to_str().unwrap(),
                );
            }
        }

        return ItemPreview::Text("No README.md found".to_string());
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::project::Project;

    #[test]
    fn test_can_generate_the_names() {
        let project = Project {
            path: "/home/user/code/php/my-project".to_string(),
            name: "my-project".to_string(),
            group: Some("PHP".to_string()),
            icon: Some("".to_string()),
            color: None,
        };

        assert_eq!(Cow::from("[PHP] my-project"), project.session_name());
        assert_eq!(Cow::from("  my-project"), project.tmux_display());
        assert_eq!(Cow::from("  my-project"), project.skim_display());
        assert_eq!(Cow::from("[PHP] my-project"), project.skim_text());
    }

    #[test]
    fn test_can_generate_the_names_with_color() {
        let project = Project {
            path: "/home/user/code/php/my-another".to_string(),
            name: "my-project".to_string(),
            group: Some("PHP".to_string()),
            icon: Some(" ".to_string()),
            color: Some("#ff0000".to_string()),
        };

        assert_eq!(Cow::from("[PHP] my-project"), project.session_name());
        assert_eq!(
            Cow::from("#[fg=#ff0000] #[fg=default] my-project"),
            project.tmux_display()
        );
        assert_eq!(
            Cow::from("\x1b[38;2;255;0;0m \x1b[m my-project"),
            project.skim_display()
        );
        assert_eq!(Cow::from("[PHP] my-project"), project.skim_text());
    }
}
