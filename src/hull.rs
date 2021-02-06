use crate::Point;

const EMPTY: usize = usize::max_value();

/// A value between 0.0 and 1.0 which monotonically increases with real angle,
/// but doesn't need expensive trigonometry.
fn pseudo_angle(p: Point) -> f64 {
    let k = p.x / (p.x.abs() + p.y.abs());
    (if p.y > 0.0 { 3.0 - k } else { 1.0 + k }) / 4.0
}

// data structure for tracking the edges of the advancing convex hull
pub(crate) struct Hull {
    pub(crate) start: usize,
    pub(crate) prev: Vec<usize>,
    pub(crate) next: Vec<usize>,
    pub(crate) tri: Vec<usize>,
    hash: Vec<usize>,
    center: Point,
}

impl Hull {
    pub fn new(n: usize, center: Point, i0: usize, i1: usize, i2: usize, points: &[Point]) -> Self {
        let hash_len = (n as f64).sqrt() as usize;

        let mut hull = Self {
            prev: vec![EMPTY; n],        // vertex to prev vertex
            next: vec![EMPTY; n],        // vertex to next vertex
            tri: vec![EMPTY; n],         // vertex to adjacent halfedge
            hash: vec![EMPTY; hash_len], // angular edge hash
            start: i0,
            center,
        };

        hull.next[i0] = i1;
        hull.prev[i2] = i1;
        hull.next[i1] = i2;
        hull.prev[i0] = i2;
        hull.next[i2] = i0;
        hull.prev[i1] = i0;

        hull.tri[i0] = 0;
        hull.tri[i1] = 1;
        hull.tri[i2] = 2;

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
        self.hash[key] = i;
    }

    pub(crate) fn find_visible_edge(&self, p: Point, points: &[Point]) -> (usize, bool) {
        let mut start: usize = 0;
        let key = self.hash_key(p);
        let len = self.hash.len();
        for j in 0..len {
            start = self.hash[(key + j) % len];
            if start != EMPTY && self.next[start] != EMPTY {
                break;
            }
        }
        start = self.prev[start];
        let mut e = start;

        while !p.orient(points[e], points[self.next[e]]) {
            e = self.next[e];
            if e == start {
                return (EMPTY, false);
            }
        }
        (e, e == start)
    }

    pub(crate) fn swap_halfedge(&mut self, from_halfedge: usize, to_halfedge: usize) {
        let mut v = self.start;
        loop {
            if self.tri[v] == from_halfedge {
                self.tri[v] = to_halfedge;
                break;
            }
            v = self.prev[v];
            if v == self.start {
                break;
            }
        }
    }
}
