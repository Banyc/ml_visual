# `ml_visual`

## Build the WASM file

This requires `wasm-bindgen-cli` and the rustup target `wasm32-unknown-unknown`.

```bash
# Install the target
rustup target add wasm32-unknown-unknown

# Install the CLI
cargo install wasm-bindgen-cli

# Build the WASM file
cargo xtask build
```

## Serve the web

```bash
# Build the WASM file
cargo xtask build

# Or use other file servers to serve the current directory
python3 -m http.server

# Click this to view it on the browser:
# http://127.0.0.1:8000/index.html
```

## Decompile the built WASM file

This requires `wasm-tools`.

```bash
# Install the CLI
cargo install wasm-tools

# Optional: to pipe it somewhere else
cargo xtask decompile
```
