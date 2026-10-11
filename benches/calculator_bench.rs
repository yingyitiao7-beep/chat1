use chat1::Ecpr;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn calculation_benchmark(c: &mut Criterion) {
    c.bench_function("calculator_simple", |b| {
        b.iter(|| {
            let mut expr = Ecpr::new(black_box("2+3*4"));
        });
    });
}

criterion_group!(benches, calculation_benchmark);
criterion_main!(benches);
