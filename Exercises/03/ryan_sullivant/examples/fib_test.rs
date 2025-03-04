use fm2gp::algebra::*;
use std::error::Error;

fn fib(n: i32) -> i32 {
    let mat = [1, 1, 1, 0];
    let mat = Mat2 { mat };
    let mult = Mat2Mult::<i32>::new();

    let res = mult.power_monoid(&mat, n);

    // println!("Fib matrix: {:?}", &res);
    res.mat[0]
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: fib_test <n>");
        return Ok(());
    }
    let n: i32 = args[1].parse()?;
    println!("Fib {} = {}", n, fib(n));
    Ok(())
}
