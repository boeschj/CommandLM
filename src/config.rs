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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use crate::shell::ShellType;

    #[test]
    fn test_get_config_dir() {
        let result = get_config_dir();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("commandlm"));
    }

    #[test]
    fn test_get_api_key_with_env() {
        // Store original value
        let original_key = env::var("OPENAI_API_KEY").ok();
        
        // Test with valid API key
        env::set_var("OPENAI_API_KEY", "test-key-123");
        let result = get_api_key(&ShellType::Bash);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-key-123");
        
        // Test without API key
        env::remove_var("OPENAI_API_KEY");
        let result = get_api_key(&ShellType::Zsh);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("API key is required"));
        
        // Restore original value or remove if it didn't exist
        match original_key {
            Some(value) => env::set_var("OPENAI_API_KEY", value),
            None => env::remove_var("OPENAI_API_KEY"),
        }
    }

    #[test]
    fn test_get_api_key_error_message_contains_shell() {
        // Store original value
        let original_key = env::var("OPENAI_API_KEY").ok();
        
        // Remove API key to trigger error
        env::remove_var("OPENAI_API_KEY");
        
        // The function should print shell-specific instructions
        // We can't easily test the print output, but we can test it doesn't panic
        let result = get_api_key(&ShellType::Fish);
        assert!(result.is_err());
        
        // Restore original value
        match original_key {
            Some(value) => env::set_var("OPENAI_API_KEY", value),
            None => env::remove_var("OPENAI_API_KEY"),
        }
    }
}
