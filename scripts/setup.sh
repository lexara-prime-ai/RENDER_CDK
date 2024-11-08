#!/bin/bash

# Define the LLVM version to install (e.g., 18)
LLVM_VERSION="18"

echo "Starting setup for LLVM $LLVM_VERSION..."

# Update package list and install LLVM, Clang, and lld
echo "Installing LLVM, Clang, and lld..."
sudo apt update
sudo apt install -y llvm-$LLVM_VERSION clang-$LLVM_VERSION lld

echo "Verifying installations..."

# Check LLVM version
echo -n "LLVM version: "
llvm-config --version

# Check Clang version
echo -n "Clang version: "
clang --version | head -n 1

echo "Setup completed. Please run 'source ~/.bashrc' to refresh environment variables if needed."