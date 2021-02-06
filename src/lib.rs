#![allow(clippy::clippy::many_single_char_names)]

/*!
A very fast 2D [Delaunay Triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation) library for Rust.
A port of [Delaunator](https://github.com/mapbox/delaunator).

# Example

```rust
use delaunator::{Point, Triangulation};

let points = vec![
    Point { x: 0., y: 0. },
    Point { x: 1., y: 0. },
    Point { x: 1., y: 1. },
    Point { x: 0., y: 1. },
];

let result = Triangulation::new(&points).expect("No triangulation exists.");
println!("{:?}", result.triangles); // [0, 2, 1, 0, 3, 2]
```
*/

pub mod elem;
mod hull;
pub mod iter;
pub mod point;
pub mod traits;
pub mod triangulation;
pub mod util;

#[cfg(feature = "mint")]
mod mint;

pub use elem::{HalfEdge, Triangle, Vertex};
pub use point::Point;
pub use triangulation::Triangulation;
