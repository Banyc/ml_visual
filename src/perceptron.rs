use olive_rs::{Pixel, RealPoint, RealSpace, BLUE, RED};
use wasm_bindgen::prelude::*;

use crate::canvas::Pixels2DWrapper;

pub mod models {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Debug, Clone, Copy)]
    pub struct PerceptronParam {
        w_1: f64,
        w_2: f64,
        b: f64,
    }

    #[wasm_bindgen]
    impl PerceptronParam {
        #[wasm_bindgen(constructor)]
        pub fn new(w_1: f64, w_2: f64, b: f64) -> Self {
            Self { w_1, w_2, b }
        }

        pub fn w_1(&self) -> f64 {
            self.w_1
        }

        pub fn w_2(&self) -> f64 {
            self.w_2
        }

        pub fn b(&self) -> f64 {
            self.b
        }
    }

    #[wasm_bindgen]
    pub struct PerceptronExample {
        x_1: f64,
        x_2: f64,
        y: bool,
    }

    #[wasm_bindgen]
    impl PerceptronExample {
        #[wasm_bindgen(constructor)]
        pub fn new(x_1: f64, x_2: f64, y: bool) -> Self {
            Self { x_1, x_2, y }
        }

        pub fn x_1(&self) -> f64 {
            self.x_1
        }

        pub fn x_2(&self) -> f64 {
            self.x_2
        }

        pub fn y(&self) -> bool {
            self.y
        }
    }
}

const X_1_RANGE: std::ops::RangeInclusive<f64> = -1.0..=1.0;
const X_2_RANGE: std::ops::RangeInclusive<f64> = -1.0..=1.0;
const REAL_SPACE: RealSpace = RealSpace::new(X_1_RANGE, X_2_RANGE);

#[wasm_bindgen]
pub fn perceptron_draw_classification(
    param: models::PerceptronParam,
    pixels: &mut Pixels2DWrapper,
) {
    pixels.canvas().fill_by_function(&REAL_SPACE, |point| {
        let x_1 = point.x();
        let x_2 = point.y();
        let res = decision_function(x_1, param.w_1(), x_2, param.w_2(), param.b());
        let pixel = match res {
            true => Pixel::new(0, 0, 100, u8::MAX),
            false => Pixel::new(100, 0, 0, u8::MAX),
        };
        Some(pixel)
    });
}

#[wasm_bindgen]
pub fn perceptron_draw_examples(examples: &str, pixels: &mut Pixels2DWrapper) {
    let Some(examples) = parse_examples(examples) else {
        return;
    };
    const R: f64 = 0.05;
    for example in examples {
        let color = match example.y() {
            true => BLUE,
            false => RED,
        };
        let point = RealPoint::new(example.x_1(), example.x_2());
        pixels
            .canvas()
            .fill_real_circle(&REAL_SPACE, point, R, color)
    }
}

#[wasm_bindgen]
pub fn perceptron_learn(
    examples: &str,
    param: models::PerceptronParam,
    learning_rate: f64,
) -> Option<models::PerceptronParam> {
    let Some(examples) = parse_examples(examples) else {
        return None;
    };
    Some(learn(examples.into_iter(), param, learning_rate))
}

fn parse_examples(examples: &str) -> Option<Vec<models::PerceptronExample>> {
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
    let examples: Result<Vec<models::PerceptronExample>, ()> = examples
        .into_iter()
        .map(|(x_1, x_2, y)| {
            let y = match y {
                1 => true,
                0 => false,
                _ => return Err(()),
            };
            Ok(models::PerceptronExample::new(x_1, x_2, y))
        })
        .collect();
    let Ok(examples) = examples else {
        return None;
    };
    Some(examples)
}

fn decision_function(x_1: f64, w_1: f64, x_2: f64, w_2: f64, b: f64) -> bool {
    x_1 * w_1 + x_2 * w_2 + b >= 0.
}

fn learn(
    examples: impl Iterator<Item = models::PerceptronExample>,
    param: models::PerceptronParam,
    learning_rate: f64,
) -> models::PerceptronParam {
    let example_and_y_hat = examples.map(|example| {
        let y_hat = decision_function(
            example.x_1(),
            param.w_1(),
            example.x_2(),
            param.w_2(),
            param.b(),
        );
        (example, y_hat)
    });
    example_and_y_hat.fold(param, |param, (example, y_hat)| {
        let diff = i8::from(example.y()) - i8::from(y_hat);
        let update = f64::from(diff) * learning_rate;

        let w_1 = param.w_1() + update * example.x_1();
        let w_2 = param.w_2() + update * example.x_2();
        let b = param.b() + update;
        models::PerceptronParam::new(w_1, w_2, b)
    })
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
        assert_eq!(examples[0].x_1(), 3.0);
        assert_eq!(examples[0].x_2(), 2.1);
        assert!(examples[0].y());

        let examples = "[[3, 2.1, 1], [3, 2.1, 1]]";
        let examples = parse_examples(examples).unwrap();
        assert_eq!(examples.len(), 2);
        assert_eq!(examples[0].x_1(), 3.0);
        assert_eq!(examples[0].x_2(), 2.1);
        assert!(examples[0].y());
        assert_eq!(examples[1].x_1(), 3.0);
        assert_eq!(examples[1].x_2(), 2.1);
        assert!(examples[1].y());
    }
}
