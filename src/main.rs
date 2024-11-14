use atty::Stream;
use clap::{CommandFactory, Parser};
use indicatif::{ProgressBar, ProgressStyle};
use shellgpt::assistant::Assistant;
use shellgpt::cli::{Cli, Commands};
use std::io::{self, Read};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Chat) => {
            let assistant = Assistant::new()?;
            assistant.interactive_chat().await?;
        }
        None => {
            // Read from stdin if there's piped input
            let piped_input = if !atty::is(Stream::Stdin) {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                Some(buffer)
            } else {
                None
            };

            match (piped_input, &cli.query) {
                (Some(input), Some(query)) => {
                    // Both piped input and query argument provided
                    process_query(query, Some(&input)).await?;
                }
                (Some(input), None) => {
                    // Only piped input, use it as the query
                    process_query(&input, None).await?;
                }
                (None, Some(query)) => {
                    // Only query argument
                    process_query(query, None).await?;
                }
                (None, None) => {
                    // No input at all, show help
                    Cli::command().print_help()?;
                }
            }
        }
    }

    Ok(())
}

async fn process_query(query: &str, context: Option<&str>) -> anyhow::Result<()> {
    let assistant = Assistant::new()?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
            .template("{spinner} Thinking...")?,
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let (explanation, command) = assistant.get_command_suggestion(query, context).await?;

    spinner.finish_and_clear();

    if !command.is_empty() {
        println!("\n{}", console::style(&explanation).blue());
        println!("\n{}", console::style("Command:").green());
        println!("{}\n", console::style(&command).white().bold());
    } else {
        println!(
            "\n{}",
            console::style("No command suggestion available").red()
        );
    }

    Ok(())
}
