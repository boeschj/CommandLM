"""Command line interface for shellgpt."""
import click
from rich.console import Console

from .assistant import Assistant

console = Console()

@click.group(invoke_without_command=True)
@click.pass_context
@click.argument('query', nargs=-1, required=False)
def main(ctx, query):
    """ShellGPT - Your AI-powered command line assistant
    
    Set up your API key by adding this to your ~/.zshrc:
    export OPENAI_API_KEY='your-key-here'
    """
    if ctx.invoked_subcommand is None:
        if query:
            assistant = Assistant()
            explanation, command = assistant.get_command_suggestion(' '.join(query))
            
            if command:
                console.print(f"\n[blue]{explanation}[/blue]")
                console.print("\n[green]Command:[/green]")
                console.print(f"[bold white]{command}[/bold white]\n")
            else:
                console.print("\n[red]No command suggestion available[/red]")
        else:
            # Show help if no query and no subcommand
            click.echo(ctx.get_help())

@main.command()
def chat():
    """Start an interactive chat session"""
    assistant = Assistant()
    assistant.interactive_chat()

if __name__ == '__main__':
    main()