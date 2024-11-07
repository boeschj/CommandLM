# ShellGPT

ShellGPT is an AI powered command-line tool that suggests and explain shell commands based on natural language queries. Perfect for those cases where you forget how to exit vim.

## Features

- Natural language command suggestions
- Command explanations
- General chat mode
- Context-aware recommendations using recent command history

## Local Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/shellGPT.git
cd shellGPT
```

2. Create and activate a virtual environment:
```bash
python -m venv venv
source venv/bin/activate
```

3. Install the package:
```bash
pip install -e .
```

## Setup

1. If you don't already have an OpenAI API key, follow the setup instructions from [OpenAI](https://platform.openai.com/api-keys)

2. Add your API key to your shell environment:
```bash
# Add this line to your ~/.zshrc
export OPENAI_API_KEY='your-api-key-here'

# Reload your shell configuration
source ~/.zshrc
```

For more information about API keys and setup, see the [OpenAI Developer Quickstart](https://platform.openai.com/docs/quickstart)

## Usage

### Get Command Suggestions

```bash
# Ask how to perform a task
shellgpt "scaffold a new react application called MyCoolApp using typescript and vite"

# Get help with complex commands
shellgpt "create a tar archive and compress it"

# Understand what a command does
shellgpt "what does chmod 755 do"
```

### Interactive Chat Mode

Start an interactive chat session for more complex discussions:
```bash
shellgpt chat
```

## Examples

```bash
$ shellgpt chat
Starting interactive chat session (type 'exit' to quit)

You> What's the difference between chmod and chown?
Assistant: chmod (change mode) and chown (change owner) serve different purposes. chmod is used to modify file permissions (read, write, execute) while chown changes the owner and group of files or directories. For example, 'chmod 755 file.txt' gives read/write/execute to owner and read/execute to others, while 'chown user:group file.txt' changes the file's owner and group.

You> How do I use find with exec?
Assistant: The find command's -exec option lets you perform actions on found files. Here's the basic syntax: 'find path -type f -name "pattern" -exec command {} \;'. The {} represents each found file, and \; marks the end of the command. For example, to find all .txt files and copy them: 'find . -type f -name "*.txt" -exec cp {} backup/ \;'

You> exit
```

## Development

1. Fork the repository

2. Install development dependencies:
```bash
pip install -e .
```

3. Make your changes

4. Test your changes:
```bash
shellgpt "test command"
```

## Gotchas/Limitations

- As ShellGPT is still in prototype phase, so DO NOT blindly execute the commands it gives you without first understanding them.
- By default, shellGPT sends the last 5 commands in history to add context for recommendations. Ensure you aren't sending sensitive information over the API.
- ShellGPT is only compatible with `zsh` at this time.

## Dependencies

- Python 3.8+
- click
- openai
- rich

## Troubleshooting

If you get an API key error:
1. Check that your API key is correctly set in your environment
2. Ensure you've reloaded your shell after setting the key
3. Verify your API key is valid at [OpenAI's platform](https://platform.openai.com/api-keys)

For more help, see the [OpenAI Documentation](https://platform.openai.com/docs/quickstart)
