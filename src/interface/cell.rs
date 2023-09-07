//! Physical maze structure.
//!
//! Mazes are fundamentally a collection of cells. These cells define the physical structure of the
//! maze. The collections of cells are stored in
//! [`MazeBuffer`][crate::interface::buffer::MazeBuffer]s.
//!
//! # Recommended Reading
//!
//! 1. [`crate::interface`'s section on cells][crate::interface#cell] --- a detailed introduction
//! to cells and what they are.
//! 2. [`ConnectionType`] --- the ways that points can be connected.

use std::fmt::Debug;
use std::hash::Hash;

/// Location of a cell.
///
/// They are physical positions in a maze. They also act as a cell's "address". For
/// example, to query a cell's [value][CellValue], you give the cell's `CellLocation`
/// to the [`MazeCoordinator`][crate::interface::coordinator::MazeCoordinator].
pub trait CellLocation: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug {}

/// A unique identifier for a cell within a given maze.
///
/// Identifiers are sequential and start from zero.
///
/// They are primarily used by [buffers][crate::interface::buffer::MazeBuffer]
/// for the purposes of storing mazes.
///
/// The exact semantics of how cells are given their identifiers is
/// unspecified and up to each [`MazeCoordinator`][crate::interface::coordinator::MazeCoordinator].
/// Typically it will differ between implementations, and is not API.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CellID(pub usize);

/// Value of a cell.
///
/// Cells can be marked. Marks are a general-purpose signalling mechanism, and have
/// no universal interpretation. It is recommended you clean up any marks you make,
/// as other components may make marks of their own and get confused by pre-existing
/// marks.
pub trait CellValue: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Default + Debug {
    /// If the cell has not been fully generated (visited). Partially-generated
    /// cells are not considered fully visited. This is because when applying
    /// templates, such as a solid boundary around the edge of the maze, the cells
    /// that get touched should still be considered by generators.
    #[must_use]
    fn is_fully_visited(&self) -> bool;

    /// If the cell is currently marked.
    #[must_use]
    fn is_marked(&self) -> bool;

    /// Set the cell's mark flag.
    fn set_marked(&mut self, marked: bool);
}

/// Type of connection (graph theory: *edge*) (e.g. wall, passage) between two points.
///
/// # Priority
///
/// Sometimes there are situations when multiple connection types could apply. One such situation
/// would be when both cells in a connection independently store the connection type. In these
/// cases, the following priority order applies:
/// 1. `BOUNDARY`
/// 2. `UNVISITED`
/// 3. `WALL`
/// 4. `PASSAGE`
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ConnectionType {
    /// A connection is considered a passage if one can move through it.
    PASSAGE,
    /// A connection is considered a wall if one cannot move through
    /// it in any capacity. Walls may be converted into passages by
    /// generators.
    WALL,
    /// Like a wall, but it will never be touched by a generator.
    /// Boundaries are ideal for the outlines of mazes and other
    /// important structural features.
    BOUNDARY,
    /// The default state, when cells have not yet been fully generated
    /// by a generator. An unvisited connection should never be accessible
    /// from within the maze itself after generation is complete.
    UNVISITED
}
