use std::iter::FusedIterator;

use super::elem::*;
use super::Triangulation;
use crate::{
    traits::Index,
    util::{next_halfedge, prev_halfedge},
};

/// Iterates over all [HalfEdge]s that start at a [Vertex].
///
/// Order of iteration is undefined (generally counter-clockwise, but will
/// switch to clockwise if the iteration hits the convex hull).
///
/// Note that on the convex hull, one half-edge connected to the vertex does
/// not start at that vertex and therefore will not be visited by this iteration.
#[derive(Clone, Copy)]
pub struct VertexEdgeIter<'a, I> {
    pub(crate) triangulation: &'a Triangulation<I>,
    pub(crate) start: Option<usize>,
    pub(crate) index: Option<usize>,
}

impl<'a, I: Index> Iterator for VertexEdgeIter<'a, I> {
    type Item = HalfEdge<'a, I>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.index, self.start) {
            (None, _) => None,
            (Some(index), None) => {
                // We've previously hit the convex hull and are now iterating backwards
                let e = self.triangulation.halfedges[index].get().map(I::as_usize);
                self.index = e.map(next_halfedge);

                Some(HalfEdge {
                    triangulation: self.triangulation,
                    index,
                })
            }
            (Some(index), Some(start)) => {
                self.index = match self.triangulation.halfedges[prev_halfedge(index)]
                    .get()
                    .map(I::as_usize)
                {
                    None => {
                        // We've hit the convex hull, start over from the starting index and iterate backwards
                        let e = self.triangulation.halfedges[start].get().map(I::as_usize);
                        self.start = None;
                        e.map(next_halfedge)
                    }
                    Some(e) if e.as_usize() == start => None,
                    e => e,
                };

                Some(HalfEdge {
                    triangulation: self.triangulation,
                    index,
                })
            }
        }
    }
}

impl<'a, I: Index> FusedIterator for VertexEdgeIter<'a, I> {}

/// Iterates over all [Triangle]s that are adjacent to [Vertex].
///
/// Order of iteration is undefined (generally counter-clockwise, but will
/// switch to clockwise if the iteration hits the convex hull).
#[derive(Clone, Copy)]
pub struct VertexTriangleIter<'a, I> {
    pub(crate) inner: VertexEdgeIter<'a, I>,
}

impl<'a, I: Index> Iterator for VertexTriangleIter<'a, I> {
    type Item = Triangle<'a, I>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| x.left())
    }
}

impl<'a, I: Index> FusedIterator for VertexTriangleIter<'a, I> {}

/// Iterates over the three [HalfEdge]s of a [Triangle]
#[derive(Clone, Copy)]
pub struct TriangleEdgeIter<'a, I> {
    pub(crate) triangulation: &'a Triangulation<I>,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

impl<'a, I> Iterator for TriangleEdgeIter<'a, I> {
    type Item = HalfEdge<'a, I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            let index = self.index;
            self.index += 1;
            Some(HalfEdge {
                triangulation: self.triangulation,
                index,
            })
        } else {
            None
        }
    }
}

impl<'a, I> DoubleEndedIterator for TriangleEdgeIter<'a, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            self.end -= 1;
            let index = self.end;
            Some(HalfEdge {
                triangulation: self.triangulation,
                index,
            })
        } else {
            None
        }
    }
}

impl<'a, I> FusedIterator for TriangleEdgeIter<'a, I> {}

impl<'a, I> ExactSizeIterator for TriangleEdgeIter<'a, I> {
    fn len(&self) -> usize {
        self.end - self.index
    }
}

/// Iterates over the three [Vertex]s of a [Triangle]
#[derive(Clone, Copy)]
pub struct TriangleVertexIter<'a, I> {
    pub(crate) triangulation: &'a Triangulation<I>,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

impl<'a, I> Iterator for TriangleVertexIter<'a, I> {
    type Item = Vertex<'a, I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            let index = self.index;
            self.index += 1;
            Some(Vertex {
                triangulation: self.triangulation,
                index,
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, I> DoubleEndedIterator for TriangleVertexIter<'a, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            self.end -= 1;
            let index = self.end;
            Some(Vertex {
                triangulation: self.triangulation,
                index,
            })
        } else {
            None
        }
    }
}

impl<'a, I> FusedIterator for TriangleVertexIter<'a, I> {}

impl<'a, I> ExactSizeIterator for TriangleVertexIter<'a, I> {
    fn len(&self) -> usize {
        self.end - self.index
    }
}

/// Iterates over the [Triangle]s in a [Triangulation]
#[derive(Clone, Copy)]
pub struct TriangleIter<'a, I> {
    pub(crate) triangulation: &'a Triangulation<I>,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

impl<'a, I> Iterator for TriangleIter<'a, I> {
    type Item = Triangle<'a, I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            let index = self.index;
            self.index += 3;
            Some(Triangle {
                triangulation: self.triangulation,
                index,
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, I> DoubleEndedIterator for TriangleIter<'a, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            self.end -= 3;
            let index = self.end;
            Some(Triangle {
                triangulation: self.triangulation,
                index,
            })
        } else {
            None
        }
    }
}

impl<'a, I> FusedIterator for TriangleIter<'a, I> {}

impl<'a, I> ExactSizeIterator for TriangleIter<'a, I> {
    fn len(&self) -> usize {
        (self.triangulation.triangles.len() - self.index) / 3
    }
}

/// Iterates over the [HalfEdge]s in a [Triangulation]
#[derive(Clone, Copy)]
pub struct HalfEdgeIter<'a, I> {
    pub(crate) triangulation: &'a Triangulation<I>,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

impl<'a, I> Iterator for HalfEdgeIter<'a, I> {
    type Item = HalfEdge<'a, I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            let index = self.index;
            self.index += 1;
            Some(HalfEdge {
                triangulation: self.triangulation,
                index,
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, I> DoubleEndedIterator for HalfEdgeIter<'a, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            self.end -= 1;
            let index = self.end;
            Some(HalfEdge {
                triangulation: self.triangulation,
                index,
            })
        } else {
            None
        }
    }
}

impl<'a, I> FusedIterator for HalfEdgeIter<'a, I> {}

impl<'a, I> ExactSizeIterator for HalfEdgeIter<'a, I> {
    fn len(&self) -> usize {
        self.end - self.index
    }
}

#[cfg(feature = "vertices")]
/// Iterates over the [Vertex]es in a [Triangulation]
pub struct VertexIter<'a, I> {
    pub(crate) triangulation: &'a Triangulation<I>,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

#[cfg(feature = "vertices")]
impl<'a, I: Index> Iterator for VertexIter<'a, I> {
    type Item = Vertex<'a, I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            let index = self.triangulation.vertices[self.index].as_usize();
            self.index += 1;
            Some(Vertex {
                triangulation: self.triangulation,
                index,
            })
        } else {
            None
        }
    }
}

#[cfg(feature = "vertices")]
impl<'a, I: Index> DoubleEndedIterator for VertexIter<'a, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            self.end -= 1;
            let index = self.triangulation.vertices[self.end].as_usize();
            Some(Vertex {
                triangulation: self.triangulation,
                index,
            })
        } else {
            None
        }
    }
}

#[cfg(feature = "vertices")]
impl<'a, I: Index> FusedIterator for VertexIter<'a, I> {}

#[cfg(feature = "vertices")]
impl<'a, I: Index> ExactSizeIterator for VertexIter<'a, I> {
    fn len(&self) -> usize {
        self.end - self.index
    }
}

#[cfg(test)]
mod test {
    use crate::{Point, Triangulation};

    #[test]
    fn test_vertex_edge_iter() {
        let points = [
            Point::new(0.0, 0.0),  //        /|\ 2
            Point::new(1.0, 0.0),  //     3 /_|_\ 1
            Point::new(0.0, 1.0),  //       \0| /
            Point::new(-1.0, 0.0), //        \|/ 4
            Point::new(0.0, -1.0),
        ];

        let triangulation = Triangulation::<usize>::new(&points).unwrap();
        assert_eq!(
            triangulation.triangles,
            vec![1, 2, 0, 2, 3, 0, 3, 4, 0, 0, 4, 1]
        );

        let triangle = triangulation.get_triangle(0).unwrap();

        // Never hits the convex hull
        let vertex = triangle.vertices().find(|x| x.id() == 0).unwrap();
        let edges = vertex.edges().map(|x| x.id()).collect::<Vec<_>>();
        assert_eq!(edges, vec![2, 5, 8, 9]);

        // Hits the convex hull at the end of the iteration
        let vertex = triangle.vertices().find(|x| x.id() == 1).unwrap();
        let edges = vertex.edges().map(|x| x.id()).collect::<Vec<_>>();
        assert_eq!(edges, vec![0, 11]);

        // Hits the convex hull in the middle of the iteration
        let vertex = triangle.vertices().find(|x| x.id() == 2).unwrap();
        let edges = vertex.edges().map(|x| x.id()).collect::<Vec<_>>();
        assert_eq!(edges, vec![1, 3]);
    }
}
