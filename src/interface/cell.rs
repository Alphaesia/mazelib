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
/// *Note: In the source, `<<Self as CellManager>::CoordSpace as CoordinateSpace>::PtType` is
/// contracted to `pt!()` for readability.*
///
/// # Selecting a Cell Space
///
/// The cell space you use will depend entirely based on the types of coordinate space and
/// cell values you use.
///
/// There is a list of built-in CellSpaces under the [Implementors](#implementors) header at
/// the bottom of this page. If it does not contain the combination of coordinate space and
/// cell value you want, or if you are using custom coordinate spaces / cell values, you will
/// need to implement your own.
pub trait CellManager: Debug {
    /// Since each CellManager will be specialised for dealing with a particular type
    /// of coordinate space and cell value, these are associated types rather than
    /// generic parameters.
    type CoordSpace: CoordinateSpace;
    type CellVal: CellValue;

    fn coord_space(&self) -> &Self::CoordSpace;

    /// Get the value of a cell at `pt`.
    fn get(&self, pt: pt!()) -> Self::CellVal;

    /// Get the "connection" between two points.
    ///
    /// If you were walking from `from` to `to`, this is what
    /// you would encounter.
    ///
    /// More specifically:
    /// * If either side is a [CellValueType::BOUNDARY],
    ///   return [CellValueType::BOUNDARY].
    /// * Else, if either side is [CellValueType::UNVISITED],
    ///   return [CellValueType::UNVISITED].
    /// * Else, if either side is a [CellValueType::WALL],
    ///   return [CellValueType::WALL].
    /// * Else, return [CellValueType::PASSAGE].
    fn get_connection(&self, from: pt!(), to: pt!()) -> CellConnectionType;

    /// Returns true when [Self::get_connection] returns [CellValueType::PASSAGE].
    fn is_passage_between(&self, from: pt!(), to: pt!()) -> bool {
        self.get_connection(from, to) == CellConnectionType::PASSAGE
    }

    /// Returns true when [Self::get_connection] returns [CellValueType::WALL].
    fn is_wall_between(&self, from: pt!(), to: pt!()) -> bool {
        self.get_connection(from, to) == CellConnectionType::WALL
    }

    /// Returns true when [Self::get_connection] returns [CellValueType::BOUNDARY].
    fn is_boundary_between(&self, from: pt!(), to: pt!()) -> bool {
        self.get_connection(from, to) == CellConnectionType::BOUNDARY
    }

    /// Returns true when [Self::get_connection] returns [CellValueType::UNVISITED].
    fn is_unvisited_between(&self, from: pt!(), to: pt!()) -> bool {
        self.get_connection(from, to) == CellConnectionType::UNVISITED
    }

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

/// Structs that implement this trait are "addresses" for cells. You can
/// give them to a [`CellManager`] to retrieve the [value][CellValue] of a cell.
///
/// This trait does not have any associated logic, nor does any generic interface
/// accept them. Rather, it is a signifier of the intent and purpose of structs
/// that implement it. It allows implementers to clearly delineate to readers
/// and future code editors what role a struct players in the maze model.
pub trait CellLocation {}

/// A value of a cell.
///
/// Cells differ points in one key way. While points are live in the abstract
/// world of a [CoordinateSpace][crate::interface::point::space::CoordinateSpace],
/// cells are in the presentation layer of a maze, the [CellManager]. They are
/// also what is stored are buffers.
pub trait CellValue: Copy + Clone + Send + Sync + Sized + PartialEq + Eq + Default + Debug {
    /// If the cell has not been fully generated (visited). Partially-generated
    /// cells are not considered fully visited. This is because when applying
    /// templates, such as a solid boundary around the edge of the maze, the cells
    /// that get touched should still be considered by generators.
    fn is_fully_visited(&self) -> bool;
}

// TODO rename to CellEdgeType to be consistent with graph theory?
//  (also maps better onto traditional 2D mazes)
/// A type of connection (e.g. wall, boundary) between two cells.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CellConnectionType {
    /// A connection is considered a passage one can move through it.
    PASSAGE,
    /// A connection is considered a wall if one cannot move through
    /// it in any capacity. Walls may be converted into passages by
    /// generation algorithms.
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