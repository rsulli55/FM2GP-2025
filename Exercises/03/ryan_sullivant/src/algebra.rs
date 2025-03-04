use num::Integer;
use num::Num;
use std::marker::PhantomData;

pub fn two<N: Num>() -> N {
    N::one() + N::one()
}

pub fn half<I: Integer>(x: I) -> I {
    x.div_floor(&two())
}

pub trait SemiGroupOp {
    /// The domain of the operation
    type Dom: Clone + Eq;

    // required to be defined
    // Precondition: op is associative
    fn op(&self, x: &Self::Dom, y: &Self::Dom) -> Self::Dom;

    // implementations using op
    /// Computes op(r, a^n) using the semigroup operation
    /// Precondition n >= 0
    /// Note: `r` must be a valid value in the domain e.g. already reduced for
    /// modular arithmetic
    fn power_acc_semigrp<I: Integer>(&self, r: &Self::Dom, a: &Self::Dom, n: I) -> Self::Dom {
        // TODO: is there another way to do this?
        let mut n = n;
        let mut r = r.clone();
        let mut a = a.clone();
        if n == I::zero() {
            return r;
        }
        loop {
            if n.is_odd() {
                r = self.op(&r, &a);
                if n == I::one() {
                    return r;
                }
            }

            n = half(n);
            a = self.op(&a, &a);
        }
    }

    /// Computes a^n using op.
    /// Precondition n > 0
    fn power_semigrp<I: Integer>(&self, a: &Self::Dom, n: I) -> Self::Dom {
        let mut a = a.clone();
        let mut n = n;
        while !n.is_odd() {
            a = self.op(&a, &a);
            n = half(n);
        }

        if n == I::one() {
            return a;
        }

        self.power_acc_semigrp(&a, &self.op(&a, &a), half(n - I::one()))
    }
}

pub trait MonoidOp: SemiGroupOp {
    // MonoidOp in addition requires an identity for the operations
    fn identity(&self) -> Self::Dom;

    /// Precondition n >= 0
    fn power_monoid<I: Integer>(&self, a: &Self::Dom, n: I) -> Self::Dom {
        if n == I::zero() {
            return self.identity();
        }

        self.power_semigrp(&a, n)
    }
}

#[allow(dead_code)]
trait GroupOp: MonoidOp {
    // GroupOp in addition requires an inverse for the operation
    fn inverse(&self, a: Self::Dom) -> Self::Dom;

    fn power_group<I: Integer>(&self, a: &Self::Dom, n: I) -> Self::Dom {
        let mut a = a.clone();
        let mut n = n;
        if n < I::zero() {
            n = I::zero() - n;
            a = self.inverse(a);
        }

        self.power_monoid(&a, n)
    }
}

/// Combines a commutative monoid operation (`+`, `0`) and
/// a monoid operation (`*`, `1`) on the same domain to make a semiring
/// The operations must satisfying the following Laws
/// ## Laws
/// ```text
/// a * 0 = 0 = 0 * a
/// a * (b + c) = (a * b) + (b * c)
/// (b + c) * a = (b * a) + (c * a)
/// ```
#[allow(dead_code)]
pub trait SemiRingOps {
    /// AddOp is required to be commutative
    type AddOp: MonoidOp;
    type MulOp: MonoidOp;

    const ADD_OP: Self::AddOp;
    const MUL_OP: Self::MulOp;

    fn add(
        &self,
        x: &<Self::AddOp as SemiGroupOp>::Dom,
        y: &<Self::AddOp as SemiGroupOp>::Dom,
    ) -> <Self::AddOp as SemiGroupOp>::Dom {
        Self::ADD_OP.op(x, y)
    }

    fn mul(
        &self,
        x: &<Self::MulOp as SemiGroupOp>::Dom,
        y: &<Self::MulOp as SemiGroupOp>::Dom,
    ) -> <Self::MulOp as SemiGroupOp>::Dom {
        Self::MUL_OP.op(x, y)
    }
}

struct NatAdd {}

impl SemiGroupOp for NatAdd {
    type Dom = usize;

    fn op(&self, x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        x + y
    }
}

pub struct ZmodnZ {
    modulus: usize,
}

impl SemiGroupOp for ZmodnZ {
    type Dom = usize;

    fn op(&self, x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        (x + y).mod_floor(&self.modulus)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mat2<I: Integer> {
    pub mat: [I; 4],
}

pub struct Mat2Mult<I: Integer> {
    phantom: PhantomData<I>,
}

impl<I: Integer> Mat2Mult<I> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<I: Integer + Copy> SemiGroupOp for Mat2Mult<I> {
    type Dom = Mat2<I>;

    fn op(&self, x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        let x = &x.mat;
        let y = &y.mat;
        let mat = [
            x[0] * y[0] + x[1] * y[2],
            x[0] * y[1] + x[1] * y[3],
            x[2] * y[0] + x[3] * y[2],
            x[2] * y[1] + x[3] * y[3],
        ];
        Mat2 { mat }
    }
}

impl<I: Integer + Copy> MonoidOp for Mat2Mult<I> {
    fn identity(&self) -> Self::Dom {
        let mat = [I::one(), I::zero(), I::one(), I::zero()];
        Mat2 { mat }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nat_add() {
        let nat = NatAdd {};
        assert_eq!(nat.power_acc_semigrp(&3, &1, 0), 3);
        assert_eq!(nat.power_acc_semigrp(&0, &1, 5), 5);
        assert_eq!(nat.power_acc_semigrp(&0, &2, 4), 8);
    }

    #[test]
    fn test_zmodnz() {
        let zmod3 = ZmodnZ { modulus: 3 };

        assert_eq!(zmod3.power_acc_semigrp(&1, &1, 2), 0);
        assert_eq!(zmod3.power_acc_semigrp(&0, &1, 5), 2);
        assert_eq!(zmod3.power_acc_semigrp(&0, &2, 5), 1);
    }
}
