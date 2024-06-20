use math::graphics::lerp;
use olive_rs::{FloatSpace, Pixel};
use wasm_bindgen::prelude::*;

use crate::{
    canvas::Pixels2DWrapper,
    linear::{
        logistic_regression,
        models::{LinearTwoFeatureParam, TwoFeatures},
        net_input, prediction_function,
    },
};

use super::three_classes::draw_examples_three_classes;

const X_1_RANGE: std::ops::RangeInclusive<f64> = -2.0..=2.0;
const X_2_RANGE: std::ops::RangeInclusive<f64> = -2.0..=2.0;
const REAL_SPACE: FloatSpace = FloatSpace::new(X_1_RANGE, X_2_RANGE);

#[wasm_bindgen]
pub fn linear_draw_classification_binary_class(
    param: LinearTwoFeatureParam,
    pixels: &mut Pixels2DWrapper,
) {
    pixels.canvas().fill_virtual_pixels(&REAL_SPACE, |point| {
        let x_1 = point.x();
        let x_2 = point.y();
        let feature = TwoFeatures::new(x_1, x_2);
        let y_hat = prediction_function(param, feature);
        let pixel = match y_hat {
            true => Pixel::new(0, 0, 100, u8::MAX),
            false => Pixel::new(100, 0, 0, u8::MAX),
        };
        Some(pixel)
    });
}

#[wasm_bindgen]
pub fn logistic_regression_draw_classification_binary_class(
    param: LinearTwoFeatureParam,
    pixels: &mut Pixels2DWrapper,
) {
    pixels.canvas().fill_virtual_pixels(&REAL_SPACE, |point| {
        let x_1 = point.x();
        let x_2 = point.y();
        let feature = TwoFeatures::new(x_1, x_2);
        let net_input = net_input(param, feature);
        let prob = logistic_regression::decision_function(net_input);
        let signed_color = lerp(&(-100.0..=100.0), prob);
        let color = signed_color.abs().round() as u8;
        let pixel = if signed_color >= 0.0 {
            Pixel::new(0, 0, color, u8::MAX)
        } else {
            Pixel::new(color, 0, 0, u8::MAX)
        };
        Some(pixel)
    });
}

#[wasm_bindgen]
pub fn draw_examples_binary_class(examples: &str, pixels: &mut Pixels2DWrapper) {
    draw_examples_three_classes(examples, pixels)
}
