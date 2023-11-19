use olive_rs::{Pixel, RealPoint, RealSpace, BLUE, RED};
use wasm_bindgen::prelude::*;

use crate::{
    canvas::Pixels2DWrapper, linear::models::LinearTwoFeatureParam, math::StandardizedExt,
};

use self::models::BinaryClassExample;

use super::{models::TwoFeatures, prediction_function};

pub mod models {
    use wasm_bindgen::prelude::*;

    use crate::linear::models::TwoFeatures;

    #[wasm_bindgen]
    #[derive(Debug, Clone, Copy)]
    pub struct BinaryClassExample {
        feature: TwoFeatures,
        y: bool,
    }

    #[wasm_bindgen]
    impl BinaryClassExample {
        #[wasm_bindgen(constructor)]
        pub fn new(feature: TwoFeatures, y: bool) -> Self {
            Self { feature, y }
        }

        pub fn feature(&self) -> TwoFeatures {
            self.feature
        }

        pub fn y(&self) -> bool {
            self.y
        }
    }
}

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
    let Some(examples) = parse_examples(examples) else {
        return;
    };
    let examples = standardize(examples.into_iter());
    const R: f64 = 0.05;
    for example in examples {
        let color = match example.y() {
            true => BLUE,
            false => RED,
        };
        let point = RealPoint::new(example.feature().x_1(), example.feature().x_2());
        pixels
            .canvas()
            .fill_real_circle(&REAL_SPACE, point, R, color)
    }
}

pub fn parse_examples(examples: &str) -> Option<Vec<BinaryClassExample>> {
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
    let examples: Result<Vec<BinaryClassExample>, ()> = examples
        .into_iter()
        .map(|(x_1, x_2, y)| {
            let y = match y {
                1 => true,
                0 => false,
                _ => return Err(()),
            };
            let feature = TwoFeatures::new(x_1, x_2);
            Ok(BinaryClassExample::new(feature, y))
        })
        .collect();
    let Ok(examples) = examples else {
        return None;
    };
    Some(examples)
}

pub fn standardize(
    examples: impl Iterator<Item = BinaryClassExample> + Clone,
) -> impl Iterator<Item = BinaryClassExample> + Clone {
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
        .map(|(feature, example)| BinaryClassExample::new(feature, example.y()))
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
        assert!(examples[0].y());

        let examples = "[[3, 2.1, 1], [3, 2.1, 1]]";
        let examples = parse_examples(examples).unwrap();
        assert_eq!(examples.len(), 2);
        assert_eq!(examples[0].feature().x_1(), 3.0);
        assert_eq!(examples[0].feature().x_2(), 2.1);
        assert!(examples[0].y());
        assert_eq!(examples[1].feature().x_1(), 3.0);
        assert_eq!(examples[1].feature().x_2(), 2.1);
        assert!(examples[1].y());
    }
}
