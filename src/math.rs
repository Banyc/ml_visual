/// Ref: <https://en.wikipedia.org/wiki/Linear_interpolation>
pub fn lerp(v: &std::ops::RangeInclusive<f64>, t: f64) -> f64 {
    assert!((0.0..=1.0).contains(&t));
    (1.0 - t) * v.start() + t * v.end()
}

pub trait StandardizedExt: Iterator {
    fn standardized(self) -> impl Iterator<Item = f64> + Clone;
}
impl<T> StandardizedExt for T
where
    T: Iterator<Item = f64> + Clone,
{
    fn standardized(self) -> impl Iterator<Item = f64> + Clone {
        let mean = self.clone().mean();
        let standard_deviation = self.clone().standard_deviation();
        self.map(move |x| (x - mean) / standard_deviation)
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
