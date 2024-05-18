use std::sync::Arc;

use getset::{CopyGetters, Getters};
use math::{
    transformer::{Estimate, EstimateExt, Transform, TransformExt},
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
        let (i, _) = self
            .classified_examples()
            .into_iter()
            .enumerate()
            .max_by_key(|(_, amount)| *amount)?;
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
        let meta = meta?;
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

    pub fn fit<'a, E: Estimate<f64, Output = T>, T: Transform<f64>>(
        &'a self,
        estimator: &'a E,
    ) -> impl Iterator<Item = Result<T, E::Err>> + '_ {
        (0..self.features).map(move |j| {
            self.examples
                .iter()
                .map(|x| x.feature_value(j))
                .fit(estimator)
        })
    }

    pub fn transform_by<T, E>(&self, transformers: impl Iterator<Item = T>) -> Self
    where
        T: Transform<f64, Err = E>,
        E: std::error::Error,
    {
        let feature_vectors =
            (0..self.features).map(|j| self.examples.iter().map(move |x| x.feature_value(j)));
        let feature_vectors = feature_vectors
            .zip(transformers)
            .map(|(f, t)| f.transform_by(t))
            .map(|transformed| transformed.collect::<Result<Vec<f64>, E>>());
        let feature_vectors = feature_vectors
            .collect::<Result<Vec<Vec<f64>>, E>>()
            .unwrap();
        let feature_vectors = feature_vectors
            .into_iter()
            .map(|v| v.into_iter())
            .collect::<Vec<_>>();
        let feature_zip = VecZip::new(feature_vectors);
        let label_vector = self.examples.iter().map(|x| x.true_label());

        let examples = feature_zip
            .zip(label_vector)
            .map(|(features, label)| Example::new(features.into(), label))
            .map(Arc::new);

        Self::new(examples.collect(), self.features, self.classes).unwrap()
    }

    pub fn fit_transform<E: Estimate<f64, Output = T, Err = T::Err>, T: Transform<f64>>(
        &self,
        estimator: &E,
    ) -> Result<Self, E::Err>
    where
        E::Output: Transform<f64>,
        E::Err: std::error::Error,
    {
        let t: Vec<_> = self.fit(estimator).collect::<Result<_, _>>()?;
        Ok(self.transform_by(t.into_iter()))
    }
}
