use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::Point;

/// Provides approximate equality for floating point values.
pub trait ApproxEq: Copy {
    fn approx_eq(self, other: Self) -> bool;
}

impl ApproxEq for f32 {
    fn approx_eq(self, other: Self) -> bool {
        const EPSILON: f32 = std::f32::EPSILON;
        (self - other).abs() <= EPSILON
    }
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: Self) -> bool {
        const EPSILON: f64 = 2.0 * std::f64::EPSILON;
        (self - other).abs() <= EPSILON
    }
}

pub trait Index: Copy + PartialEq<Self> {
    fn max_value() -> Self;
    fn from_usize(n: usize) -> Self;
    fn as_usize(self) -> usize;
}

impl Index for u16 {
    #[inline]
    fn max_value() -> Self {
        u16::max_value()
    }

    #[inline]
    fn from_usize(n: usize) -> Self {
        n as Self
    }

    #[inline]
    fn as_usize(self) -> usize {
        usize::from(self)
    }
}

impl Index for u32 {
    #[inline]
    fn max_value() -> Self {
        u32::max_value()
    }

    #[inline]
    fn from_usize(n: usize) -> Self {
        n as Self
    }

    #[inline]
    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Index for usize {
    #[inline]
    fn max_value() -> Self {
        usize::max_value()
    }

    #[inline]
    fn from_usize(n: usize) -> Self {
        n
    }

    #[inline]
    fn as_usize(self) -> usize {
        self
    }
}

pub trait Scalar:
    Copy
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + PartialOrd<Self>
    + From<f32>
    + Into<f64>
{
    fn abs(self) -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn infinity() -> Self;
}

impl Scalar for f32 {
    #[inline(always)]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline(always)]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    fn infinity() -> Self {
        f32::INFINITY
    }
}

impl Scalar for f64 {
    #[inline(always)]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline(always)]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    fn infinity() -> Self {
        f64::INFINITY
    }
}

pub trait HasPosition<T: Scalar> {
    fn pos(&self) -> Point<T>;
}

#[cfg(not(feature = "mint"))]
impl<T: Scalar> HasPosition<T> for Point<T> {
    fn pos(&self) -> Point<T> {
        *self
    }
}
