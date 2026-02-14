#!/bin/sh

set -e

if ! command -v pre-commit >/dev/null 2>&1; then
    echo "pre-commit is not installed. Install with: pip install pre-commit"
    exit 1
fi

pre-commit install
echo "pre-commit installed successfully"
