"""Configuration management with secure API key handling."""
import os
from pathlib import Path
from rich.console import Console

console = Console()

class Config:
    def __init__(self):
        self.config_dir = Path.home() / '.config' / 'shellgpt'
        self.config_file = self.config_dir / 'config.json'

    def get_api_key(self) -> str:
        api_key = os.environ.get('OPENAI_API_KEY')
        
        if not api_key:
            console.print("\n[yellow]No OpenAI API key found in environment variables.[/yellow]")
            console.print("\nTo set up your API key securely, add this to your ~/.zshrc file:")
            console.print("[green]export OPENAI_API_KEY='your-key-here'[/green]")
            console.print("\nThen reload your terminal or run:")
            console.print("[green]source ~/.zshrc[/green]")
            console.print("\nFor more information, please see the official OpenAI Developer Quickstart:")
            console.print("[blue]https://platform.openai.com/docs/quickstart[/blue]")
            
            raise ValueError(
                "API key is required. Please set the OPENAI_API_KEY environment variable.\n"
                "See https://platform.openai.com/docs/quickstart for more information."
            )
        
        return api_key