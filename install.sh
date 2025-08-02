#!/bin/bash

# Portman Installation Script
# This script installs the latest version of portman

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="yourusername/portman"  # Replace with actual GitHub username
BINARY_NAME="portman"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect platform
detect_platform() {
    local platform=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case "$platform" in
        linux*)
            if [[ "$arch" == "x86_64" ]]; then
                echo "linux"
            else
                log_error "Unsupported architecture: $arch"
                exit 1
            fi
            ;;
        darwin*)
            if [[ "$arch" == "x86_64" ]]; then
                echo "macos"
            elif [[ "$arch" == "arm64" ]]; then
                echo "macos-arm64"
            else
                log_error "Unsupported architecture: $arch"
                exit 1
            fi
            ;;
        *)
            log_error "Unsupported platform: $platform"
            exit 1
            ;;
    esac
}

# Get latest release version
get_latest_version() {
    local latest_url="https://api.github.com/repos/$REPO/releases/latest"
    
    if command -v curl >/dev/null 2>&1; then
        curl -s "$latest_url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$latest_url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    else
        log_error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi
}

# Download and install
install_portman() {
    local platform=$(detect_platform)
    local version=$(get_latest_version)
    
    if [[ -z "$version" ]]; then
        log_error "Failed to get latest version"
        exit 1
    fi
    
    log_info "Latest version: $version"
    log_info "Platform: $platform"
    log_info "Installing to: $INSTALL_DIR"
    
    # Determine archive name and extension
    local archive_name="portman-${platform}.tar.gz"
    local download_url="https://github.com/$REPO/releases/download/$version/$archive_name"
    
    # Create temporary directory
    local tmp_dir=$(mktemp -d)
    cd "$tmp_dir"
    
    log_info "Downloading $archive_name..."
    
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL -o "$archive_name" "$download_url"
    elif command -v wget >/dev/null 2>&1; then
        wget -q -O "$archive_name" "$download_url"
    else
        log_error "Neither curl nor wget is available"
        exit 1
    fi
    
    log_info "Extracting archive..."
    tar -xzf "$archive_name"
    
    # Find the binary in the extracted directory
    local binary_path=$(find . -name "$BINARY_NAME" -type f | head -n1)
    
    if [[ -z "$binary_path" ]]; then
        log_error "Binary not found in archive"
        exit 1
    fi
    
    # Install binary
    log_info "Installing binary to $INSTALL_DIR..."
    
    if [[ ! -d "$INSTALL_DIR" ]]; then
        log_warning "Creating directory $INSTALL_DIR"
        sudo mkdir -p "$INSTALL_DIR"
    fi
    
    if [[ -w "$INSTALL_DIR" ]]; then
        cp "$binary_path" "$INSTALL_DIR/$BINARY_NAME"
        chmod +x "$INSTALL_DIR/$BINARY_NAME"
    else
        sudo cp "$binary_path" "$INSTALL_DIR/$BINARY_NAME"
        sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
    fi
    
    # Cleanup
    cd /
    rm -rf "$tmp_dir"
    
    # Verify installation
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        log_success "Portman installed successfully!"
        echo
        log_info "Try running: $BINARY_NAME --help"
        echo
        "$BINARY_NAME" --version
    else
        log_warning "Installation completed, but $BINARY_NAME is not in PATH."
        log_info "You may need to add $INSTALL_DIR to your PATH or restart your shell."
    fi
}

# Main execution
main() {
    echo "🔧 Portman Installation Script"
    echo "=============================="
    echo
    
    # Check if already installed
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        local current_version=$("$BINARY_NAME" --version 2>/dev/null | head -n1 || echo "unknown")
        log_warning "Portman is already installed: $current_version"
        read -p "Do you want to update to the latest version? [y/N]: " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Installation cancelled"
            exit 0
        fi
    fi
    
    install_portman
}

# Run main function
main "$@"