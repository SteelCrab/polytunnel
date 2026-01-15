#!/bin/sh
# Setup script for git hooks
# Run this once after cloning the repository

git config core.hooksPath .githooks
chmod +x .githooks/*

echo "Git hooks configured successfully!"
