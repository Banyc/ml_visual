use wasm_bindgen::prelude::*;

use crate::linear::models;

use super::{
    binary_class::{self, models::BinaryClassExample},
    prediction_function,
};

#[wasm_bindgen]
pub fn perceptron_learn_binary_class(
    examples: &str,
    param: models::LinearTwoFeatureParam,
    learning_rate: f64,
) -> Option<models::LinearTwoFeatureParam> {
    let Some(examples) = binary_class::parse_examples(examples) else {
        return None;
    };
    let examples = binary_class::standardize(examples.into_iter());
    Some(learn(examples, param, learning_rate))
}

fn learn(
    examples: impl Iterator<Item = BinaryClassExample>,
    param: models::LinearTwoFeatureParam,
    learning_rate: f64,
) -> models::LinearTwoFeatureParam {
    let example_and_y_hat = examples.map(|example| {
        let y_hat = prediction_function(param, example.feature());
        (example, y_hat)
    });
    example_and_y_hat.fold(param, |param, (example, y_hat)| {
        let diff = i8::from(example.y()) - i8::from(y_hat);
        let update = f64::from(diff) * learning_rate;

        let w_1 = param.w_1() + update * example.feature().x_1();
        let w_2 = param.w_2() + update * example.feature().x_2();
        let b = param.b() + update;
        models::LinearTwoFeatureParam::new(w_1, w_2, b)
    })
}
