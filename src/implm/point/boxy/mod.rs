//! Box coordinate spaces. This includes square mazes, rectangular mazes, cuboid mazes,
//! and other hyper-rectangular constructions.
//!
//! Commonly used coordinate spaces:
//! * [TwoDimensionalBoxCoordinateSpace]
//! * [ThreeDimensionalBoxCoordinateSpace]
//!
//! # Dimensions
//!
//! x always represents the "lowest" dimension. A one-dimensional point's only
//! has an x dimension, a two-dimensional point has an x dimension and a y
//! dimension.
//!
//! The x dimension represents columns, the y dimension represents rows, the
//! z dimension represents layers, and so on.
//!
//! ## Order of Dimensions
//!
//! In this crate, the *most minor dimension always comes first*. For example, you
//! would index a [CoordinateTuplet] could be represented as `[x, y, z]`, not `[z, y, x]`.
//! Likewise, a coordinate space for a maze with three layers, four rows, and five columns
//! could be constructed as:
//! ```
//! # use mazelib::implm::point::boxy::{BoxCoordinateSpace, ThreeDimensionalBoxCoordinateSpace};
//! BoxCoordinateSpace::new_checked([5, 4, 3]);
//! # ThreeDimensionalBoxCoordinateSpace::new_checked([5, 4, 3]);  // Verify the comment below
//! ```
//! You could also use [ThreeDimensionalBoxCoordinateSpace] in this example instead of
//! [BoxCoordinateSpace] — they would be equivalent.

pub use self::iterator::BoxCoordinateSpaceIterator;
pub use self::point::CoordinateTuplet;
pub use self::space::BoxCoordinateSpace;

mod space;
mod point;
mod iterator;

/// A flat rectangular coordinate space. The most common type of coordinate space for mazes.
///
/// A 2D coordinate space will produce mazes you can print out on paper.
///
/// While it is optimised for square mazes, you can use whatever maze outline you want.
/// (You can create custom outlines by mutating the maze before passing it into a generator).
///
/// *This is a specialisation of [BoxCoordinateSpace] for two dimensions. See
/// [CoordinateSpace][crate::interface::point::CoordinateSpace] and [BoxCoordinateSpace] for
/// information on available methods and such.*
pub type TwoDimensionalBoxCoordinateSpace = BoxCoordinateSpace<2>;

/// An (x, y) pair for referencing points in a [TwoDimensionalBoxCoordinateSpace].
///
/// The x-coordinate represents the column, and the y-coordinate represents the row.
pub type CoordinatePair = CoordinateTuplet<2>;

/// A three dimensional rectangular cuboid coordinate space.
///
/// Like [TwoDimensionalBoxCoordinateSpace], but 3D.
///
/// While this is optimised for cuboid mazes, you can use whatever maze outline you want.
/// (You can create custom outlines by mutating the maze before passing it into a generator).
///
/// *This is a specialisation of [BoxCoordinateSpace] for three dimensions. See
/// [CoordinateSpace][crate::interface::point::CoordinateSpace] and [BoxCoordinateSpace] for
/// information on available methods and such.*
pub type ThreeDimensionalBoxCoordinateSpace = BoxCoordinateSpace<3>;

/// An (x, y, z) triplet for referencing points in a [ThreeDimensionalBoxCoordinateSpace].
///
/// The x-coordinate represents the column, the y-coordinate represents the row,
/// and the z-coordinate the layer.
pub type CoordinateTriplet = CoordinateTuplet<3>;
