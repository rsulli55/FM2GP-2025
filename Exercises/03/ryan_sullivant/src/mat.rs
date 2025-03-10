use crate::algebra::{CommutativeOp, MonoidOp, SemiAlgebra, SemiGroupOp, SemiModule, SemiRingOps};
use crate::integer::Integer;
use std::convert::identity;
use std::fmt::Display;
use std::marker::PhantomData;

/// NxN matrices
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatN<const N: usize, I: Integer> {
    mat: Vec<I>,
}

impl<const N: usize, I: Integer> MatN<N, I> {
    pub fn new(v: Vec<I>) -> Self {
        assert_eq!(v.len(), N * N);
        Self { mat: v }
    }

    pub fn at(&self, i: usize, j: usize) -> &I {
        self.mat.get(i * N + j).unwrap()
    }
}

impl<const N: usize, I: Integer> Display for MatN<N, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..N {
            for j in 0..N {
                write!(f, "{:<10}", self.at(i, j))?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

/// To multiply N x N matrices, we need to elements to come from
/// a semiring with defined addition and multiplication operations.
/// We also need to be able to convert between elements of the semiring R
/// and elements of the matrix (I)
pub struct MatNMult<const N: usize, I, A, M, R>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
    phantom: PhantomData<(usize, I, A, M, R)>,
}

impl<const N: usize, I, A, M, R> MatNMult<N, I, A, M, R>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<const N: usize, I, A, M, R> SemiGroupOp for MatNMult<N, I, A, M, R>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
    type Dom = MatN<N, I>;

    fn op(&self, x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        let mut v: Vec<I> = Vec::with_capacity(N * N);
        for i in 0..N {
            for j in 0..N {
                let a: R::Dom = (*x.at(i, 0)).into();
                let b: R::Dom = (*y.at(0, j)).into();
                let mut sum: R::Dom = R::mul(&a, &b);
                for k in 1..N {
                    let a: R::Dom = (*x.at(i, k)).into();
                    let b: R::Dom = (*y.at(k, j)).into();
                    let prod = R::mul(&a, &b);
                    sum = R::add(&sum, &prod);
                }

                v.push(sum.into());
            }
        }

        Self::Dom::new(v)
    }
}

impl<const N: usize, I, A, M, R> MonoidOp for MatNMult<N, I, A, M, R>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
    fn identity(&self) -> Self::Dom {
        let mut mat: Vec<I> = Vec::new();
        for i in 0..N {
            for j in 0..N {
                if i == j {
                    mat.push(I::one());
                } else {
                    mat.push(I::zero());
                }
            }
        }

        Self::Dom::new(mat)
    }
}

impl<const N: usize, I, A, M, R> CommutativeOp for MatNMult<N, I, A, M, R>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
}

/// Addition of matrices
pub struct MatNAdd<const N: usize, I, A, M, R>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
    phantom: PhantomData<(usize, I, A, M, R)>,
}

/// Matrix addition
impl<const N: usize, I, A, M, R> MatNAdd<N, I, A, M, R>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<const N: usize, I, A, M, R> SemiGroupOp for MatNAdd<N, I, A, M, R>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
    type Dom = MatN<N, I>;

    fn op(&self, x: &Self::Dom, y: &Self::Dom) -> Self::Dom {
        let mut v: Vec<I> = Vec::with_capacity(N * N);
        for i in 0..N {
            for j in 0..N {
                let a: R::Dom = (*x.at(i, j)).into();
                let b: R::Dom = (*y.at(i, j)).into();
                let sum: R::Dom = R::add(&a, &b);
                v.push(sum.into());
            }
        }

        Self::Dom::new(v)
    }
}

impl<const N: usize, I, A, M, R> MonoidOp for MatNAdd<N, I, A, M, R>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
    fn identity(&self) -> Self::Dom {
        let mut mat: Vec<I> = Vec::new();
        for _ in 0..N {
            for _ in 0..N {
                mat.push(I::zero())
            }
        }

        Self::Dom::new(mat)
    }
}
impl<const N: usize, I, A, M, R> CommutativeOp for MatNAdd<N, I, A, M, R>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
}

/// Matrices form a semimodule and algebra
impl<const N: usize, I, A, M, R> SemiModule<MatNAdd<N, I, A, M, R>, A, M, R> for MatN<N, I>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I && N::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
    type Dom = MatN<N, I>;
    const ADD_OP: MatNAdd<N, I, A, M, R> = MatNAdd::new();

    fn scale(r: &R::Dom, m: &Self::Dom) -> Self::Dom {
        let mut v: Vec<I> = Vec::with_capacity(N * N);
        for i in 0..N {
            for j in 0..N {
                let a: R::Dom = (*m.at(i, j)).into();
                let scaled: R::Dom = R::mul(&r, &a);

                v.push(scaled.into());
            }
        }

        Self::Dom::new(v)
    }
}

/// Matrices form a semimodule and algebra
impl<const N: usize, I, A, M, R> SemiAlgebra<MatNAdd<N, I, A, M, R>, A, M, R> for MatN<N, I>
where
    I: Integer,
    A: MonoidOp + CommutativeOp,
    M: MonoidOp,
    M::Dom: From<A::Dom>,
    A::Dom: From<M::Dom>,
    R: SemiRingOps<A, M>,
    // R::Dom == I && N::Dom == I
    I: From<R::Dom>,
    I: Into<R::Dom>,
{
    fn dot(m: &Self::Dom, n: &Self::Dom) -> Self::Dom {
        let mut v: Vec<I> = Vec::with_capacity(N * N);
        for i in 0..N {
            for j in 0..N {
                let a: R::Dom = (*m.at(i, 0)).into();
                let b: R::Dom = (*n.at(0, j)).into();
                let mut sum: R::Dom = R::mul(&a, &b);
                for k in 1..N {
                    let a: R::Dom = (*m.at(i, k)).into();
                    let b: R::Dom = (*n.at(k, j)).into();
                    let prod = R::mul(&a, &b);
                    sum = R::add(&sum, &prod);
                }

                v.push(sum.into());
            }
        }

        Self::Dom::new(v)
    }

    fn one() -> Self::Dom {
        let mul: MatNMult<N, I, A, M, R> = MatNMult::new();
        mul.identity()
    }
}
