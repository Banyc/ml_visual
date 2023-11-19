use wasm_bindgen::prelude::*;

use crate::linear::models::LinearTwoFeatureParam;

use super::{models::MulticlassExample, prediction_function, standardize, three_classes};

#[wasm_bindgen]
pub fn perceptron_learn_binary_class(
    examples: &str,
    param: LinearTwoFeatureParam,
    learning_rate: f64,
) -> Option<LinearTwoFeatureParam> {
    perceptron_learn_multiclass(examples, 1, param, learning_rate)
}

#[wasm_bindgen]
pub fn perceptron_learn_multiclass(
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
    examples: impl Iterator<Item = MulticlassExample>,
    class: u8,
    param: LinearTwoFeatureParam,
    learning_rate: f64,
) -> LinearTwoFeatureParam {
    let example_and_y_hat = examples.map(|example| {
        let y_hat = prediction_function(param, example.feature());
        (example, y_hat)
    });
    example_and_y_hat.fold(param, |param, (example, y_hat)| {
        let diff = i8::from(example.y() == class) - i8::from(y_hat);
        let update = f64::from(diff) * learning_rate;

        let w_1 = param.w_1() + update * example.feature().x_1();
        let w_2 = param.w_2() + update * example.feature().x_2();
        let b = param.b() + update;
        LinearTwoFeatureParam::new(w_1, w_2, b)
    })
}
