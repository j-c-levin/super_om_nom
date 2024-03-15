#!/bin/bash

# Navigate to your project directory if necessary
# cd /path/to/your/project

# Command 1: Build your project
cargo build --release --target wasm32-unknown-unknown

# Command 2: Generate JavaScript bindings
wasm-bindgen --no-typescript --out-name super_om_nom --out-dir wasm --target web target/wasm32-unknown-unknown/release/super_om_nom.wasm

# Command 3: Copy assets to the wasm directory
cp -r assets wasm/