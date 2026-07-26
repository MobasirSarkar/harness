#!/bin/bash
# ==============================================================================
# S-Tier AI Engineering Harness — Universal OS & User Agnostic Installer
# Works on Linux (Arch, Debian/Ubuntu, RHEL/Fedora) and macOS (Darwin)
# Supports any system user ($HOME) and automatically detects package managers
# ==============================================================================

set -e

# Detect Script Directory (Where git repo is cloned)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Color Tokens
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
RESET='\033[0m'

echo -e "${CYAN}======================================================================${RESET}"
echo -e "${GREEN} 🚀 S-Tier AI Harness: Universal OS & User Agnostic Installer${RESET}"
echo -e "${CYAN}======================================================================${RESET}"

# 1. Detect OS & User Environment
OS_TYPE="$(uname -s)"
USER_HOME="$HOME"
USER_NAME="${USER:-$(whoami)}"

echo -e "${CYAN}OS:${RESET} $OS_TYPE | ${CYAN}User:${RESET} $USER_NAME | ${CYAN}Home:${RESET} $USER_HOME"
echo -e "${CYAN}Repo Source:${RESET} $SCRIPT_DIR"

# 2. Ensure Local Binaries & Settings Directories
mkdir -p "$USER_HOME/.local/bin"
mkdir -p "$USER_HOME/.gemini"
mkdir -p "$USER_HOME/.gemini/antigravity-cli"
mkdir -p "$USER_HOME/.config/ponytail"

# 3. Detect Package Manager & Install System CLI Dependencies
echo -e "\n${YELLOW}[1/7] Auditing & Installing System Dependencies...${RESET}"

install_cmd=""
if command -v pacman &>/dev/null; then
    install_cmd="sudo pacman -S --noconfirm --needed"
elif command -v apt-get &>/dev/null; then
    install_cmd="sudo apt-get update && sudo apt-get install -y"
elif command -v dnf &>/dev/null; then
    install_cmd="sudo dnf install -y"
elif command -v brew &>/dev/null; then
    install_cmd="brew install"
fi

# Function to check and install CLI tool
ensure_cli() {
    local cmd="$1"
    local pkg="${2:-$1}"
    if ! command -v "$cmd" &>/dev/null; then
        echo -e "Installing missing CLI: ${CYAN}$cmd${RESET}..."
        if [ -n "$install_cmd" ]; then
            $install_cmd "$pkg" || echo -e "${RED}Notice: Could not auto-install $pkg via system package manager.${RESET}"
        fi
    else
        echo -e "Found CLI: ${GREEN}$cmd${RESET}"
    fi
}

ensure_cli fd fd-find
ensure_cli rg ripgrep
ensure_cli fzf fzf
ensure_cli eza eza
ensure_cli yazi yazi

# 4. Install ast-grep & sd via npm / cargo
echo -e "\n${YELLOW}[2/7] Checking ast-grep (sg) & sd CLI...${RESET}"
if ! command -v ast-grep &>/dev/null; then
    if command -v npm &>/dev/null; then
        echo -e "Installing ${CYAN}ast-grep${RESET} via npm..."
        npm install -g @ast-grep/cli || true
    fi
fi

if ! command -v sd &>/dev/null; then
    if command -v cargo &>/dev/null; then
        echo -e "Installing ${CYAN}sd${RESET} via cargo..."
        cargo install sd || true
    fi
fi

# 5. Install FFF MCP Server (Official pre-compiled Rust binary)
echo -e "\n${YELLOW}[3/7] Installing FFF MCP Server...${RESET}"
curl -sSL https://dmtrkovalenko.dev/install-fff-mcp.sh | bash || true

# 6. Compile & Install Rust harness-cli Binary
echo -e "\n${YELLOW}[4/7] Compiling Rust harness-cli binary...${RESET}"
if [ -d "$SCRIPT_DIR/harness-cli" ]; then
    cd "$SCRIPT_DIR/harness-cli"
    if command -v cargo &>/dev/null; then
        cargo build --release
        cp -f target/release/harness-cli "$USER_HOME/.local/bin/harness-cli"
        chmod +x "$USER_HOME/.local/bin/harness-cli"
        echo -e "${GREEN}Compiled & installed harness-cli to $USER_HOME/.local/bin/harness-cli${RESET}"
    fi
fi

# 7. Dynamically Deploy GEMINI.md, CLAUDE.md & Statusline
echo -e "\n${YELLOW}[5/7] Deploying Rules (GEMINI.md, CLAUDE.md) & Statusline...${RESET}"

if [ -f "$SCRIPT_DIR/GEMINI.md" ]; then
    cp -f "$SCRIPT_DIR/GEMINI.md" "$USER_HOME/.gemini/GEMINI.md"
fi

if [ -f "$SCRIPT_DIR/CLAUDE.md" ]; then
    cp -f "$SCRIPT_DIR/CLAUDE.md" "$USER_HOME/CLAUDE.md"
    cp -f "$SCRIPT_DIR/CLAUDE.md" "$USER_HOME/.gemini/CLAUDE.md"
fi

if [ -f "$SCRIPT_DIR/statusline.sh" ]; then
    cp -f "$SCRIPT_DIR/statusline.sh" "$USER_HOME/statusline.sh"
    chmod +x "$USER_HOME/statusline.sh"
fi

# 8. Dynamically Deploy & Sanitize settings.json (User Home Replacement)
echo -e "\n${YELLOW}[6/7] Deploying User-Agnostic settings.json...${RESET}"

if [ -f "$SCRIPT_DIR/settings.json" ]; then
    # Dynamically replace any hardcoded home paths with target $USER_HOME
    sed "s|/home/[a-zA-Z0-9_-]*|$USER_HOME|g" "$SCRIPT_DIR/settings.json" > "$USER_HOME/.gemini/settings.json"
    sed "s|/home/[a-zA-Z0-9_-]*|$USER_HOME|g" "$SCRIPT_DIR/settings.json" > "$USER_HOME/.gemini/antigravity-cli/settings.json"
fi

# 9. Verify PATH
echo -e "\n${YELLOW}[7/7] Verifying PATH & Executables...${RESET}"
if [[ ":$PATH:" != *":$USER_HOME/.local/bin:"* ]]; then
    echo -e "${YELLOW}Notice: Add $USER_HOME/.local/bin to your PATH in ~/.bashrc or ~/.zshrc:${RESET}"
    echo -e "  ${CYAN}export PATH=\"\$HOME/.local/bin:\$PATH\"${RESET}"
fi

echo -e "\n${CYAN}======================================================================${RESET}"
echo -e "${GREEN} 🎉 S-Tier AI Harness setup complete for user $USER_NAME!${RESET}"
echo -e "${CYAN}======================================================================${RESET}"
