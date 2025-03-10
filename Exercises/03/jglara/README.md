# Exercise 3: Fibonacci using power operation

## Rust implementation

semigroup.rs has the generic implementation of power multiplication

### Run tests, to check that power fibonacci produces the same output as iterative version

cargo test

### Run benchmarks to compare powered version to iterative version. Calculates first 10.000 fibonacci numbers

cargo bench

```
    Running benches/fibo_bench.rs (target/release/deps/fibo_bench-83bf5dfd52dde0e6)
Benchmarking fibonacci powered: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 13.6s, or reduce sample count to 30.
fibonacci powered       time:   [135.32 ms 135.82 ms 136.42 ms]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

Benchmarking fibonacci iterative: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 90.9s, or reduce sample count to 10.
fibonacci iterative     time:   [926.48 ms 927.49 ms 928.66 ms]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
```

## Q implementation

Compare iterative fibonacci with powered fibonacci

q fib.q

```
KDB+ 4.1 2024.10.16 Copyright (C) 1993-2024 Kx Systems
l64/ 16(24)core 31878MB jglara jglara-legion-5-15ith6h 127.0.1.1 EXPIRE 2025.11.25 jglara@gmail.com KDB PLUS TRIAL #5023046

0 7200
10 560
```


