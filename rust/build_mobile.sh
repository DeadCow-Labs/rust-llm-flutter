#!/bin/bash
set -e  # Exit on any error

# Get the directory containing the Rust project
RUST_DIR="$(pwd)"
FLUTTER_DIR="$(pwd)/flutter_package"

# Create directories
mkdir -p "$FLUTTER_DIR/ios/Frameworks"

# Build for iOS
echo "Building for iOS..."
cd "$RUST_DIR"

# Add iOS target
rustup target add aarch64-apple-ios

# Set minimum iOS version
export IPHONEOS_DEPLOYMENT_TARGET=11.0

# Clean previous builds
cargo clean

# Build for iOS with verbose output to see what's happening
echo "Building static library for iOS..."
RUST_BACKTRACE=1 cargo build --release --target aarch64-apple-ios -vv

if [ -f "target/aarch64-apple-ios/release/libllm_runner.a" ]; then
    mkdir -p "$FLUTTER_DIR/ios/Frameworks"
    cp "target/aarch64-apple-ios/release/libllm_runner.a" "$FLUTTER_DIR/ios/Frameworks/"
    echo "iOS build successful"
else
    echo "iOS build failed - library not found"
    exit 1
fi

echo "iOS build complete!" 