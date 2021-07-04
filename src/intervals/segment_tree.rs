//! The SegmentTree inspired by <https://cp-algorithms.com/data_structures/segment_tree.html>
use std::{
    fmt::Debug,
    io::{stdout, Write},
    ops::RangeInclusive,
    rc::Rc,
};

use colored::Colorize;

/// The SegmentTree. Inspired by <https://cp-algorithms.com/data_structures/segment_tree.html>
pub struct ArrayBasedSegmentTree<T: Debug + Default + Clone> {
    segments: Vec<T>,
    merge_fn: Box<dyn Fn(T, T) -> T>,
    size: usize,
}

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
fn contains(larger: &RangeInclusive<usize>, smaller: &RangeInclusive<usize>) -> bool {
    // return the current segment if the current range is within the query range
    *larger.start() <= *smaller.start() && *larger.end() >= *smaller.end()
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

pub struct DynamicSegmentTree<T: Debug + Default + Clone> {
    left_child: Option<Box<DynamicSegmentTree<T>>>,
    right_child: Option<Box<DynamicSegmentTree<T>>>,
    left: usize,
    right: usize,
    value: T,
    merge_fn: Rc<dyn Fn(T, T) -> T>,
}

impl<T: Debug + Default + Clone> Debug for DynamicSegmentTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[left={}, right={}, value={:?}]\nleft_child={:?}\nright_child={:?}",
            self.left, self.right, self.value, self.left_child, self.right_child
        ))
    }
}

impl<T: Debug + Default + Clone> DynamicSegmentTree<T> {
    /// Creates an instance of a Dynamic Segment Tree
    pub fn new(range: RangeInclusive<usize>, merge_fn: Rc<dyn Fn(T, T) -> T>) -> Self {
        DynamicSegmentTree::inner_new(range, merge_fn)
    }

    /// Creates an instance of a Dynamic Segment Tree
    pub fn new_with_values(values: &[T], merge_fn: Rc<dyn Fn(T, T) -> T>) -> Self {
        let mut dst = DynamicSegmentTree::new(0..=(values.len() - 1), merge_fn);
        values.iter().enumerate().for_each(|(i, v)| {
            dst.insert(i, v.clone());
        });
        dst
    }

    fn inner_new(range: RangeInclusive<usize>, merge_fn: Rc<dyn Fn(T, T) -> T>) -> Self {
        DynamicSegmentTree {
            left_child: None,
            right_child: None,
            left: *range.start(),
            right: *range.end(),
            merge_fn,
            value: T::default(),
        }
    }

    fn extend_if_needed(&mut self) {
        if self.left_child.is_none() && self.left < self.right {
            let mid = (self.left + self.right) / 2;
            // extend the left one
            self.left_child = Some(Box::new(DynamicSegmentTree::inner_new(
                self.left..=mid,
                self.merge_fn.clone(),
            )));
            // extend the left one
            self.right_child = Some(Box::new(DynamicSegmentTree::inner_new(
                mid + 1..=self.right,
                self.merge_fn.clone(),
            )));
        }
    }

    pub fn update(&mut self, index: usize, value: T) {
        self.insert(index, value);
    }
    /// Inserts a value at a given index
    pub fn insert(&mut self, index: usize, value: T) {
        // extend if needed.
        self.extend_if_needed();
        // merge the value
        self.value = self.merge_fn.as_ref()(self.value.clone(), value.clone());

        if let (Some(left_child), Some(right_child)) =
            (self.left_child.as_mut(), self.right_child.as_mut())
        {
            // insert on the left half, if it belongs to the left half
            if index <= left_child.right {
                left_child.insert(index, value)
            } else {
                // or it is in the right half
                right_child.insert(index, value);
            }
        }
    }

    /// Queries the value of a given range
    pub fn query(&self, range: RangeInclusive<usize>) -> Option<T> {
        if range.is_empty() {
            return None;
        }
        let q_start = *range.start();
        let q_end = *range.end();
        // If the current range holds the queried range, return that
        if q_start <= self.left && q_end >= self.right {
            return Some(self.value.clone());
            // if there is no overlap return none
        } else if q_start > self.right || q_end < self.left {
            return None;
            // if there is overlap, recurse.
        } else if let (Some(left_child), Some(right_child)) =
            (self.left_child.as_ref(), self.right_child.as_ref())
        {
            return merge(
                left_child.query(q_start..=q_end),
                right_child.query(q_start..=q_end),
                self.merge_fn.as_ref(),
            );
        }
        None
    }

    fn pretty_print_to<W: Write>(&self, write: &mut W) {
        fn pretty_print<T: Debug + Default + Clone, W: Write>(
            node: &DynamicSegmentTree<T>,
            prefix: String,
            last: bool,
            write: &mut W,
        ) {
            let prefix_current = if last { "`- " } else { "|- " };
            {
                let segment = format!("{}..={}", node.left, node.right);
                let value = format!("{:?}", node.value);
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
            if let (Some(left_child), Some(right_child)) =
                (node.left_child.as_ref(), node.right_child.as_ref())
            {
                pretty_print(left_child, prefix.clone(), false, write);
                pretty_print(right_child, prefix, true, write);
            }
        }
        writeln!(write, "{}", "DYNAMIC_SEGMENT_TREE".bold()).expect("write");
        pretty_print(&self, "  ".to_string(), true, write);
    }

    pub fn pretty_print(&self) {
        self.pretty_print_to(&mut stdout())
    }
}

#[cfg(test)]
mod tests {
    use colored::Colorize;
    use std::fmt::Debug;
    use std::io::Write;
    use std::ops::RangeInclusive;

    use super::DynamicSegmentTree;

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

    enum LeftOrRight {
        Left,
        Right,
    }

    fn get_child<T>(
        directions: Vec<LeftOrRight>,
        node: &DynamicSegmentTree<T>,
    ) -> Option<&DynamicSegmentTree<T>>
    where
        T: Default + Clone + Debug,
    {
        let mut result: Option<&DynamicSegmentTree<T>> = Some(node);
        directions.into_iter().for_each(|d| {
            result = match d {
                LeftOrRight::Left => result.unwrap().left_child.as_ref().map(|v| v.as_ref()),
                LeftOrRight::Right => result.unwrap().right_child.as_ref().map(|v| v.as_ref()),
            };
        });
        result
    }

    mod dynamic_segment_tree {
        use std::rc::Rc;

        use crate::intervals::segment_tree::{
            tests::{expect_output, get_child, sum, LeftOrRight},
            DynamicSegmentTree,
        };

        #[test]
        fn initializes_correctly() {
            let values: Vec<u32> = (1..=5).collect();
            let mut dst: DynamicSegmentTree<u32> =
                DynamicSegmentTree::new(0..=4, Rc::new(|a, b| a + b));
            for (i, v) in values.iter().enumerate() {
                dst.insert(i, *v);
            }
            let dst = Box::new(dst);
            assert_eq!(dst.value, 15);
            assert_eq!((dst.left, dst.right), (0, 4));
            let left_child = get_child(vec![LeftOrRight::Left], &dst).unwrap();
            assert_eq!(left_child.value, 6);
            assert_eq!((left_child.left, left_child.right), (0, 2));
            let right_child = get_child(vec![LeftOrRight::Right], &dst).unwrap();
            assert_eq!(right_child.value, 9);
            assert_eq!((right_child.left, right_child.right), (3, 4));
            let left_child = get_child(vec![LeftOrRight::Left, LeftOrRight::Left], &dst).unwrap();
            assert_eq!(left_child.value, 3);
            assert_eq!((left_child.left, left_child.right), (0, 1));
            let right_child = get_child(vec![LeftOrRight::Left, LeftOrRight::Right], &dst).unwrap();
            assert_eq!(right_child.value, 3);
            assert_eq!((right_child.left, right_child.right), (2, 2));
            let left_child = get_child(
                vec![LeftOrRight::Left, LeftOrRight::Left, LeftOrRight::Left],
                &dst,
            )
            .unwrap();
            assert_eq!(left_child.value, 1);
            assert_eq!((left_child.left, left_child.right), (0, 0));
            let right_child = get_child(
                vec![LeftOrRight::Left, LeftOrRight::Left, LeftOrRight::Right],
                &dst,
            )
            .unwrap();
            assert_eq!(right_child.value, 2);
            assert_eq!((right_child.left, right_child.right), (1, 1));
            let left_child = get_child(
                vec![
                    LeftOrRight::Left,
                    LeftOrRight::Left,
                    LeftOrRight::Left,
                    LeftOrRight::Left,
                ],
                &dst,
            );
            let right_child = get_child(
                vec![
                    LeftOrRight::Left,
                    LeftOrRight::Left,
                    LeftOrRight::Left,
                    LeftOrRight::Right,
                ],
                &dst,
            );
            assert_eq!(left_child.is_none(), true);
            assert_eq!(right_child.is_none(), true);
            let left_child = get_child(
                vec![
                    LeftOrRight::Left,
                    LeftOrRight::Left,
                    LeftOrRight::Right,
                    LeftOrRight::Left,
                ],
                &dst,
            );
            let right_child = get_child(
                vec![
                    LeftOrRight::Left,
                    LeftOrRight::Left,
                    LeftOrRight::Right,
                    LeftOrRight::Left,
                ],
                &dst,
            );
            assert_eq!(left_child.is_none(), true);
            assert_eq!(right_child.is_none(), true);
            let left_child = get_child(
                vec![LeftOrRight::Left, LeftOrRight::Right, LeftOrRight::Left],
                &dst,
            );
            let right_child = get_child(
                vec![LeftOrRight::Left, LeftOrRight::Right, LeftOrRight::Right],
                &dst,
            );
            assert_eq!(left_child.is_none(), true);
            assert_eq!(right_child.is_none(), true);
            let left_child = get_child(
                vec![LeftOrRight::Right, LeftOrRight::Right, LeftOrRight::Left],
                &dst,
            );
            let right_child = get_child(
                vec![LeftOrRight::Right, LeftOrRight::Right, LeftOrRight::Right],
                &dst,
            );
            assert_eq!(left_child.is_none(), true);
            assert_eq!(right_child.is_none(), true);
            let left_child = get_child(
                vec![LeftOrRight::Right, LeftOrRight::Left, LeftOrRight::Left],
                &dst,
            );
            let right_child = get_child(
                vec![LeftOrRight::Right, LeftOrRight::Left, LeftOrRight::Right],
                &dst,
            );
            assert_eq!(left_child.is_none(), true);
            assert_eq!(right_child.is_none(), true);
            let left_child = get_child(vec![LeftOrRight::Right, LeftOrRight::Left], &dst).unwrap();
            let right_child =
                get_child(vec![LeftOrRight::Right, LeftOrRight::Right], &dst).unwrap();
            assert_eq!(left_child.value, 4);
            assert_eq!((left_child.left, left_child.right), (3, 3));
            assert_eq!(right_child.value, 5);
            assert_eq!((right_child.left, right_child.right), (4, 4));
        }

        #[test]
        fn query_works() {
            let values: Vec<u32> = (1..=11).collect();
            let mut dst: DynamicSegmentTree<u32> =
                DynamicSegmentTree::new(0..=10, Rc::new(|a, b| a + b));
            for (i, v) in values.iter().enumerate() {
                dst.insert(i, *v);
            }
            assert_eq!(dst.query(0..=5), Some(sum(0..=5, values.as_slice())));
            assert_eq!(dst.query(2..=5), Some(sum(2..=5, values.as_slice())));
            assert_eq!(dst.query(5..=10), Some(sum(5..=10, values.as_slice())));
            assert_eq!(dst.query(6..=9), Some(sum(6..=9, values.as_slice())));
        }

        #[test]
        fn pretty_print_works() {
            let values: Vec<u32> = (1..=3).collect();
            let mut dst: DynamicSegmentTree<u32> =
                DynamicSegmentTree::new(0..=2, Rc::new(|a, b| a + b));
            for (i, v) in values.iter().enumerate() {
                dst.insert(i, *v);
            }
            let mut actual_write_buffer: Vec<u8> = vec![];
            let mut expected_write_buffer: Vec<u8> = vec![];
            dst.pretty_print_to(&mut actual_write_buffer);
            expect_output(&mut expected_write_buffer, "DYNAMIC_SEGMENT_TREE");
            assert_eq!(actual_write_buffer, expected_write_buffer);
        }
    }

    mod segment_tree {
        use crate::intervals::segment_tree::tests::{expect_output, sum};

        use super::super::ArrayBasedSegmentTree;

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
                    45, 15, 30, 6, 9, 13, 17, 3, 3, 4, 5, 6, 7, 8, 9, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
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
}
