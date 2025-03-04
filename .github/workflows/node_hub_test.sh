#!/bin/bash
set -euo

# List of ignored modules 
ignored_folders=("dora-internvl" "dora-parler" "dora-keyboard" "dora-microphone" "terminal-input")

# Get current working directory
dir=$(pwd)

# Get the base name of the directory (without the path)
base_dir=$(basename "$dir")

# Check if the directory name is in the ignored list
if [[ " ${ignored_folders[@]} " =~ " ${base_dir} " ]]; then
    echo "Skipping $base_dir as we cannot test it on the CI..."
else
    if [[ -f "Cargo.toml" && -f "pyproject.toml" ]]; then
        echo "Running build and tests for Rust project in $dir..."

        cargo check
        cargo clippy
        cargo build
        cargo test

        pip install "maturin[zig]"
        maturin build --zig
                
        # aarch64-unknown-linux-gnu
        rustup target add aarch64-unknown-linux-gnu
        maturin build --target aarch64-unknown-linux-gnu --zig
                
        # armv7-unknown-linux-musleabihf
        rustup target add armv7-unknown-linux-musleabihf
        maturin build --target armv7-unknown-linux-musleabihf --zig
    else
        if [ -f "$dir/Cargo.toml" ]; then
            echo "Running build and tests for Rust project in $dir..."
            cargo check
            cargo clippy
            cargo build
            cargo test
        else
            if [ -f "$dir/pyproject.toml" ]; then
            echo "CI: Installing in $dir..."
            uv venv --seed -p 3.11
            uv pip install .
            echo "CI: Running Linting in $dir..."
            uv run ruff check .
            echo "CI: Running Pytest in $dir..."
            uv run pytest
            fi
        fi 
    fi 
fi