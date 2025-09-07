#!/bin/bash

# GitType installer script
# Usage: curl -sSL https://raw.githubusercontent.com/unhappychoice/gittype/main/install.sh | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
VERSION="latest"
INSTALL_DIR="/usr/local/bin"

# Help function
show_help() {
    cat << EOF
GitType installer script

Usage: $0 [options]

Options:
    -v, --version VERSION    Install specific version (default: latest)
    -d, --dir DIRECTORY      Install directory (default: /usr/local/bin)
    -h, --help              Show this help message

Examples:
    $0                      # Install latest version to /usr/local/bin
    $0 -v v0.5.0           # Install specific version
    $0 -d ~/.local/bin     # Install to user directory (no sudo required)

One-liner installation:
    curl -sSL https://raw.githubusercontent.com/unhappychoice/gittype/main/install.sh | bash
EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -d|--dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}" >&2
            show_help
            exit 1
            ;;
    esac
done

# Detect OS and architecture
detect_platform() {
    local os
    local arch
    
    case "$(uname -s)" in
        Darwin*)
            os="apple-darwin"
            ;;
        Linux*)
            os="unknown-linux-gnu"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            os="pc-windows-msvc"
            ;;
        *)
            echo -e "${RED}Unsupported operating system: $(uname -s)${NC}" >&2
            exit 1
            ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="aarch64"
            ;;
        *)
            echo -e "${RED}Unsupported architecture: $(uname -m)${NC}" >&2
            exit 1
            ;;
    esac
    
    echo "${arch}-${os}"
}

# Get latest version from GitHub API
get_latest_version() {
    curl -sSL https://api.github.com/repos/unhappychoice/gittype/releases/latest | \
        grep '"tag_name":' | \
        cut -d'"' -f4
}

# Check if sudo is needed for installation directory
check_sudo_needed() {
    local install_dir="$1"
    
    # If directory doesn't exist, check parent directory
    local check_dir="$install_dir"
    while [[ ! -d "$check_dir" && "$check_dir" != "/" ]]; do
        check_dir="$(dirname "$check_dir")"
    done
    
    # Test if we can write to the directory
    if [[ -w "$check_dir" ]]; then
        return 1  # No sudo needed
    else
        return 0  # Sudo needed
    fi
}

# Download and install gittype
install_gittype() {
    local platform
    local download_url
    local temp_dir
    local binary_name="gittype"
    
    platform=$(detect_platform)
    
    if [[ "$VERSION" == "latest" ]]; then
        echo -e "${BLUE}Fetching latest version...${NC}"
        VERSION=$(get_latest_version)
    fi
    
    if [[ "$platform" == *"pc-windows-msvc"* ]]; then
        download_url="https://github.com/unhappychoice/gittype/releases/download/${VERSION}/gittype-${VERSION}-${platform}.zip"
        binary_name="gittype.exe"
    else
        download_url="https://github.com/unhappychoice/gittype/releases/download/${VERSION}/gittype-${VERSION}-${platform}.tar.gz"
    fi
    
    echo -e "${BLUE}Installing GitType ${VERSION} for ${platform}...${NC}"
    echo -e "${BLUE}Download URL: ${download_url}${NC}"
    
    # Create temporary directory
    temp_dir=$(mktemp -d)
    trap "rm -rf $temp_dir" EXIT
    
    # Download archive
    echo -e "${BLUE}Downloading...${NC}"
    if ! curl -sSL "$download_url" -o "$temp_dir/gittype-archive"; then
        echo -e "${RED}Failed to download GitType. Please check if version ${VERSION} exists.${NC}" >&2
        exit 1
    fi
    
    # Extract archive
    echo -e "${BLUE}Extracting...${NC}"
    cd "$temp_dir"
    if [[ "$download_url" == *.zip ]]; then
        unzip -q gittype-archive
    else
        tar -xzf gittype-archive
    fi
    
    # Check if sudo is needed
    local use_sudo=""
    if check_sudo_needed "$INSTALL_DIR"; then
        echo -e "${YELLOW}⚠️  Installing to ${INSTALL_DIR} requires sudo privileges${NC}"
        
        # Check if sudo is available
        if ! command -v sudo >/dev/null 2>&1; then
            echo -e "${RED}Error: sudo is required but not available${NC}" >&2
            echo -e "${BLUE}💡 Try installing to a user directory instead:${NC}"
            echo -e "${BLUE}   $0 -d \$HOME/.local/bin${NC}"
            exit 1
        fi
        
        echo -e "${BLUE}You may be prompted for your password...${NC}"
        use_sudo="sudo"
    fi
    
    # Create install directory if it doesn't exist
    if [[ -n "$use_sudo" ]]; then
        sudo mkdir -p "$INSTALL_DIR"
    else
        mkdir -p "$INSTALL_DIR"
    fi
    
    # Install binary
    echo -e "${BLUE}Installing to ${INSTALL_DIR}...${NC}"
    if [[ -f "$binary_name" ]]; then
        if [[ -n "$use_sudo" ]]; then
            sudo cp "$binary_name" "$INSTALL_DIR/"
            sudo chmod +x "$INSTALL_DIR/$binary_name"
        else
            cp "$binary_name" "$INSTALL_DIR/"
            chmod +x "$INSTALL_DIR/$binary_name"
        fi
    else
        echo -e "${RED}Binary not found in archive${NC}" >&2
        exit 1
    fi
    
    echo -e "${GREEN}✅ GitType ${VERSION} installed successfully!${NC}"
    echo -e "${GREEN}   Location: ${INSTALL_DIR}/${binary_name}${NC}"
    
    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo -e "${YELLOW}⚠️  Warning: ${INSTALL_DIR} is not in your PATH${NC}"
        echo -e "${YELLOW}   Add it to your shell profile:${NC}"
        echo -e "${YELLOW}   export PATH=\"${INSTALL_DIR}:\$PATH\"${NC}"
        echo
    fi
    
    # Test installation
    if command -v gittype >/dev/null 2>&1; then
        echo -e "${GREEN}🎮 Ready to play! Run 'gittype' to start typing practice${NC}"
    else
        echo -e "${BLUE}💡 Run '${INSTALL_DIR}/gittype' to start typing practice${NC}"
    fi
}

# Main execution
main() {
    echo -e "${GREEN}GitType Installation Script${NC}"
    echo "================================="
    echo
    
    # Check required commands
    for cmd in curl tar; do
        if ! command -v $cmd >/dev/null 2>&1; then
            echo -e "${RED}Error: $cmd is required but not installed${NC}" >&2
            exit 1
        fi
    done
    
    install_gittype
}

main "$@"