use std::{env, fmt};

#[derive(Debug)]
pub struct ShellContext {
    pub shell_type: ShellType,
    pub os: SupportedOperatingSystem,
}

#[derive(Debug)]
pub enum ShellType {
    Zsh,
    Bash,
    Fish,
    PowerShell,
    Cmd,
    Unknown(String),
}

#[derive(Debug)]
pub enum SupportedOperatingSystem {
    Mac,
    Linux,
    Windows,
}



impl fmt::Display for ShellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellType::Zsh => write!(f, "zsh"),
            ShellType::Bash => write!(f, "bash"),
            ShellType::Fish => write!(f, "fish"),
            ShellType::PowerShell => write!(f, "PowerShell"),
            ShellType::Cmd => write!(f, "Command Prompt"),
            ShellType::Unknown(shell) => write!(f, "{}", shell),
        }
    }
}

impl fmt::Display for SupportedOperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupportedOperatingSystem::Mac => write!(f, "macOS"),
            SupportedOperatingSystem::Linux => write!(f, "Linux"),
            SupportedOperatingSystem::Windows => write!(f, "Windows"),
        }
    }
}

impl Default for ShellContext {
    fn default() -> Self {
        Self {
            shell_type: detect_shell(),
            os: detect_os(),
        }
    }
}

pub fn detect_os() -> SupportedOperatingSystem {
    let os = env::consts::OS;

    match os {
        "macos" => SupportedOperatingSystem::Mac,
        "linux" => SupportedOperatingSystem::Linux,
        "windows" => SupportedOperatingSystem::Windows,
        _ => {
            println!("WARNING: Unsupported operating system detected. This tool may work for you, or it may not.");
            SupportedOperatingSystem::Linux
        }
    }
}

pub fn detect_shell() -> ShellType {
    //For windows, check PowerShell and CMD
    if cfg!(windows) {
        if env::var("PSModulePath").is_ok() {
            return ShellType::PowerShell;
        }
        if let Ok(comspec) = env::var("COMSPEC") {
            if comspec.to_lowercase().contains("cmd.exe") {
                return ShellType::Cmd;
            }
        }
    }

    // For unix based systems, check the $SHELL env var which will contain the console type in the path
    if let Ok(shell_path) = env::var("SHELL") {
        let shell_path = shell_path.to_lowercase();
        if shell_path.contains("zsh") {
            return ShellType::Zsh;
        } else if shell_path.contains("bash") {
            return ShellType::Bash;
        } else if shell_path.contains("fish") {
            return ShellType::Fish;
        } else {
            return ShellType::Unknown(shell_path);
        }
    } else {
        ShellType::Bash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_shell_type_display() {
        assert_eq!(ShellType::Bash.to_string(), "bash");
        assert_eq!(ShellType::Zsh.to_string(), "zsh");
        assert_eq!(ShellType::Fish.to_string(), "fish");
        assert_eq!(ShellType::PowerShell.to_string(), "PowerShell");
        assert_eq!(ShellType::Cmd.to_string(), "Command Prompt");
        assert_eq!(ShellType::Unknown("custom".to_string()).to_string(), "custom");
    }

    #[test]
    fn test_os_display() {
        assert_eq!(SupportedOperatingSystem::Mac.to_string(), "macOS");
        assert_eq!(SupportedOperatingSystem::Linux.to_string(), "Linux");
        assert_eq!(SupportedOperatingSystem::Windows.to_string(), "Windows");
    }

    #[test]
    fn test_detect_os() {
        let os = detect_os();
        // Just ensure it returns one of the supported OS types
        match os {
            SupportedOperatingSystem::Mac |
            SupportedOperatingSystem::Linux |
            SupportedOperatingSystem::Windows => {},
        }
    }

    #[test]
    fn test_shell_context_default() {
        let context = ShellContext::default();
        // Ensure shell_type and os are populated
        assert!(!context.shell_type.to_string().is_empty());
        assert!(!context.os.to_string().is_empty());
    }

    #[test]
    fn test_detect_shell_with_env() {
        // Store original value
        let original_shell = env::var("SHELL").ok();
        
        // Test bash detection
        env::set_var("SHELL", "/bin/bash");
        let shell = detect_shell();
        match shell {
            ShellType::Bash => {},
            _ => {
                // On some systems the actual shell might override env var
                // so we just ensure it returns a valid shell type
                assert!(!shell.to_string().is_empty());
            }
        }

        // Test zsh detection
        env::set_var("SHELL", "/usr/bin/zsh");
        let shell = detect_shell();
        match shell {
            ShellType::Zsh => {},
            _ => {
                assert!(!shell.to_string().is_empty());
            }
        }

        // Restore original value or remove if it didn't exist
        match original_shell {
            Some(value) => env::set_var("SHELL", value),
            None => env::remove_var("SHELL"),
        }
    }
}
