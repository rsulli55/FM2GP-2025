use fm2gp::algebra::{IntAdd, IntAddMul, IntMul, MonoidOp};
use fm2gp::mat::{MatN, MatNMult};
use std::error::Error;

type Mat2 = MatN<2, usize>;
type Mat2Mult = MatNMult<2, usize, IntAdd<usize>, IntMul<usize>, IntAddMul<usize>>;

fn fib(n: usize) -> usize {
    let mat = vec![1, 1, 1, 0];
    let mat = Mat2::new(mat);
    let mult = Mat2Mult::new();

    let res = mult.power_monoid(&mat, n);

    *res.at(0, 0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: fib <n>");
        return Ok(());
    }
    let n: usize = args[1].parse()?;
    println!("Fib {} = {}", n, fib(n));
    Ok(())
}
