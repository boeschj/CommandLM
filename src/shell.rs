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
