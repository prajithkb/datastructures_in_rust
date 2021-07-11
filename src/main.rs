use std::rc::Rc;

use datastructures_in_rust::{
    intervals::segment_tree::dynamic_segment_tree::DynamicSegmentTree, LeetCodeParser,
};
struct Solution;
fn less_than_half(v: i64) -> i64 {
    if v % 2 == 0 {
        v / 2 - 1
    } else {
        v / 2
    }
}
impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let min = *nums.iter().min().unwrap() as i64;
        let max = *nums.iter().max().unwrap() as i64;
        let mut total = 0;
        let mut dst: DynamicSegmentTree<i32> =
            DynamicSegmentTree::new(min..=max, Rc::new(|a, b| a + b));
        let int_min = i32::MIN as i64;
        nums.iter().rev().map(|v| *v as i64).for_each(|v| {
            dbg!(&v);
            dst.pretty_print();
            let range = dbg!(int_min..=less_than_half(v));
            total += dbg!(dst.query(range).unwrap_or(0));
            dst.insert(v, 1);
        });
        total
    }
}

fn main() {
    let mut lt = LeetCodeParser::new();
    let range: Vec<i32> = lt.next_arg_as_vec_of_nums::<_>().unwrap();
    println!("{}", Solution::reverse_pairs(range));
}
