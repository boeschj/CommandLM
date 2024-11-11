use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "ShellGPT - Your AI-powered command line assistant", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    pub query: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Chat,
}