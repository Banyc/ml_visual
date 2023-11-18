use olive_rs::{Pixel, RealPoint, RealSpace, BLUE, GREEN, RED};
use wasm_bindgen::prelude::*;

use crate::{canvas::Pixels2DWrapper, math::StandardizedExt};

use super::decision_function;

pub mod models {
    use wasm_bindgen::prelude::*;

    pub use crate::perceptron::models::*;

    #[wasm_bindgen]
    #[derive(Debug, Clone, Copy)]
    pub struct PerceptronExample2 {
        feature: PerceptronFeatureSet,
        y: u8,
    }

    #[wasm_bindgen]
    impl PerceptronExample2 {
        #[wasm_bindgen(constructor)]
        pub fn new(feature: PerceptronFeatureSet, y: u8) -> Self {
            Self { feature, y }
        }

        pub fn feature(&self) -> PerceptronFeatureSet {
            self.feature
        }

        pub fn y(&self) -> u8 {
            self.y
        }
    }
}

const X_1_RANGE: std::ops::RangeInclusive<f64> = -2.0..=2.0;
const X_2_RANGE: std::ops::RangeInclusive<f64> = -2.0..=2.0;
const REAL_SPACE: RealSpace = RealSpace::new(X_1_RANGE, X_2_RANGE);

#[wasm_bindgen]
pub fn perceptron_2_draw_classification(
    param_0: models::PerceptronParam,
    param_1: models::PerceptronParam,
    param_2: models::PerceptronParam,
    pixels: &mut Pixels2DWrapper,
) {
    pixels.canvas().fill_by_function(&REAL_SPACE, |point| {
        let x_1 = point.x();
        let x_2 = point.y();
        let feature = models::PerceptronFeatureSet::new(x_1, x_2);
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
pub fn perceptron_2_draw_examples(examples: &str, pixels: &mut Pixels2DWrapper) {
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

#[wasm_bindgen]
pub fn perceptron_2_adaline_learn(
    examples: &str,
    class: u8,
    param: models::PerceptronParam,
    learning_rate: f64,
) -> Option<models::PerceptronParam> {
    let Some(examples) = parse_examples(examples) else {
        return None;
    };
    let examples = standardize(examples.into_iter());
    Some(adaline_learn(examples, class, param, learning_rate))
}

fn parse_examples(examples: &str) -> Option<Vec<models::PerceptronExample2>> {
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
    let examples: Result<Vec<models::PerceptronExample2>, ()> = examples
        .into_iter()
        .map(|(x_1, x_2, y)| {
            let feature = models::PerceptronFeatureSet::new(x_1, x_2);
            Ok(models::PerceptronExample2::new(feature, y))
        })
        .collect();
    let Ok(examples) = examples else {
        return None;
    };
    Some(examples)
}

fn standardize(
    examples: impl Iterator<Item = models::PerceptronExample2> + Clone,
) -> impl Iterator<Item = models::PerceptronExample2> + Clone {
    let x_1 = examples
        .clone()
        .map(|example| example.feature().x_1())
        .standardized();
    let x_2 = examples
        .clone()
        .map(|example| example.feature().x_2())
        .standardized();

    x_1.zip(x_2)
        .map(|(x_1, x_2)| models::PerceptronFeatureSet::new(x_1, x_2))
        .zip(examples)
        .map(|(feature, example)| models::PerceptronExample2::new(feature, example.y()))
}

fn adaline_learn(
    examples: impl Iterator<Item = models::PerceptronExample2> + Clone,
    class: u8,
    param: models::PerceptronParam,
    learning_rate: f64,
) -> models::PerceptronParam {
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

    models::PerceptronParam::new(
        param.w_1() + change_w_1,
        param.w_2() + change_w_2,
        param.b() + change_b,
    )
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
