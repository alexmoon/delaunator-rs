use std::iter::FusedIterator;

use super::elem::*;
use super::Triangulation;
use crate::util::{next_halfedge, prev_halfedge, OptionIndex};

/// Iterates over all [HalfEdge]s that start at a [Vertex].
///
/// Order of iteration is undefined (generally counter-clockwise, but will
/// switch to clockwise if the iteration hits the convex hull).
///
/// Note that on the convex hull, one half-edge connected to the vertex does
/// not start at that vertex and therefore will not be visited by this iteration.
#[derive(Clone, Copy)]
pub struct VertexEdgeIter<'a> {
    pub(crate) triangulation: &'a Triangulation,
    pub(crate) start: OptionIndex,
    pub(crate) index: OptionIndex,
}

impl<'a> Iterator for VertexEdgeIter<'a> {
    type Item = HalfEdge<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.index.get(), self.start.get()) {
            (None, _) => None,
            (Some(index), None) => {
                // We've previously hit the convex hull and are now iterating backwards
                let e = self.triangulation.halfedges[index].get();
                self.index = e.map(next_halfedge).into();

                Some(HalfEdge {
                    triangulation: self.triangulation,
                    index,
                })
            }
            (Some(index), Some(start)) => {
                self.index = match self.triangulation.halfedges[prev_halfedge(index)].get() {
                    None => {
                        // We've hit the convex hull, start over from the starting index and iterate backwards
                        let e = self.triangulation.halfedges[start].get();
                        self.start = None.into();
                        e.map(next_halfedge)
                    }
                    Some(e) if e == start => None,
                    e => e,
                }
                .into();

                Some(HalfEdge {
                    triangulation: self.triangulation,
                    index,
                })
            }
        }
    }
}

impl<'a> FusedIterator for VertexEdgeIter<'a> {}

/// Iterates over the three [HalfEdge]s of a [Triangle]
#[derive(Clone, Copy)]
pub struct TriangleEdgeIter<'a> {
    pub(crate) triangulation: &'a Triangulation,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

impl<'a> Iterator for TriangleEdgeIter<'a> {
    type Item = HalfEdge<'a>;

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

impl<'a> DoubleEndedIterator for TriangleEdgeIter<'a> {
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

impl<'a> FusedIterator for TriangleEdgeIter<'a> {}

impl<'a> ExactSizeIterator for TriangleEdgeIter<'a> {
    fn len(&self) -> usize {
        self.end - self.index
    }
}

/// Iterates over the three [Vertex]s of a [Triangle]
#[derive(Clone, Copy)]
pub struct TriangleVertexIter<'a> {
    pub(crate) triangulation: &'a Triangulation,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

impl<'a> Iterator for TriangleVertexIter<'a> {
    type Item = Vertex<'a>;

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

impl<'a> DoubleEndedIterator for TriangleVertexIter<'a> {
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

impl<'a> FusedIterator for TriangleVertexIter<'a> {}

impl<'a> ExactSizeIterator for TriangleVertexIter<'a> {
    fn len(&self) -> usize {
        self.end - self.index
    }
}

/// Iterates over the [Triangle]s in a [Triangulation]
#[derive(Clone, Copy)]
pub struct TriangleIter<'a> {
    pub(crate) triangulation: &'a Triangulation,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

impl<'a> Iterator for TriangleIter<'a> {
    type Item = Triangle<'a>;

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

impl<'a> DoubleEndedIterator for TriangleIter<'a> {
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

impl<'a> FusedIterator for TriangleIter<'a> {}

impl<'a> ExactSizeIterator for TriangleIter<'a> {
    fn len(&self) -> usize {
        (self.triangulation.triangles.len() - self.index) / 3
    }
}

/// Iterates over the [HalfEdge]s in a [Triangulation]
#[derive(Clone, Copy)]
pub struct HalfEdgeIter<'a> {
    pub(crate) triangulation: &'a Triangulation,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

impl<'a> Iterator for HalfEdgeIter<'a> {
    type Item = HalfEdge<'a>;

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

impl<'a> DoubleEndedIterator for HalfEdgeIter<'a> {
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

impl<'a> FusedIterator for HalfEdgeIter<'a> {}

impl<'a> ExactSizeIterator for HalfEdgeIter<'a> {
    fn len(&self) -> usize {
        self.end - self.index
    }
}

#[cfg(feature = "vertices")]
/// Iterates over the [Vertex]es in a [Triangulation]
pub struct VertexIter<'a> {
    pub(crate) triangulation: &'a Triangulation,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

#[cfg(feature = "vertices")]
impl<'a> Iterator for VertexIter<'a> {
    type Item = Vertex<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            let index = self.triangulation.vertices[self.index];
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
impl<'a> DoubleEndedIterator for VertexIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            self.end -= 1;
            let index = self.triangulation.vertices[self.end];
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
impl<'a> FusedIterator for VertexIter<'a> {}

#[cfg(feature = "vertices")]
impl<'a> ExactSizeIterator for VertexIter<'a> {
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

        let triangulation = Triangulation::new(&points).unwrap();
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
