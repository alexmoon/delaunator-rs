pub trait ApproxEq: Copy {
    fn approx_eq(self, other: Self) -> bool;
}

impl ApproxEq for f32 {
    fn approx_eq(self, other: Self) -> bool {
        const EPSILON: f32 = 2.0 * std::f32::EPSILON;
        (self - other).abs() <= EPSILON
    }
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: Self) -> bool {
        const EPSILON: f64 = 2.0 * std::f64::EPSILON;
        (self - other).abs() <= EPSILON
    }
}
