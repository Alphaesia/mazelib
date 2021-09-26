//! Everything to do with reading and manipulating a maze at a high-level.
//!
//! Mazes are fundamentally a collection of cells. These collections are stored
//! in [MazeBuffer][crate::interface::buffer::MazeBuffer]s.
//!
//! Each cell has a value. While different types of cells can store different information,
//! the value of each cell is one of four [types][CellValueType]: [PASSAGE][CellValueType::PASSAGE],
//! [WALL][CellValueType::WALL], [BOUNDARY][CellValueType::BOUNDARY], or
//! [UNVISITED][CellValueType::UNVISITED].
//!
//! While a collection of cells constitutes a maze, they need to be
//! interpreted before any useful analysis can be done. This is the purpose
//! of [CellManager] - to hold the necessary information for interpreting
//! cells.
//!
//! Recommended reading order:
//! 1. [CellValue]
//! 2. [CellValueType]
//! 3. [CellManager]

use std::fmt::Debug;
use crate::interface::point::CoordinateSpace;
use crate::internal::noise_util::pt;

/// TODO rewrite this
///
/// A cell space determines how cells physically connect.
///
/// It operates on a conceptual level between the raw storage (the buffer) and the abstract
/// graph representation of the maze - the cell space arbitrates how individual cells connect.
///
/// A CellSpace handles updating the maze buffer. When you or a generator want to modify a
/// cell, you must talk to the associated cell space.
///
/// For instance, if the generator wants to connect two cells together as passages, the
/// cell space is responsible for updating the buffer with the relevant cell changes
/// (e.g. removing the adjacent wall from the origin cell, and marking the destination cell
/// as visited).
///
/// A maze is accompanied by a cell space at all times, and it cannot be changed after creation.
/// It is a logical error to use a cell space with a different maze object, or a maze object
/// with a different cell space.
///
/// Passing in an out-of-bounds point as an argument will cause a panic with debug assertions
/// enabled, and is undefined with them off. However, an out-of-bounds point will never cause
/// an out-of-bounds write; memory safety is always preserved.
///
/// *Note: In the source, `<Self::CoordSpace as CoordinateSpace>::PtType` is contracted to `pt!()`
/// for readability.*
pub trait CellManager: Debug {
    /// Since each CellManager will be specialised for dealing with a particular type
    /// of coordinate space and cell value, these are associated types rather than
    /// generic parameters.
    type CoordSpace: CoordinateSpace;
    type CellVal: CellValue;

    fn coord_space(&self) -> &Self::CoordSpace;

    /// Get the value of a cell at `pt`.
    fn get(&self, pt: pt!()) -> Self::CellVal;

    /// Make a passage at `pt`.
    fn make_passage(&mut self, pt: pt!());

    /// Make a passage from `from` to `to`. The points must be adjacent.
    /// If they are not, this function will panic with debug assertions enabled.
    /// Passing in non-adjacent points without debug assertions enabled is undefined.
    fn make_passage_between(&mut self, from: pt!(), to: pt!());

    /// Make a wall at `pt`.
    fn make_wall(&mut self, pt: pt!());

    /// Make a wall from `from` to `to`. The points must be adjacent.
    /// If they are not, this function will panic with debug assertions enabled.
    /// Passing in non-adjacent points without debug assertions enabled is undefined.
    fn make_wall_between(&mut self, from: pt!(), to: pt!());

    /// Make a boundary at `pt`.
    fn make_boundary(&mut self, pt: pt!());

    /// Make a boundary from `from` to `to`. The points must be adjacent.
    /// If they are not, this function will panic with debug assertions enabled.
    /// Passing in non-adjacent points without debug assertions enabled is undefined.
    fn make_boundary_between(&mut self, from: pt!(), to: pt!());
}

// TODO rename CellValue to Cell? or rename BufferLocation to Cell?
//  (and if the former, keep CellValueType as-is, or rename to CellType?)
/// A value of a cell. These are what is physically stored in a maze.
///
/// Cells can hold any data they like. However, each cell falls under one
/// of four classes. See [CellValueType] for information on each of these.
///
/// Remember that [Point][super::point::Point]s map to cells, so this also
/// represents the value at any given [Point][super::point::Point].
///
/// # Implementing
///
/// For any implementation of CellValue, its default value (as per the [Default] trait)
/// should be a should an [CellValueType::UNVISITED] cell.
pub trait CellValue: Copy + Clone + Send + Sync + Sized + PartialEq + Eq + Default + Debug {
    /// The cell's class.
    ///
    /// For more information, see [CellValueType].
    fn get_type(&self) -> CellValueType;

    /// Helper function - equivalent to `get_type() == CellValueType::PASSAGE`.
    #[inline]
    fn is_passage(&self) -> bool {
        self.get_type() == CellValueType::PASSAGE
    }

    /// Helper function - equivalent to `get_type() == CellValueType::WALL`.
    fn is_wall(&self) -> bool {
        self.get_type() == CellValueType::WALL
    }

    /// Helper function - equivalent to `get_type() == CellValueType::BOUNDARY`.
    fn is_boundary(&self) -> bool {
        self.get_type() == CellValueType::BOUNDARY
    }

    /// Helper function - equivalent to `get_type() == CellValueType::UNVISITED`.
    fn is_unvisited(&self) -> bool {
        self.get_type() == CellValueType::UNVISITED
    }
}

/// While cells can store any data they want, for the purposes of generating and
/// solving mazes there are four types of cell.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CellValueType {
    /// A cell is considered a passage one can move through it. Even if a cell is
    /// tracking its walls with respect to its neighbours (like an
    /// [InlineCellValue][crate::implm::cell::inline::InlineCellValue] does), it is
    /// still considered a passage.
    PASSAGE,
    /// A cell is considered a wall if one cannot move through it in any capacity.
    /// Walls may be converted into passages by generation algorithms.
    WALL,
    /// Like a wall, but it will never be touched by a generator. Boundaries
    /// are ideal for the outlines of mazes and other important structural features.
    BOUNDARY,
    /// This cell has not been generated yet by a generator. An unvisited cell
    /// should never be accessible from a passage after generation is complete.
    UNVISITED
}