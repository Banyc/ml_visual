use math::transformer::{standard_scaler::StandardScalingEstimator, TransformExt};

use self::models::{MulticlassExample, TwoFeatures};

pub mod adaline;
pub mod draw;
pub mod logistic_regression;
pub mod perceptron;

pub mod models {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Debug, Clone, Copy)]
    pub struct LinearTwoFeatureParam {
        w_1: f64,
        w_2: f64,
        b: f64,
    }

    #[wasm_bindgen]
    impl LinearTwoFeatureParam {
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
    #[derive(Debug, Clone, Copy)]
    pub struct MulticlassExample {
        feature: TwoFeatures,
        y: u8,
    }

    #[wasm_bindgen]
    impl MulticlassExample {
        #[wasm_bindgen(constructor)]
        pub fn new(feature: TwoFeatures, y: u8) -> Self {
            Self { feature, y }
        }

        pub fn feature(&self) -> TwoFeatures {
            self.feature
        }

        pub fn y(&self) -> u8 {
            self.y
        }
    }

    #[wasm_bindgen]
    #[derive(Debug, Clone, Copy)]
    pub struct TwoFeatures {
        x_1: f64,
        x_2: f64,
    }

    impl TwoFeatures {
        pub fn new(x_1: f64, x_2: f64) -> Self {
            Self { x_1, x_2 }
        }

        pub fn x_1(&self) -> f64 {
            self.x_1
        }

        pub fn x_2(&self) -> f64 {
            self.x_2
        }
    }
}

pub fn net_input(param: models::LinearTwoFeatureParam, feature: models::TwoFeatures) -> f64 {
    // the net input function
    feature.x_1() * param.w_1() + feature.x_2() * param.w_2() + param.b()
}

pub fn prediction_function(
    param: models::LinearTwoFeatureParam,
    feature: models::TwoFeatures,
) -> bool {
    net_input(param, feature) >= 0.
}

pub fn standardize(
    examples: impl Iterator<Item = MulticlassExample> + Clone,
) -> impl Iterator<Item = MulticlassExample> + Clone {
    let x_1 = examples
        .clone()
        .map(|example| example.feature().x_1())
        .fit_transform(&StandardScalingEstimator)
        .unwrap();
    let x_1: Vec<f64> = x_1.collect::<Result<Vec<f64>, _>>().unwrap();
    let x_2 = examples
        .clone()
        .map(|example| example.feature().x_2())
        .fit_transform(&StandardScalingEstimator)
        .unwrap();
    let x_2: Vec<f64> = x_2.collect::<Result<Vec<f64>, _>>().unwrap();

    x_1.into_iter()
        .zip(x_2)
        .map(|(x_1, x_2)| TwoFeatures::new(x_1, x_2))
        .zip(examples)
        .map(|(feature, example)| MulticlassExample::new(feature, example.y()))
}

pub fn parse_examples(examples: &str) -> Option<Vec<MulticlassExample>> {
    let examples = jsonc_parser::parse_to_serde_value(examples, &Default::default())
        .ok()
        .flatten();
    let examples = examples?;
    let examples: Option<Vec<(f64, f64, u8)>> =
        serde_json::from_value::<Vec<(f64, f64, u8)>>(examples).ok();
    let examples = examples?;
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
