use clap::{Parser, CommandFactory};
use shellgpt::cli::{Cli, Commands};
use shellgpt::assistant::Assistant;
use indicatif::{ProgressBar, ProgressStyle};

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
                
                let spinner = ProgressBar::new_spinner();
                spinner.set_style(
                    ProgressStyle::default_spinner()
                        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
                        .template("{spinner} Thinking...")?
                );
                spinner.enable_steady_tick(std::time::Duration::from_millis(80));
                
                let (explanation, command) = assistant.get_command_suggestion(&query).await?;
                
                spinner.finish_and_clear();
                
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