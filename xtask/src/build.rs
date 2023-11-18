use clap::Args;
use xshell::{cmd, Shell};

#[derive(Debug, Args)]
pub struct BuildArgs {}

impl BuildArgs {
    pub fn run(self) {
        let sh = Shell::new().unwrap();
        cmd!(sh, "cargo build --release --target wasm32-unknown-unknown")
            .run()
            .unwrap();
        sh.create_dir("assets/wasm").unwrap();
        cmd!(sh, "wasm-bindgen --out-dir assets/wasm --target web --no-typescript target/wasm32-unknown-unknown/release/ml_visual.wasm")
            .run()
            .expect("Solution: run `cargo install wasm-bindgen-cli`");
    }
}
