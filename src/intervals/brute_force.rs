//! Brute force O(N) ^2  module over an array

use std::ops::RangeInclusive;

pub struct BruteForce<T: Clone> {
    values: Vec<T>,
    merge_fn: Box<dyn Fn(T, T) -> T>,
}

impl<'a, T: Clone> BruteForce<T> {
    pub fn new(values: &[T], merge_fn: Box<dyn Fn(T, T) -> T>) -> Self {
        BruteForce {
            values: values.to_vec(),
            merge_fn,
        }
    }
    /// On^2 query operation
    pub fn query(&self, range: RangeInclusive<usize>) -> Option<T> {
        self.values[range].iter().cloned().reduce(&self.merge_fn)
    }

    /// On^2 query operation
    pub fn update(&mut self, range: RangeInclusive<usize>, value: T) {
        self.values[range]
            .iter_mut()
            .for_each(|v| *v = value.clone());
    }
}

#[cfg(test)]
mod test {
    use super::BruteForce;

    #[test]
    fn query_works() {
        let values = [1, 2, 3, 4, 5];
        let brute_force = BruteForce {
            values: values.to_vec(),
            merge_fn: Box::new(|a, b| a + b),
        };
        assert_eq!(brute_force.query(0..=2), Some(6));
        assert_eq!(brute_force.query(0..=4), Some(15));
        assert_eq!(brute_force.query(3..=4), Some(9));
        assert_eq!(brute_force.query(2..=2), Some(3));
    }
}
