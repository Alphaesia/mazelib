use std::fmt::Debug;
use crate::interface::cell::CellValue;

/// A MazeBuffer is a storage vessel for a maze in memory.
/// It is used to mutate the maze at the most primitive level.
///
/// It only accepts single BufferLocations.
///
/// For general interaction with a maze, a user should use the higher-level
/// [CellManager][crate::interface::cell::CellManager] API. A maze buffer should only be directly read
/// from or written to by a [CellManager][crate::interface::cell::CellManager].
///
/// Likewise, MazeBuffers are not intended to be constructed directly. Rather, they
/// will be constructed by [CellManager][crate::interface::cell::CellManager]s from information such as the
/// coordinate space.
///
/// A MazeBuffer's size is determined at construction, and cannot be changed.
pub trait MazeBuffer<CellVal: CellValue> : Debug + Send {
    /// Construct a new MazeBuffer.
    ///
    /// `cell_count` is the number of cells that the buffer is required to track. This
    /// is the buffer's size and capacity. It cannot be changed once a buffer is constructed.
    fn new(cell_count: usize) -> Self;

    /// Get the value of a given cell.
    ///
    /// This should *not* be called from maze generation code. This should only be called by
    /// a [CellManager][crate::interface::cell::CellManager]. All other users should proxy through
    /// [CellManager::get()][crate::interface::cell::CellManager::get()].
    ///
    /// # Panics
    ///
    /// Panics if `loc` is greater than or equal to this buffer's size.
    fn get(&self, loc: BufferLocation) -> CellVal;

    /// Set the value of a given cell.
    ///
    /// This should *not* be called from maze generation code. This should only be called by
    /// a [CellManager][crate::interface::cell::CellManager]. All other users should proxy through
    /// the various [CellManager][crate::interface::cell::CellManager] methods (such as
    /// [CellManager::make_passage()][crate::interface::cell::CellManager::make_passage()]).
    ///
    /// # Panics
    ///
    /// Panics if `loc` is greater than or equal to this buffer's size.
    fn set(&mut self, loc: BufferLocation, new_value: CellVal);
}

pub struct BufferLocation(pub usize);