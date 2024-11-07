"""Shell interaction utilities."""
import subprocess
from pathlib import Path
from typing import List

class ShellContext:
    def __init__(self):
        self.history_file = Path.home() / '.zsh_history'

    def get_recent_commands(self, limit: int = 5) -> List[str]:
        commands = []
        try:
            result = subprocess.run(
                'history -5',
                shell=True,
                capture_output=True,
                text=True,
                executable='/bin/zsh'
            )
            if result.returncode == 0:
                for line in result.stdout.splitlines():
                    if ' ' in line:
                        cmd = line.split(' ', 1)[1].strip()
                        if cmd and not cmd.startswith('shellgpt'):
                            commands.append(cmd)
        except Exception:
            pass
            
        return commands[-limit:] if commands else []

#WIP to have the command automatically show up in your next line to execute
    # def prefill_command(self, command: str):
    #     """Put command into the zsh buffer."""
    #     if not command:
    #         return
            
    #     try:
    #         # Escape any quotes in the command
    #         escaped_command = command.replace('"', '\\"')
    #         subprocess.run(
    #             f'print -z "{escaped_command}"',
    #             shell=True,
    #             executable='/bin/zsh'
    #         )
    #     except Exception as e:
    #         # Fallback: just print the command
    #         print(f"\nSuggested command: {command}")