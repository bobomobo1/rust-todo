#!/bin/bash

# Build the project in release mode
cargo build --release

# Move the binary to /usr/local/bin
sudo cp target/release/rust-todo /usr/local/bin/todo

echo "Installation complete. You can now use the 'todo' command."
