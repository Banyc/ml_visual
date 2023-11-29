use std::sync::Arc;

use getset::{CopyGetters, Getters};
use rand::{seq::SliceRandom, Rng};

#[derive(Debug, Clone, Getters, CopyGetters)]
pub struct Example {
    #[getset(get = "pub")]
    features: Arc<[f64]>,
    #[getset(get_copy = "pub")]
    true_label: usize,
}
impl Example {
    pub fn new(features: Arc<[f64]>, true_label: usize) -> Self {
        Self {
            features,
            true_label,
        }
    }

    pub fn feature_value(&self, feature: usize) -> f64 {
        self.features[feature]
    }
}

#[derive(Debug, Clone, Getters, CopyGetters)]
pub struct ExampleBatch {
    #[getset(get = "pub")]
    examples: Arc<[Arc<Example>]>,
    #[getset(get_copy = "pub")]
    features: usize,
    #[getset(get_copy = "pub")]
    classes: usize,
}
impl ExampleBatch {
    pub fn new(examples: Arc<[Arc<Example>]>, features: usize, classes: usize) -> Option<Self> {
        for example in examples.iter() {
            if example.true_label() >= classes {
                return None;
            }
            if example.features.len() != features {
                return None;
            }
        }
        Some(Self {
            examples,
            features,
            classes,
        })
    }

    pub fn from_examples(examples: Arc<[Arc<Example>]>) -> Option<Self> {
        struct Meta {
            num_features: usize,
            max_label: usize,
        }
        let mut meta = None::<Meta>;

        for example in examples.iter() {
            if let Some(meta) = &mut meta {
                if example.features().len() != meta.num_features {
                    return None;
                }
                meta.max_label = meta.max_label.max(example.true_label());
            } else {
                meta = Some(Meta {
                    num_features: example.features().len(),
                    max_label: example.true_label(),
                })
            }
        }
        let Some(meta) = meta else {
            return None;
        };
        ExampleBatch::new(examples, meta.num_features, meta.max_label + 1)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.examples.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn choose_multiple_with_replacement(
        &self,
        amount: usize,
        rng: &mut impl Rng,
    ) -> Option<Self> {
        if self.examples.is_empty() {
            return None;
        }

        let drawn = (0..amount)
            .map(|_| self.examples.choose(rng).unwrap())
            .map(Arc::clone)
            .collect();
        Some(Self::new(drawn, self.features, self.classes).unwrap())
    }
}
