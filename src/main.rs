use atty::Stream;
use clap::{CommandFactory, Parser};
use dialoguer::Confirm;
use indicatif::{ProgressBar, ProgressStyle};
use commandlm::assistant::{create_client, get_command_suggestion, interactive_chat};
use commandlm::cli::{Cli, Commands};
use commandlm::shell::ShellContext;
use std::io::{self, Read};
use std::process::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let shell_context = ShellContext::default();
    let client = create_client(&shell_context)?;

    match cli.command {
        Some(Commands::Chat) => {
            interactive_chat(&client, &shell_context).await?;
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
    let shell_context = ShellContext::default();
    let client = create_client(&shell_context)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
            .template("{spinner} Thinking...")?,
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let (explanation, command) = get_command_suggestion(&client, &shell_context, query, context).await?;

    spinner.finish_and_clear();

    if !command.is_empty() {
        println!("\n{}", console::style(&explanation).blue());
        println!("\n{}", console::style("Command:").green());
        println!("{}\n", console::style(&command).white().bold());

        let confirmed = Confirm::new()
            .with_prompt("Execute this command?")
            .default(false)
            .interact()?;

        if confirmed {
            println!("{}", console::style("Executing...").yellow());
            execute_command(&command)?;
        } else {
            println!("{}", console::style("Command not executed.").dim());
        }
    } else {
        println!(
            "\n{}",
            console::style("No command suggestion available").red()
        );
    }

    Ok(())
}

fn execute_command(command: &str) -> anyhow::Result<()> {
    let shell_context = ShellContext::default();
    
    let (shell_cmd, shell_args) = match shell_context.shell_type {
        commandlm::shell::ShellType::Bash => ("bash", vec!["-c"]),
        commandlm::shell::ShellType::Zsh => ("zsh", vec!["-c"]),
        commandlm::shell::ShellType::Fish => ("fish", vec!["-c"]),
        commandlm::shell::ShellType::PowerShell => ("powershell", vec!["-Command"]),
        commandlm::shell::ShellType::Cmd => ("cmd", vec!["/C"]),
        commandlm::shell::ShellType::Unknown(_) => ("sh", vec!["-c"]),
    };

    let mut cmd_args = shell_args;
    cmd_args.push(command);

    let output = Command::new(shell_cmd)
        .args(&cmd_args)
        .output()?;

    if output.status.success() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
        if !output.stderr.is_empty() {
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }
        println!("{}", console::style("✓ Command completed successfully").green());
    } else {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        println!("{}", console::style("✗ Command failed").red());
        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(())
}
