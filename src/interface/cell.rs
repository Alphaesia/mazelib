//! Reading and manipulating mazes at a high level.
//!
//! Mazes are fundamentally a collection of cells. These collections are stored in
//! [`MazeBuffer`][crate::interface::buffer::MazeBuffer]s.
//!
//! While a collection of cells constitutes a maze, they need to be interpreted before any
//! useful analysis can be done. This is the purpose of the [`CellManager`] - to hold the
//! necessary information for interpreting cells.
//!
//! # Recommended Reading
//! *(In order)*
//! 1. [`crate::interface`'s section on cells][crate::interface#cell] --- a detailed introduction
//! to cells and what they are.
//! 2. [`ConnectionType`] --- the ways that points can be connected.
//! 3. [`CellManager`] --- the unifying glue and centre of the maze model.

use std::fmt::Debug;
use crate::interface::point::CoordinateSpace;
use crate::internal::noise_util::pt;

/// Translates between [points][crate::interface::point::Point], [cells][super], and
/// [buffer locations][crate::interface::buffer::BufferLocation]. Executes queries
/// on mazes and modifications to mazes.
///
/// [Cells have no inherent meaning][crate::interface]. They must be interpreted in context.
/// This is exactly what a Cell Manager does.
///
/// Due to requiring intimate knowledge of how the coordinate space works and how the cell space
/// works, each Cell Manager is specific to a single CoordinateSpace-CellValue pair. There
/// is generally only one such manager for each pair.
pub trait CellManager: Debug {
    /*
     * Note: In the source, `<<Self as CellManager>::CoordSpace as CoordinateSpace>::PtType` is
     * contracted to `pt!()` for brevity.
     */

    /// The type of coordinate space this cell manager supports.
    type CoordSpace: CoordinateSpace;

    /// The value of the type of cell this cell manager uses.
    type CellVal: CellValue;

    /// Return the coordinate space for this maze.
    fn coord_space(&self) -> &Self::CoordSpace;

    /// Return the value of the point `pt`.
    fn get(&self, pt: pt!()) -> Self::CellVal;

    /// Return the type of connection (graph theory: *edge*) between two points.
    ///
    /// If you attempted to walk from `from` to `to` this is what you would encounter.
    ///
    /// The order of the arguments is important and has semantic meaning. Swapping
    /// the arguments may produce different results.
    ///
    /// This method follows the
    /// [priority order outlined in `ConnectionType`][ConnectionType#priority].
    fn get_connection(&self, from: pt!(), to: pt!()) -> ConnectionType;

    /// Return true when [`get_connection(from, to)`][Self::get_connection] returns
    /// [`ConnectionType::PASSAGE`].
    fn is_passage_between(&self, from: pt!(), to: pt!()) -> bool {
        self.get_connection(from, to) == ConnectionType::PASSAGE
    }

    /// Return true when [`get_connection(from, to)`][Self::get_connection] returns
    /// [`ConnectionType::WALL`].
    fn is_wall_between(&self, from: pt!(), to: pt!()) -> bool {
        self.get_connection(from, to) == ConnectionType::WALL
    }

    /// Return true when [`get_connection(from, to)`][Self::get_connection] returns
    /// [`ConnectionType::BOUNDARY`].
    fn is_boundary_between(&self, from: pt!(), to: pt!()) -> bool {
        self.get_connection(from, to) == ConnectionType::BOUNDARY
    }

    /// Return true when [`get_connection(from, to)`][Self::get_connection] returns
    /// [`ConnectionType::UNVISITED`].
    fn is_unvisited_between(&self, from: pt!(), to: pt!()) -> bool {
        self.get_connection(from, to) == ConnectionType::UNVISITED
    }

    /// Make a passage at `pt`.
    fn make_passage(&mut self, pt: pt!());

    /// Make a passage from `from` to `to`. The points must be adjacent.
    /// If they are not, this function will panic with debug assertions enabled.
    /// Passing in non-adjacent points without debug assertions enabled is undefined.
    ///
    /// The order of the arguments is important and has semantic meaning. Swapping
    /// the arguments may produce different results.
    fn make_passage_between(&mut self, from: pt!(), to: pt!());

    /// Make a wall at `pt`.
    fn make_wall(&mut self, pt: pt!());

    /// Make a wall from `from` to `to`. The points must be adjacent.
    /// If they are not, this function will panic with debug assertions enabled.
    /// Passing in non-adjacent points without debug assertions enabled is undefined.
    ///
    /// The order of the arguments is important and has semantic meaning. Swapping
    /// the arguments may produce different results.
    fn make_wall_between(&mut self, from: pt!(), to: pt!());

    /// Make a boundary at `pt`.
    fn make_boundary(&mut self, pt: pt!());

    /// Make a boundary from `from` to `to`. The points must be adjacent.
    /// If they are not, this function will panic with debug assertions enabled.
    /// Passing in non-adjacent points without debug assertions enabled is undefined.
    ///
    /// The order of the arguments is important and has semantic meaning. Swapping
    /// the arguments may produce different results.
    fn make_boundary_between(&mut self, from: pt!(), to: pt!());
}

/// Location of a cell.
///
/// Structs that implement this trait are "addresses" for cells. You can
/// give them to a [`CellManager`] to retrieve the [value][CellValue] of a cell.
///
/// This trait does not have any associated logic, nor does any generic interface
/// accept them. Rather, it is a signifier of the intent and purpose of structs
/// that implement it. It allows implementers to clearly delineate to readers
/// and future code editors what role a struct plays in the maze model.
pub trait CellLocation {}

/// Value of a cell.
///
/// Cells can be marked. Marks are a general-purpose signalling mechanism, and have
/// no universal interpretation. It is recommended you clean up any marks you make,
/// as other components may make marks of their own and get confused by pre-existing
/// marks.
pub trait CellValue: Copy + Clone + Send + Sync + Sized + PartialEq + Eq + Default + Debug {
    /// If the cell has not been fully generated (visited). Partially-generated
    /// cells are not considered fully visited. This is because when applying
    /// templates, such as a solid boundary around the edge of the maze, the cells
    /// that get touched should still be considered by generators.
    fn is_fully_visited(&self) -> bool;

    /// If the cell is currently marked.
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

/// Graph theory nomenclature alias for [`ConnectionType`].
pub type EdgeType = ConnectionType;