use std::{num::NonZeroUsize, sync::Arc};

use getset::Getters;
use math::{
    statistics::DistanceExt,
    transformer::{
        standard_scaler::{StandardScaler, StandardScalingEstimator},
        Transform,
    },
};

use crate::example::{Example, ExampleBatch};

pub mod draw;

#[derive(Debug, Getters)]
pub struct Knn {
    #[getset(get = "pub")]
    example_batch: ExampleBatch,
    sc: Arc<[StandardScaler]>,
}
impl Knn {
    pub fn fit(example_batch: ExampleBatch) -> Option<Self> {
        if example_batch.examples().is_empty() {
            return None;
        }
        let Some(sc) = example_batch
            .fit(&StandardScalingEstimator)
            .collect::<Result<Arc<[_]>, _>>()
            .ok()
        else {
            return None;
        };
        let example_batch = example_batch.transform_by::<StandardScaler>(sc.iter().copied());
        Some(Self { example_batch, sc })
    }

    pub fn predict(&self, features: &[f64], k: NonZeroUsize, p: NonZeroUsize) -> usize {
        let features = features
            .iter()
            .zip(self.sc.iter())
            .map(|(x, sc)| sc.transform(*x));

        struct Neighbor<'caller> {
            distance: f64,
            example: &'caller Arc<Example>,
        }

        let mut neighbors: Vec<_> = self
            .example_batch
            .examples()
            .iter()
            .map(|example| {
                let distance = features
                    .clone()
                    .zip(example.features().iter().copied())
                    .distance(p);
                Neighbor { distance, example }
            })
            .collect();
        neighbors.sort_unstable_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        let k_neighbors = neighbors.iter().take(k.get());

        let mut distances = vec![0.0; self.example_batch.classes()];
        let mut counts = vec![0; self.example_batch.classes()];
        for neighbor in k_neighbors {
            let class = neighbor.example.true_label();
            counts[class] += 1;
            distances[class] += neighbor.distance;
        }

        let mut max_count = (0, vec![]);
        for (i, count) in counts.into_iter().enumerate() {
            match max_count.0.cmp(&count) {
                std::cmp::Ordering::Less => max_count = (count, vec![i]),
                std::cmp::Ordering::Equal => max_count.1.push(i),
                std::cmp::Ordering::Greater => (),
            }
        }

        // Resolve ties
        let first_class = max_count.1[0];
        let mut min_distance = (distances[first_class], Some(first_class));
        for i in max_count.1 {
            if distances[i] < min_distance.0 {
                min_distance = (distances[i], Some(i));
            }
        }
        min_distance.1.unwrap()
    }
}
