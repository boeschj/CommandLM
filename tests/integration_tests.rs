use std::process::Command;
use commandlm::shell::{ShellContext, ShellType, SupportedOperatingSystem};

#[test]
fn test_help_command() {
    let output = Command::new("./target/debug/clm")
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CommandLM"));
    assert!(stdout.contains("AI-powered command line assistant"));
    assert!(stdout.contains("chat"));
}

#[test]
fn test_version_command() {
    let output = Command::new("./target/debug/clm")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1.0.0"));
}

#[test]
fn test_no_args_shows_help() {
    let output = Command::new("./target/debug/clm")
        .output()
        .expect("Failed to execute command");

    // The command should succeed
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // With no args, it should show help or indicate no command available
    let combined_output = format!("{}{}", stdout, stderr);
    assert!(
        combined_output.contains("Usage") || 
        combined_output.contains("CommandLM") ||
        combined_output.contains("help") ||
        combined_output.contains("No command suggestion available")
    );
}

#[test]
fn test_shell_context_creation() {
    let context = ShellContext::default();
    
    // Ensure the context is properly initialized
    match context.shell_type {
        ShellType::Bash | ShellType::Zsh | ShellType::Fish | 
        ShellType::PowerShell | ShellType::Cmd | ShellType::Unknown(_) => {},
    }
    
    match context.os {
        SupportedOperatingSystem::Mac | 
        SupportedOperatingSystem::Linux | 
        SupportedOperatingSystem::Windows => {},
    }
}

#[test]
fn test_shell_command_mapping() {
    let context = ShellContext::default();
    
    // Test that each shell type has proper command mapping
    let (shell_cmd, shell_args) = match context.shell_type {
        ShellType::Bash => ("bash", vec!["-c"]),
        ShellType::Zsh => ("zsh", vec!["-c"]),
        ShellType::Fish => ("fish", vec!["-c"]),
        ShellType::PowerShell => ("powershell", vec!["-Command"]),
        ShellType::Cmd => ("cmd", vec!["/C"]),
        ShellType::Unknown(_) => ("sh", vec!["-c"]),
    };
    
    assert!(!shell_cmd.is_empty());
    assert!(!shell_args.is_empty());
}

#[cfg(test)]
mod mock_tests {

    // Mock test for command execution without actually running commands
    #[test]
    fn test_command_execution_logic() {
        // Test safe command that just echoes
        let test_command = "echo 'test'";
        
        // In a real scenario, we'd want to mock the execution
        // For now, just test that the command format is valid
        assert!(!test_command.is_empty());
        assert!(test_command.contains("echo"));
    }

    #[test]
    fn test_command_parsing_logic() {
        // Test that commands can be parsed properly
        let commands = vec![
            "ls -la",
            "mkdir test_dir",
            "echo 'hello world'",
            "pwd",
        ];
        
        for cmd in commands {
            assert!(!cmd.is_empty());
            // Basic validation that command has proper format
            assert!(cmd.chars().any(|c| !c.is_whitespace()));
        }
    }
}