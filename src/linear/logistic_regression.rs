use math::{ml::sigmoid, prob::Probability};
use wasm_bindgen::prelude::*;

use super::{
    models::{LinearTwoFeatureParam, MulticlassExample},
    net_input, parse_examples, standardize,
};

#[wasm_bindgen]
pub fn logistic_regression_learn_binary_class(
    examples: &str,
    param: LinearTwoFeatureParam,
    learning_rate: f64,
    regularization_parameter: f64,
) -> Option<LinearTwoFeatureParam> {
    logistic_regression_learn_multiclass(
        examples,
        1,
        param,
        learning_rate,
        regularization_parameter,
    )
}

#[wasm_bindgen]
pub fn logistic_regression_learn_multiclass(
    examples: &str,
    class: u8,
    param: LinearTwoFeatureParam,
    learning_rate: f64,
    regularization_parameter: f64,
) -> Option<LinearTwoFeatureParam> {
    let Some(examples) = parse_examples(examples) else {
        return None;
    };
    let examples = standardize(examples.into_iter());
    Some(learn(
        examples,
        class,
        param,
        learning_rate,
        regularization_parameter,
    ))
}

fn learn(
    examples: impl Iterator<Item = MulticlassExample> + Clone,
    class: u8,
    param: LinearTwoFeatureParam,
    learning_rate: f64,
    regularization_parameter: f64,
) -> LinearTwoFeatureParam {
    let example_and_diff = examples.clone().map(|example| {
        let net_input = net_input(param, example.feature());
        let activation = decision_function(net_input);

        // `example.y()`: the example label
        let diff = f64::from(example.y() == class) - activation.get();
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

    let gradient_at_b = -2.0 * sum_differences / n as f64;
    let gradient_at_w_1 = -2.0 * sum_x_1_weighted_differences / n as f64
        + regularization_parameter * param.w_1() / n as f64;
    let gradient_at_w_2 = -2.0 * sum_x_2_weighted_differences / n as f64
        + regularization_parameter * param.w_2() / n as f64;

    let change_b = -learning_rate * gradient_at_b;
    let change_w_1 = -learning_rate * gradient_at_w_1;
    let change_w_2 = -learning_rate * gradient_at_w_2;

    LinearTwoFeatureParam::new(
        param.w_1() + change_w_1,
        param.w_2() + change_w_2,
        param.b() + change_b,
    )
}

pub fn decision_function(net_input: f64) -> Probability {
    sigmoid(net_input)
}
