use std::ops::RangeInclusive;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use datastructures_in_rust::intervals::{brute_force::BruteForce, segment_tree::SegmentTree};
use rand::{thread_rng, Rng};

trait Query<T> {
    fn query(&self, range: RangeInclusive<usize>) -> Option<T>;
}

impl Query<i32> for SegmentTree<i32> {
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
        // (1..=10000).collect::<Vec<i32>>(),
        // (1..=100000).collect::<Vec<i32>>(),
        // (1..=1000000).collect::<Vec<i32>>(),
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("SegmentTree", i.len()), i, |b, i| {
            b.iter(|| SegmentTree::new(i, Box::new(|a, b| a + b)))
        });
        group.bench_with_input(BenchmarkId::new("BruteForce", i.len()), i, |b, i| {
            b.iter(|| BruteForce::new(i, Box::new(|a, b| a + b)))
        });
    }
}

pub fn queries(c: &mut Criterion) {
    const MAX: i32 = 1000000;
    let values = (1..=MAX).collect::<Vec<i32>>();
    let st = SegmentTree::new(&values, Box::new(|a, b| a + b));
    let bt = BruteForce::new(&values, Box::new(|a, b| a + b));
    let mut group = c.benchmark_group("Interval_Queries");
    for queries in [
        query_range(10, MAX),
        query_range(100, MAX),
        query_range(1000, MAX),
        // query_range(10000, MAX),
        // query_range(100000, MAX),
        // query_range(10000000, MAX),
    ]
    .iter()
    {
        group.bench_with_input(
            BenchmarkId::new("SegmentTree", queries.len()),
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
            BenchmarkId::new("BruteForce", queries.len()),
            queries,
            |b, queries| {
                b.iter(|| {
                    queries.iter().for_each(|q| {
                        bt.query(q.clone());
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

criterion_group!(benches, initializations);
criterion_main!(benches);
