use mint::Point2;

use crate::{
    traits::{HasPosition, Scalar},
    Point,
};

impl<T: Scalar> From<Point2<T>> for Point<T> {
    fn from(other: Point2<T>) -> Self {
        Point {
            x: other.x,
            y: other.y,
        }
    }
}

impl<T: Scalar> From<Point<T>> for Point2<T> {
    fn from(other: Point<T>) -> Self {
        Self {
            x: other.x,
            y: other.y,
        }
    }
}

impl<S: Scalar, P: Clone + Into<Point2<S>>> HasPosition<S> for P {
    fn pos(&self) -> Point<S> {
        Point::from(self.clone().into())
    }
}
