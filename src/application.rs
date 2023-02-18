use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("~/.config/projects.json"))]
    pub config_file: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Bind { key: Option<String> },
    Forget { key: Option<String> },
    List,
    RemoveCache,
}
