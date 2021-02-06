use crate::Point;

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

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OptionIndex(usize);

impl OptionIndex {
    #[inline(always)]
    pub const fn none() -> Self {
        OptionIndex(usize::max_value())
    }

    #[inline(always)]
    pub const fn new(n: usize) -> Option<Self> {
        if n == usize::max_value() {
            None
        } else {
            Some(OptionIndex(n))
        }
    }

    #[inline(always)]
    pub const unsafe fn new_unchecked(n: usize) -> Self {
        OptionIndex(n)
    }

    #[inline(always)]
    pub const fn is_none(self) -> bool {
        self.0 == usize::max_value()
    }

    #[inline(always)]
    pub const fn is_some(self) -> bool {
        !self.is_none()
    }

    #[inline(always)]
    pub const fn get(self) -> Option<usize> {
        if self.0 == usize::max_value() {
            None
        } else {
            Some(self.0)
        }
    }
}

impl Default for OptionIndex {
    #[inline(always)]
    fn default() -> Self {
        OptionIndex::none()
    }
}

impl std::fmt::Debug for OptionIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl From<usize> for OptionIndex {
    #[inline(always)]
    fn from(n: usize) -> Self {
        OptionIndex(n)
    }
}

impl From<Option<usize>> for OptionIndex {
    #[inline(always)]
    fn from(n: Option<usize>) -> Self {
        match n {
            None => OptionIndex::none(),
            Some(n) => OptionIndex::new(n).unwrap(),
        }
    }
}

impl From<OptionIndex> for Option<usize> {
    #[inline(always)]
    fn from(n: OptionIndex) -> Option<usize> {
        n.get()
    }
}

/// Next halfedge in a triangle.
pub fn next_halfedge(i: usize) -> usize {
    if i % 3 == 2 {
        i - 2
    } else {
        i + 1
    }
}

/// Previous halfedge in a triangle.
pub fn prev_halfedge(i: usize) -> usize {
    if i % 3 == 0 {
        i + 2
    } else {
        i - 1
    }
}

pub fn calc_bbox_center(points: &[Point]) -> Point {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for p in points.iter() {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }
    Point {
        x: (min_x + max_x) / 2.0,
        y: (min_y + max_y) / 2.0,
    }
}

pub fn find_closest_point(points: &[Point], p0: Point) -> Option<usize> {
    let mut min_dist = f64::INFINITY;
    let mut k: usize = 0;
    for (i, &p) in points.iter().enumerate() {
        let d = p0.dist2(p);
        if d > 0.0 && d < min_dist {
            k = i;
            min_dist = d;
        }
    }
    if min_dist == f64::INFINITY {
        None
    } else {
        Some(k)
    }
}

pub fn find_seed_triangle(points: &[Point]) -> Option<(usize, usize, usize)> {
    // pick a seed point close to the center
    let bbox_center = calc_bbox_center(points);
    let i0 = find_closest_point(points, bbox_center)?;
    let p0 = points[i0];

    // find the point closest to the seed
    let i1 = find_closest_point(points, p0)?;
    let p1 = points[i1];

    // find the third point which forms the smallest circumcircle with the first two
    let mut min_radius = f64::INFINITY;
    let mut i2: usize = 0;
    for (i, &p) in points.iter().enumerate() {
        if i == i0 || i == i1 {
            continue;
        }
        let r = p0.circumradius2(p1, p);
        if r < min_radius {
            i2 = i;
            min_radius = r;
        }
    }

    if min_radius == f64::INFINITY {
        None
    } else {
        // swap the order of the seed points for counter-clockwise orientation
        Some(if p0.orient(p1, points[i2]) {
            (i0, i2, i1)
        } else {
            (i0, i1, i2)
        })
    }
}
