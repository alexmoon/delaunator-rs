use crate::{
    traits::{HasPosition, Index, Scalar},
    util::OptionIndex,
    Point,
};

/// A value between 0.0 and 1.0 which monotonically increases with real angle,
/// but doesn't need expensive trigonometry.
fn pseudo_angle<T: Scalar>(p: Point<T>) -> T {
    let k = p.x / (p.x.abs() + p.y.abs());
    (if p.y > T::from(0.0) {
        T::from(3.0) - k
    } else {
        T::from(1.0) + k
    }) / T::from(4.0)
}

// data structure for tracking the edges of the advancing convex hull
pub(crate) struct Hull<T: Scalar, I> {
    pub(crate) start: usize,
    pub(crate) prev: Vec<OptionIndex<usize>>,
    pub(crate) next: Vec<OptionIndex<usize>>,
    pub(crate) tri: Vec<OptionIndex<I>>,
    hash: Vec<OptionIndex<usize>>,
    center: Point<T>,
}

impl<T: Scalar, I: Index> Hull<T, I> {
    pub fn new<P: HasPosition<T>>(
        n: usize,
        center: Point<T>,
        i0: usize,
        i1: usize,
        i2: usize,
        points: &[P],
    ) -> Self {
        let hash_len = (n as f64).sqrt() as usize;

        let mut hull = Self {
            prev: vec![Default::default(); n],        // vertex to prev vertex
            next: vec![Default::default(); n],        // vertex to next vertex
            tri: vec![Default::default(); n],         // vertex to adjacent halfedge
            hash: vec![Default::default(); hash_len], // angular edge hash
            start: i0,
            center,
        };

        hull.next[i0] = i1.into();
        hull.prev[i2] = i1.into();
        hull.next[i1] = i2.into();
        hull.prev[i0] = i2.into();
        hull.next[i2] = i0.into();
        hull.prev[i1] = i0.into();

        hull.tri[i0] = I::from_usize(0).into();
        hull.tri[i1] = I::from_usize(1).into();
        hull.tri[i2] = I::from_usize(2).into();

        hull.hash_edge(points[i0].pos(), i0);
        hull.hash_edge(points[i1].pos(), i1);
        hull.hash_edge(points[i2].pos(), i2);

        hull
    }

    fn hash_key(&self, p: Point<T>) -> usize {
        let len = self.hash.len();
        ((T::from(len as f32) * pseudo_angle(p - self.center)).into() as usize) % len
    }

    pub(crate) fn hash_edge(&mut self, p: Point<T>, i: usize) {
        let key = self.hash_key(p);
        self.hash[key] = i.into();
    }

    pub(crate) fn find_visible_edge<P: HasPosition<T>>(
        &self,
        p: Point<T>,
        points: &[P],
    ) -> (Option<usize>, bool) {
        let mut start = OptionIndex::none();
        let key = self.hash_key(p);
        let len = self.hash.len();
        for j in 0..len {
            start = self.hash[(key + j) % len];
            if start.get().and_then(|x| self.next[x].get()).is_some() {
                break;
            }
        }
        let start = self.prev[start.unwrap()].unwrap();
        let mut e = start;

        while !p.is_clockwise(points[e].pos(), points[self.next[e].unwrap()].pos()) {
            e = self.next[e].unwrap();
            if e == start {
                return (None, false);
            }
        }
        (Some(e), e == start)
    }

    pub(crate) fn swap_halfedge(&mut self, from_halfedge: I, to_halfedge: I) {
        let mut v = self.start;
        loop {
            if self.tri[v] == from_halfedge.into() {
                self.tri[v] = to_halfedge.into();
                break;
            }
            v = self.prev[v].unwrap();
            if v == self.start {
                break;
            }
        }
    }
}
