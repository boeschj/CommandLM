"""OpenAI interaction with secure API key handling."""
from openai import OpenAI
from rich.console import Console
from rich.prompt import Prompt
from typing import Tuple
import sys

from .config import Config
from .shell import ShellContext

console = Console()

class Assistant:
    def __init__(self):
        self.config = Config()
        self.shell_context = ShellContext()
        
        try:
            api_key = self.config.get_api_key()
            self.client = OpenAI(api_key=api_key)
        except ValueError as e:
            console.print(f"\n[red]{str(e)}[/red]")
            sys.exit(1)

    def get_command_suggestion(self, query: str) -> Tuple[str, str]:
        """Get command suggestion from OpenAI."""
        recent_commands = self.shell_context.get_recent_commands()
        
        context = ""
        if recent_commands:
            context += "Recent commands:\n" + "\n".join(f"- {cmd}" for cmd in recent_commands)

        messages = [
            {
                "role": "system", 
                "content": """You are a command line assistant for zsh on MacOS. 
                Always respond in this format: EXPLANATION|COMMAND
                where EXPLANATION is one sentence explaining what the command does,
                and COMMAND is the exact command to run.
                Example: 'Shows last 10 commands from history|history 10'"""
            },
            {
                "role": "user", 
                "content": f"Context:\n{context}\n\nQuery: {query}"
            }
        ]

        try:
            response = self.client.chat.completions.create(
                model="gpt-4o-mini",
                messages=messages,
                temperature=0.3,
                max_tokens=150
            )
            
            content = response.choices[0].message.content
            if '|' in content:
                explanation, command = content.split('|', 1)
                return explanation.strip(), command.strip()
            return content.strip(), ""
            
        except Exception as e:
            console.print(f"[red]Error: {str(e)}[/red]")
            if 'authentication' in str(e).lower() or 'api key' in str(e).lower():
                console.print("\n[yellow]This appears to be an API key issue. Please check that your OPENAI_API_KEY environment variable is set correctly.[/yellow]")
            return "Error occurred", ""

    def interactive_chat(self):
        """Start an interactive chat session."""
        console.print("[blue]Starting interactive chat session (type 'exit' to quit)[/blue]")
        
        messages = [
            {"role": "system", "content": "You are a helpful command line assistant for MacOS. You can help with shell commands, explain terminal concepts, and provide general assistance."}
        ]
        
        while True:
            query = Prompt.ask("\nYou")
            
            if query.lower() in ('exit', 'quit'):
                break
                
            try:
                messages.append({"role": "user", "content": query})
                
                response = self.client.chat.completions.create(
                    model="gpt-4o-mini",
                    messages=messages,
                    temperature=0.7
                )
                
                assistant_message = response.choices[0].message.content
                console.print(f"\n[green]Assistant:[/green] {assistant_message}")
                
                messages.append({"role": "assistant", "content": assistant_message})
                
            except Exception as e:
                console.print(f"\n[red]Error: {str(e)}[/red]")