use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pausable_clock::*;
use instant::Instant;

fn now_benchmark(c: &mut Criterion) {
    let clock = PausableClock::default();

    let repeat = 1000;

    c.bench_function("Std Instant Now", |b| {
        b.iter(|| {
            for _ in 0..repeat {
                black_box(Instant::now());
            }
        })
    });
    c.bench_function("Pausable Clock Now", |b| {
        b.iter(|| {
            for _ in 0..repeat {
                black_box(clock.now());
            }
        })
    });
}

criterion_group!(benches, now_benchmark,);
criterion_main!(benches);
