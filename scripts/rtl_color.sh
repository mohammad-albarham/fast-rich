#!/bin/bash
# RTL Demo with colors - works around fribidi's ANSI corruption
# Usage: ./scripts/rtl_color.sh

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Run the plain demo through fribidi, then colorize key lines
cargo run --example rtl_demo_plain --features rtl 2>/dev/null | fribidi --nopad | while IFS= read -r line; do
    case "$line" in
        *"═══"*)
            echo -e "${BLUE}${line}${NC}"
            ;;
        *"Fast-Rich RTL Demo"*)
            echo -e "${BOLD}${CYAN}${line}${NC}"
            ;;
        *".1"*|*".2"*|*".3"*|*".4"*|*".5"*|*".6"*|*".7"*)
            echo -e "${GREEN}${line}${NC}"
            ;;
        *"┌"*|*"├"*|*"└"*|*"│"*|*"┬"*|*"┴"*|*"┼"*|*"─"*)
            echo -e "${YELLOW}${line}${NC}"
            ;;
        *"RTL:"*)
            echo -e "${CYAN}${line}${NC}"
            ;;
        *"Demo completed"*|*"ﺡﺎﺠﻨﺑ"*)
            echo -e "${BOLD}${GREEN}${line}${NC}"
            ;;
        *)
            echo "$line"
            ;;
    esac
done
