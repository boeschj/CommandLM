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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parsing_with_query() {
        let args = vec!["clm", "list files"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        assert_eq!(cli.query, Some("list files".to_string()));
        assert!(cli.command.is_none());
    }

    #[test]
    fn test_cli_parsing_with_chat_command() {
        let args = vec!["clm", "chat"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        assert!(cli.query.is_none());
        assert!(matches!(cli.command, Some(Commands::Chat)));
    }

    #[test]
    fn test_cli_parsing_help() {
        let args = vec!["clm", "--help"];
        let result = Cli::try_parse_from(args);
        
        // Should fail with help (which is expected behavior)
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parsing_version() {
        let args = vec!["clm", "--version"];
        let result = Cli::try_parse_from(args);
        
        // Should fail with version (which is expected behavior)
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parsing_no_args() {
        let args = vec!["clm"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        assert!(cli.query.is_none());
        assert!(cli.command.is_none());
    }

    #[test]
    fn test_cli_parsing_complex_query() {
        let args = vec!["clm", "create a tar archive with compression"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        assert_eq!(cli.query, Some("create a tar archive with compression".to_string()));
        assert!(cli.command.is_none());
    }
}
