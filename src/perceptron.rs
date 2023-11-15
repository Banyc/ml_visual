use olive_rs::{RealSpace, BLUE, RED};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::canvas::Pixels2DWrapper;

#[wasm_bindgen]
pub fn draw_perceptron_two_features(w_1: f64, w_2: f64, b: f64, pixels: &mut Pixels2DWrapper) {
    const X_1_RANGE: std::ops::RangeInclusive<f64> = -1.0..=1.0;
    const X_2_RANGE: std::ops::RangeInclusive<f64> = -1.0..=1.0;
    const REAL_SPACE: RealSpace = RealSpace::new(X_1_RANGE, X_2_RANGE);

    pixels.canvas().fill_by_function(&REAL_SPACE, |point| {
        let x_1 = point.x();
        let x_2 = point.y();
        let res = decision_function(x_1, w_1, x_2, w_2, b);
        let pixel = match res {
            true => BLUE,
            false => RED,
        };
        Some(pixel)
    });
}

fn decision_function(x_1: f64, w_1: f64, x_2: f64, w_2: f64, b: f64) -> bool {
    x_1 * w_1 + x_2 * w_2 + b >= 0.
}
