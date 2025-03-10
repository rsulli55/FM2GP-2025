# Exercise 6: Shortest Path using power operation

The power operation is using tropical semiring operation: min of sums

## Rust implementation

semigroup.rs has the generic implementation of power multiplication

### Run tests, with a simple graph

```graphviz
digraph foo {
rankdir=LR
 0 -> 1 [label = "7"]
 0 -> 2 [label = "3"]
 1 -> 2 [label = "1"]
 1 -> 3 [label = "2"]
 1 -> 4 [label = "6"]
 2 -> 3 [label = "2"]
 3 -> 4 [label = "4"]
 1 -> 0 [label = "7"]
 2 -> 0 [label = "3"]
 2 -> 1 [label = "1"]
 3 -> 1 [label = "2"]
 4 -> 1 [label = "6"]
 3 -> 2 [label = "2"]
 4 -> 3 [label = "4"]
}
```

cargo test


## Q implementation

Solve shortest_path on a simple graph:

```
q shortest_path.q
```

# Exercise 6: Part 2
Polynomial evaluator using Horner's rule

```
q poly.q
```


