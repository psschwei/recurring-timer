#!/bin/bash

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}Round Timer Installer${NC}"
echo "===================="
echo ""

# Determine if we have a prebuilt binary or need to build
BINARY_PATH=""
if [ -f "round-timer" ]; then
    echo -e "${BLUE}Using prebuilt binary...${NC}"
    BINARY_PATH="round-timer"
elif [ -f "target/release/round-timer" ]; then
    echo -e "${BLUE}Using existing built binary...${NC}"
    BINARY_PATH="target/release/round-timer"
else
    # Need to build from source
    echo -e "${BLUE}Building from source...${NC}"

    # Check if cargo is installed
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: cargo is not installed. Please install Rust first.${NC}"
        echo "Visit: https://rustup.rs/"
        exit 1
    fi

    # Build the release binary
    cargo build --release

    if [ ! -f "target/release/round-timer" ]; then
        echo -e "${RED}Error: Build failed. Binary not found at target/release/round-timer${NC}"
        exit 1
    fi

    BINARY_PATH="target/release/round-timer"
    echo -e "${GREEN}Build successful!${NC}"
fi

echo ""

# Create directories if they don't exist
echo -e "${BLUE}Creating installation directories...${NC}"
mkdir -p ~/.local/bin
mkdir -p ~/.local/share/applications
mkdir -p ~/.local/share/icons/hicolor/scalable/apps

# Install the binary
echo -e "${BLUE}Installing binary to ~/.local/bin/round-timer${NC}"
cp "$BINARY_PATH" ~/.local/bin/round-timer
chmod +x ~/.local/bin/round-timer

# Install the desktop file
echo -e "${BLUE}Installing desktop file...${NC}"
cp round-timer.desktop ~/.local/share/applications/round-timer.desktop

# Install the icon
echo -e "${BLUE}Installing icon...${NC}"
cp assets/round-timer.svg ~/.local/share/icons/hicolor/scalable/apps/round-timer.svg

# Update desktop database
echo -e "${BLUE}Updating desktop database...${NC}"
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database ~/.local/share/applications
else
    echo "update-desktop-database not found, skipping..."
fi

echo ""
echo -e "${GREEN}Installation complete!${NC}"
echo ""
echo "Round Timer has been installed to ~/.local/bin/round-timer"
echo "It should now appear in your Applications menu."
echo ""
echo "If you don't see it immediately, try:"
echo "  - Logging out and back in"
echo "  - Restarting your desktop environment"
echo ""
echo "To uninstall, run: ./uninstall.sh"
