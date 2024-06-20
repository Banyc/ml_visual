use std::{
    num::{NonZeroU32, NonZeroUsize},
    sync::Arc,
};

use math::graphics::brew_colors;
use olive_rs::{FloatPoint, FloatSpace, Pixel, BLACK};
use wasm_bindgen::prelude::*;

use crate::{
    canvas::Pixels2DWrapper,
    example::{Example, ExampleBatch},
};

use super::Knn;

#[wasm_bindgen]
pub struct WasmKnnBuilder;
#[wasm_bindgen]
impl WasmKnnBuilder {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> WasmKnnBuilder {
        WasmKnnBuilder
    }

    pub fn build(&self, examples: &str) -> Option<WasmKnn> {
        WasmKnn::new(examples)
    }
}

#[wasm_bindgen]
pub struct WasmKnn {
    knn: Knn,
}
#[wasm_bindgen]
impl WasmKnn {
    fn new(examples: &str) -> Option<WasmKnn> {
        let batch = parse_examples(examples)?;
        let knn = Knn::fit(batch)?;
        Some(WasmKnn { knn })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &self,
        examples: &str,
        x_axis_start: f64,
        x_axis_end: f64,
        y_axis_start: f64,
        y_axis_end: f64,
        pixels: &mut Pixels2DWrapper,
        k: usize,
    ) {
        pixels.canvas().fill(BLACK);

        let Some(k) = NonZeroUsize::new(k) else {
            return;
        };
        let distance_p = NonZeroU32::new(2).unwrap();

        let colors = brew_colors(self.knn.example_batch().classes());
        let real_space = FloatSpace::new(x_axis_start..=x_axis_end, y_axis_start..=y_axis_end);

        // Draw decision boundaries
        pixels.canvas().fill_virtual_pixels(&real_space, |p| {
            let y_hat = self.knn.predict(&[p.x(), p.y()], k, distance_p);
            let (r, g, b) = colors[y_hat];
            fn dim(primary: u8) -> u8 {
                let primary = primary as f64 * 100.0 / u8::MAX as f64;
                primary as u8
            }
            Some(Pixel::new(dim(r), dim(g), dim(b), u8::MAX))
        });

        // Draw examples
        let Some(batch) = parse_examples(examples) else {
            return;
        };
        if batch.features() < 2 {
            return;
        }
        for example in batch.examples().iter() {
            let c = FloatPoint::new(example.feature_value(0), example.feature_value(1));
            const R: f64 = 0.05;
            // let y_hat = self.tree.predict(&[c.x(), c.y()]);
            let y = example.true_label();
            let Some((r, g, b)) = colors.get(y) else {
                return;
            };
            let color = Pixel::new(*r, *g, *b, u8::MAX);
            pixels
                .canvas()
                .fill_virtual_circle(&real_space, c, R, color);
        }
    }
}

pub fn parse_examples(examples: &str) -> Option<ExampleBatch> {
    const CLASS_LIMIT: usize = 32;
    let examples = jsonc_parser::parse_to_serde_value(examples, &Default::default())
        .ok()
        .flatten();
    let examples = examples?;
    let examples: Option<Vec<Vec<f64>>> = serde_json::from_value(examples).ok();
    let mut example_vectors = examples?;
    let mut examples = vec![];
    while let Some(example) = example_vectors.pop() {
        let label = example.last()?;
        let label = label.round() as usize;
        if label >= CLASS_LIMIT {
            return None;
        }
        let features = &example[..example.len() - 1];
        let example = Example::new(features.into(), label);
        examples.push(Arc::new(example));
    }

    ExampleBatch::from_examples(examples.into())
}
