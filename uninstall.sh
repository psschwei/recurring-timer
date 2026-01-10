#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}Round Timer Uninstaller${NC}"
echo "======================"
echo ""

# Check if files exist before removing
BINARY=~/.local/bin/round-timer
DESKTOP=~/.local/share/applications/round-timer.desktop
ICON=~/.local/share/icons/hicolor/scalable/apps/round-timer.svg

FILES_FOUND=false

if [ -f "$BINARY" ]; then
    echo -e "${BLUE}Removing binary...${NC}"
    rm "$BINARY"
    FILES_FOUND=true
fi

if [ -f "$DESKTOP" ]; then
    echo -e "${BLUE}Removing desktop file...${NC}"
    rm "$DESKTOP"
    FILES_FOUND=true
fi

if [ -f "$ICON" ]; then
    echo -e "${BLUE}Removing icon...${NC}"
    rm "$ICON"
    FILES_FOUND=true
fi

if [ "$FILES_FOUND" = false ]; then
    echo -e "${YELLOW}Round Timer does not appear to be installed.${NC}"
    echo "No files found to remove."
    exit 0
fi

# Update desktop database
echo -e "${BLUE}Updating desktop database...${NC}"
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database ~/.local/share/applications
else
    echo "update-desktop-database not found, skipping..."
fi

echo ""
echo -e "${GREEN}Uninstallation complete!${NC}"
echo ""
echo "Round Timer has been removed from your system."
