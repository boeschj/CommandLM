# CommandLM

CommandLM is an AI powered command-line tool that you can ask to suggest or explain shell commands based on natural language queries.


## Installation

### MacOS (via Homebrew)
```bash
brew tap boeschj/tap
brew install clm
```

### Manual Installation (Linux/Windows)
1. Download the appropriate binary for your system from the [releases page](https://github.com/boeschj/CommandLM/releases):
   - Linux: `clm-linux-x86_64.tar.gz`
   - Windows: `clm-windows-x86_64.zip`

2. Extract and install:
```bash
# Linux
tar xvf clm-linux-x86_64.tar.gz
sudo mv clm /usr/local/bin/
chmod +x /usr/local/bin/clm

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

#### DISCLAIMER: CommandLM generates shell commands using AI and may be inaccurate. **ALWAYS review and understand any command before executing it** and never blindly run suggested commands. You are responsible for any commands you execute.
 
```bash
# Ask how to perform a task
clm "scaffold a new react application called MyCoolApp using typescript and vite"

# Get help with complex commands
clm "create a tar archive and compress it"
# Shows: tar -czf archive.tar.gz folder/
# Prompt: Execute this command? [y/N]

# Understand what a command does
clm "what does chmod 755 do"
```

CommandLM will suggest a command and ask if you want to execute it. You can safely review the command before confirming.

### Interactive Chat Mode
```bash
clm chat
```

## Examples

```bash
$ clm chat
Starting interactive chat session (type 'exit' to quit)

You> What's the difference between chmod and chown?
Assistant: chmod (change mode) and chown (change owner) serve different purposes...

You> exit
```

## Development & Releases

### Building From Source
```bash
# Clone the repository
git clone https://github.com/boeschj/CommandLM.git
cd CommandLM

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
curl -LO https://github.com/boeschj/CommandLM/releases/latest/download/clm-mac-x86_64.tar.gz
shasum -a 256 clm-mac-x86_64.tar.gz

# Update homebrew-tap repository:
# 1. Change VERSION in Formula/clm.rb
# 2. Update SHA256
# 3. Commit and push changes
```

## Gotchas/Limitations

- Currently only compatible with gpt-4o-mini
- CommandLM sends the last 5 commands from history for context - be mindful of sensitive information
- This tool was primarily designed and tested for macOS `zsh` environments.

## License

MIT License - see LICENSE file for details.
