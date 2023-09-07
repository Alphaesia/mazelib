//! Manipulating and managing mazes.
//!
//! Mazes contain [many different components][crate::interface]. There needs to exist an entity
//! that can understand each component and unify them into a single construct. That is the
//! coordinator.
//!
//! # Recommended Reading
//!
//! 1. [`MazeCoordinator`] --- the primary maze trait, responsible for coordinating all of a maze's
//!                            components.

use std::fmt::Debug;
use crate::interface::cell::{CellLocation, CellValue, ConnectionType};
use crate::interface::point::CoordinateSpace;
use crate::pt;

/// Handles high-level queries on mazes.
///
/// The coordinator defines the mapping between a maze's coordinate space and its cell space.
/// This means it understands how to translate high-level queries (like *"What is the
/// [connection type][ConnectionType] between these points?"*) into checks on specific cells
/// in the maze's buffer. It also understands which cells need to be modified and how to perform
/// basic changes to a maze, like carving a new passageway.
///
/// The coordinator owns all of a maze's components. This means that the `MazeCoordinator`
/// effectively is the maze itself.
///
/// Each coordinator has different characteristics that dramatically affect the maze's structure
/// so you should consult your chosen implementation carefully.
pub trait MazeCoordinator: Debug {
    /*
     * Note: In the source, `<<Self as MazeCoordinator>::CoordSpace as CoordinateSpace>::PtType` is
     * contracted to `pt!()` for brevity.
     */

    /// The type of coordinate space this coordinator supports.
    type CoordSpace: CoordinateSpace;

    /// The location type for the cell type this coordinator uses.
    type CellLoc: CellLocation;

    /// The value type for the cell type this coordinator uses.
    type CellVal: CellValue;

    /// Return the coordinate space for this maze.
    #[must_use]
    fn coord_space(&self) -> &Self::CoordSpace;

    /// Return the value of the point `pt`.
    #[must_use]
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
    #[must_use]
    fn get_connection(&self, from: pt!(), to: pt!()) -> ConnectionType;

    /// Return true when [`get_connection(from, to)`][Self::get_connection] returns
    /// [`ConnectionType::PASSAGE`].
    #[must_use]
    fn is_passage_between(&self, from: pt!(), to: pt!()) -> bool {
        self.get_connection(from, to) == ConnectionType::PASSAGE
    }

    /// Return true when [`get_connection(from, to)`][Self::get_connection] returns
    /// [`ConnectionType::WALL`].
    #[must_use]
    fn is_wall_between(&self, from: pt!(), to: pt!()) -> bool {
        self.get_connection(from, to) == ConnectionType::WALL
    }

    /// Return true when [`get_connection(from, to)`][Self::get_connection] returns
    /// [`ConnectionType::BOUNDARY`].
    #[must_use]
    fn is_boundary_between(&self, from: pt!(), to: pt!()) -> bool {
        self.get_connection(from, to) == ConnectionType::BOUNDARY
    }

    /// Return true when [`get_connection(from, to)`][Self::get_connection] returns
    /// [`ConnectionType::UNVISITED`].
    #[must_use]
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
