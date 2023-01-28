use std::fmt::Debug;
use std::hash::Hash;

use rand::Rng;

/// # See Also
///
/// [`Point`] -- the individual parts of a coordinate space
pub trait CoordinateSpace : Sized + Clone + Copy + Send + Sync + Debug {
    /// The [`Point`] type that goes with this coordinate space.
    type PtType: Point;
    type Iter: Iterator<Item = Self::PtType>;

    /// Return the number of points in this coordinate space. This never
    /// changes.
    fn logical_size(&self) -> usize;

    /// Return a vector containing every point in the coordinate space
    /// that is adjacent to `pt` (as determined by [`are_adjacent()`](Self::are_adjacent)).
    fn neighbours_of_pt(&self, pt: Self::PtType) -> Vec<Self::PtType>;

    /// Return whether two points are adjacent in this coordinate space.
    ///
    /// A point is **not** considered to be adjacent to itself.
    fn are_adjacent(pt1: Self::PtType, pt2: Self::PtType) -> bool;

    /// Return an iterator that yields every point in this coordinate space.
    ///
    /// Every point yielded is adjacent to a point previously yielded.
    /// The one exception to this is where there is no possible path from the
    /// current point to any previously yielded point. For example, the first
    /// point, or if the space contains two or more isolated components, the
    /// first point of each component.
    ///
    /// If the coordinate space has any notion of an origin, it is the first
    /// point yielded.
    fn iter(&self) -> Self::Iter;

    /// Return an iterator that behaves exactly like [`iter()`](Self::iter),
    /// but skips every point up to and including `pt`.
    ///
    /// Points that are skipped are still considered to have been yielded for
    /// the purposes of the path constraint in `into_iter()`.
    fn iter_from(&self, pt: Self::PtType) -> Self::Iter;

    /// Return a random point in this coordinate space.
    fn choose<RNG: Rng + ?Sized>(&self, rng: &mut RNG) -> Self::PtType;
}

/// A logical location in a maze.
///
/// In less abstract terms, points are basically the potential junctions
/// in a maze.
///
/// Unlike cells, points have semantic meaning, and it is the connection
/// of points (by the [`CoordinateSpace`]) that determine the logical
/// structure of the maze.
///
/// A detailed introduction can be found in the
/// [section on points in the core maze components explanation][crate::interface#point].
///
/// # See Also
///
/// * [`CoordinateSpace`] -- the parent container of points.
/// * [`CellLocation`][crate::interface::cell::CellLocation] -- physical (rather than logical) locations in a maze.
///
/// # Implementation
///
/// With regards to implementation, points are or similar to coordinate
/// tuplets (e.g. `(x, y)`). Points do not have to be coordinates, but
/// must effectively act like them. More specifically, they must have the
/// following properties:
/// * Immutable
/// * Fixed Size
/// * Cloneable
/// * Equality must be reflexive, symmetric, transitive, *and anti-symmetric*
/// * Hashable
pub trait Point: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug {}