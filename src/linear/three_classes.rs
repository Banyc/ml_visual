use olive_rs::{Pixel, RealPoint, RealSpace, BLUE, GREEN, RED};
use wasm_bindgen::prelude::*;

use crate::{
    canvas::Pixels2DWrapper, linear::models::LinearTwoFeatureParam, math::StandardizedExt,
};

use super::{
    decision_function,
    models::{MulticlassExample, TwoFeatures},
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
        let score_0 = decision_function(param_0, feature);
        let score_1 = decision_function(param_1, feature);
        let score_2 = decision_function(param_2, feature);
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

pub fn parse_examples(examples: &str) -> Option<Vec<MulticlassExample>> {
    let examples = jsonc_parser::parse_to_serde_value(examples, &Default::default())
        .ok()
        .flatten();
    let Some(examples) = examples else {
        return None;
    };
    let examples: Option<Vec<(f64, f64, u8)>> =
        serde_json::from_value::<Vec<(f64, f64, u8)>>(examples).ok();
    let Some(examples) = examples else {
        return None;
    };
    let examples: Result<Vec<MulticlassExample>, ()> = examples
        .into_iter()
        .map(|(x_1, x_2, y)| {
            let feature = TwoFeatures::new(x_1, x_2);
            Ok(MulticlassExample::new(feature, y))
        })
        .collect();
    let Ok(examples) = examples else {
        return None;
    };
    Some(examples)
}

pub fn standardize(
    examples: impl Iterator<Item = MulticlassExample> + Clone,
) -> impl Iterator<Item = MulticlassExample> + Clone {
    let x_1 = examples
        .clone()
        .map(|example| example.feature().x_1())
        .standardized();
    let x_2 = examples
        .clone()
        .map(|example| example.feature().x_2())
        .standardized();

    x_1.zip(x_2)
        .map(|(x_1, x_2)| TwoFeatures::new(x_1, x_2))
        .zip(examples)
        .map(|(feature, example)| MulticlassExample::new(feature, example.y()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_examples() {
        let examples = "[]";
        let examples = parse_examples(examples).unwrap();
        assert!(examples.is_empty());

        let examples = "[[3, 2.1, 1]]";
        let examples = parse_examples(examples).unwrap();
        assert_eq!(examples.len(), 1);
        assert_eq!(examples[0].feature().x_1(), 3.0);
        assert_eq!(examples[0].feature().x_2(), 2.1);
        assert_eq!(examples[0].y(), 1);

        let examples = "[[3, 2.1, 1], [3, 2.1, 1]]";
        let examples = parse_examples(examples).unwrap();
        assert_eq!(examples.len(), 2);
        assert_eq!(examples[0].feature().x_1(), 3.0);
        assert_eq!(examples[0].feature().x_2(), 2.1);
        assert_eq!(examples[0].y(), 1);
        assert_eq!(examples[1].feature().x_1(), 3.0);
        assert_eq!(examples[1].feature().x_2(), 2.1);
        assert_eq!(examples[1].y(), 1);
    }
}
