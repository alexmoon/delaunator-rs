use crate::point::{Point, Scalar};

impl<T: Scalar> From<mint::Point2<T>> for Point<T> {
    fn from(other: mint::Point2<T>) -> Self {
        Point {
            x: other.x,
            y: other.y,
        }
    }
}

impl<T: Scalar> From<Point<T>> for mint::Point2<T> {
    fn from(other: Point<T>) -> Self {
        Self {
            x: other.x,
            y: other.y,
        }
    }
}
