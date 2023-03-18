#!/bin/sh
set -e

# Set the Cargo and Rustup home directories
export CARGO_HOME="$HOME/.cargo"
export RUSTUP_HOME="$HOME/.rustup"

# Install Rust
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
export PATH="$CARGO_HOME/bin:$PATH"

# Install trunk
cargo install trunk

# Update wasm target
rustup target add wasm32-unknown-unknown

# Build the project
trunk build --release

# Move the built files to /www folder
mkdir www
# Replace 'dist' with the output directory generated by your build process if it's different
mv dist www/
