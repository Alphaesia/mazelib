//! Block cells are cells where each cell is exactly one of:
//! [passage][BlockCellPrimaryValue::PASSAGE], [wall][BlockCellPrimaryValue::WALL],
//! [boundary][BlockCellPrimaryValue::BOUNDARY], or [unvisited][BlockCellPrimaryValue::UNVISITED].
//! 
//! They are named block cells because they have a blocky or pixellated appearance.
//!
//! # Examples
//!
//! ![A pixellated-looking maze, where every cell is one pixel][box-space-block-cell-coordinator-example]
#![doc = embed_doc_image::embed_image!("box-space-block-cell-coordinator-example", "src/doc/img/coordinate/box-space-block-cell/example-large.png")]

use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

use crate::implm::point::boxy::CoordinateTuplet;
use crate::interface::cell::{CellLocation, CellValue};

/// [`CellLocation`] for block cells.
///
/// *See also: [`BlockCellValue`]*
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockCellLocation<const DIMENSION: usize>(pub CoordinateTuplet<DIMENSION>);

impl <const DIMENSION: usize> CellLocation for BlockCellLocation<DIMENSION> {}

impl <const DIMENSION: usize> BlockCellLocation<DIMENSION> {
    /// Get the point at a given offset from this point
    /// (dimension refers to the direction of the offset - e.g. x-direction is dimension 0).
    #[must_use]
    pub fn offset(&self, axis: usize, offset: isize) -> Self {
        Self(self.0.offset(axis, offset))
    }

    /// Replace the position along the given axis with a new value.
    /// Useful when iterating along an axis.
    /// Suggestions for better names are welcome.
    #[must_use]
    pub fn at(&self, axis: usize, position: usize) -> Self {
        Self(self.0.at(axis, position))
    }
}

impl <const DIMENSION: usize> Index<usize> for BlockCellLocation<DIMENSION> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl <const DIMENSION: usize> IndexMut<usize> for BlockCellLocation<DIMENSION> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl <const DIMENSION: usize> Debug for BlockCellLocation<DIMENSION> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cl{:?}", self.0)
    }
}

impl <const DIMENSION: usize> From<[usize; DIMENSION]> for BlockCellLocation<DIMENSION> {
    fn from(coordinates: [usize; DIMENSION]) -> Self {
        BlockCellLocation(coordinates.into())
    }
}

impl <const DIMENSION: usize> From<[isize; DIMENSION]> for BlockCellLocation<DIMENSION> {
    fn from(pt: [isize; DIMENSION]) -> Self {
        BlockCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[u32; DIMENSION]> for BlockCellLocation<DIMENSION> {
    fn from(pt: [u32; DIMENSION]) -> Self {
        BlockCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[i32; DIMENSION]> for BlockCellLocation<DIMENSION> {
    fn from(pt: [i32; DIMENSION]) -> Self {
        BlockCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[u16; DIMENSION]> for BlockCellLocation<DIMENSION> {
    fn from(pt: [u16; DIMENSION]) -> Self {
        BlockCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[i16; DIMENSION]> for BlockCellLocation<DIMENSION> {
    fn from(pt: [i16; DIMENSION]) -> Self {
        BlockCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[u8; DIMENSION]> for BlockCellLocation<DIMENSION> {
    fn from(pt: [u8; DIMENSION]) -> Self {
        BlockCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[i8; DIMENSION]> for BlockCellLocation<DIMENSION> {
    fn from(pt: [i8; DIMENSION]) -> Self {
        BlockCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<BlockCellLocation<DIMENSION>> for [usize; DIMENSION] {
    fn from(pt: BlockCellLocation<DIMENSION>) -> Self {
        pt.0.into()
    }
}

/// A cell type where cells are either passage cells or wall
/// wall cells, with no in between. They are called Block Cells
/// because the resulting mazes look blocky / pixellated.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct BlockCellValue {
    /// The specific type or value of the cell. For more information
    /// see [`BlockCellPrimaryValue`].
    pub cell_type: BlockCellPrimaryValue,

    /// Whether this cell has been marked or flagged. This is a
    /// general-use field, with no specific meaning.
    pub marked: bool,
}

/*
 * If you were to represent a primary value as a 2-bit number, I'd do it like this:
 * 
 * 00 = UNVISITED
 * 01 = PASSAGE
 * 10 = WALL
 * 11 = BOUNDARY
 *
 * UNVISITED is assigned to 00 so that zero-initialising a buffer is equivalent to initialising it
 * to UNVISITED, which is what you want. Additionally, each bit individually represents something.
 * If the upper bit is set, the cell cannot be moved through (wall or boundary). If the lower bit
 * is set however, the cell can be carved through (to create a passage).
 *
 * Though alternatively, it could be nice to encode them in priority order.
 */
/// The possible values that a [block cell][super] can hold (ignoring flags and other data).
/// 
/// Each possibility directly maps to a [`ConnectionType`][crate::interface::cell::ConnectionType].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum BlockCellPrimaryValue {
    /// A cell that has not yet been generated.
    /// 
    /// An unvisited cell should never be accessible from within the maze itself after generation is
    /// complete.
    /// 
    /// # See Also
    /// 
    /// [`ConnectionType::UNVISITED`][crate::interface::cell::ConnectionType], the connection type
    /// this cell will yield (subject to
    /// [priority][crate::interface::cell::ConnectionType#priority]).
    #[default]
    UNVISITED,

    /// A cell that cannot be moved through.
    /// 
    /// A boundary cell should never be carved through.
    ///
    /// # See Also
    ///
    /// [`Self::WALL`], a cell that cannot be moved through initially but can be carved through to
    /// create a [`Self::PASSAGE`] cell.
    /// [`ConnectionType::BOUNDARY`][crate::interface::cell::ConnectionType], the connection type
    /// this cell will yield.
    BOUNDARY,

    /// A cell that cannot be moved through.
    ///
    /// A wall cell may be carved through, turning it into a [`Self::PASSAGE`] cell.
    ///
    /// # See Also
    ///
    /// [`Self::BOUNDARY`], a cell that cannot be moved and cannot be carved through.
    /// [`ConnectionType::UNVISITED`][crate::interface::cell::ConnectionType], the connection type
    /// this cell will yield (subject to
    /// [priority][crate::interface::cell::ConnectionType#priority]).
    WALL,

    /// A cell that can be moved through.
    ///
    /// # See Also
    ///
    /// [`ConnectionType::UNVISITED`][crate::interface::cell::ConnectionType], the connection type
    /// this cell will yield (subject to
    /// [priority][crate::interface::cell::ConnectionType#priority]).
    PASSAGE,
}

impl CellValue for BlockCellValue {
    fn is_fully_visited(&self) -> bool {
        self.cell_type != BlockCellPrimaryValue::UNVISITED
    }

    fn is_marked(&self) -> bool {
        self.marked
    }

    fn set_marked(&mut self, marked: bool) {
        self.marked = marked
    }
}