use crate::{
    elem::*,
    hull::Hull,
    iter::*,
    traits::{ApproxEq, HasPosition, Index, Scalar},
    util::{self, OptionIndex},
};

/// Result of the Delaunay triangulation.
pub struct Triangulation<I> {
    #[cfg(feature = "vertices")]
    /// A vector of triangle point indices where the `i`-th vertex in the array
    /// corresponds to vertex `triangles[i]` for the first triangle containing
    /// vertex `i`.
    pub vertices: Vec<I>,

    /// A vector of point indices where each triple represents a Delaunay triangle.
    /// All triangles are directed counter-clockwise in a right-handed coordinate system.
    pub triangles: Vec<I>,

    /// A vector of adjacent halfedge indices that allows traversing the triangulation graph.
    ///
    /// `i`-th half-edge in the array corresponds to vertex `triangles[i]`
    /// the half-edge is coming from. `halfedges[i]` is the index of a twin half-edge
    /// in an adjacent triangle (or `EMPTY` for outer half-edges on the convex hull).
    pub halfedges: Vec<OptionIndex<I>>,

    /// A vector of indices that reference points on the convex hull of the triangulation,
    /// counter-clockwise in a right-handed coordinate system.
    pub hull: Vec<I>,
}

impl<I: Index> Triangulation<I> {
    fn alloc(n: usize) -> Self {
        let max_triangles = 2 * n - 5;
        Self {
            #[cfg(feature = "vertices")]
            vertices: Vec::new(),
            triangles: Vec::with_capacity(max_triangles * 3),
            halfedges: Vec::with_capacity(max_triangles * 3),
            hull: Vec::new(),
        }
    }

    /// Triangulate a set of 2D points.
    /// Returns `None` if no triangulation exists for the input (e.g. all points are collinear).
    pub fn new<T: Scalar + ApproxEq, P: HasPosition<T>>(points: &[P]) -> Option<Self> {
        Some(Triangulation::with_seed_triangle(
            points,
            util::find_seed_triangle(points)?,
        ))
    }

    pub fn with_seed_triangle<T: Scalar + ApproxEq, P: HasPosition<T>>(
        points: &[P],
        seed_triangle: (usize, usize, usize),
    ) -> Self {
        let n = points.len();
        let (i0, i1, i2) = seed_triangle;
        let center = points[i0]
            .pos()
            .circumcenter(points[i1].pos(), points[i2].pos());

        let mut triangulation = Triangulation::<I>::alloc(n);
        triangulation.add_triangle(i0, i1, i2, None.into(), None.into(), None.into());

        // sort the points by distance from the seed triangle circumcenter
        let mut dists: Vec<_> = points
            .iter()
            .enumerate()
            .map(|(i, point)| (i, center.distance_squared(point.pos())))
            .collect();

        dists.sort_unstable_by(|&(_, da), &(_, db)| da.partial_cmp(&db).unwrap());

        let mut hull = Hull::new(n, center, i0, i1, i2, points);

        for (k, &(i, _)) in dists.iter().enumerate() {
            let p = points[i].pos();

            // skip near-duplicates
            if k > 0 && p.nearly_equals(points[dists[k - 1].0].pos()) {
                continue;
            }
            // skip seed triangle points
            if i == i0 || i == i1 || i == i2 {
                continue;
            }

            // find a visible edge on the convex hull using edge hash
            let (e, walk_back) = hull.find_visible_edge(p, points);
            let mut e = match e {
                None => continue, // likely a near-duplicate point; skip it
                Some(e) => e,
            };

            // add the first triangle from the point
            let t = triangulation.add_triangle(
                e,
                i,
                hull.next[e].unwrap(),
                None.into(),
                None.into(),
                hull.tri[e],
            );

            // recursively flip triangles from the point until they satisfy the Delaunay condition
            hull.tri[i] = I::from_usize(triangulation.legalize(t + 2, points, &mut hull)).into();
            hull.tri[e] = I::from_usize(t).into(); // keep track of boundary triangles on the hull

            // walk forward through the hull, adding more triangles and flipping recursively
            let mut n = hull.next[e].unwrap();
            loop {
                let q = hull.next[n].unwrap();
                if !p.is_clockwise(points[n].pos(), points[q].pos()) {
                    break;
                }
                let t = triangulation.add_triangle(n, i, q, hull.tri[i], None.into(), hull.tri[n]);
                hull.tri[i] =
                    I::from_usize(triangulation.legalize(t + 2, points, &mut hull)).into();
                hull.next[n] = OptionIndex::none(); // mark as removed
                n = q;
            }

            // walk backward from the other side, adding more triangles and flipping
            if walk_back {
                loop {
                    let q = hull.prev[e].unwrap();
                    if !p.is_clockwise(points[q].pos(), points[e].pos()) {
                        break;
                    }
                    let t =
                        triangulation.add_triangle(q, i, e, None.into(), hull.tri[e], hull.tri[q]);
                    triangulation.legalize(t + 2, points, &mut hull);
                    hull.tri[q] = I::from_usize(t).into();
                    hull.next[e] = OptionIndex::none(); // mark as removed
                    e = q;
                }
            }

            // update the hull indices
            hull.prev[i] = e.into();
            hull.next[i] = n.into();
            hull.prev[n] = i.into();
            hull.next[e] = i.into();
            hull.start = e;

            // save the two new edges in the hash table
            hull.hash_edge(p, i);
            hull.hash_edge(points[e].pos(), e);
        }

        // expose hull as a vector of point indices
        let mut e = hull.start;
        loop {
            triangulation.hull.push(I::from_usize(e));
            e = hull.next[e].unwrap();
            if e == hull.start {
                break;
            }
        }

        triangulation.triangles.shrink_to_fit();
        triangulation.halfedges.shrink_to_fit();

        #[cfg(feature = "vertices")]
        {
            triangulation.vertices.resize(n, I::max_value());
            for (i, &j) in triangulation.triangles.iter().enumerate() {
                let j = j.as_usize();
                if triangulation.vertices[j] == I::max_value() {
                    triangulation.vertices[j] = I::from_usize(i);
                }
            }
        }

        triangulation
    }

    /// The number of triangles in the triangulation.
    pub fn len(&self) -> usize {
        self.triangles.len() / 3
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn triangles(&self) -> TriangleIter<'_, I> {
        TriangleIter {
            triangulation: self,
            index: 0,
            end: self.triangles.len(),
        }
    }

    pub fn half_edges(&self) -> HalfEdgeIter<'_, I> {
        HalfEdgeIter {
            triangulation: self,
            index: 0,
            end: self.halfedges.len(),
        }
    }

    #[cfg(feature = "vertices")]
    pub fn vertices(&self) -> VertexIter<'_, I> {
        VertexIter {
            triangulation: self,
            index: 0,
            end: self.vertices.len(),
        }
    }

    #[cfg(feature = "vertices")]
    pub fn get_vertex(&self, id: usize) -> Option<Vertex<'_, I>> {
        if id < self.vertices.len() {
            Some(Vertex {
                triangulation: self,
                index: self.vertices[id].as_usize(),
            })
        } else {
            None
        }
    }

    pub fn get_triangle(&self, id: usize) -> Option<Triangle<'_, I>> {
        let index = 3 * id;
        if index < self.triangles.len() {
            Some(Triangle {
                triangulation: self,
                index,
            })
        } else {
            None
        }
    }

    pub fn get_half_edge(&self, id: usize) -> Option<HalfEdge<'_, I>> {
        if id < self.halfedges.len() {
            Some(HalfEdge {
                triangulation: self,
                index: id,
            })
        } else {
            None
        }
    }

    fn add_triangle(
        &mut self,
        i0: usize,
        i1: usize,
        i2: usize,
        a: OptionIndex<I>,
        b: OptionIndex<I>,
        c: OptionIndex<I>,
    ) -> usize {
        let t = self.triangles.len();

        self.triangles.push(I::from_usize(i0));
        self.triangles.push(I::from_usize(i1));
        self.triangles.push(I::from_usize(i2));

        self.halfedges.push(a);
        self.halfedges.push(b);
        self.halfedges.push(c);

        if let Some(a) = a.get() {
            self.halfedges[a.as_usize()] = I::from_usize(t).into();
        }
        if let Some(b) = b.get() {
            self.halfedges[b.as_usize()] = I::from_usize(t + 1).into();
        }
        if let Some(c) = c.get() {
            self.halfedges[c.as_usize()] = I::from_usize(t + 2).into();
        }

        t
    }

    fn legalize<T: Scalar, P: HasPosition<T>>(
        &mut self,
        a: usize,
        points: &[P],
        hull: &mut Hull<T, I>,
    ) -> usize {
        let b = self.halfedges[a];

        // if the pair of triangles doesn't satisfy the Delaunay condition
        // (p1 is inside the circumcircle of [p0, pl, pr]), flip them,
        // then do the same check/flip recursively for the new pair of triangles
        //
        //           pl                    pl
        //          /||\                  /  \
        //       al/ || \bl            al/    \a
        //        /  ||  \              /      \
        //       /  a||b  \    flip    /___ar___\
        //     p0\   ||   /p1   =>   p0\---bl---/p1
        //        \  ||  /              \      /
        //       ar\ || /br             b\    /br
        //          \||/                  \  /
        //           pr                    pr
        //
        let ar = util::prev_halfedge(a);

        let b = match b.get() {
            None => return ar,
            Some(b) => b.as_usize(),
        };

        let al = util::next_halfedge(a);
        let bl = util::prev_halfedge(b);

        let p0 = self.triangles[ar].as_usize();
        let pr = self.triangles[a].as_usize();
        let pl = self.triangles[al].as_usize();
        let p1 = self.triangles[bl].as_usize();

        let illegal =
            points[p1]
                .pos()
                .is_in_circle(points[p0].pos(), points[pr].pos(), points[pl].pos());
        if illegal {
            self.triangles[a] = I::from_usize(p1);
            self.triangles[b] = I::from_usize(p0);

            let hbl = self.halfedges[bl];
            let har = self.halfedges[ar];

            // edge swapped on the other side of the hull (rare); fix the halfedge reference
            if hbl.is_none() {
                hull.swap_halfedge(I::from_usize(bl), I::from_usize(a));
            }

            self.halfedges[a] = hbl;
            self.halfedges[b] = har;
            self.halfedges[ar] = I::from_usize(bl).into();

            if let Some(hbl) = hbl.get() {
                self.halfedges[hbl.as_usize()] = I::from_usize(a).into();
            }
            if let Some(har) = har.get() {
                self.halfedges[har.as_usize()] = I::from_usize(b).into();
            }
            self.halfedges[bl] = I::from_usize(ar).into();

            let br = util::next_halfedge(b);

            self.legalize(a, points, hull);
            return self.legalize(br, points, hull);
        }
        ar
    }
}
