use fm2gp::algebra::{IntMin, IntSatAdd, IntTropical, SemiGroupOp};
use fm2gp::integer::Integer;
use fm2gp::mat::{MatN, MatNMult};

type TropicalMatNMult<const N: usize, I> = MatNMult<N, I, IntMin<I>, IntSatAdd<I>, IntTropical<I>>;

fn shortest_paths<const N: usize, I: Integer>(adj_mat: &MatN<N, I>) -> MatN<N, I> {
    let mult: TropicalMatNMult<N, I> = TropicalMatNMult::new();
    mult.power_semigrp(&adj_mat, N - 1)
}

fn main() {
    let m = u8::MAX;
    // A <-> B <-> C
    #[rustfmt::skip]
    let adj_vec = vec![
        0, 1, m,
        1, 0, 1,
        m, 1, 0
    ];
    let adj_mat = MatN::<3usize, u8>::new(adj_vec);
    println!("Adjacency matrix\n{}", adj_mat);
    println!("Result\n{}", shortest_paths(&adj_mat));

    let m = u8::MAX;
    // Conor's graph
    #[rustfmt::skip]
    let adj_vec = vec![
        // KI RH HC JB AW
           0, 7, m, m, 5,
           7, 0, 6, 3, m,
           m, 6, 0, m, 2,
           m, 3, m, 0, 4,
           5, m, 2, 4, 0      
    ];
    let adj_mat = MatN::<5usize, u8>::new(adj_vec);
    println!("Adjacency matrix\n{}", adj_mat);
    println!("Result\n{}", shortest_paths(&adj_mat));
}
