use num_bigint::BigInt;
use num_traits::One;

use crate::semigroup::power;
type Mat22<T> = [[T; 2]; 2];


pub fn fibonacci(n: u64) -> BigInt {

    let m: Mat22<BigInt> = [[BigInt::one(), BigInt::one()], [BigInt::one(), BigInt::ZERO]];
    let r: Mat22<BigInt> = power(m, n, |a,b| {
        [
            [
                &a[0][0] * &b[0][0] + &a[0][1] * &b[1][0],
                &a[0][0] * &b[0][1] + &a[0][1] * &b[1][1],
            ],
            [
                &a[1][0] * &b[0][0] + &a[1][1] * &b[1][0],
                &a[1][0] * &b[0][1] + &a[1][1] * &b[1][1],
            ],
        ]
    });
    r[0][1].clone()
}

pub fn iterative_fibonacci(n: u64) -> BigInt {
    if n == 0 {
        return BigInt::ZERO;
    }
    let mut a = BigInt::ZERO;
    let mut b = BigInt::one();
    for _ in 1..n {
        let c = a + &b;
        a = b;
        b = c;
    }
    b
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_fib_it() {
        assert_eq!(iterative_fibonacci(10), BigInt::from(55));
    }

    #[test]
    fn test_fib() {
        for i in 1..100 {
            assert_eq!(fibonacci(i), iterative_fibonacci(i));
        }
    }
}
