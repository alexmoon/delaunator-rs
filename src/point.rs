use std::ops::Sub;

use crate::util::ApproxEq;

/// Represents a 2D point in the input vector.
#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn dist2(self, p: Self) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        dx * dx + dy * dy
    }

    pub fn orient(self, q: Self, r: Self) -> bool {
        (q.y - self.y) * (r.x - q.x) - (q.x - self.x) * (r.y - q.y) > 0.0
    }

    fn circumdelta(self, b: Self, c: Self) -> (f64, f64) {
        let dx = b.x - self.x;
        let dy = b.y - self.y;
        let ex = c.x - self.x;
        let ey = c.y - self.y;

        let bl = dx * dx + dy * dy;
        let cl = ex * ex + ey * ey;
        let d = 0.5 / (dx * ey - dy * ex);

        let x = (ey * bl - dy * cl) * d;
        let y = (dx * cl - ex * bl) * d;
        (x, y)
    }

    pub fn circumradius2(self, b: Self, c: Self) -> f64 {
        let (x, y) = self.circumdelta(b, c);
        x * x + y * y
    }

    pub fn circumcenter(self, b: Self, c: Self) -> Self {
        let (x, y) = self.circumdelta(b, c);
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn in_circle(self, b: Self, c: Self, p: Self) -> bool {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        let ex = b.x - p.x;
        let ey = b.y - p.y;
        let fx = c.x - p.x;
        let fy = c.y - p.y;

        let ap = dx * dx + dy * dy;
        let bp = ex * ex + ey * ey;
        let cp = fx * fx + fy * fy;

        dx * (ey * cp - bp * fy) - dy * (ex * cp - bp * fx) + ap * (ex * fy - ey * fx) > 0.0
    }

    pub fn nearly_equals(self, p: Self) -> bool {
        self.x.approx_eq(p.x) && self.y.approx_eq(p.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dist2() {
        let a = Point::new(1.0, 0.0);
        let b = Point::new(0.0, 1.0);
        assert!(a.dist2(b).approx_eq(2.0));

        let a = Point::new(2.0, 0.0);
        let b = Point::new(0.0, -3.0);
        assert!(a.dist2(b).approx_eq(13.0));
    }

    #[test]
    fn test_orient() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
        let c = Point::new(1.0, 1.0);

        // Counter-clockwise
        assert!(!a.orient(b, c));
        assert!(!b.orient(c, a));
        assert!(!c.orient(a, b));

        // Clockwise
        assert!(a.orient(c, b));
        assert!(c.orient(b, a));
        assert!(b.orient(a, c));

        // Co-linear points
        let d = Point::new(2.0, 2.0);
        assert!(!a.orient(c, d));
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
    fn test_circumradius2() {
        // unit circle centered at (0.5, 0.5)
        let a = Point::new(-0.5, 0.5);
        let b = Point::new(1.5, 0.5);
        let c = Point::new(0.5, 1.5);
        assert!(a.circumradius2(b, c).approx_eq(1.0));
        assert!(a.circumradius2(c, b).approx_eq(1.0));
        assert!(b.circumradius2(a, c).approx_eq(1.0));

        // radius 2.0 circle centered at (1.0, 1.0)
        let a = Point::new(-1.0, 1.0);
        let b = Point::new(3.0, 1.0);
        let c = Point::new(1.0, 3.0);
        assert!(a.circumradius2(b, c).approx_eq(4.0));
    }

    #[test]
    fn test_in_circle() {
        let a = Point::new(-0.5, 0.5);
        let b = Point::new(1.5, 0.5);
        let c = Point::new(0.5, 1.5);

        assert_eq!(a.in_circle(b, c, a), false);
        assert_eq!(a.in_circle(b, c, b), false);
        assert_eq!(a.in_circle(b, c, c), false);

        let p = Point::new(0.5, -0.5);
        assert_eq!(a.in_circle(b, c, p), false);

        let p = Point::new(1.0, -1.0);
        assert_eq!(a.in_circle(b, c, p), false);

        let p = Point::new(0.5, 0.5);
        assert_eq!(a.in_circle(b, c, p), true);

        let p = Point::new(0.0, 0.0);
        assert_eq!(a.in_circle(b, c, p), true);
    }
}
