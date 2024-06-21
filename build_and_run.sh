#!/bin/sh

# Ensure necessary dependencies are installed (if any additional ones are needed)
apt-get update && apt-get install -y \
    libssl-dev \

    pkg-config

# Build the application

cargo build --release


# Run the application
cargo run --release

