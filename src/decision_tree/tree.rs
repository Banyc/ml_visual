use std::{
    borrow::Cow,
    cell::{Ref, RefCell},
    collections::VecDeque,
    fmt,
    rc::Rc,
    sync::Arc,
};

use getset::{CopyGetters, Getters};
use math::{
    graphics::brew_colors,
    prob::{FractionExt, Probability, WeightedSumExt},
    statistics::MeanExt,
};
use rand::{seq::SliceRandom, Rng};

use crate::example::ExampleBatch;

const CONSTANT_FEATURE_THRESHOLD: f64 = 1e-7;

#[derive(Debug)]
pub struct BinaryDecisionTree {
    root: Rc<RefCell<BinaryNode>>,
}
impl BinaryDecisionTree {
    /// - Ref: <https://github.com/scikit-learn/scikit-learn/blob/0816e0012ce6446f28ffbb5430e4afad2fa44125/sklearn/tree/_tree.pyx#L166>
    pub fn fit(example_batch: ExampleBatch, training_features: usize) -> Option<Self> {
        if example_batch.features() < training_features {
            return None;
        }
        let Some(root) = BinaryNode::new(example_batch) else {
            return None;
        };
        let root = Rc::new(RefCell::new(root));

        let mut breath_first_queue = VecDeque::new();
        breath_first_queue.push_back(Rc::clone(&root));
        while let Some(ptr) = breath_first_queue.pop_front() {
            let mut node = ptr.as_ref().borrow_mut();
            node.split_best(training_features);
            // Breath-first-search the other nodes
            if let Some(children) = node.children() {
                breath_first_queue.push_back(Rc::clone(children.left_ptr()));
                breath_first_queue.push_back(Rc::clone(children.right_ptr()));
            }
        }

        Some(Self { root })
    }

    pub fn predict(&self, features: &[f64]) -> usize {
        self.root.borrow().predict(features)
    }

    pub fn root(&self) -> Ref<'_, BinaryNode> {
        Ref::map(self.root.borrow(), |x| x)
    }
}
impl Clone for BinaryDecisionTree {
    fn clone(&self) -> Self {
        let root = self.root.borrow().clone();
        Self {
            root: Rc::new(RefCell::new(root)),
        }
    }
}

#[derive(Debug)]
/// Display the tree in Graphviz DOT language
pub struct BinaryDecisionTreeDisplayDot<'caller> {
    tree: &'caller BinaryDecisionTree,
    feature_names: &'caller [String],
}
impl<'caller> BinaryDecisionTreeDisplayDot<'caller> {
    /// # Panic
    ///
    /// If the number of feature names is not equal to the training features
    pub fn new(tree: &'caller BinaryDecisionTree, feature_names: &'caller [String]) -> Self {
        assert_eq!(tree.root().example_batch().features(), feature_names.len());
        Self {
            tree,
            feature_names,
        }
    }
}
impl fmt::Display for BinaryDecisionTreeDisplayDot<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut id = 0;
        let mut new_node_id = || -> String {
            let node_id = format!("n{id}");
            id += 1;
            node_id
        };

        struct Feature<'caller> {
            name: &'caller str,
        }

        fn write_pre_order(
            f: &mut fmt::Formatter<'_>,
            node: &BinaryNode,
            new_node_id: &mut impl FnMut() -> String,
            features: &[Feature],
            colors: &[(u8, u8, u8)],
        ) -> Result<String, fmt::Error> {
            let id = new_node_id();

            let children = node
                .children()
                .map(|c| (&features[c.cond_feature()].name, c.cond_threshold));
            let impurity = node.impurity();
            let examples = node.example_batch().examples().len();
            let values = node.example_batch().classified_examples();

            let mut label = String::new();
            if let Some((feature_name, threshold)) = children {
                label.push_str(&format!("{feature_name} <= {threshold}\\n"));
            }
            label.push_str(&format!("impurity = {impurity:.3}\\n"));
            label.push_str(&format!("examples = {examples}\\n"));
            label.push_str(&format!("values = {values:?}\\n"));

            let (max_index, max) = values
                .iter()
                .cloned()
                .enumerate()
                .max_by_key(|(_, x)| *x)
                .unwrap();
            let max_count = values.iter().filter(|x| **x == max).count();
            let color: Cow<str> = match max_count {
                1 => {
                    let (r, g, b) = colors[max_index];
                    format!("color=\"#{r:x}{g:x}{b:x}\"").into()
                }
                _ => "".into(),
            };

            let node_def = format!("{id}[shape=box label=\"{label}\" {color}]");
            writeln!(f, "{node_def}")?;

            if let Some(children) = node.children() {
                let left_id = write_pre_order(f, &children.left(), new_node_id, features, colors)?;
                let right_id =
                    write_pre_order(f, &children.right(), new_node_id, features, colors)?;
                // If `ordering="out"`, then the outedges of a node, that is, edges with the node as its tail node, must appear left-to-right in the same order in which they are defined in the input.
                // - Ref: <https://graphviz.org/docs/attrs/ordering/>
                let left_edge = format!("{id} -> {left_id} [ordering=\"out\"]");
                let right_edge = format!("{id} -> {right_id} [ordering=\"out\"]");
                writeln!(f, "{left_edge}")?;
                writeln!(f, "{right_edge}")?;
            }

            Ok(id)
        }

        let features: Vec<_> = self
            .feature_names
            .iter()
            .map(|name| Feature { name })
            .collect();

        let root = self.tree.root();
        let colors = brew_colors(root.example_batch().classes());

        writeln!(f, "digraph {{")?;
        write_pre_order(f, &root, &mut new_node_id, &features, &colors)?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

#[derive(Debug, Getters)]
pub struct BinaryNode {
    #[getset(get = "pub")]
    example_batch: ExampleBatch,
    children: Option<BinaryNodeChildren>,
}
impl BinaryNode {
    /// # Option
    ///
    /// Return `None` if the examples are empty
    pub fn new(example_batch: ExampleBatch) -> Option<Self> {
        if example_batch.is_empty() {
            return None;
        }
        Some(Self {
            example_batch,
            children: None,
        })
    }

    /// - Ref: <https://github.com/scikit-learn/scikit-learn/blob/c08afded996d08a7dde8441708ed9ca4cbb40559/sklearn/tree/_splitter.pyx#L289>
    pub fn split_best(&mut self, training_features: usize) {
        #[derive(Debug)]
        struct Best {
            ig: f64,
            children: BinaryNodeChildren,
        }
        let impurity = self.impurity();
        if impurity == 0.0 {
            return;
        }
        if self.example_batch.features() <= 1 {
            return;
        }
        // Shuffle features
        let mut features: Vec<_> = (0..self.example_batch.features()).collect();
        let (features, _) = features.partial_shuffle(&mut rand::thread_rng(), training_features);
        // For each feature from a part of the features
        let mut best = None::<Best>;
        for feature in features.iter().copied() {
            // Sort the feature values
            let mut examples: Vec<_> = self.example_batch.examples().iter().collect();
            examples.sort_unstable_by(|a, b| {
                a.feature_value(feature)
                    .partial_cmp(&b.feature_value(feature))
                    .unwrap()
            });
            let examples = examples;
            // Skip constant features
            if examples.last().unwrap().feature_value(feature)
                - examples.first().unwrap().feature_value(feature)
                < CONSTANT_FEATURE_THRESHOLD
            {
                continue;
            }
            // Loop through the feature values and find the best threshold
            for win in examples.windows(2) {
                let threshold = win
                    .iter()
                    .map(|example| example.feature_value(feature))
                    .mean();
                let Some(children) = self.split(feature, threshold) else {
                    continue;
                };
                let ig = children.information_gain(impurity);
                let set_best = move |best: &mut Option<Best>| {
                    *best = Some(Best { ig, children });
                };
                match &best {
                    Some(best_so_far) => {
                        if ig > best_so_far.ig {
                            set_best(&mut best);
                        }
                    }
                    None => set_best(&mut best),
                }
            }
        }
        // Split the node
        if let Some(best) = best {
            self.set_children(best.children);
        }
    }

    pub fn impurity(&self) -> f64 {
        impurity_from_classified_examples(self.example_batch.classified_examples().into_iter())
    }

    /// # Panic
    ///
    /// If the feature does not exists
    pub fn split(&self, feature: usize, threshold: f64) -> Option<BinaryNodeChildren> {
        assert!(feature < self.example_batch.features());
        let (left, right) = self.example_batch.examples().iter().fold(
            (vec![], vec![]),
            |(mut left, mut right), example| {
                let x = example.features()[feature];
                match side(x, threshold) {
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
        let (left, right) = match (left, right) {
            (Some(left), Some(right)) => (left, right),
            _ => return None,
        };
        Some(BinaryNodeChildren::new(feature, threshold, left, right))
    }

    pub fn set_children(&mut self, children: BinaryNodeChildren) {
        self.children = Some(children);
    }

    pub fn children(&self) -> Option<&BinaryNodeChildren> {
        self.children.as_ref()
    }

    /// # Panic
    ///
    /// If the number of features is not equal to the training features
    pub fn predict(&self, features: &[f64]) -> usize {
        assert_eq!(features.len(), self.example_batch.features());
        if let Some(children) = &self.children {
            return children.predict(features);
        }

        let (i, _) = self
            .example_batch
            .classified_examples()
            .into_iter()
            .enumerate()
            .max_by_key(|(_i, n)| *n)
            .unwrap();
        i
    }
}
impl Clone for BinaryNode {
    fn clone(&self) -> Self {
        Self {
            example_batch: self.example_batch.clone(),
            children: self.children.clone(),
        }
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

    pub(self) fn left_ptr(&self) -> &Rc<RefCell<BinaryNode>> {
        &self.left
    }

    pub(self) fn right_ptr(&self) -> &Rc<RefCell<BinaryNode>> {
        &self.right
    }

    pub fn left(&self) -> Ref<'_, BinaryNode> {
        Ref::map(self.left.borrow(), |x| x)
    }

    pub fn right(&self) -> Ref<'_, BinaryNode> {
        Ref::map(self.right.borrow(), |x| x)
    }

    pub fn information_gain(&self, parent_impurity: f64) -> f64 {
        let child = [&self.left, &self.right].into_iter();
        let child_impurity = child.clone().map(|node| node.borrow().impurity());
        let child_examples = child.map(|node| node.borrow().example_batch().len());
        information_gain(parent_impurity, child_impurity, child_examples)
    }

    pub fn predict(&self, features: &[f64]) -> usize {
        match side(features[self.cond_feature], self.cond_threshold) {
            true => self.left.borrow(),
            false => self.right.borrow(),
        }
        .predict(features)
    }
}
impl Clone for BinaryNodeChildren {
    fn clone(&self) -> Self {
        let left = self.left.borrow().clone();
        let right = self.right.borrow().clone();
        Self {
            cond_feature: self.cond_feature,
            cond_threshold: self.cond_threshold,
            left: Rc::new(RefCell::new(left)),
            right: Rc::new(RefCell::new(right)),
        }
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

pub trait ShuffleLastNExt {
    fn shuffle_last_n(&mut self, last_n: usize);
}
impl<T> ShuffleLastNExt for [T] {
    /// Shuffle only into the `last_n` elements
    ///
    /// Fisher-Yates-based algorithm
    fn shuffle_last_n(&mut self, last_n: usize) {
        assert!(self.len() >= last_n);
        let mut rng = rand::thread_rng();
        for i in (0..self.len()).rev().take(last_n) {
            let j = rng.gen_range(0..=i);
            self.swap(i, j);
        }
    }
}

fn side(feature_value: f64, threshold: f64) -> bool {
    feature_value <= threshold
}

#[cfg(test)]
mod tests {
    use math::float_ext::FloatExt;

    use crate::example::Example;

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

    #[test]
    fn test_tree() {
        let examples = [
            ([0.0, 0.0], 0),
            ([0.0, 0.0], 0),
            ([0.0, 0.0], 0),
            ([1.0, 0.0], 1),
            ([1.0, 0.0], 1),
            ([1.0, 0.0], 1),
            ([0.0, 1.0], 1),
            ([0.0, 1.0], 1),
            ([0.0, 1.0], 1),
            ([1.0, 1.0], 0),
            ([1.0, 1.0], 0),
            ([1.0, 1.0], 2),
        ];
        let examples: Arc<[Arc<Example>]> = examples
            .into_iter()
            .map(|(features, label)| Example::new(features.into(), label))
            .map(Arc::new)
            .collect();
        let batch = ExampleBatch::new(examples, 2, 3).unwrap();
        let tree = BinaryDecisionTree::fit(batch, 2).unwrap();
        dbg!(&tree);
        assert_eq!(tree.predict(&[0.0, 0.0]), 0);
        assert_eq!(tree.predict(&[1.0, 0.0]), 1);
        assert_eq!(tree.predict(&[0.0, 1.0]), 1);
        assert_eq!(tree.predict(&[1.0, 1.0]), 0);
        let features = ["x_1", "x_2"].map(|x| x.into());
        let display = BinaryDecisionTreeDisplayDot::new(&tree, &features);
        println!("{display}");
    }
}
