use crate::math::StandardizedExt;

use self::models::{MulticlassExample, TwoFeatures};

pub mod adaline;
pub mod binary_class;
pub mod perceptron;
pub mod three_classes;

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

pub fn decision_function(
    param: models::LinearTwoFeatureParam,
    feature: models::TwoFeatures,
) -> f64 {
    // the net input function
    feature.x_1() * param.w_1() + feature.x_2() * param.w_2() + param.b()
}

pub fn prediction_function(
    param: models::LinearTwoFeatureParam,
    feature: models::TwoFeatures,
) -> bool {
    decision_function(param, feature) >= 0.
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
