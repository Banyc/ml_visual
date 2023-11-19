use wasm_bindgen::prelude::*;

use crate::linear::models::LinearTwoFeatureParam;

use super::{decision_function, models::MulticlassExample, standardize, three_classes};

#[wasm_bindgen]
pub fn adaline_learn_binary_class(
    examples: &str,
    param: LinearTwoFeatureParam,
    learning_rate: f64,
) -> Option<LinearTwoFeatureParam> {
    adaline_learn_multiclass(examples, 1, param, learning_rate)
}

#[wasm_bindgen]
pub fn adaline_learn_multiclass(
    examples: &str,
    class: u8,
    param: LinearTwoFeatureParam,
    learning_rate: f64,
) -> Option<LinearTwoFeatureParam> {
    let Some(examples) = three_classes::parse_examples(examples) else {
        return None;
    };
    let examples = standardize(examples.into_iter());
    Some(learn(examples, class, param, learning_rate))
}

fn learn(
    examples: impl Iterator<Item = MulticlassExample> + Clone,
    class: u8,
    param: LinearTwoFeatureParam,
    learning_rate: f64,
) -> LinearTwoFeatureParam {
    let example_and_diff = examples.clone().map(|example| {
        // Why `+ 0.5`:
        // - We set 0 as the classification threshold at `prediction_function`
        // - We get the error using direct comparison
        //   between the activation function and the example label
        // - The example label is either 0 or 1
        // - `+ 0.5` pushes the threshold to 0.5
        let activation = decision_function(param, example.feature()) + 0.5;

        // `example.y()`: the example label
        let diff = f64::from(example.y() == class) - activation;
        (example, diff)
    });
    let sum_differences: f64 = example_and_diff.clone().map(|(_, diff)| diff).sum();
    let sum_x_1_weighted_differences: f64 = example_and_diff
        .clone()
        .map(|(example, diff)| example.feature().x_1() * diff)
        .sum();
    let sum_x_2_weighted_differences: f64 = example_and_diff
        .clone()
        .map(|(example, diff)| example.feature().x_2() * diff)
        .sum();
    let n = examples.count();

    let gradient_at_b = -2.0 / n as f64 * sum_differences;
    let gradient_at_w_1 = -2.0 / n as f64 * sum_x_1_weighted_differences;
    let gradient_at_w_2 = -2.0 / n as f64 * sum_x_2_weighted_differences;

    let change_b = -learning_rate * gradient_at_b;
    let change_w_1 = -learning_rate * gradient_at_w_1;
    let change_w_2 = -learning_rate * gradient_at_w_2;

    LinearTwoFeatureParam::new(
        param.w_1() + change_w_1,
        param.w_2() + change_w_2,
        param.b() + change_b,
    )
}
