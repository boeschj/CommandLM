use std::process::Command;

pub struct ShellContext;

impl ShellContext {
    pub fn new() -> Self {
        Self
    }

    pub fn get_recent_commands(&self) -> Vec<String> {
        let mut commands = Vec::new();
        
        if let Ok(output) = Command::new("/bin/zsh")
            .arg("-c")
            .arg("history -5")
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                for line in stdout.lines() {
                    if let Some(cmd) = line.split_once(' ') {
                        let cmd = cmd.1.trim();
                        if !cmd.is_empty() && !cmd.starts_with("shellgpt") {
                            commands.push(cmd.to_string());
                        }
                    }
                }
            }
        }
        
        commands.truncate(5);
        commands
    }
}