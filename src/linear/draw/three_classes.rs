use olive_rs::{Pixel, RealPoint, RealSpace, BLUE, GREEN, RED};
use wasm_bindgen::prelude::*;

use crate::{
    canvas::Pixels2DWrapper,
    linear::{
        models::{LinearTwoFeatureParam, TwoFeatures},
        net_input, parse_examples, standardize,
    },
};

const X_1_RANGE: std::ops::RangeInclusive<f64> = -2.0..=2.0;
const X_2_RANGE: std::ops::RangeInclusive<f64> = -2.0..=2.0;
const REAL_SPACE: RealSpace = RealSpace::new(X_1_RANGE, X_2_RANGE);

#[wasm_bindgen]
pub fn linear_draw_classification_three_classes(
    param_0: LinearTwoFeatureParam,
    param_1: LinearTwoFeatureParam,
    param_2: LinearTwoFeatureParam,
    pixels: &mut Pixels2DWrapper,
) {
    pixels.canvas().fill_by_function(&REAL_SPACE, |point| {
        let x_1 = point.x();
        let x_2 = point.y();
        let feature = TwoFeatures::new(x_1, x_2);
        let score_0 = net_input(param_0, feature);
        let score_1 = net_input(param_1, feature);
        let score_2 = net_input(param_2, feature);
        let scores = [score_0, score_1, score_2];
        let max = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap();

        let pixel = match max {
            2 => Pixel::new(0, 100, 0, u8::MAX),
            1 => Pixel::new(0, 0, 100, u8::MAX),
            0 => Pixel::new(100, 0, 0, u8::MAX),
            _ => unreachable!(),
        };
        Some(pixel)
    });
}

#[wasm_bindgen]
pub fn draw_examples_three_classes(examples: &str, pixels: &mut Pixels2DWrapper) {
    let Some(examples) = parse_examples(examples) else {
        return;
    };
    let examples = standardize(examples.into_iter());
    const R: f64 = 0.05;
    for example in examples {
        let color = match example.y() {
            2 => GREEN,
            1 => BLUE,
            0 => RED,
            _ => unreachable!(),
        };
        let point = RealPoint::new(example.feature().x_1(), example.feature().x_2());
        pixels
            .canvas()
            .fill_real_circle(&REAL_SPACE, point, R, color)
    }
}
