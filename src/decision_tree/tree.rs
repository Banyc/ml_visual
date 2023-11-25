use std::{cell::RefCell, collections::VecDeque, rc::Rc, sync::Arc};

use getset::{CopyGetters, Getters, MutGetters};
use math::prob::{FractionExt, Probability, WeightedSumExt};
use rand::Rng;

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
}

pub struct BinaryDecisionTree {
    root: Rc<RefCell<BinaryNode>>,
}
impl BinaryDecisionTree {
    pub fn new(example_batch: ExampleBatch) -> Self {
        let root = BinaryNode::new(example_batch);
        let root = Rc::new(RefCell::new(root));
        Self { root }
    }

    pub fn learn(&mut self) {
        let mut breath_first_queue = VecDeque::new();
        breath_first_queue.push_back(Rc::clone(&self.root));
        while let Some(ptr) = breath_first_queue.pop_front() {
            let mut node = ptr.as_ref().borrow_mut();
            node.split_best();
            // Breath-first-search the other nodes
            if let Some(children) = node.children() {
                breath_first_queue.push_back(Rc::clone(children.left_ptr()));
                breath_first_queue.push_back(Rc::clone(children.right_ptr()));
            }
        }
    }
}

#[derive(Debug, Getters, MutGetters)]
pub struct BinaryNode {
    example_batch: ExampleBatch,
    children: Option<BinaryNodeChildren>,
}
impl BinaryNode {
    pub fn new(example_batch: ExampleBatch) -> Self {
        Self {
            example_batch,
            children: None,
        }
    }

    pub fn split_best(&mut self) {
        // Shuffle features
        todo!();
        let mut best = None;
        // For each feature
        {
            // Sort the feature values
            todo!();
            // Loop through the feature values and find the best threshold
            todo!();
        }
        // Split the node
        if let Some((feature, threshold)) = best {
            self.split(feature, threshold);
        }
    }

    pub fn impurity(&self) -> f64 {
        let mut classified_examples = vec![];
        self.example_batch.examples().iter().for_each(|example| {
            let min_len = example.true_label() + 1;
            if classified_examples.len() < min_len {
                classified_examples.resize(min_len, 0);
            }
            classified_examples[example.true_label()] += 1;
        });
        impurity_from_classified_examples(classified_examples.into_iter())
    }

    pub fn split(&mut self, feature: usize, threshold: f64) {
        assert!(feature < self.example_batch.features());
        let (left, right) = self.example_batch.examples().iter().fold(
            (vec![], vec![]),
            |(mut left, mut right), example| {
                let x = example.features()[feature];
                match x <= threshold {
                    true => &mut left,
                    false => &mut right,
                }
                .push(Arc::clone(example));
                (left, right)
            },
        );
        let mut node = [left, right].into_iter().map(|examples| {
            let batch = ExampleBatch::new(
                examples.into(),
                self.example_batch.features(),
                self.example_batch.classes(),
            )
            .unwrap();
            BinaryNode::new(batch)
        });
        let left = node.next().unwrap();
        let right = node.next().unwrap();
        self.children = Some(BinaryNodeChildren::new(feature, threshold, left, right));
    }

    pub fn children(&self) -> Option<&BinaryNodeChildren> {
        self.children.as_ref()
    }
}

#[derive(Debug, CopyGetters)]
pub struct BinaryNodeChildren {
    #[getset(get_copy = "pub")]
    cond_feature: usize,
    #[getset(get_copy = "pub")]
    cond_threshold: f64,
    left: Rc<RefCell<BinaryNode>>,
    right: Rc<RefCell<BinaryNode>>,
}
impl BinaryNodeChildren {
    pub fn new(
        cond_feature: usize,
        cond_threshold: f64,
        left: BinaryNode,
        right: BinaryNode,
    ) -> Self {
        Self {
            cond_feature,
            cond_threshold,
            left: Rc::new(RefCell::new(left)),
            right: Rc::new(RefCell::new(right)),
        }
    }

    pub fn left_ptr(&self) -> &Rc<RefCell<BinaryNode>> {
        &self.left
    }

    pub fn right_ptr(&self) -> &Rc<RefCell<BinaryNode>> {
        &self.right
    }
}

pub fn impurity_from_classified_examples(
    classified_examples: impl Iterator<Item = usize> + Clone,
) -> f64 {
    gini_impurity(
        classified_examples
            .map(|x| x as f64)
            .fraction()
            .map(|x| x.unwrap()),
    )
    .unwrap()
}

pub fn information_gain_from_classified_examples(
    parent_classified_examples: impl Iterator<Item = usize> + Clone,
    child_classified_examples: &[impl Iterator<Item = usize> + Clone],
) -> f64 {
    let parent_impurity = impurity_from_classified_examples(parent_classified_examples);
    let child_impurity = child_classified_examples
        .iter()
        .cloned()
        .map(|c| impurity_from_classified_examples(c));

    let child_examples = child_classified_examples.iter().cloned().map(|c| c.sum());

    information_gain(parent_impurity, child_impurity, child_examples)
}

pub fn information_gain(
    parent_impurity: f64,
    child_impurity: impl Iterator<Item = f64>,
    child_examples: impl Iterator<Item = usize> + Clone,
) -> f64 {
    let child_weight = child_examples
        .map(|x| x as f64)
        .fraction()
        .map(|x| x.unwrap());
    let children_impurity = child_weight.zip(child_impurity).weighted_sum().unwrap();
    parent_impurity - children_impurity
}

pub fn entropy(prob_classified: impl Iterator<Item = Probability>) -> Option<f64> {
    prob_classified
        .filter_map(|p| {
            if p == Probability::impossibility() {
                return None;
            }
            Some((p, f64::log2(p.get())))
        })
        .weighted_sum()
        .map(|x| -x)
}

pub fn gini_impurity(prob_classified: impl Iterator<Item = Probability>) -> Option<f64> {
    fn prob_misclassification(classified: Probability) -> Probability {
        classified.complementary()
    }
    prob_classified
        .map(|p| (p, prob_misclassification(p).get()))
        .weighted_sum()
}

/// Fisher-Yates-based algorithm
pub fn shuffle<T>(a: &mut [T]) {
    let mut rng = rand::thread_rng();
    for i in (0..a.len()).rev() {
        let j = rng.gen_range(0..=i);
        a.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use math::float_ext::FloatExt;

    use super::*;

    #[test]
    fn test_entropy_even() {
        let data = [0.5, 0.5];
        let entropy = entropy(data.into_iter().map(|p| Probability::new(p).unwrap()));
        assert_eq!(entropy.unwrap(), 1.0);
    }

    #[test]
    fn test_entropy_zero() {
        let data = [1.0, 0.0];
        let entropy = entropy(data.into_iter().map(|p| Probability::new(p).unwrap()));
        assert_eq!(entropy.unwrap(), 0.0);
    }

    #[test]
    fn test_gini_impurity_even() {
        let data = [0.5, 0.5];
        let gini_impurity = gini_impurity(data.into_iter().map(|p| Probability::new(p).unwrap()));
        assert_eq!(gini_impurity.unwrap(), 0.5);
    }

    #[test]
    fn test_gini_impurity_zero() {
        let data = [1.0, 0.0];
        let gini_impurity = gini_impurity(data.into_iter().map(|p| Probability::new(p).unwrap()));
        assert_eq!(gini_impurity.unwrap(), 0.0);
    }

    #[test]
    fn test_ig() {
        let parent = [40, 40];
        let left_child = [30, 10];
        let right_child = [10, 30];
        let ig = information_gain_from_classified_examples(
            parent.into_iter(),
            &[left_child.into_iter(), right_child.into_iter()],
        );
        assert!(ig.closes_to(0.125));

        let parent = [40, 40];
        let left_child = [20, 40];
        let right_child = [20, 0];
        let ig = information_gain_from_classified_examples(
            parent.into_iter(),
            &[left_child.into_iter(), right_child.into_iter()],
        );
        assert!(ig.closes_to(1.0 / 6.0));
    }
}
