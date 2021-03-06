#[cfg(feature = "pretty_print")]
use colored::Colorize;
use std::{
    fmt::Debug,
    io::{stdout, Write},
    ops::RangeInclusive,
};

use super::{contains, merge, split};

/// The SegmentTree. Inspired by <https://cp-algorithms.com/data_structures/segment_tree.html>
pub struct ArrayBasedSegmentTree<T: Debug + Default + Clone> {
    segments: Vec<T>,
    merge_fn: Box<dyn Fn(T, T) -> T>,
    size: usize,
}

impl<T: Debug + Default + Clone> ArrayBasedSegmentTree<T> {
    /// Creates a new instance
    pub fn new(values: &[T], merge_fn: Box<dyn Fn(T, T) -> T>) -> Self {
        let size = values.len();
        let mut segments: Vec<T> = vec![T::default(); 4 * size];
        ArrayBasedSegmentTree::initialize(
            values,
            segments.as_mut(),
            0..=(values.len() - 1),
            0,
            &merge_fn,
        );
        ArrayBasedSegmentTree {
            segments,
            merge_fn,
            size,
        }
    }

    fn initialize(
        values: &[T],
        segments: &mut [T],
        range: RangeInclusive<usize>,
        segment_index: usize,
        merge_fn: &dyn Fn(T, T) -> T,
    ) -> T {
        let start = *range.start();
        let end = *range.end();
        if start == end {
            let index = start;
            segments[segment_index] = values[index].clone();
            values[index].clone()
        } else {
            let (left, right) = split(start, end);
            let left = ArrayBasedSegmentTree::initialize(
                values,
                segments,
                left,
                2 * segment_index + 1,
                merge_fn,
            );
            let right = ArrayBasedSegmentTree::initialize(
                values,
                segments,
                right,
                2 * segment_index + 2,
                merge_fn,
            );
            segments[segment_index] = merge_fn(left, right);
            segments[segment_index].clone()
        }
    }

    /// Queries the value given range. O (logN) operation
    pub fn query(&self, range: RangeInclusive<usize>) -> Option<T> {
        ArrayBasedSegmentTree::query_with_segments(
            &self.segments,
            &range,
            0..=self.size - 1,
            0,
            &self.merge_fn,
        )
    }

    fn query_with_segments(
        segments: &[T],
        query_range: &RangeInclusive<usize>,
        current_range: RangeInclusive<usize>,
        index: usize,
        merge_fn: &dyn Fn(T, T) -> T,
    ) -> Option<T> {
        let qr_start = *query_range.start();
        let qr_end = *query_range.end();
        let cr_start = *current_range.start();
        let cr_end = *current_range.end();
        // Return None if the query is outside the current range
        if qr_start > cr_end || qr_end < cr_start {
            return None;
        }
        // return the current segment if the current range is within the query range
        if contains(query_range, &current_range) {
            Some(segments[index].clone())
        } else {
            // There is partial overlap, we need to traverse this range to find the proper overlapping ranges.
            let (left, right) = split(cr_start, cr_end);
            let left = ArrayBasedSegmentTree::query_with_segments(
                segments,
                query_range,
                left,
                2 * index + 1,
                merge_fn,
            );
            let right = ArrayBasedSegmentTree::query_with_segments(
                segments,
                query_range,
                right,
                2 * index + 2,
                merge_fn,
            );
            merge(left, right, merge_fn)
        }
    }
    #[cfg(feature = "pretty_print")]
    fn pretty_print_to<W: Write>(&self, write: &mut W) {
        fn pretty_print<T: Debug, W: Write>(
            segments: &[T],
            index: usize,
            segment: RangeInclusive<usize>,
            prefix: String,
            last: bool,
            write: &mut W,
        ) {
            let prefix_current = if last { "`- " } else { "|- " };
            {
                let segment = format!("{:?}", segment);
                let value = format!("{:?}", segments[index]);
                writeln!(
                    write,
                    "{}{}[{}] {} {}",
                    prefix.green().bold(),
                    prefix_current.green().bold(),
                    segment.bold(),
                    "=>".blue(),
                    value.bold()
                )
                .expect("Write");
            }
            let prefix_child = if last { "   " } else { "|  " };
            let prefix = prefix + prefix_child;
            let start = *segment.start();
            let end = *segment.end();
            if start < end {
                let (left, right) = split(start, end);
                pretty_print(&segments, 2 * index + 1, left, prefix.clone(), false, write);
                pretty_print(&segments, 2 * index + 2, right, prefix, true, write);
            }
        }
        writeln!(write, "{}", "SEGMENT_TREE".bold()).expect("write");
        pretty_print(
            &self.segments,
            0,
            0..=self.size - 1,
            "  ".to_string(),
            true,
            write,
        );
    }
    /// Used to pretty print the segment tree.
    /// This is an expensive operation and should be used only for debugging.
    /// ```ignore
    /// # use datastructures_in_rust::intervals::segment_tree::SegmentTree;
    /// let st :SegmentTree<u32> = SegmentTree::new(&[1, 2, 3], Box::new(|a, b| a + b));
    /// st.pretty_print();
    /// ```
    /// will print the following output
    ///```text
    /// SEGMENT_TREE
    ///     `- [0..=2] => 6
    ///     |- [0..=1] => 3
    ///     |  |- [0..=0] => 1
    ///     |  `- [1..=1] => 2
    ///     `- [2..=2] => 3
    ///```
    #[cfg(feature = "pretty_print")]
    pub fn pretty_print(&self) {
        self.pretty_print_to(&mut stdout())
    }

    pub fn update(&mut self, range: RangeInclusive<usize>, value: T) {
        ArrayBasedSegmentTree::update_with_segments(
            &mut self.segments,
            &range,
            0..=self.size - 1,
            0,
            value,
            &self.merge_fn,
        );
    }

    fn update_with_segments(
        segments: &mut [T],
        update_range: &RangeInclusive<usize>,
        current_range: RangeInclusive<usize>,
        index: usize,
        value: T,
        merge_fn: &dyn Fn(T, T) -> T,
    ) -> T {
        let ur_start = *update_range.start();
        let ur_end = *update_range.end();
        let cr_start = *current_range.start();
        let cr_end = *current_range.end();
        // return the current value if there is no overlap
        if ur_start > cr_end || ur_end < cr_start {
            return segments[index].clone();
        }
        if cr_end == cr_start {
            segments[index] = value;
            return segments[index].clone();
        }
        // partial overlap
        let (left, right) = split(cr_start, cr_end);
        let left = ArrayBasedSegmentTree::update_with_segments(
            segments,
            update_range,
            left,
            2 * index + 1,
            value.clone(),
            merge_fn,
        );
        let right = ArrayBasedSegmentTree::update_with_segments(
            segments,
            update_range,
            right,
            2 * index + 2,
            value,
            merge_fn,
        );
        segments[index] = merge_fn(left, right);
        segments[index].clone()
    }
}

#[cfg(test)]
mod tests {
    use colored::Colorize;
    use std::io::Write;
    use std::ops::RangeInclusive;

    use crate::intervals::segment_tree::array_based_segment_tree::ArrayBasedSegmentTree;

    fn sum(range: RangeInclusive<usize>, items: &[u32]) -> u32 {
        let v = &items[range];
        // dbg!(v);
        v.iter().sum()
    }

    fn expect_output<W: Write>(expected_write_buffer: &mut W, title: &str) {
        writeln!(expected_write_buffer, "{}", title.bold()).expect("write");
        writeln!(
            expected_write_buffer,
            "{}{}[{}] {} {}",
            "  ".green().bold(),
            "`- ".green().bold(),
            "0..=2".bold(),
            "=>".blue(),
            "6".bold()
        )
        .expect("write");
        writeln!(
            expected_write_buffer,
            "{}{}[{}] {} {}",
            "     ".green().bold(),
            "|- ".green().bold(),
            "0..=1".bold(),
            "=>".blue(),
            "3".bold()
        )
        .expect("write");
        writeln!(
            expected_write_buffer,
            "{}{}[{}] {} {}",
            "     |  ".green().bold(),
            "|- ".green().bold(),
            "0..=0".bold(),
            "=>".blue(),
            "1".bold()
        )
        .expect("write");
        writeln!(
            expected_write_buffer,
            "{}{}[{}] {} {}",
            "     |  ".green().bold(),
            "`- ".green().bold(),
            "1..=1".bold(),
            "=>".blue(),
            "2".bold()
        )
        .expect("write");
        writeln!(
            expected_write_buffer,
            "{}{}[{}] {} {}",
            "     ".green().bold(),
            "`- ".green().bold(),
            "2..=2".bold(),
            "=>".blue(),
            "3".bold()
        )
        .expect("write");
    }

    #[test]
    fn initializes_correctly() {
        let values: Vec<u32> = (1..=5).collect();
        let st: ArrayBasedSegmentTree<u32> =
            ArrayBasedSegmentTree::new(&values, Box::new(|a, b| a + b));
        assert_eq!(
            st.segments,
            vec![15, 6, 9, 3, 3, 4, 5, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        let values: Vec<u32> = (1..=9).collect();
        let st: ArrayBasedSegmentTree<u32> =
            ArrayBasedSegmentTree::new(&values, Box::new(|a, b| a + b));
        assert_eq!(
            st.segments,
            vec![
                45, 15, 30, 6, 9, 13, 17, 3, 3, 4, 5, 6, 7, 8, 9, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn queries_correctly() {
        let values: Vec<u32> = (1..12).collect();
        let st: ArrayBasedSegmentTree<u32> =
            ArrayBasedSegmentTree::new(&values, Box::new(|a, b| a + b));
        assert_eq!(st.query(0..=5), Some(sum(0..=5, values.as_slice())));
        assert_eq!(st.query(2..=5), Some(sum(2..=5, values.as_slice())));
        assert_eq!(st.query(5..=10), Some(sum(5..=10, values.as_slice())));
        assert_eq!(st.query(6..=9), Some(sum(6..=9, values.as_slice())));
    }

    #[test]
    fn updates_correctly() {
        let mut values: Vec<u32> = (1..=5).collect();
        let mut st: ArrayBasedSegmentTree<u32> =
            ArrayBasedSegmentTree::new(&values, Box::new(|a, b| a + b));
        assert_eq!(st.query(0..=4), Some(sum(0..=4, values.as_slice())));
        st.update(0..=0, 1);
        values[0] = 1;
        assert_eq!(st.query(0..=4), Some(sum(0..=4, values.as_slice())));
        st.update(2..=4, 2);
        values[2..=4].iter_mut().for_each(|i| *i = 2);
        assert_eq!(st.query(0..=4), Some(sum(0..=4, values.as_slice())));
        st.update(3..=4, 10);
        values[3..=4].iter_mut().for_each(|i| *i = 10);
        assert_eq!(st.query(2..=4), Some(sum(2..=4, values.as_slice())));
    }

    #[test]
    fn pretty_print_prints_correctly() {
        let values: Vec<u32> = (1..=3).collect();
        let st: ArrayBasedSegmentTree<u32> =
            ArrayBasedSegmentTree::new(&values, Box::new(|a, b| a + b));
        let mut actual_write_buffer: Vec<u8> = vec![];
        let mut expected_write_buffer: Vec<u8> = vec![];
        st.pretty_print_to(&mut actual_write_buffer);
        expect_output(&mut expected_write_buffer, "SEGMENT_TREE");
        assert_eq!(actual_write_buffer, expected_write_buffer);
    }
}
