cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/ml_visual.wasm assets/lib.wasm
simple-http-server ./
