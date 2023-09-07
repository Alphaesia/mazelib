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
    pub fn offset(&self, axis: usize, offset: isize) -> Self {
        Self(self.0.offset(axis, offset))
    }

    /// Replace the position along the given axis with a new value.
    /// Useful when iterating along an axis.
    /// Suggestions for better names are welcome.
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
    /// see [`BlockCellValueType`].
    pub cell_type: BlockCellValueType,

    /// Whether this cell has been marked or flagged. This is a
    /// general-use field, with no specific meaning.
    pub marked: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum BlockCellValueType {
    #[default]
    UNVISITED,
    BOUNDARY,
    WALL,
    PASSAGE,
}

impl CellValue for BlockCellValue {
    fn is_fully_visited(&self) -> bool {
        self.cell_type != BlockCellValueType::UNVISITED
    }

    fn is_marked(&self) -> bool {
        self.marked
    }

    fn set_marked(&mut self, marked: bool) {
        self.marked = marked
    }
}