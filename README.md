# ShellGPT

ShellGPT is an AI powered command-line tool that suggests and explain shell commands based on natural language queries. Perfect for those cases where you forget how to exit vim.

## Installation

### MacOS (via Homebrew)
```bash
brew tap boeschj/tap
brew install shellgpt
```

### Manual Installation (Linux/Windows)
1. Download the appropriate binary for your system from the [releases page](https://github.com/boeschj/shellgpt/releases):
   - Linux: `shellgpt-linux-x86_64.tar.gz`
   - Windows: `shellgpt-windows-x86_64.zip`

2. Extract and install:
```bash
# Linux
tar xvf shellgpt-linux-x86_64.tar.gz
sudo mv shellgpt /usr/local/bin/
chmod +x /usr/local/bin/shellgpt

# Windows
# Extract the ZIP file and add the directory to your PATH
```

## Setup

1. If you don't already have an OpenAI API key, follow the setup instructions from [OpenAI](https://platform.openai.com/api-keys)

2. Add your API key to your shell environment:
```bash
# Add this line to your shell config (~/.zshrc, ~/.bashrc, etc.)
export OPENAI_API_KEY='your-api-key-here'

# Reload your shell configuration
source ~/.zshrc  # or source ~/.bashrc
```

## Usage

### Get Command Suggestions

#### DISCLAIMER: ShellGPT generates shell commands using AI. While it aims to be helpful, it can potentially generate harmful commands. **ALWAYS review and understand any command before executing it** and never blindly run suggested commands. You are responsible for any commands you execute.
 
```bash
# Ask how to perform a task
shellgpt "scaffold a new react application called MyCoolApp using typescript and vite"

# Get help with complex commands
shellgpt "create a tar archive and compress it"

# Understand what a command does
shellgpt "what does chmod 755 do"
```

### Interactive Chat Mode
```bash
shellgpt chat
```

## Examples

```bash
$ shellgpt chat
Starting interactive chat session (type 'exit' to quit)

You> What's the difference between chmod and chown?
Assistant: chmod (change mode) and chown (change owner) serve different purposes...

You> exit
```

## Development & Releases

### Building From Source
```bash
# Clone the repository
git clone https://github.com/boeschj/shellgpt.git
cd shellgpt

# Build
cargo build --release
```

### Creating a New Release

1. Update version in `Cargo.toml`

2. Create and push a new tag:
```bash
git tag v0.x.x
git push origin v0.x.x
```

3. Wait for GitHub Actions to build and upload release artifacts

4. Update Homebrew formula:
```bash
# Get SHA of new Mac binary
curl -LO https://github.com/boeschj/shellgpt/releases/latest/download/shellgpt-mac-x86_64.tar.gz
shasum -a 256 shellgpt-mac-x86_64.tar.gz

# Update homebrew-tap repository:
# 1. Change VERSION in Formula/shellgpt.rb
# 2. Update SHA256
# 3. Commit and push changes
```

## Gotchas/Limitations

- Currently only compatible with gpt-4o-mini
- ShellGPT sends the last 5 commands from history for context - be mindful of sensitive information
- This tool was primarily designed and tested for macOS `zsh` environments.

## License

MIT License - see LICENSE file for details.