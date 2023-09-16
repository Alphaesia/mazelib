//! Working maze storage.
//!
//! Mazes are stored in [`MazeBuffer`]s. These typically store the mazes in memory.
//!
//! For exporting mazes to persistent forms (like images), see [`crate::interface::export`].
//!
//! # Recommended Reading
//! 1. [`MazeBuffer`] -- the buffer trait.

use std::fmt::Debug;
use std::num::NonZeroUsize;

use crate::interface::cell::{CellID, CellValue};

/// A maze cell container.
///
/// Maze buffers store the values of all cells in a maze. This is the meat of the maze. They
/// are not responsible for other [core maze components](crate::interface#core-components).
/// These are maintained by the [maze coordinator][crate::interface::coordinate::MazeCoordinator]
/// (which also holds a maze's buffer).
///
/// A buffer's size is determined at construction and cannot be changed.
///
/// Buffers are very low-level and not intended to be interacted with directly. They should
/// always be accessed through the maze coordinator. Likewise, they should not be constructed
/// directly. The maze coordinator is responsible for the construction of the buffer.
///
/// While most buffers would store their contents in memory, it is conceivable that a buffer
/// could stream the maze from disk (or elsewhere). This would allow mazes too large to fit in
/// memory to be generated.
pub trait MazeBuffer<CellVal: CellValue> : Debug + Send {
    /// Construct a new buffer.
    ///
    /// As discussed in the type documentation, typically you wouldn't invoke this function
    /// directly. Normally you'd use a [`MazeBuffer`] which would abstract away interacting with
    /// the buffer from you.
    ///
    ///  # Parameters
    /// `cell_count` --- the size of the maze's cell space, and the number of cells that the
    ///                  buffer is required to track. If the buffer differentiates between size
    ///                  and capacity, this must be be less than its capacity. The size cannot
    ///                  be changed once a buffer is constructed. As a maze must always have at
    ///                  least one point, and every point maps to at least one cell, all mazes
    ///                  must have at least one cell, hence `cell_count` is a `NonZeroUsize`.
    ///
    /// # Examples
    /// ```
    /// # use mazelib::implm::buffer::VecBuffer;
    /// # use mazelib::implm::cell::block::BlockCellValue;
    /// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
    /// # use mazelib::interface::buffer::MazeBuffer;
    /// # use mazelib::interface::point::CoordinateSpace;
    /// #
    /// let coord_space = BoxCoordinateSpace::new_checked([3, 3]);
    ///
    /// let buffer = VecBuffer::<BlockCellValue>::new(coord_space.logical_size());
    /// ```
    #[must_use]
    fn new(cell_count: NonZeroUsize) -> Self;

    /// Get the value of a given cell.
    ///
    /// # Parameters
    /// `cell` --- the ID of the cell to retrieve. Must be inbounds (<= this buffer's size).
    ///
    /// # Examples
    /// ```
    /// # use std::num::NonZeroUsize;
    /// # use mazelib::implm::buffer::VecBuffer;
    /// # use mazelib::implm::cell::block::BlockCellValue;
    /// # use mazelib::interface::buffer::MazeBuffer;
    /// # use mazelib::interface::cell::CellID;
    /// #
    /// // Make a new buffer
    /// let buffer = VecBuffer::<BlockCellValue>::new(NonZeroUsize::new(1).expect("If this fails the sky is falling"));
    ///
    /// assert_eq!(BlockCellValue::default(), buffer.get(CellID(0)));
    /// ```
    #[must_use]
    fn get(&self, cell: CellID) -> CellVal;

    /// Get the value of a given cell for mutation.
    ///
    /// Useful for modifying parts of a cell while leaving the rest of it untouched. If you wish
    /// to change the value of a cell in its entirety, consider [`set()`][Self::set].
    ///
    /// # Parameters
    /// `cell` --- the ID of the cell to retrieve. Must be inbounds (≤ this buffer's size).
    ///
    /// # Examples
    /// ```
    /// # use std::num::NonZeroUsize;
    /// # use mazelib::implm::buffer::VecBuffer;
    /// # use mazelib::implm::cell::block::{BlockCellValue, BlockCellValueType};
    /// # use mazelib::interface::buffer::MazeBuffer;
    /// # use mazelib::interface::cell::CellID;
    /// #
    /// // Make a new buffer
    /// let mut buffer = VecBuffer::<BlockCellValue>::new(NonZeroUsize::new(1).expect("If this fails the sky is falling"));
    ///
    /// let cell_id = CellID(0);
    /// let cell = BlockCellValue { cell_type: BlockCellValueType::PASSAGE, marked: false };
    ///
    /// buffer.set(cell_id, cell);
    ///
    /// assert!(buffer.get(cell_id).marked == false);
    ///
    /// buffer.get_mut(cell_id).marked = true;
    ///
    /// assert!(buffer.get(cell_id).marked);
    /// ```
    #[must_use]
    fn get_mut(&mut self, cell: CellID) -> &mut CellVal;

    /// Set the value of a given cell.
    ///
    /// If you wish to modify only part of a cell while leaving the rest of it untouched,
    /// consider [`get_mut()`][Self::get_mut].
    ///
    /// # Parameters
    /// `cell` --- the ID of the cell to modify. Must be inbounds (≤ this buffer's size).
    ///
    /// # Examples
    /// ```
    /// # use std::num::NonZeroUsize;
    /// # use mazelib::implm::buffer::VecBuffer;
    /// # use mazelib::implm::cell::block::{BlockCellValue, BlockCellValueType};
    /// # use mazelib::interface::buffer::MazeBuffer;
    /// # use mazelib::interface::cell::CellID;
    /// #
    /// // Make a new buffer
    /// let mut buffer = VecBuffer::<BlockCellValue>::new(NonZeroUsize::new(1).expect("If this fails the sky is falling"));
    ///
    /// let cell_id = CellID(0);
    /// let cell = BlockCellValue { cell_type: BlockCellValueType::PASSAGE, marked: false };
    ///
    /// buffer.set(cell_id, cell);
    ///
    /// assert_eq!(cell, buffer.get(cell_id));
    /// ```
    fn set(&mut self, cell: CellID, new_value: CellVal);
}