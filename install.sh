#!/usr/bin/env bash

# ShellGPT Installer
#
# Usage:
#   curl -sSL https://raw.githubusercontent.com/boeschj/ShellGPT/refs/heads/main/install.sh | bash
# or
#   wget -qO- https://raw.githubusercontent.com/boeschj/ShellGPT/refs/heads/main/install.sh | bash

set -e

# -------------------------------- Variables -------------------------------- #
VERSION="v0.1.0"
INSTALL_DIR="/usr/local/bin"
REQUIRED_PKGS=("python3" "pip3")

# -------------------------------- Check Root -------------------------------- #
check_root() {
    if [ "$EUID" -ne 0 ]; then
        echo "Please run as root with sudo."
        exit 1
    fi
}

# ------------------------- Install Required Packages ------------------------ #
install_package() {
    local package=$1
    echo "Installing $package..."

    case $OSTYPE in
        darwin*)
            if ! command -v brew >/dev/null 2>&1; then
                echo "Homebrew is required. Please install it first:"
                echo '/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"'
                exit 1
            fi
            brew install "$package"
            ;;
        linux-gnu*)
            if [[ -f /etc/debian_version ]]; then
                apt-get update && apt-get install -y "$package"
            elif [[ -f /etc/redhat-release ]]; then
                yum install -y "$package"
            elif [[ -f /etc/fedora-release ]]; then
                dnf install -y "$package"
            else
                echo "Unsupported Linux distribution."
                exit 1
            fi
            ;;
        *)
            echo "Unsupported operating system: $OSTYPE"
            exit 1
            ;;
    esac
}

check_system_requirements() {
    local all_installed=true

    for pkg in "${REQUIRED_PKGS[@]}"; do
        if ! command -v "$pkg" >/dev/null 2>&1; then
            echo "$pkg is not installed."
            install_package "$pkg"
            all_installed=false
        fi
    done

    if [ "$all_installed" = true ]; then
        echo "All system requirements satisfied."
    fi
}

# ------------------------------- Installation ------------------------------- #
install_shellgpt() {
    echo "Installing ShellGPT..."
    
    # Create temporary directory
    TMP_DIR=$(mktemp -d)
    cd "$TMP_DIR"

    # Download the latest release
    echo "Downloading ShellGPT..."
    curl -sSL "https://github.com/boeschj/shellGPT/archive/refs/tags/${VERSION}.tar.gz" | tar xz

    # Install the package
    cd "shellGPT-${VERSION#v}"
    pip3 install --upgrade pip
    pip3 install .

    # Create symlink if needed
    if [ ! -f "$INSTALL_DIR/shellgpt" ]; then
        ln -s "$(which shellgpt)" "$INSTALL_DIR/shellgpt"
    fi

    # Cleanup
    cd
    rm -rf "$TMP_DIR"
}

# ----------------------------- Post Installation ---------------------------- #
post_install_message() {
    echo
    echo "ShellGPT installed successfully! 🎉"
    echo
    echo "To get started, you need to set up your OpenAI API key:"
    echo "1. Get your API key from https://platform.openai.com/api-keys"
    echo "2. Add this line to your ~/.zshrc:"
    echo "   export OPENAI_API_KEY='your-api-key-here'"
    echo
    echo "Then reload your shell:"
    echo "   source ~/.zshrc"
    echo
    echo "Try it out:"
    echo "   shellgpt \"find large files\""
    echo
    echo "For more information, visit:"
    echo "https://github.com/boeschj/ShellGPT"
    echo
}

# ---------------------------------------------------------------------------- #
#                                     Main                                       #
# ---------------------------------------------------------------------------- #

echo "Installing ShellGPT version $VERSION..."

check_root
check_system_requirements
install_shellgpt
post_install_message