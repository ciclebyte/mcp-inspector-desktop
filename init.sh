#!/bin/bash

# MCP Inspector Desktop - Development Environment Setup
# This script sets up and starts the development environment

set -e  # Exit on error

echo "🚀 MCP Inspector Desktop - Development Environment Setup"
echo "========================================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored output
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Check if we're in the project root
if [ ! -f "package.json" ] || [ ! -f "tauri.conf.json" ]; then
    print_error "Must be run from project root directory"
    exit 1
fi

# Step 1: Check Node.js
echo "📦 Checking Node.js..."
if ! command -v node &> /dev/null; then
    print_error "Node.js is not installed"
    echo "Please install Node.js from https://nodejs.org/"
    exit 1
fi
NODE_VERSION=$(node --version)
print_success "Node.js version: $NODE_VERSION"

# Step 2: Check Rust
echo ""
echo "🦀 Checking Rust..."
if ! command -v cargo &> /dev/null; then
    print_warning "Rust toolchain not found"
    echo "Please install Rust from https://rustup.rs/"
    echo "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
RUST_VERSION=$(rustc --version)
print_success "Rust version: $RUST_VERSION"

# Step 3: Install npm dependencies
echo ""
echo "📥 Installing npm dependencies..."
if [ ! -d "node_modules" ]; then
    npm install
    print_success "Dependencies installed"
else
    print_warning "node_modules already exists, skipping npm install"
    print_warning "Run 'npm install' manually if you need to update dependencies"
fi

# Step 4: Check Tauri CLI
echo ""
echo "🔧 Checking Tauri CLI..."
if ! npm list tauri &> /dev/null; then
    print_warning "Tauri CLI not found, installing..."
    npm install -D @tauri-apps/cli
fi
print_success "Tauri CLI ready"

# Step 5: Check for feature_list.json
echo ""
echo "📋 Checking project files..."
if [ ! -f "feature_list.json" ]; then
    print_warning "feature_list.json not found"
else
    print_success "feature_list.json found"
fi

if [ ! -f "claude-progress.txt" ]; then
    print_warning "claude-progress.txt not found"
else
    print_success "claude-progress.txt found"
fi

# Step 6: Start development server
echo ""
echo "🎯 Starting development server..."
echo ""
print_warning "Press Ctrl+C to stop the server"
echo ""

npm run tauri dev
