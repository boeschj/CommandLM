use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use std::env;

use crate::shell::ShellType;

pub fn get_config_dir() -> Result<std::path::PathBuf> {
    let project_dirs = ProjectDirs::from("com", "commandlm", "commandlm")
        .ok_or_else(|| anyhow!("Failed to get project directories"))?;
    Ok(project_dirs.config_dir().to_path_buf())
}

pub fn get_api_key(shell_type: &ShellType) -> Result<String> {
    match env::var("OPENAI_API_KEY") {
        Ok(key) => Ok(key),
        Err(_) => {
            println!(
                "\n{}",
                console::style("No OpenAI API key found in environment variables.").yellow()
            );
            println!("{}", format!("\nTo set up your API key securely, add this to your {shell_type} configuration file:"));
            println!(
                "{}",
                console::style("export OPENAI_API_KEY='your-key-here'").green()
            );
            println!("\nThen reload your terminal to see your changes take effect.");
            println!(
                "\nFor more information, please see the official OpenAI Developer Quickstart:"
            );
            println!(
                "{}",
                console::style("https://platform.openai.com/docs/quickstart").blue()
            );

            Err(anyhow!(
                "API key is required. Please set the OPENAI_API_KEY environment variable.\n\
                See https://platform.openai.com/docs/quickstart for more information."
            ))
        }
    }
}
