use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use std::env;

pub struct Config {
    _config_dir: std::path::PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let project_dirs = ProjectDirs::from("com", "shellgpt", "shellgpt")
            .expect("Failed to get project directories");
            
        Self {
            _config_dir: project_dirs.config_dir().to_path_buf(),
        }
    }

    pub fn get_api_key(&self) -> Result<String> {
        match env::var("OPENAI_API_KEY") {
            Ok(key) => Ok(key),
            Err(_) => {
                println!("\n{}",
                    console::style("No OpenAI API key found in environment variables.")
                        .yellow()
                );
                println!("\nTo set up your API key securely, add this to your ~/.zshrc file:");
                println!("{}",
                    console::style("export OPENAI_API_KEY='your-key-here'")
                        .green()
                );
                println!("\nThen reload your terminal or run:");
                println!("{}", console::style("source ~/.zshrc").green());
                println!("\nFor more information, please see the official OpenAI Developer Quickstart:");
                println!("{}", 
                    console::style("https://platform.openai.com/docs/quickstart")
                        .blue()
                );
                
                Err(anyhow!(
                    "API key is required. Please set the OPENAI_API_KEY environment variable.\n\
                    See https://platform.openai.com/docs/quickstart for more information."
                ))
            }
        }
    }
}