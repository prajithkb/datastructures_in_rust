//! Segment tree module
//! Contains twp implementations - 1) array based and 2) dynamic
use std::ops::RangeInclusive;

pub mod array_based_segment_tree;
pub mod dynamic_segment_tree;
pub mod dynamic_segment_tree_with_range_updates;
#[inline]
fn split(start: usize, end: usize) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let left_index = start;
    let right_index = end;
    let middle_index = left_index + (right_index - left_index) / 2;
    (left_index..=middle_index, middle_index + 1..=right_index)
}

#[inline]
fn merge<T>(left: Option<T>, right: Option<T>, merge_fn: &dyn Fn(T, T) -> T) -> Option<T> {
    match (left, right) {
        (None, None) => None,
        (None, Some(v)) => Some(v),
        (Some(v), None) => Some(v),
        (Some(l), Some(r)) => Some(merge_fn(l, r)),
    }
}

#[inline]
fn contains<T: PartialEq + PartialOrd>(
    larger: &RangeInclusive<T>,
    smaller: &RangeInclusive<T>,
) -> bool {
    // return the current segment if the current range is within the query range
    *larger.start() <= *smaller.start() && *larger.end() >= *smaller.end()
}
