use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "CommandLM - Your AI-powered command line assistant", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(value_name = "QUERY")]
    pub query: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Chat,
}
