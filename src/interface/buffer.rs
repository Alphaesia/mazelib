//! Working maze storage.
//!
//! Mazes are typically stored in memory, but do not have to be. It
//! is up to the implementation of the [`MazeBuffer`].
//!
//! For exporting mazes to persistent forms (like images), see [`super::render`].
//!
//! # Recommended Reading
//! 1. [`MazeBuffer`] -- the buffer trait.

use std::fmt::Debug;

use crate::interface::cell::{CellID, CellValue};

/// A maze cell container.
///
/// Maze buffers store the values of all cells in a maze. This is the meat of the maze. They are
/// not responsible for other [core maze components](crate::interface#core-components), which are held by the
/// [cell manager][crate::interface::cell::CellManager] instead.
///
/// A buffer's size is determined at construction, and cannot be changed.
///
/// Buffers are very low-level and not intended to be interacted with directly. They should
/// always be accessed through the maze's cell manager.
///
/// Likewise, they should not be constructed directly. The maze's cell manager is responsible
/// for the construction of the buffer.
///
/// While most buffers would store their contents in memory, it is conceivable that a buffer
/// could stream the maze from disk (or elsewhere). This would allow mazes too big to fit in
/// memory to be generated.
pub trait MazeBuffer<CellVal: CellValue> : Debug + Send {
    /// Construct a new buffer.
    ///
    /// `cell_count` is the size of the maze's cell space, and is the number of cells that the
    /// buffer is required to track. This is the buffer's size and capacity. It cannot be changed
    /// once a buffer is constructed.
    fn new(cell_count: usize) -> Self;

    /// Get the value of a given cell.
    ///
    /// # Panics
    ///
    /// Panics if `cell` out of bounds (is greater than or equal to this buffer's size).
    fn get(&self, cell: CellID) -> CellVal;

    /// Get the value of a given cell for mutation.
    ///
    /// # Panics
    ///
    /// Panics if `cell` out of bounds (is greater than or equal to this buffer's size).
    fn get_mut(&mut self, cell: CellID) -> &mut CellVal;

    /// Set the value of a given cell.
    ///
    /// # Panics
    ///
    /// Panics if `cell` out of bounds (is greater than or equal to this buffer's size).
    fn set(&mut self, cell: CellID, new_value: CellVal);
}