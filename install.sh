#!/bin/sh

set -e  # Exit immediately if a command exits with a non-zero status

# Check for cargo
if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: cargo is not installed. Please install Rust and try again."
    exit 1
fi

# Build the release binary
echo "Building release binary..."
cargo build --release

# Define install path
INSTALL_DIR="$HOME/.local/bin"
BIN_PATH="$INSTALL_DIR/todolist"

# Create the install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Copy the compiled binary
echo "Installing todolist to $INSTALL_DIR"
cp target/release/todolist "$BIN_PATH"
chmod +x "$BIN_PATH"

# Determine appropriate shell config file
if [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
else
    case "$SHELL" in
        */zsh) SHELL_CONFIG="$HOME/.zshrc" ;;
        */bash) SHELL_CONFIG="$HOME/.bashrc" ;;
        *) SHELL_CONFIG="$HOME/.profile" ;;
    esac
fi

# Add .local/bin to PATH if not already present
if [ -f "$SHELL_CONFIG" ] && grep -q "$INSTALL_DIR" "$SHELL_CONFIG"; then
    echo "$INSTALL_DIR is already in PATH in $SHELL_CONFIG"
else
    echo "Adding $INSTALL_DIR to PATH in $SHELL_CONFIG"
    {
        echo ""
        echo "# Added by todolist installer"
        echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
    } >> "$SHELL_CONFIG"
fi

echo "Installation complete."
echo "Please restart your terminal or run: source $SHELL_CONFIG"
