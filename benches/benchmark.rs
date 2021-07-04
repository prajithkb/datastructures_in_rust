use std::{ops::RangeInclusive, rc::Rc};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use datastructures_in_rust::intervals::{
    brute_force::BruteForce,
    segment_tree::{ArrayBasedSegmentTree, DynamicSegmentTree},
};
use rand::{thread_rng, Rng};

trait Query<T> {
    fn query(&self, range: RangeInclusive<usize>) -> Option<T>;
}

impl Query<i32> for ArrayBasedSegmentTree<i32> {
    fn query(&self, range: RangeInclusive<usize>) -> Option<i32> {
        self.query(range)
    }
}

impl Query<i32> for BruteForce<i32> {
    fn query(&self, range: RangeInclusive<usize>) -> Option<i32> {
        self.query(range)
    }
}

pub fn initializations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Interval_Initialization");
    for i in [
        (1..=10).collect::<Vec<i32>>(),
        (1..=100).collect::<Vec<i32>>(),
        (1..=1000).collect::<Vec<i32>>(),
        (1..=10000).collect::<Vec<i32>>(),
        (1..=100000).collect::<Vec<i32>>(),
    ]
    .iter()
    {
        group.bench_with_input(
            BenchmarkId::new("ArrayBasedSegmentTree", i.len()),
            i,
            |b, i| b.iter(|| ArrayBasedSegmentTree::new(i, Box::new(|a, b| a + b))),
        );
        group.bench_with_input(
            BenchmarkId::new("DynamicSegmentTree", i.len()),
            i,
            |b, i| b.iter(|| DynamicSegmentTree::new_with_values(i, Rc::new(|a, b| a + b))),
        );
    }
}

pub fn queries(c: &mut Criterion) {
    const MAX: i32 = 1000000;
    let values = (1..=MAX).collect::<Vec<i32>>();
    let st = ArrayBasedSegmentTree::new(&values, Box::new(|a, b| a + b));
    let dst = DynamicSegmentTree::new_with_values(&values, Rc::new(|a, b| a + b));
    let mut group = c.benchmark_group("Interval_Queries");
    for queries in [
        query_range(10, MAX),
        query_range(100, MAX),
        query_range(1000, MAX),
        query_range(10000, MAX),
        query_range(100000, MAX),
    ]
    .iter()
    {
        group.bench_with_input(
            BenchmarkId::new("ArrayBasedSegmentTree", queries.len()),
            queries,
            |b, queries| {
                b.iter(|| {
                    queries.iter().for_each(|q| {
                        st.query(q.clone());
                    })
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("DynamicSegmentTree", queries.len()),
            queries,
            |b, queries| {
                b.iter(|| {
                    queries.iter().for_each(|q| {
                        dst.query(q.clone());
                    })
                })
            },
        );
    }
}

pub fn updates(c: &mut Criterion) {
    const MAX: i32 = 1000000;
    let values = (1..=MAX).collect::<Vec<i32>>();
    let mut st = ArrayBasedSegmentTree::new(&values, Box::new(|a, b| a + b));
    let mut dst: DynamicSegmentTree<i32> =
        DynamicSegmentTree::new(0..=(values.len() - 1), Rc::new(|a, b| a + b));
    let mut group = c.benchmark_group("Interval_Updates");
    for updates in [
        query_range_single_element(10, MAX),
        query_range_single_element(100, MAX),
        query_range_single_element(1000, MAX),
        query_range_single_element(10000, MAX),
        query_range_single_element(100000, MAX),
    ]
    .iter()
    {
        group.bench_with_input(
            BenchmarkId::new("ArrayBasedSegmentTree", updates.len()),
            updates,
            |b, updates| {
                b.iter(|| {
                    updates.iter().for_each(|q| {
                        st.update(q.clone(), 100);
                    })
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("DynamicSegmentTree", updates.len()),
            updates,
            |b, updates| {
                b.iter(|| {
                    updates.iter().for_each(|q| {
                        dst.update(*q.start(), 100);
                    })
                })
            },
        );
    }
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

fn query_range_single_element(size: usize, max: i32) -> Vec<RangeInclusive<usize>> {
    let mut result = Vec::new();
    let max: usize = max as usize;
    let mut rng = thread_rng();
    while result.len() < size {
        let index = rng.gen_range(0..max - 1);
        result.push(index..=index);
    }
    result
}

criterion_group!(benches, initializations, queries, updates);
criterion_main!(benches);
