use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn foo(c: &mut Criterion) {
    c.bench_function("foo_1000", |b| {
        b.iter(|| {
            (0..1000).for_each(|test_case| {
                black_box(black_box(test_case * 2) * black_box(test_case * 2));
            })
        });
    });
}

criterion_group!(benches, foo);
criterion_main!(benches);
