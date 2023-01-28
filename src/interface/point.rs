use std::fmt::Debug;
use std::hash::Hash;

use rand::Rng;

/// # See Also
///
/// [`Point`] -- the individual parts of a coordinate space
pub trait CoordinateSpace : Sized + Clone + Copy + Send + Sync + IntoIterator<Item = Self::PtType> + Debug {
    /// The [`Point`] type that goes with this coordinate space.
    type PtType: Point;
    type IntoIter: Iterator<Item = Self::PtType>;

    /// The maximum number of points/nodes in this coordinate space.
    fn logical_size(&self) -> usize;

    /// Every point that is directly adjacent
    fn neighbours_of_pt(&self, pt: Self::PtType) -> Vec<Self::PtType>;

    /// Note, a point is NOT considered to be adjacent to itself
    fn are_adjacent(pt1: Self::PtType, pt2: Self::PtType) -> bool;

    fn iter_from(&self, pt: Self::PtType) -> <Self as CoordinateSpace>::IntoIter;

    /// Return a random point in this coordinate space.
    fn choose<RNG: Rng + ?Sized>(&self, rng: &mut RNG) -> Self::PtType;
}

/// A logical location in a maze.
///
/// In less abstract terms, points are basically the potential junctions
/// in a maze.
///
/// Unlike cells, have semantic meaning, and it is the connection of points
/// (by the [`CoordinateSpace`]) that determine the logical structure of
/// the maze.
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