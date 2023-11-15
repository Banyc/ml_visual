cargo build --release --target wasm32-unknown-unknown
mkdir assets/wasm
wasm-bindgen --out-dir assets/wasm --target web --no-typescript target/wasm32-unknown-unknown/release/ml_visual.wasm
simple-http-server ./
