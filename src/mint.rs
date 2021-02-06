use crate::point::Point;

impl From<mint::Point2<f64>> for Point {
    fn from(other: mint::Point2<f64>) -> Self {
        Point {
            x: other.x,
            y: other.y,
        }
    }
}

impl From<Point> for mint::Point2<f64> {
    fn from(other: Point) -> Self {
        Self {
            x: other.x,
            y: other.y,
        }
    }
}
