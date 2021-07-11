use std::ops::RangeInclusive;
use std::rc::Rc;

use datastructures_in_rust::intervals::brute_force::BruteForce;
use datastructures_in_rust::intervals::segment_tree::{ArrayBasedSegmentTree, DynamicSegmentTree};
use rand::{thread_rng, Rng};
#[test]
fn query_works() {
    let values = (1..=1000).collect::<Vec<i32>>();
    let st = ArrayBasedSegmentTree::new(&values, Box::new(|a, b| a + b));
    let dst: DynamicSegmentTree<i32> =
        DynamicSegmentTree::new_with_values(&values, Rc::new(|a, b| a + b));
    let bt = BruteForce::new(&values, Box::new(|a, b| a + b));
    let queries = query_range(values.len(), 1000);
    queries.iter().for_each(|q| {
        assert_eq!(st.query(q.clone()), bt.query(q.clone()));
        assert_eq!(
            dst.query(*q.start() as i64..=*q.end() as i64),
            bt.query(q.clone())
        );
    });
}

fn query_range(size: usize, max: i32) -> Vec<RangeInclusive<usize>> {
    let mut result = Vec::new();
    let max: usize = max as usize;
    let mut rng = thread_rng();
    while result.len() < size {
        let left = rng.gen_range(0..max - 1);
        let right = rng.gen_range(left..max);
        if left > right {
            result.push(right..=left);
        } else {
            result.push(left..=right);
        }
    }
    result
}
