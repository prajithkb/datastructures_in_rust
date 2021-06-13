use criterion::{criterion_group, criterion_main, Criterion};

pub fn thread_benchmark(c: &mut Criterion) {
    // let mut group = c.benchmark_group("Size in Bytes (power of 2)");
    // group.finish();
}

criterion_group!(benches, thread_benchmark);
criterion_main!(benches);
