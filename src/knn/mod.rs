use std::sync::Arc;

use getset::Getters;

use crate::example::{Example, ExampleBatch};

pub mod draw;

#[derive(Debug, Getters)]
pub struct Knn {
    #[getset(get = "pub")]
    example_batch: ExampleBatch,
}
impl Knn {
    pub fn fit(example_batch: ExampleBatch) -> Option<Self> {
        if example_batch.examples().is_empty() {
            return None;
        }
        Some(Self { example_batch })
    }

    pub fn predict(&self, features: &[f64], k: usize) -> usize {
        struct Neighbor<'caller> {
            distance: f64,
            example: &'caller Arc<Example>,
        }

        fn distance(a: &[f64], b: &[f64]) -> f64 {
            assert_eq!(a.len(), b.len());
            let sum: f64 = a
                .iter()
                .copied()
                .zip(b.iter().copied())
                .map(|(a, b)| (a - b).abs())
                .sum();
            sum.sqrt()
        }

        let mut neighbors: Vec<_> = self
            .example_batch
            .examples()
            .iter()
            .map(|example| {
                let distance = distance(features, example.features());
                Neighbor { distance, example }
            })
            .collect();
        neighbors.sort_unstable_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        let k_neighbors: Vec<_> = neighbors
            .iter()
            .take(k)
            .map(|neighbor| Arc::clone(neighbor.example))
            .collect();
        let k_neighbors = ExampleBatch::new(
            k_neighbors.into(),
            self.example_batch.features(),
            self.example_batch.classes(),
        )
        .unwrap();
        k_neighbors.major_class().unwrap()
    }
}
