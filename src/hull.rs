use crate::{util::OptionIndex, Point};

/// A value between 0.0 and 1.0 which monotonically increases with real angle,
/// but doesn't need expensive trigonometry.
fn pseudo_angle(p: Point) -> f64 {
    let k = p.x / (p.x.abs() + p.y.abs());
    (if p.y > 0.0 { 3.0 - k } else { 1.0 + k }) / 4.0
}

// data structure for tracking the edges of the advancing convex hull
pub(crate) struct Hull {
    pub(crate) start: usize,
    pub(crate) prev: Vec<OptionIndex>,
    pub(crate) next: Vec<OptionIndex>,
    pub(crate) tri: Vec<OptionIndex>,
    hash: Vec<OptionIndex>,
    center: Point,
}

impl Hull {
    pub fn new(n: usize, center: Point, i0: usize, i1: usize, i2: usize, points: &[Point]) -> Self {
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

        hull.tri[i0] = 0.into();
        hull.tri[i1] = 1.into();
        hull.tri[i2] = 2.into();

        hull.hash_edge(points[i0], i0);
        hull.hash_edge(points[i1], i1);
        hull.hash_edge(points[i2], i2);

        hull
    }

    fn hash_key(&self, p: Point) -> usize {
        let len = self.hash.len();
        (((len as f64) * pseudo_angle(p - self.center)) as usize) % len
    }

    pub(crate) fn hash_edge(&mut self, p: Point, i: usize) {
        let key = self.hash_key(p);
        self.hash[key] = i.into();
    }

    pub(crate) fn find_visible_edge(&self, p: Point, points: &[Point]) -> (OptionIndex, bool) {
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

        while !p.is_clockwise(points[e], points[self.next[e].unwrap()]) {
            e = self.next[e].unwrap();
            if e == start {
                return (OptionIndex::none(), false);
            }
        }
        (OptionIndex::some(e), e == start)
    }

    pub(crate) fn swap_halfedge(&mut self, from_halfedge: usize, to_halfedge: usize) {
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
