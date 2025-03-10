use fm2gp::algebra::{
    CommutativeOp, IntAdd, IntAddMul, IntMul, MonoidOp, SemiAlgebra, SemiRingOps,
};
use fm2gp::mat::{MatN, MatNAdd};

fn poly_eval<Add, Mul, R, N, A>(coeffs: &Vec<R::Dom>, input: &A::Dom) -> A::Dom
where
    Add: MonoidOp + CommutativeOp,
    Mul: MonoidOp,
    Mul::Dom: From<Add::Dom>,
    Add::Dom: From<Mul::Dom>,
    R: SemiRingOps<Add, Mul>,
    N: MonoidOp + CommutativeOp,
    A: SemiAlgebra<N, Add, Mul, R>,
{
    if coeffs.len() == 0 {
        return A::identity();
    }
    let mut sum: A::Dom = A::scale(&coeffs[0], &A::one());

    for v in &coeffs[1..] {
        sum = A::dot(&sum, &input);
        sum = A::add(&sum, &A::scale(v, &A::one()));
    }
    sum
}

fn main() {
    #[rustfmt::skip]
    let mat = vec![
        1, 2,
        1, 0,
    ];
    let mat = MatN::<2usize, u8>::new(mat);
    println!("Original matrix\n{}", mat);
    let coeffs = vec![1, 2, 1];
    println!("Polynomial coeffs: {:?}", &coeffs);

    // this is ugly
    let res = poly_eval::<
        IntAdd<u8>,
        IntMul<u8>,
        IntAddMul<u8>,
        MatNAdd<2, u8, IntAdd<u8>, IntMul<u8>, IntAddMul<u8>>,
        MatN<2, u8>,
    >(&coeffs, &mat);
    println!("Result\n{}", res);
}
