use num::Integer as NumInteger;
use num::Num;
use num::PrimInt;
use std::fmt::Display;

pub trait HasMax {
    const MAX: Self;
}
pub trait Integer: PrimInt + NumInteger + HasMax + Display {}

impl HasMax for u8 {
    const MAX: Self = Self::MAX;
}

impl HasMax for u16 {
    const MAX: Self = Self::MAX;
}

impl HasMax for u32 {
    const MAX: Self = Self::MAX;
}

impl HasMax for u64 {
    const MAX: Self = Self::MAX;
}

impl HasMax for usize {
    const MAX: Self = Self::MAX;
}

impl HasMax for i8 {
    const MAX: Self = Self::MAX;
}

impl HasMax for i16 {
    const MAX: Self = Self::MAX;
}

impl HasMax for i32 {
    const MAX: Self = Self::MAX;
}

impl HasMax for i64 {
    const MAX: Self = Self::MAX;
}

impl HasMax for isize {
    const MAX: Self = Self::MAX;
}

impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for usize {}
impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for isize {}

pub fn two<N: Num>() -> N {
    N::one() + N::one()
}

pub fn half<I: Integer>(x: I) -> I {
    x.div(two())
}
