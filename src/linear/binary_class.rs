use olive_rs::{Pixel, RealSpace};
use wasm_bindgen::prelude::*;

use crate::{canvas::Pixels2DWrapper, linear::models::LinearTwoFeatureParam};

use super::{models::TwoFeatures, prediction_function, three_classes::draw_examples_three_classes};

const X_1_RANGE: std::ops::RangeInclusive<f64> = -2.0..=2.0;
const X_2_RANGE: std::ops::RangeInclusive<f64> = -2.0..=2.0;
const REAL_SPACE: RealSpace = RealSpace::new(X_1_RANGE, X_2_RANGE);

#[wasm_bindgen]
pub fn linear_draw_classification_binary_class(
    param: LinearTwoFeatureParam,
    pixels: &mut Pixels2DWrapper,
) {
    pixels.canvas().fill_by_function(&REAL_SPACE, |point| {
        let x_1 = point.x();
        let x_2 = point.y();
        let feature = TwoFeatures::new(x_1, x_2);
        let res = prediction_function(param, feature);
        let pixel = match res {
            true => Pixel::new(0, 0, 100, u8::MAX),
            false => Pixel::new(100, 0, 0, u8::MAX),
        };
        Some(pixel)
    });
}

#[wasm_bindgen]
pub fn draw_examples_binary_class(examples: &str, pixels: &mut Pixels2DWrapper) {
    draw_examples_three_classes(examples, pixels)
}
