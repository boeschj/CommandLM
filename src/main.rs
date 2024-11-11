use clap::{Parser, CommandFactory};
use shellgpt::cli::{Cli, Commands};
use shellgpt::assistant::Assistant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Chat) => {
            let assistant = Assistant::new()?;
            assistant.interactive_chat().await?;
        }
        None => {
            if let Some(query) = cli.query {
                let assistant = Assistant::new()?;
                let (explanation, command) = assistant.get_command_suggestion(&query).await?;
                
                if !command.is_empty() {
                    println!("\n{}", console::style(&explanation).blue());
                    println!("\n{}", console::style("Command:").green());
                    println!("{}\n", console::style(&command).white().bold());
                } else {
                    println!("\n{}", console::style("No command suggestion available").red());
                }
            } else {
                Cli::command().print_help()?;
            }
        }
    }
    
    Ok(())
}