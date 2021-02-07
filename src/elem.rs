use super::iter::*;
use super::Triangulation;
use crate::{
    traits::Index,
    util::{next_halfedge, prev_halfedge},
};

/// One triangle within a [Triangulation]
pub struct Triangle<'a, I> {
    pub(crate) triangulation: &'a Triangulation<I>,
    pub(crate) index: usize,
}

impl<'a, I: Index> Triangle<'a, I> {
    /// A fixed identifier for this triangle which can be used to get it from its [Triangulation].
    pub fn id(&self) -> usize {
        self.index / 3
    }

    /// An iterator over the [HalfEdge]s of this triangle.
    pub fn edges(&self) -> TriangleEdgeIter<'a, I> {
        TriangleEdgeIter {
            triangulation: self.triangulation,
            index: self.index,
            end: self.index + 3,
        }
    }

    /// An iterator over the [Vertex]es of this triangle.
    pub fn vertices(&self) -> TriangleVertexIter<'a, I> {
        TriangleVertexIter {
            triangulation: self.triangulation,
            index: self.index,
            end: self.index + 3,
        }
    }

    /// The first [Vertex] of this triangle.
    pub fn a(&self) -> Vertex<'a, I> {
        Vertex {
            triangulation: self.triangulation,
            index: self.index,
        }
    }

    /// The second [Vertex] of this triangle.
    pub fn b(&self) -> Vertex<'a, I> {
        Vertex {
            triangulation: self.triangulation,
            index: self.index + 1,
        }
    }

    /// The third [Vertex] of this triangle.
    pub fn c(&self) -> Vertex<'a, I> {
        Vertex {
            triangulation: self.triangulation,
            index: self.index + 2,
        }
    }

    /// The [HalfEdge] between the first and second vertices of this triangle.
    pub fn ab(&self) -> HalfEdge<'a, I> {
        HalfEdge {
            triangulation: self.triangulation,
            index: self.index,
        }
    }

    /// The [HalfEdge] between the second and third vertices of this triangle.
    pub fn bc(&self) -> HalfEdge<'a, I> {
        HalfEdge {
            triangulation: self.triangulation,
            index: self.index + 1,
        }
    }

    /// The [HalfEdge] between the third and first vertices of this triangle.
    pub fn ca(&self) -> HalfEdge<'a, I> {
        HalfEdge {
            triangulation: self.triangulation,
            index: self.index + 2,
        }
    }
}

/// One half-edge within a [Triangulation]
#[derive(Clone, Copy)]
pub struct HalfEdge<'a, I> {
    pub(crate) triangulation: &'a Triangulation<I>,
    pub(crate) index: usize,
}

impl<'a, I: Index> HalfEdge<'a, I> {
    /// A fixed identifier for this half-edge which can be used to get it from its [Triangulation].
    pub fn id(&self) -> usize {
        self.index
    }

    /// The corresponding half-edge in the other direction for the adjacent triangle.
    /// Returns `None` if this half-edge is on the convex hull.
    pub fn twin(&self) -> Option<Self> {
        self.triangulation.halfedges[self.index]
            .get()
            .map(I::as_usize)
            .map(|index| HalfEdge {
                triangulation: self.triangulation,
                index,
            })
    }

    /// The next (counter-clockwise) half-edge of the [Triangle] to the left of this half-edge.
    pub fn next(&self) -> Self {
        let index = next_halfedge(self.index);
        HalfEdge {
            triangulation: self.triangulation,
            index,
        }
    }

    /// The previous (clockwise) half-edge of the [Triangle] to the left of this half-edge.
    pub fn prev(&self) -> Self {
        let index = prev_halfedge(self.index);
        HalfEdge {
            triangulation: self.triangulation,
            index,
        }
    }

    /// The starting [Vertex] of this half-edge.
    pub fn start(&self) -> Vertex<'a, I> {
        Vertex {
            triangulation: self.triangulation,
            index: self.index,
        }
    }

    /// The ending [Vertex] of this half-edge.
    pub fn end(&self) -> Vertex<'a, I> {
        let index = next_halfedge(self.index);
        Vertex {
            triangulation: self.triangulation,
            index,
        }
    }

    /// The [Triangle] to the left of this half-edge.
    pub fn left(&self) -> Triangle<'a, I> {
        Triangle {
            triangulation: self.triangulation,
            index: self.index - self.index % 3,
        }
    }

    /// The [Triangle] to the right of this half-edge or `None` if this half-edge
    /// is on the convex hull.
    pub fn right(&self) -> Option<Triangle<'a, I>> {
        self.triangulation.halfedges[self.index]
            .get()
            .map(I::as_usize)
            .map(|j| Triangle {
                triangulation: self.triangulation,
                index: j - j % 3,
            })
    }
}

/// One vertex within a [Triangulation]
#[derive(Clone, Copy)]
pub struct Vertex<'a, I> {
    pub(crate) triangulation: &'a Triangulation<I>,
    pub(crate) index: usize,
}

impl<'a, I: Index> Vertex<'a, I> {
    /// A fixed identifier for this vertex which can be used to get it from its [Triangulation].
    pub fn id(&self) -> usize {
        self.triangulation.triangles[self.index].as_usize()
    }

    // An iterator over the [HalfEdge]s that start from this vertex.
    pub fn edges(&self) -> VertexEdgeIter<'a, I> {
        let index = Some(self.index);
        VertexEdgeIter {
            triangulation: self.triangulation,
            start: index,
            index,
        }
    }

    /// An iterator over the [Triangle]s that are adjacent to this vertex.
    pub fn triangles(&self) -> VertexTriangleIter<'a, I> {
        VertexTriangleIter {
            inner: self.edges(),
        }
    }
}
