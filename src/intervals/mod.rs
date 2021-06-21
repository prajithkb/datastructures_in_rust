pub mod brute_force;
pub mod segment_tree;

// pub trait Merge<Rhs = Self> {
//     fn merge(&self, other: &Rhs) -> Self;
// }

// impl Merge for i32 {
//     fn merge(&self, other: &Self) -> Self {
//         self + other
//     }
// }

pub trait Merge<T> {
    fn merge(this: T, that: T) -> T;
}
