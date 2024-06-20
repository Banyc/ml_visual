use std::sync::Arc;

use math::graphics::brew_colors;
use olive_rs::{FloatPoint, FloatSpace, Pixel, BLACK};
use wasm_bindgen::prelude::*;

use crate::{
    canvas::Pixels2DWrapper,
    decision_tree::tree::BinaryDecisionTreeDisplayDot,
    example::{Example, ExampleBatch},
};

use super::tree::BinaryDecisionTree;

#[wasm_bindgen]
pub struct WasmBinaryDecisionTreeBuilder;
#[wasm_bindgen]
impl WasmBinaryDecisionTreeBuilder {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> WasmBinaryDecisionTreeBuilder {
        WasmBinaryDecisionTreeBuilder
    }

    pub fn build(&self, examples: &str) -> Option<WasmBinaryDecisionTree> {
        WasmBinaryDecisionTree::new(examples)
    }
}

#[wasm_bindgen]
pub struct WasmBinaryDecisionTree {
    tree: BinaryDecisionTree,
}
#[wasm_bindgen]
impl WasmBinaryDecisionTree {
    fn new(examples: &str) -> Option<WasmBinaryDecisionTree> {
        let batch = parse_examples(examples)?;
        let training_features = batch.features();
        let tree = BinaryDecisionTree::fit(batch, training_features)?;
        Some(WasmBinaryDecisionTree { tree })
    }

    pub fn dot(&self, feature_names: &str) -> Option<String> {
        let mut feature_names = parse_feature_names(feature_names)?;
        feature_names.resize(self.tree.root().example_batch().features(), "?".into());
        let display = BinaryDecisionTreeDisplayDot::new(&self.tree, &feature_names);
        Some(display.to_string())
    }

    pub fn draw(
        &self,
        examples: &str,
        x_axis_start: f64,
        x_axis_end: f64,
        y_axis_start: f64,
        y_axis_end: f64,
        pixels: &mut Pixels2DWrapper,
    ) {
        pixels.canvas().fill(BLACK);

        let colors = brew_colors(self.tree.root().example_batch().classes());
        let real_space = FloatSpace::new(x_axis_start..=x_axis_end, y_axis_start..=y_axis_end);

        // Draw decision boundaries
        pixels.canvas().fill_virtual_pixels(&real_space, |p| {
            let y_hat = self.tree.predict(&[p.x(), p.y()]);
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

fn parse_feature_names(feature_names: &str) -> Option<Vec<String>> {
    let feature_names = jsonc_parser::parse_to_serde_value(feature_names, &Default::default())
        .ok()
        .flatten();
    let feature_names = feature_names?;
    let feature_names: Option<Vec<String>> = serde_json::from_value(feature_names).ok();
    feature_names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_examples() {
        let src = r#"[[0,0,2]]"#;
        let batch = parse_examples(src).unwrap();
        dbg!(&batch);
        let builder = WasmBinaryDecisionTreeBuilder::new();
        let tree = builder.build(src).unwrap();
        let dot = tree.dot(r#"["x_1", "x_2"]"#);
        dbg!(&dot);
    }
}
