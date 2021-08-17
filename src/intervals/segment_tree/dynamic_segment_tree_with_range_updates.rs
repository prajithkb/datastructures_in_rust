#[cfg(feature = "pretty_print")]
use colored::Colorize;
#[cfg(feature = "pretty_print")]
use std::io::stdout;
#[cfg(feature = "pretty_print")]
use std::io::Write;

use std::{fmt::Debug, ops::RangeInclusive, rc::Rc};

use super::merge;

/// Dynamic Segment Tree
/// https://cp-algorithms.com/data_structures/segment_tree.html#toc-tgt-13
pub struct DynamicSegmentTreeWithRangeUpdates<T: Debug + Default + Clone> {
    left_child: Option<Box<DynamicSegmentTreeWithRangeUpdates<T>>>,
    right_child: Option<Box<DynamicSegmentTreeWithRangeUpdates<T>>>,
    left: i64,
    right: i64,
    value: T,
    pending_child_update: Option<T>,
    merge_fn: Rc<dyn Fn(T, T) -> T>,
}

impl<T: Debug + Default + Clone> Debug for DynamicSegmentTreeWithRangeUpdates<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "left={}, right={}, value={:?}, p_c_u={:?}, left_child={:?}, right_child={:?}\n",
            self.left,
            self.right,
            self.value,
            self.pending_child_update,
            self.left_child.is_some(),
            self.right_child.is_some()
        ))
    }
}

impl<T: Debug + Default + Clone> DynamicSegmentTreeWithRangeUpdates<T> {
    /// Creates an instance of a Dynamic Segment Tree
    pub fn new(range: RangeInclusive<i64>, merge_fn: Rc<dyn Fn(T, T) -> T>) -> Self {
        DynamicSegmentTreeWithRangeUpdates::inner_new(range, merge_fn)
    }

    /// Creates an instance of a Dynamic Segment Tree
    pub fn new_with_values(values: &[T], merge_fn: Rc<dyn Fn(T, T) -> T>) -> Self {
        let mut dst =
            DynamicSegmentTreeWithRangeUpdates::new(0..=((values.len() - 1) as i64), merge_fn);
        values.iter().enumerate().for_each(|(i, v)| {
            let range = i as i64..=i as i64;
            dst.update(range, v.clone());
        });
        dst
    }

    fn inner_new(range: RangeInclusive<i64>, merge_fn: Rc<dyn Fn(T, T) -> T>) -> Self {
        DynamicSegmentTreeWithRangeUpdates {
            left_child: None,
            right_child: None,
            left: *range.start(),
            right: *range.end(),
            pending_child_update: None,
            merge_fn,
            value: T::default(),
        }
    }

    fn extend_if_needed(&mut self) {
        if self.left_child.is_none() && self.left < self.right {
            let delta = (self.right - self.left) / 2;
            let mid = self.left + delta;
            // extend the children
            self.left_child = Some(Box::new(DynamicSegmentTreeWithRangeUpdates::inner_new(
                self.left..=mid,
                self.merge_fn.clone(),
            )));
            self.right_child = Some(Box::new(DynamicSegmentTreeWithRangeUpdates::inner_new(
                mid + 1..=self.right,
                self.merge_fn.clone(),
            )));
        }
    }

    /// returns true if the given range overlaps with self
    pub fn overlaps_range(&self, other_range: &RangeInclusive<i64>) -> bool {
        !(*other_range.end() < self.left || *other_range.start() > self.right)
    }

    /// returns true if the given range is contained within  with self
    pub fn contains_range(&self, other_range: &RangeInclusive<i64>) -> bool {
        self.left <= *other_range.start() && self.right >= *other_range.end()
    }

    /// returns the range for self
    pub fn range(&self) -> RangeInclusive<i64> {
        self.left..=self.right
    }

    /// returns true if this is a leaf node
    pub fn is_leaf(&self) -> bool {
        self.left == self.right
    }

    /// Inserts a value at a given range
    pub fn update(&mut self, range: RangeInclusive<i64>, value: T) {
        // if outside bounds, nothing to do here.
        if !self.overlaps_range(&range) {
            return;
        }
        // extend if needed.
        self.extend_if_needed();
        let up_r_left = *range.start();
        let up_r_right = *range.end();
        // set the pending update value will be propagated to children as neccesary
        if !self.is_leaf() {
            self.pending_child_update = Some(value.clone());
        }
        // if the update range is bigger than the current range update and return
        if up_r_left <= self.left && up_r_right >= self.right {
            self.value = value;
            return;
        }
        // else we need to visit the children
        if let (Some(left_child), Some(right_child)) =
            (self.left_child.as_mut(), self.right_child.as_mut())
        {
            // Take the new value for children
            let new_value = self.pending_child_update.take().unwrap();
            // There are three possibilities now
            // 1. It is just within left
            // 2. It is just within right
            // 3. It is split across left and right
            if left_child.contains_range(&range) {
                // 1
                left_child.update(range, new_value);
            } else if right_child.contains_range(&range) {
                // 2
                right_child.update(range, new_value);
            } else {
                // 3
                left_child.update(up_r_left..=left_child.right, new_value.clone());
                right_child.update(right_child.left..=up_r_right, new_value);
            }
            let merge_fn = self.merge_fn.as_ref();
            self.value = merge_fn(left_child.value.clone(), right_child.value.clone());
        } else {
            panic!("impossible case");
        }
    }

    /// Queries the value of a given range
    pub fn query(&mut self, range: RangeInclusive<i64>) -> Option<T> {
        // println!("{:?}", range);
        // Invalid range
        if range.start() > range.end() {
            return None;
        }
        let q_left = *range.start();
        let q_right = *range.end();

        // if the query range is bigger than the current range update and return
        if q_left <= self.left && q_right >= self.right {
            return Some(self.value.clone());
        }
        // else we need to visit the children
        if let (Some(left_child), Some(right_child)) =
            (self.left_child.as_mut(), self.right_child.as_mut())
        {
            // if there are pending updates, apply them for children.
            if let Some(v) = self.pending_child_update.take() {
                left_child.update(left_child.range(), v.clone());
                right_child.update(right_child.range(), v);
            }
            // There are three possibilities now
            // 1. It is just within left
            // 2. It is just within right
            // 3. It is split across left and right
            if left_child.contains_range(&range) {
                // 1
                left_child.query(range)
            } else if right_child.contains_range(&range) {
                // 2
                right_child.query(range)
            } else {
                // 3
                return merge(
                    left_child.query(q_left..=left_child.right),
                    right_child.query(right_child.left..=q_right),
                    self.merge_fn.as_ref(),
                );
            }
        } else {
            None
        }
    }

    #[cfg(feature = "pretty_print")]
    fn pretty_print_to<W: Write>(&self, write: &mut W) {
        fn pretty_print<T: Debug + Default + Clone, W: Write>(
            node: &DynamicSegmentTreeWithRangeUpdates<T>,
            prefix: String,
            last: bool,
            write: &mut W,
        ) {
            let prefix_current = if last { "`- " } else { "|- " };
            {
                let segment = format!("{}..={}", node.left, node.right);
                let value = format!("{:?} (pending:{:?})", node.value, node.pending_child_update);
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
    use std::{cmp, ops::RangeInclusive, rc::Rc};

    use crate::intervals::segment_tree::dynamic_segment_tree_with_range_updates::DynamicSegmentTreeWithRangeUpdates;

    fn update(items: &mut [u32], range: RangeInclusive<usize>, value: u32) {
        items[range].iter_mut().for_each(|i| *i = value);
    }

    fn max(items: &[u32], range: RangeInclusive<usize>) -> Option<u32> {
        let v = &items[range];
        v.iter().max().copied()
    }

    #[test]
    fn update_and_query_works_for_max() {
        let mut values: Vec<u32> = vec![0; 14];
        //https://drive.google.com/file/d/1aURFiakwaUSisvfwwLboyfuQwiyZJuqQ/view?usp=sharing
        let mut dst: DynamicSegmentTreeWithRangeUpdates<u32> =
            DynamicSegmentTreeWithRangeUpdates::new(0..=13, Rc::new(cmp::max));
        for (i, v) in values.iter().enumerate() {
            dst.update(i as i64..=i as i64, *v);
        }

        assert_eq!(dst.query(7..=9), max(&values, 7..=9));
        assert_eq!(dst.query(0..=5), max(&values, 0..=5));

        let updates: Vec<(RangeInclusive<usize>, u32)> = vec![
            (0..=0, 1),
            (1..=4, 4),
            (1..=1, 1),
            (3..=4, 2),
            (4..=5, 2),
            (4..=8, 5),
            (9..=12, 4),
            (10..=13, 4),
            (6..=7, 2),
            (9..=10, 2),
        ];

        updates.iter().enumerate().for_each(|(_i, (range, v))| {
            let u = *v + max(&values, range.clone()).unwrap_or(0);
            update(&mut values, range.clone(), u);
            let (s, e) = (*range.start() as i64, *range.end() as i64);
            let c = dst.query(s..=e).unwrap_or(0);
            dst.update(s..=e, *v + c);
            // dst.pretty_print();
            for from in 0..=13 {
                for to in from + 1..=13 {
                    assert_eq!(
                        dst.query(from..=to),
                        max(&values, from as usize..=to as usize),
                        "\n Query: {}..={}\n",
                        from,
                        to
                    );
                }
            }
        });
    }
}
