use clap::Args;
use xshell::{cmd, Shell};

#[derive(Debug, Args)]
pub struct DecompileArgs {}

impl DecompileArgs {
    pub fn run(self) {
        let sh = Shell::new().unwrap();
        cmd!(sh, "wasm-tools print assets/wasm/ml_visual_bg.wasm")
            .run()
            .expect("Solution: run `cargo install wasm-tools`");
    }
}
