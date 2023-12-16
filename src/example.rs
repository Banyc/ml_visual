use std::sync::Arc;

use getset::{CopyGetters, Getters};
use math::{
    transformer::{TransformExt, Transformer},
    two_dim::VecZip,
};
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

    pub fn classified_examples(&self) -> Vec<usize> {
        let mut classified_examples = vec![0; self.classes()];
        self.examples().iter().for_each(|example| {
            classified_examples[example.true_label()] += 1;
        });
        classified_examples
    }

    pub fn major_class(&self) -> Option<usize> {
        let Some((i, _)) = self
            .classified_examples()
            .into_iter()
            .enumerate()
            .max_by_key(|(_, amount)| *amount)
        else {
            return None;
        };
        Some(i)
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

    pub fn fit<T: Transformer<Value = f64>>(&self) -> impl Iterator<Item = Result<T, T::Err>> + '_ {
        (0..self.features).map(|j| self.examples.iter().map(|x| x.feature_value(j)).fit())
    }

    pub fn transform_by<T: Transformer<Value = f64>>(
        &self,
        transformers: impl Iterator<Item = T>,
    ) -> Self {
        let feature_vectors =
            (0..self.features).map(|j| self.examples.iter().map(move |x| x.feature_value(j)));
        let feature_vectors = feature_vectors
            .zip(transformers)
            .map(|(f, t)| f.transform_by(t));
        let feature_zip = VecZip::new(feature_vectors.collect());
        let label_vector = self.examples.iter().map(|x| x.true_label());

        let examples = feature_zip
            .zip(label_vector)
            .map(|(features, label)| Example::new(features.into(), label))
            .map(Arc::new);

        Self::new(examples.collect(), self.features, self.classes).unwrap()
    }

    pub fn fit_transform<T: Transformer<Value = f64>>(&self) -> Result<Self, T::Err> {
        let t: Vec<_> = self.fit::<T>().collect::<Result<_, _>>()?;
        Ok(self.transform_by(t.into_iter()))
    }
}
