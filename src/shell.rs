use std::{env, fmt};

#[derive(Debug)]
pub struct ShellContext {
    shell_type: ShellType,
    os: SupportedOperatingSystem,
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

impl SupportedOperatingSystem {
    pub fn to_string(&self) -> String {
        match self {
            SupportedOperatingSystem::Mac => "macOS".to_string(),
            SupportedOperatingSystem::Linux => "Linux".to_string(),
            SupportedOperatingSystem::Windows => "Windows".to_string(),
        }
    }
}

impl ShellType {
    pub fn to_string(&self) -> String {
        match self {
            ShellType::Zsh => "zsh".to_string(),
            ShellType::Bash => "bash".to_string(),
            ShellType::Fish => "fish".to_string(),
            ShellType::PowerShell => "PowerShell".to_string(),
            ShellType::Cmd => "Command Prompt".to_string(),
            ShellType::Unknown(shell) => shell.clone(),
        }
    }
}

//Implement Display so these can be used smoothly upstream
impl fmt::Display for ShellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for SupportedOperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ShellContext {
    pub fn new() -> Self {
        let shell_type = Self::detect_shell();
        let os = Self::detect_os();
        Self { shell_type, os }
    }

    pub fn get_current_shell(&self) -> &ShellType {
        &self.shell_type
    }

    pub fn get_current_os(&self) -> &SupportedOperatingSystem {
        &self.os
    }

    fn detect_os() -> SupportedOperatingSystem {
        let os = env::consts::OS;

        match os {
            "macos" => SupportedOperatingSystem::Mac,
            "linux" => SupportedOperatingSystem::Linux,
            "windows" => SupportedOperatingSystem::Windows,
            _ => {
                println!("WARNING: Unsupported operating system detected. This tool may work for you, or it may not.");
                return SupportedOperatingSystem::Linux;
            }
        }
    }

    fn detect_shell() -> ShellType {
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
}
