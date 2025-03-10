use criterion::{criterion_group, criterion_main, Criterion};


fn fibo_benchmark(c: &mut Criterion) {
    
    c.bench_function("fibonacci powered", |bencher| bencher.iter(|| {
            for i in 1..10000 {
                fibonacci::fibonacci::fibonacci(i);
            }}));

    c.bench_function("fibonacci iterative", |bencher| bencher.iter(|| {
        for i in 1..10000 {
            fibonacci::fibonacci::iterative_fibonacci(i);
        }}));
    
}


criterion_group!(benches, fibo_benchmark);
criterion_main!(benches);
