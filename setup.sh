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
sudo ln -sf /usr/local/bin/arch-achieve /usr/local/bin/achievix

echo -e "${BLUE}Install KDE overlay launcher...${NC}"
sudo cp achievix-overlay.sh /usr/local/bin/achievix-overlay
sudo chmod +x /usr/local/bin/achievix-overlay

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

echo -e "${BLUE}Fish configuration...${NC}"
FISH_RC="$HOME/.config/fish/config.fish"
FISH_PLUGIN_PATH="$(pwd)/arch-achievements.fish"

if command -v fish &> /dev/null; then
    mkdir -p "$(dirname "$FISH_RC")"
    touch "$FISH_RC"

    if ! grep -q "source $FISH_PLUGIN_PATH" "$FISH_RC"; then
        echo "" >> "$FISH_RC"
        echo "# Arch-Achievements RPG Plugin" >> "$FISH_RC"
        echo "source $FISH_PLUGIN_PATH" >> "$FISH_RC"
        echo -e "${GREEN}Plugin add to config.fish !${NC}"
    else
        echo -e "${BLUE}ℹ️ plugin already in config.fish.${NC}"
    fi
else
    echo -e "${BLUE}ℹ️ fish not found, skipping fish hook.${NC}"
fi

mkdir -p "$HOME/.arch_achievements"
if [ ! -f "$HOME/.arch_achievements/total_xp" ]; then
    echo "0" > "$HOME/.arch_achievements/total_xp"
fi

ACHIEVIX_CONFIG_DIR="$HOME/.config/achievix"
ACHIEVIX_CONFIG="$ACHIEVIX_CONFIG_DIR/config"
mkdir -p "$ACHIEVIX_CONFIG_DIR"

if [ ! -f "$ACHIEVIX_CONFIG" ]; then
    cat > "$ACHIEVIX_CONFIG" <<EOF
tts_enabled=true
tts_model_path=~/piper/en_US-ryan-high.onnx
tts_player=mpv
EOF
    echo -e "${GREEN}Default Achievix config installed.${NC}"
else
    echo -e "${BLUE}ℹ️ Achievix config already exists.${NC}"
fi

if [ -f "$HOME/.arch_achievements/achieve.mp3" ]; then
    echo -e "${BLUE}ℹ️ custom achievement sound already exists.${NC}"
elif [ -f "assets/achieve.mp3" ]; then
    cp "assets/achieve.mp3" "$HOME/.arch_achievements/achieve.mp3"
    echo -e "${GREEN}Default achievement sound installed.${NC}"
else
    echo -e "${BLUE}ℹ️ assets/achieve.mp3 not found, skipping achievement sound.${NC}"
fi

echo -e "${BLUE}systemd user daemon...${NC}"
SYSTEMD_USER_DIR="$HOME/.config/systemd/user"
mkdir -p "$SYSTEMD_USER_DIR"
cp systemd/achievix.service "$SYSTEMD_USER_DIR/achievix.service"

if command -v systemctl &> /dev/null; then
    systemctl --user daemon-reload || true
    systemctl --user enable --now achievix.service || true
    echo -e "${GREEN}Daemon service installed as achievix.service.${NC}"
else
    echo -e "${BLUE}ℹ️ systemctl not found, daemon service copied but not enabled.${NC}"
fi

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN} INSTALLATION COMPLETE!${NC}"
echo -e "Restart your terminal or type : ${BLUE}source ~/.zshrc${NC}"
echo -e "Fish users can run : ${BLUE}source ~/.config/fish/config.fish${NC}"
echo -e "Type ${BLUE}achievements${NC} to view all achievements unlocked."
echo -e "Type ${BLUE}arch-achieve overlay${NC} to launch the KDE pet overlay."
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

/usr/local/bin/arch-achieve trigger arch-achieve "setup install"
