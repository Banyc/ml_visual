use std::sync::Arc;

use wasm_bindgen::prelude::*;

use crate::{
    canvas::Pixels2DWrapper,
    decision_tree::tree::{BinaryDecisionTreeDisplayDot, Example},
};

use super::tree::{BinaryDecisionTree, ExampleBatch};

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
        let Some(batch) = parse_examples(examples) else {
            return None;
        };
        let Some(mut tree) = BinaryDecisionTree::new(batch) else {
            return None;
        };
        tree.learn();
        Some(WasmBinaryDecisionTree { tree })
    }

    pub fn dot(&self, feature_names: &str) -> Option<String> {
        let Some(mut feature_names) = parse_feature_names(feature_names) else {
            return None;
        };
        feature_names.resize(self.tree.root().example_batch().features(), "?".into());
        let display = BinaryDecisionTreeDisplayDot::new(&self.tree, &feature_names);
        Some(display.to_string())
    }

    pub fn draw(&self, examples: &str, canvas: &mut Pixels2DWrapper) {
        let examples = parse_examples(examples);
        todo!()
    }
}

pub fn parse_examples(examples: &str) -> Option<ExampleBatch> {
    const CLASS_LIMIT: usize = 32;
    let examples = jsonc_parser::parse_to_serde_value(examples, &Default::default())
        .ok()
        .flatten();
    let Some(examples) = examples else {
        return None;
    };
    let examples: Option<Vec<Vec<f64>>> = serde_json::from_value(examples).ok();
    let Some(examples) = examples else {
        return None;
    };
    struct DatasetMeta {
        num_features: usize,
        max_label: usize,
    }
    let mut dataset = Vec::with_capacity(examples.len());
    let mut dataset_meta = None::<DatasetMeta>;
    for example in &examples {
        let Some(label) = example.last() else {
            return None;
        };
        let label = label.round() as usize;
        if label >= CLASS_LIMIT {
            return None;
        }
        let features = &example[..example.len() - 1];
        if let Some(dataset) = &mut dataset_meta {
            if features.len() != dataset.num_features {
                return None;
            }
            dataset.max_label = dataset.max_label.max(label);
        } else {
            dataset_meta = Some(DatasetMeta {
                num_features: features.len(),
                max_label: label,
            })
        }
        let example = Example::new(features.into(), label);
        dataset.push(Arc::new(example));
    }

    let Some(dataset_meta) = dataset_meta else {
        return None;
    };
    ExampleBatch::new(
        dataset.into(),
        dataset_meta.num_features,
        dataset_meta.max_label + 1,
    )
}

fn parse_feature_names(feature_names: &str) -> Option<Vec<String>> {
    let feature_names = jsonc_parser::parse_to_serde_value(feature_names, &Default::default())
        .ok()
        .flatten();
    let Some(feature_names) = feature_names else {
        return None;
    };
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
