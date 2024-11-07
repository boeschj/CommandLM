"""Command line interface for shellgpt."""
import click
from rich.console import Console

from .assistant import Assistant

console = Console()

@click.group()
def main():
    pass

@main.command()
@click.argument('query', nargs=-1)
def ask(query):
    """Ask for a command suggestion"""
    if query:
        assistant = Assistant()
        explanation, command = assistant.get_command_suggestion(' '.join(query))
        
        if command:
            console.print(f"\n[blue]{explanation}[/blue]")
            console.print("\n[green]Command:[/green]")
            console.print(f"[bold white]{command}[/bold white]\n")
        else:
            console.print("\n[red]No command suggestion available[/red]")

@main.command()
def chat():
    """Start an interactive chat session"""
    assistant = Assistant()
    assistant.interactive_chat()

@main.command(hidden=True)
def default():
    pass

main.add_command(ask, name='')

if __name__ == '__main__':
    main()