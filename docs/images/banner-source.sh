#!/bin/bash
# GitType Banner Generator Script
# This script generates the GitType banner image using freeze

export FORCE_COLOR=1
oh-my-logo "GitType" purple
echo
echo '"Show your AI who'\''s boss: just you, your keyboard,'
echo ' and your coding sins"'
echo
echo "────────────────────────────────────────────────"
echo
echo "Turn your own source code into typing challenges"
echo
echo "[*] Addictive gameplay  [>] Real-time feedback  [+] Track your progress"
echo
echo "github.com/unhappychoice/gittype"

# To regenerate the banner:
# chmod +x docs/images/banner-source.sh
# freeze --execute "docs/images/banner-source.sh" -o docs/images/gittype-banner.svg \
#   --theme "Dracula" --window --background="#1e1e2e" --margin 20 --padding 20 --border.radius 15