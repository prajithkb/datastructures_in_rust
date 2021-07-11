#[cfg(feature = "pretty_print")]
use colored::Colorize;
use std::{
    fmt::Debug,
    io::{stdout, Write},
    ops::RangeInclusive,
    rc::Rc,
};

use super::merge;

/// Dynamic Segment Tree
/// https://cp-algorithms.com/data_structures/segment_tree.html#toc-tgt-13
pub struct DynamicSegmentTree<T: Debug + Default + Clone> {
    left_child: Option<Box<DynamicSegmentTree<T>>>,
    right_child: Option<Box<DynamicSegmentTree<T>>>,
    left: i64,
    right: i64,
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
    pub fn new(range: RangeInclusive<i64>, merge_fn: Rc<dyn Fn(T, T) -> T>) -> Self {
        DynamicSegmentTree::inner_new(range, merge_fn)
    }

    /// Creates an instance of a Dynamic Segment Tree
    pub fn new_with_values(values: &[T], merge_fn: Rc<dyn Fn(T, T) -> T>) -> Self {
        let mut dst = DynamicSegmentTree::new(0..=((values.len() - 1) as i64), merge_fn);
        values.iter().enumerate().for_each(|(i, v)| {
            dst.insert(i as i64, v.clone());
        });
        dst
    }

    fn inner_new(range: RangeInclusive<i64>, merge_fn: Rc<dyn Fn(T, T) -> T>) -> Self {
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
            let delta = (self.right - self.left) / 2;
            let mid = self.left + delta;
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

    pub fn update(&mut self, index: i64, value: T) {
        self.insert(index, value);
    }
    /// Inserts a value at a given index
    pub fn insert(&mut self, index: i64, value: T) {
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
    pub fn query(&self, range: RangeInclusive<i64>) -> Option<T> {
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

    #[cfg(feature = "pretty_print")]
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

    #[cfg(feature = "pretty_print")]
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

        use crate::intervals::segment_tree::dynamic_segment_tree::{
            tests::{expect_output, get_child, sum, LeftOrRight},
            DynamicSegmentTree,
        };

        #[test]
        fn initializes_correctly() {
            let values: Vec<u32> = (1..=5).collect();
            let mut dst: DynamicSegmentTree<u32> =
                DynamicSegmentTree::new(0..=4, Rc::new(|a, b| a + b));
            for (i, v) in values.iter().enumerate() {
                dst.insert(i as i64, *v);
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
                dst.insert(i as i64, *v);
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
                dst.insert(i as i64, *v);
            }
            let mut actual_write_buffer: Vec<u8> = vec![];
            let mut expected_write_buffer: Vec<u8> = vec![];
            dst.pretty_print_to(&mut actual_write_buffer);
            expect_output(&mut expected_write_buffer, "DYNAMIC_SEGMENT_TREE");
            assert_eq!(actual_write_buffer, expected_write_buffer);
        }
    }
}
