use crate::integer::{Integer, half};
use num::Integer as NumInteger;
use std::marker::PhantomData;

/// Semigroup operation with associated domain
/// Given the operation definition `op` we can define exponentiation
/// of the operatoin
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

/// Monoid operation is a semigroup operaton which also has an identity element.
/// We can extend the exponentiation defined for semigroups by also handling
/// exponent of zero.
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

/// Group operation is a monoid operaton which also has inverses.
/// We can extend the exponentiation defined for monoids by also handling
/// negative exponents.
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

/// Tag only
pub trait CommutativeOp {}

/// Combines a commutative monoid operation (`+`, `0`) and
/// a monoid operation (`*`, `1`) on the same domain to make a semiring
/// The operations must satisfying the following Laws
/// ## Laws
/// ```text
/// a * 0 = 0 = 0 * a
/// a * (b + c) = (a * b) + (b * c)
/// (b + c) * a = (b * a) + (c * a)
/// ```
pub trait SemiRingOps<A, M>
where
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    A::Dom: From<M::Dom>,
    M::Dom: From<A::Dom>,
{
    /// Elements of `M::Dom` and `A::Dom` should be convertible
    /// to `Dom`
    type Dom: Clone + Eq + From<M::Dom> + From<A::Dom> + Into<A::Dom> + Into<M::Dom>;

    const ADD_OP: A;
    const MUL_OP: M;

    fn add(x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        // TODO: figure out lifetimes to get this to work without clone()
        Self::ADD_OP.op(&x.clone().into(), &y.clone().into()).into()
    }

    fn mul(x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        Self::MUL_OP.op(&x.clone().into(), &y.clone().into()).into()
    }
}

/// A semimodule over a semiring R is a
/// commutative monoid N with a operation
/// scale : R x N -> N satisfying the following laws
/// for all r, s in R and m, n in N
/// ## Laws
/// ```text
/// scale(r, m + n) = scale(r, m) + scale(r, n)
/// scale(r + s, m) = scale(r, m) + scale(s, m)
/// scale(r, scale(s, m)) = scale(rs, m)
/// scale(1, m) = m
/// scale(0, m) = scale(r, 0) = 0
/// ```
pub trait SemiModule<N, A, M, R>
where
    // N is commutative monoid
    N: MonoidOp + CommutativeOp,
    // R is a semiring
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    A::Dom: From<M::Dom>,
    M::Dom: From<A::Dom>,
    R: SemiRingOps<A, M>,
{
    /// Elements of `N::Dom` to `Dom`
    type Dom: Clone + Eq + From<N::Dom> + Into<N::Dom>;

    const ADD_OP: N;
    fn add(x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        // TODO: figure out lifetimes to get this to work without clone()
        Self::ADD_OP.op(&x.clone().into(), &y.clone().into()).into()
    }

    fn identity() -> Self::Dom {
        Self::ADD_OP.identity().into()
    }

    fn scale(r: &R::Dom, m: &Self::Dom) -> Self::Dom;
}

/// A semialgebra over a semiring R is a
/// is a semimodule N which in addition has
/// a second operation dot : N x N -> N
/// satisfying the following laws
/// for all r, s in R and m, n, p in N
/// ## Laws
/// ```text
/// dot(m + n, p) = dot(m, p) + dot(n, p)
/// dot(m, n + p) = dot(m, n) + dot(m, p)
/// dot(scale(r, m), scale(s, n)) = scale(rs, dot(m, n))
/// ```
pub trait SemiAlgebra<N, A, M, R>: SemiModule<N, A, M, R>
where
    // N is commutative monoid
    N: MonoidOp + CommutativeOp,
    // R is a semiring
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    A::Dom: From<M::Dom>,
    M::Dom: From<A::Dom>,
    R: SemiRingOps<A, M>,
{
    // TODO: how to handle this better?
    fn one() -> Self::Dom;
    fn dot(m: &Self::Dom, n: &Self::Dom) -> Self::Dom;
}

/// Algebraic stucture implementations

/// Usual integer addition
pub struct IntAdd<I: Integer> {
    phantom: PhantomData<I>,
}

impl<I: Integer> IntAdd<I> {
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<I: Integer> SemiGroupOp for IntAdd<I> {
    type Dom = I;

    fn op(&self, x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        *x + *y
    }
}

impl<I: Integer> MonoidOp for IntAdd<I> {
    fn identity(&self) -> Self::Dom {
        I::zero()
    }
}

impl<I: Integer> CommutativeOp for IntAdd<I> {}
pub struct IntMul<I: Integer> {
    phantom: PhantomData<I>,
}

/// Usual integer multiplication
impl<I: Integer> IntMul<I> {
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<I: Integer> SemiGroupOp for IntMul<I> {
    type Dom = I;

    fn op(&self, x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        *x * *y
    }
}

impl<I: Integer> MonoidOp for IntMul<I> {
    fn identity(&self) -> Self::Dom {
        I::one()
    }
}

impl<I: Integer> CommutativeOp for IntMul<I> {}

/// Usual integer addition/multiplication semiring
pub struct IntAddMul<I: Integer> {
    phantom: PhantomData<I>,
}

impl<I: Integer> SemiRingOps<IntAdd<I>, IntMul<I>> for IntAddMul<I> {
    type Dom = I;

    const ADD_OP: IntAdd<I> = IntAdd::new();

    const MUL_OP: IntMul<I> = IntMul::new();
}

/// Integer min operation
pub struct IntMin<I: Integer> {
    phantom: PhantomData<I>,
}

impl<I: Integer> IntMin<I> {
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
impl<I: Integer> SemiGroupOp for IntMin<I> {
    type Dom = I;

    fn op(&self, x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        *x.min(y)
    }
}

impl<I: Integer> MonoidOp for IntMin<I> {
    fn identity(&self) -> Self::Dom {
        I::MAX
    }
}

impl<I: Integer> CommutativeOp for IntMin<I> {}

/// Saturating integer add operatoin
/// Used for the tropical semiring
pub struct IntSatAdd<I: Integer> {
    phantom: PhantomData<I>,
}

impl<I: Integer> IntSatAdd<I> {
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<I: Integer> SemiGroupOp for IntSatAdd<I> {
    type Dom = I;

    fn op(&self, x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        x.saturating_add(*y)
    }
}

impl<I: Integer> MonoidOp for IntSatAdd<I> {
    fn identity(&self) -> Self::Dom {
        I::zero()
    }
}

/// Tropical semiring where addition is min and
/// multiplication is saturating add
pub struct IntTropical<I: Integer> {
    phantom: PhantomData<I>,
}

impl<I: Integer> SemiRingOps<IntMin<I>, IntSatAdd<I>> for IntTropical<I> {
    type Dom = I;

    const ADD_OP: IntMin<I> = IntMin::new();

    const MUL_OP: IntSatAdd<I> = IntSatAdd::new();
}

#[cfg(test)]
mod tests {
    use super::*;

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
