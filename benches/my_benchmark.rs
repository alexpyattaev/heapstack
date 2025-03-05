use criterion::{black_box, criterion_group, criterion_main, Criterion};
use heapstack::{do_work_heap_alloc, do_work_stack_alloc_100, do_work_stack_alloc_1000};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("heap 1000", |b| {
        b.iter(|| black_box(do_work_heap_alloc(1000, 42)))
    });
    c.bench_function("stack 1000", |b| {
        b.iter(|| black_box(do_work_stack_alloc_1000(42)))
    });
    c.bench_function("heap 100", |b| {
        b.iter(|| black_box(do_work_heap_alloc(100, 42)))
    });
    c.bench_function("stack 100", |b| {
        b.iter(|| black_box(do_work_stack_alloc_100(42)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
