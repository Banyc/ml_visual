pub mod three_classes;
pub mod two_classes;

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
    #[derive(Debug, Clone, Copy)]
    pub struct PerceptronExample {
        feature: PerceptronFeatureSet,
        y: bool,
    }

    #[wasm_bindgen]
    impl PerceptronExample {
        #[wasm_bindgen(constructor)]
        pub fn new(feature: PerceptronFeatureSet, y: bool) -> Self {
            Self { feature, y }
        }

        pub fn feature(&self) -> PerceptronFeatureSet {
            self.feature
        }

        pub fn y(&self) -> bool {
            self.y
        }
    }

    #[wasm_bindgen]
    #[derive(Debug, Clone, Copy)]
    pub struct PerceptronFeatureSet {
        x_1: f64,
        x_2: f64,
    }

    impl PerceptronFeatureSet {
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
    param: models::PerceptronParam,
    feature: models::PerceptronFeatureSet,
) -> f64 {
    // the net input function
    feature.x_1() * param.w_1() + feature.x_2() * param.w_2() + param.b()
}

pub fn prediction_function(
    param: models::PerceptronParam,
    feature: models::PerceptronFeatureSet,
) -> bool {
    decision_function(param, feature) >= 0.
}
