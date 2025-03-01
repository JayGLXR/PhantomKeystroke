#!/bin/bash
# PhantomKeystroke Plugin Builder
# Script to build PhantomKeystroke plugins as shared libraries (.so files)

set -e

echo "PhantomKeystroke Plugin Builder"
echo "===============================\n"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust and Cargo must be installed first."
    echo "Install from https://rustup.rs/"
    exit 1
fi

# Create plugins directory if it doesn't exist
mkdir -p plugins_build

# Function to build a plugin
build_plugin() {
    local plugin_name="$1"
    local plugin_file="plugins/${plugin_name}_plugin.rs"
    
    if [ ! -f "$plugin_file" ]; then
        echo "Error: Plugin source file not found: $plugin_file"
        return 1
    fi
    
    echo "Building plugin: $plugin_name"
    
    # Create temporary Cargo directory
    local build_dir="plugins_build/$plugin_name"
    mkdir -p "$build_dir/src"
    
    # Create Cargo.toml
    cat > "$build_dir/Cargo.toml" << EOF
[package]
name = "${plugin_name}_plugin"
version = "0.1.0"
edition = "2021"

[lib]
name = "${plugin_name}_plugin"
crate-type = ["cdylib"]

[dependencies]
async-trait = "0.1.77"
serde = { version = "1.0.196", features = ["derive"] }
EOF
    
    # Add specific dependencies based on plugin type
    case "$plugin_name" in
        cobaltstrike)
            echo 'reqwest = { version = "0.11.24", features = ["json"] }' >> "$build_dir/Cargo.toml"
            echo 'serde_json = "1.0.113"' >> "$build_dir/Cargo.toml"
            ;;
        sliver)
            echo 'futures-util = "0.3.30"' >> "$build_dir/Cargo.toml"
            echo 'tokio = { version = "1.36.0", features = ["full"] }' >> "$build_dir/Cargo.toml"
            echo 'tokio-tungstenite = "0.21.0"' >> "$build_dir/Cargo.toml"
            echo 'serde_json = "1.0.113"' >> "$build_dir/Cargo.toml"
            ;;
        mythic)
            echo 'reqwest = { version = "0.11.24", features = ["json"] }' >> "$build_dir/Cargo.toml"
            echo 'serde_json = "1.0.113"' >> "$build_dir/Cargo.toml"
            echo 'tokio = { version = "1.36.0", features = ["rt"] }' >> "$build_dir/Cargo.toml"
            ;;
    esac
    
    # Copy plugin source
    cp "$plugin_file" "$build_dir/src/lib.rs"
    
    # Build the plugin
    (cd "$build_dir" && cargo build --release)
    
    # Copy the compiled library to the plugins directory
    mkdir -p libs
    cp "$build_dir/target/release/lib${plugin_name}_plugin.so" "libs/"
    
    echo "Plugin $plugin_name built successfully: libs/lib${plugin_name}_plugin.so\n"
}

# Build all plugins
echo "Building all plugins..."
build_plugin "cobaltstrike"
build_plugin "sliver"
build_plugin "mythic"

echo "All plugins built successfully!"
echo "Plugin libraries are available in the 'libs' directory."
echo ""
echo "To use a custom plugin with PhantomKeystroke:"
echo "1. Update config.toml with the path to the .so file:"
echo "   [plugin]"
echo "   name = \"custom\""
echo "   [plugin.parameters]"
echo "   path = \"libs/libX_plugin.so\""
echo ""
echo "2. Run PhantomKeystroke with: cargo run -- -p custom" 