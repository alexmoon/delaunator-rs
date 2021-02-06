use core::f32;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::util::ApproxEq;

pub trait Scalar:
    Copy
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + PartialOrd<Self>
    + From<f32>
{
}

impl Scalar for f32 {}
impl Scalar for f64 {}

/// Represents a 2D point in the input vector.
#[derive(Clone, Copy, PartialEq)]
pub struct Point<T: Scalar> {
    pub x: T,
    pub y: T,
}

impl<T: Scalar + std::fmt::Debug> std::fmt::Debug for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{:?}, {:?}]", self.x, self.y)
    }
}

impl<T: Scalar> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    /// The square of the length of `self`.
    pub fn length_squared(self) -> T {
        self.x * self.x + self.y * self.y
    }

    /// Returns a `Point` that is equal to `self` rotated by 90 degrees.
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// The perpendicular dot product of `self` and `other`.
    pub fn perp_dot(self, other: Self) -> T {
        self.x * other.y - self.y * other.x
    }

    /// The square of the distance between `self` and `p`.
    pub fn distance_squared(self, p: Self) -> T {
        (self - p).length_squared()
    }

    /// Tests if the path `self` to `q` to `r` goes in a clockwise direction
    /// (assuming a right-handed coordinate system).
    pub fn is_clockwise(self, q: Self, r: Self) -> bool {
        (r - q).perp_dot(q - self) > 0.0.into()
    }

    fn circumdelta(self, b: Self, c: Self) -> Self {
        let d = b - self;
        let e = c - self;

        let bl = d.length_squared();
        let cl = e.length_squared();
        let k: T = T::from(0.5) / d.perp_dot(e);

        (d * cl - e * bl).perp() * k
    }

    /// The square of the radius of the circumcircle of `self`, `b`, and `c`.
    pub fn circumradius_squared(self, b: Self, c: Self) -> T {
        self.circumdelta(b, c).length_squared()
    }

    /// The center of the circumcircle of `self`, `b`, and `c`.
    pub fn circumcenter(self, b: Self, c: Self) -> Self {
        self + self.circumdelta(b, c)
    }

    /// Tests if `self` is in the circumcircle of `a`, `b`, and `c`.
    pub fn is_in_circle(self, a: Self, b: Self, c: Self) -> bool {
        let d = a - self;
        let e = b - self;
        let f = c - self;

        let ap = d.length_squared();
        let bp = e.length_squared();
        let cp = f.length_squared();

        let g = e * cp - f * bp;

        d.perp_dot(g) + ap * e.perp_dot(f) > 0.0.into()
    }

    pub fn nearly_equals(self, p: Self) -> bool
    where
        T: ApproxEq,
    {
        self.x.approx_eq(p.x) && self.y.approx_eq(p.y)
    }
}

impl<T: Scalar> Add<Point<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Scalar> Sub<Point<T>> for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Scalar> Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance_squared() {
        let a = Point::new(1.0, 0.0);
        let b = Point::new(0.0, 1.0);
        assert!(a.distance_squared(b).approx_eq(2.0));

        let a = Point::new(2.0, 0.0);
        let b = Point::new(0.0, -3.0);
        assert!(a.distance_squared(b).approx_eq(13.0));
    }

    #[test]
    fn test_is_clockwise() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
        let c = Point::new(1.0, 1.0);

        // Counter-clockwise
        assert!(!a.is_clockwise(b, c));
        assert!(!b.is_clockwise(c, a));
        assert!(!c.is_clockwise(a, b));

        // Clockwise
        assert!(a.is_clockwise(c, b));
        assert!(c.is_clockwise(b, a));
        assert!(b.is_clockwise(a, c));

        // Co-linear points
        let d = Point::new(2.0, 2.0);
        assert!(!a.is_clockwise(c, d));
    }

    #[test]
    fn test_circumcenter() {
        // unit circle centered at (0.5, 0.5)
        let a = Point::new(-0.5, 0.5);
        let b = Point::new(1.5, 0.5);
        let c = Point::new(0.5, 1.5);
        assert!(a.circumcenter(b, c).nearly_equals(Point::new(0.5, 0.5)));
        assert!(a.circumcenter(c, b).nearly_equals(Point::new(0.5, 0.5)));
        assert!(b.circumcenter(a, c).nearly_equals(Point::new(0.5, 0.5)));

        // radius 2.0 circle centered at (1.0, 1.0)
        let a = Point::new(-1.0, 1.0);
        let b = Point::new(3.0, 1.0);
        let c = Point::new(1.0, 3.0);
        assert!(a.circumcenter(b, c).nearly_equals(Point::new(1.0, 1.0)));
    }

    #[test]
    fn test_circumradius_squared() {
        // unit circle centered at (0.5, 0.5)
        let a = Point::new(-0.5, 0.5);
        let b = Point::new(1.5, 0.5);
        let c = Point::new(0.5, 1.5);
        assert!(a.circumradius_squared(b, c).approx_eq(1.0));
        assert!(a.circumradius_squared(c, b).approx_eq(1.0));
        assert!(b.circumradius_squared(a, c).approx_eq(1.0));

        // radius 2.0 circle centered at (1.0, 1.0)
        let a = Point::new(-1.0, 1.0);
        let b = Point::new(3.0, 1.0);
        let c = Point::new(1.0, 3.0);
        assert!(a.circumradius_squared(b, c).approx_eq(4.0));
    }

    #[test]
    fn test_in_circle() {
        let a = Point::new(-0.5, 0.5);
        let b = Point::new(1.5, 0.5);
        let c = Point::new(0.5, 1.5);

        assert_eq!(a.is_in_circle(a, b, c), false);
        assert_eq!(b.is_in_circle(a, b, c), false);
        assert_eq!(c.is_in_circle(a, b, c), false);

        let p = Point::new(0.5, -0.5);
        assert_eq!(p.is_in_circle(a, b, c), false);

        let p = Point::new(1.0, -1.0);
        assert_eq!(p.is_in_circle(a, b, c), false);

        let p = Point::new(0.5, 0.5);
        assert_eq!(p.is_in_circle(a, b, c), true);

        let p = Point::new(0.0, 0.0);
        assert_eq!(p.is_in_circle(a, b, c), true);
    }
}
