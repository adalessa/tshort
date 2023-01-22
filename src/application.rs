use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("~/.config/projects.json"))]
    config_file: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

impl Args {
    pub fn config_file(&self) -> &str {
        self.config_file.as_ref()
    }

    pub fn command(&self) -> Option<&Commands> {
        self.command.as_ref()
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Bind { key: Option<String> },
    Forget { key: Option<String> },
    List,
    RemoveCache,
}
