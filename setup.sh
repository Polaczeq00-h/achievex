#!/bin/bash

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Install Arch-Achievements...${NC}"

if ! command -v cargo &> /dev/null; then
    sudo pacman -S rust
    exit 1
fi


echo -e "${BLUE}Compilation...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ compilation error.${NC}"
    exit 1
fi

echo -e "${BLUE}Move binary to /usr/local/bin/arch-achieve...${NC}"
sudo cp target/release/arch_achievements /usr/local/bin/arch-achieve
sudo chmod +x /usr/local/bin/arch-achieve

echo -e "${BLUE}Zsh configuration...${NC}"
ZSH_RC="$HOME/.zshrc"
PLUGIN_PATH="$(pwd)/plugin.zsh"

if ! grep -q "source $PLUGIN_PATH" "$ZSH_RC"; then
    echo "" >> "$ZSH_RC"
    echo "# Arch-Achievements RPG Plugin" >> "$ZSH_RC"
    echo "source $PLUGIN_PATH" >> "$ZSH_RC"
    echo -e "${GREEN}Plugin add to .zshrc !${NC}"
else
    echo -e "${BLUE}ℹ️ plugin already in .zshrc.${NC}"
fi

mkdir -p "$HOME/.arch_achievements"
if [ ! -f "$HOME/.arch_achievements/total_xp" ]; then
    echo "0" > "$HOME/.arch_achievements/total_xp"
fi

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN} INSTALLATION COMPLETE!${NC}"
echo -e "Restart your terminal or type : ${BLUE}source ~/.zshrc${NC}"
echo -e "Type ${BLUE}achievements${NC} to view all achievements unlocked."
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

/usr/local/bin/arch-achieve trigger arch-achieve "setup install"
