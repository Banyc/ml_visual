use getset::CopyGetters;
use serde::{Deserialize, Serialize};

/// Ref: <https://en.wikipedia.org/wiki/Linear_interpolation>
pub fn lerp(v: &std::ops::RangeInclusive<f64>, t: f64) -> f64 {
    assert!((0.0..=1.0).contains(&t));
    (1.0 - t) * v.start() + t * v.end()
}

pub trait StandardizedExt: Iterator {
    /// Standardizes the iterator based on the iterator itself.
    fn standardized(self) -> impl Iterator<Item = f64> + Clone;
    /// Fits a standard scaler from the elements of the iterator,
    /// so that you can use this scaler to standardize another iterator.
    fn standard_scaler(self) -> StandardScaler;
    /// Standardizes the iterator with a standard scaler.
    ///
    /// This only scales the iterator based on the provided `sc`,
    /// not on the iterator itself.
    fn standardized_with(self, sc: StandardScaler) -> impl Iterator<Item = f64> + Clone;
}
impl<T> StandardizedExt for T
where
    T: Iterator<Item = f64> + Clone,
{
    fn standardized(self) -> impl Iterator<Item = f64> + Clone {
        let sc = self.clone().standard_scaler();
        self.standardized_with(sc)
    }

    fn standard_scaler(self) -> StandardScaler {
        StandardScaler::fit(self)
    }

    fn standardized_with(self, sc: StandardScaler) -> impl Iterator<Item = f64> + Clone {
        self.map(move |x| (x - sc.mean()) / sc.standard_deviation())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, CopyGetters)]
/// Borrowed from `sklearn.preprocessing.StandardScaler` but only for one feature.
pub struct StandardScaler {
    #[getset(get_copy = "pub")]
    mean: f64,
    #[getset(get_copy = "pub")]
    standard_deviation: f64,
}
impl StandardScaler {
    pub fn new(mean: f64, standard_deviation: f64) -> Self {
        Self {
            mean,
            standard_deviation,
        }
    }

    /// Fits a standard scaler from the elements of the iterator,
    /// so that you can use this scaler to standardize another iterator.
    pub fn fit(examples: impl Iterator<Item = f64> + Clone) -> Self {
        let mean = examples.clone().mean();
        let standard_deviation = examples.standard_deviation();
        Self {
            mean,
            standard_deviation,
        }
    }
}

pub trait MeanExt: Iterator {
    fn mean(self) -> f64;
}
impl<T> MeanExt for T
where
    T: Iterator<Item = f64> + Clone,
{
    fn mean(self) -> f64 {
        let n: usize = self.clone().count();
        let sum: f64 = self.sum();
        sum / n as f64
    }
}

pub trait StandardDeviationExt: Iterator {
    fn standard_deviation(self) -> f64;
}
impl<T> StandardDeviationExt for T
where
    T: Iterator<Item = f64> + Clone,
{
    fn standard_deviation(self) -> f64 {
        let mean = self.clone().mean();
        let n: usize = self.clone().count();
        let sum_squared_error: f64 = self.map(|x| (x - mean).powi(2)).sum();
        let variance = sum_squared_error / n as f64;
        variance.sqrt()
    }
}
