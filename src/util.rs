use crate::{
    traits::{HasPosition, Scalar},
    Point,
};

/// A space-efficient version of an `Option<usize>`.
///
/// Supports values from `0` to `usize::max_usize() - 1`.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OptionIndex(usize);

impl OptionIndex {
    /// Creates a new `OptionIndex`.
    ///
    /// Returns `None` if `n` is `usize::max_value()`.
    #[inline(always)]
    pub const fn new(n: usize) -> Option<Self> {
        if n == usize::max_value() {
            None
        } else {
            Some(OptionIndex(n))
        }
    }

    /// Creates the `None` value.
    #[inline(always)]
    pub const fn none() -> Self {
        OptionIndex(usize::max_value())
    }

    /// Creates a `Some(n)` value.
    ///
    /// # Panics
    /// Panics if `n` is `usize::max_value()`.
    #[inline(always)]
    pub fn some(n: usize) -> Self {
        assert!(n != usize::max_value());
        OptionIndex(n)
    }

    /// Creates a `Some(n)` value.
    ///
    /// # Safety
    /// `n` must be less than `usize::max_value()`.
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

    /// Converts self into an `Option<usize>`.
    #[inline(always)]
    pub const fn get(self) -> Option<usize> {
        if self.0 == usize::max_value() {
            None
        } else {
            Some(self.0)
        }
    }

    /// Returns the contained Some value, consuming the self value.
    ///
    /// # Panics
    /// Panics if the self value equals `None`.
    #[inline(always)]
    pub fn unwrap(self) -> usize {
        self.get().unwrap()
    }

    /// Returns the contained [`Some`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`None`] with a custom panic message provided by
    /// `msg`.
    #[inline(always)]
    pub fn expect(self, msg: &str) -> usize {
        self.get().expect(msg)
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
pub(crate) fn next_halfedge(i: usize) -> usize {
    if i % 3 == 2 {
        i - 2
    } else {
        i + 1
    }
}

/// Previous halfedge in a triangle.
pub(crate) fn prev_halfedge(i: usize) -> usize {
    if i % 3 == 0 {
        i + 2
    } else {
        i - 1
    }
}

pub(crate) fn calc_bbox_center<T: Scalar, P: HasPosition<T>>(points: &[P]) -> Point<T> {
    let mut min_x = T::infinity();
    let mut min_y = T::infinity();
    let mut max_x = -T::infinity();
    let mut max_y = -T::infinity();
    for p in points.iter() {
        let p = p.pos();
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }
    Point {
        x: (min_x + max_x) / 2.0.into(),
        y: (min_y + max_y) / 2.0.into(),
    }
}

pub(crate) fn find_closest_point<T: Scalar, P: HasPosition<T>>(
    points: &[P],
    p0: Point<T>,
) -> Option<usize> {
    let mut min_dist = T::infinity();
    let mut k: usize = 0;
    for (i, p) in points.iter().enumerate() {
        let d = p0.distance_squared(p.pos());
        if d > 0.0.into() && d < min_dist {
            k = i;
            min_dist = d;
        }
    }
    if min_dist == T::infinity() {
        None
    } else {
        Some(k)
    }
}

pub(crate) fn find_seed_triangle<T: Scalar, P: HasPosition<T>>(
    points: &[P],
) -> Option<(usize, usize, usize)> {
    // pick a seed point close to the center
    let bbox_center = calc_bbox_center(points);
    let i0 = find_closest_point(points, bbox_center)?;
    let p0 = points[i0].pos();

    // find the point closest to the seed
    let i1 = find_closest_point(points, p0)?;
    let p1 = points[i1].pos();

    // find the third point which forms the smallest circumcircle with the first two
    let mut min_radius = T::infinity();
    let mut i2: usize = 0;
    for (i, p) in points.iter().enumerate() {
        if i == i0 || i == i1 {
            continue;
        }
        let p = p.pos();
        let r = p0.circumradius_squared(p1, p);
        if r < min_radius {
            i2 = i;
            min_radius = r;
        }
    }

    if min_radius == T::infinity() {
        None
    } else {
        // swap the order of the seed points for counter-clockwise orientation
        Some(if p0.is_clockwise(p1, points[i2].pos()) {
            (i0, i2, i1)
        } else {
            (i0, i1, i2)
        })
    }
}
