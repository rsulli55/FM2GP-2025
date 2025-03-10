use ndarray::Array2;

use crate::semigroup::power;


pub fn shortest_path(a: Array2<u64>) -> Array2<u64> {
    let len = a.dim().0;

    let sp = power(a, len as u64 - 1, |a, b| {
        let mut result = Array2::from_elem(a.dim(), u64::MAX);
        for ((i, j), elem) in result.indexed_iter_mut() {
            *elem = a
                .row(i)
                .iter()
                .zip(b.column(j).iter())
                .map(|(x, y)| x + y)
                .min()
                .unwrap_or(u64::MAX);
        }
        result
    });

    sp
}

#[cfg(test)]
mod test {

    use super::*;

    // Test shortest path with following graph
    // @startuml
    // digraph foo {
    // rankdir=LR
    //  0 -> 1 [label = "7"]
    //  0 -> 2 [label = "3"]
    //  1 -> 2 [label = "1"]
    //  1 -> 3 [label = "2"]
    //  1 -> 4 [label = "6"]
    //  2 -> 3 [label = "2"]
    //  3 -> 4 [label = "4"]
    //  1 -> 0 [label = "7"]
    //  2 -> 0 [label = "3"]
    //  2 -> 1 [label = "1"]
    //  3 -> 1 [label = "2"]
    //  4 -> 1 [label = "6"]
    //  3 -> 2 [label = "2"]
    //  4 -> 3 [label = "4"]

    // }
    // @enduml
    #[test]
    fn test_shortest_path() {
        let a = Array2::<u64>::from_shape_vec(
            (5, 5),
            vec![
                0, 7, 3, 1_000_000, 1_000_000, 7, 0, 1, 2, 6, 3, 1, 0, 2, 1_000_000, 1_000_000, 2,
                2, 0, 4, 1_000_000, 6, 1_000_000, 4, 0,
            ],
        )
        .unwrap();

        let sp = shortest_path(a);

        assert_eq!(
            sp,
            Array2::<u64>::from_shape_vec(
                (5, 5),
                vec![0, 4, 3, 5, 9, 4, 0, 1, 2, 6, 3, 1, 0, 2, 6, 5, 2, 2, 0, 4, 9, 6, 6, 4, 0]
            )
            .unwrap()
        );
    }
}
