use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

use crate::implm::point::boxy::CoordinateTuplet;
use crate::interface::cell::{CellLocation, CellValue};

/// [`CellLocation`] for inline cells.
///
/// *See also: [`InlineCellValue`]*
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InlineCellLocation<const DIMENSION: usize>(pub CoordinateTuplet<DIMENSION>);

impl <const DIMENSION: usize> CellLocation for InlineCellLocation<DIMENSION> {}

impl <const DIMENSION: usize> InlineCellLocation<DIMENSION> {
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

impl <const DIMENSION: usize> Index<usize> for InlineCellLocation<DIMENSION> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl <const DIMENSION: usize> IndexMut<usize> for InlineCellLocation<DIMENSION> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl <const DIMENSION: usize> Debug for InlineCellLocation<DIMENSION> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cl{:?}", self.0)
    }
}

impl <const DIMENSION: usize> From<[usize; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(coordinates: [usize; DIMENSION]) -> Self {
        InlineCellLocation(coordinates.into())
    }
}

impl <const DIMENSION: usize> From<[isize; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [isize; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[u32; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [u32; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[i32; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [i32; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[u16; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [u16; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[i16; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [i16; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[u8; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [u8; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[i8; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [i8; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<InlineCellLocation<DIMENSION>> for [usize; DIMENSION] {
    fn from(pt: InlineCellLocation<DIMENSION>) -> Self {
        pt.0.into()
    }
}

/// A cell type where each cell tracks the state of
/// its own edges.
///
/// (i.e. whether an edge is a wall or a passage)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InlineCellValue<const DIMENSION: usize> {
    /// The connection type or edge type between this cell and
    /// its respective neighbour. This is purely from this cell's
    /// perspective, and does not account for the neighbour's
    /// corresponding edge type. Consequently, if the neighbour's
    /// corresponding edge is set to be a boundary, the connection
    /// as a whole will be considered to be a boundary, even if
    /// this cell's edge is set to be a passage.
    pub edges: [[InlineCellValueEdgeType; 2]; DIMENSION],

    /// Whether this cell has been marked or flagged. This is a
    /// general-use field, with no specific meaning.
    pub marked: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum InlineCellValueEdgeType {
    /// An edge that permits passage. Note that the neighbouring cell in this direction
    /// may have a wall or boundary, preventing one from moving there.
    PASSAGE,
    /// An edge that denies passage. Walls may be converted into passages by
    /// passage-carving generation algorithms.
    WALL,
    /// Like a wall, but it will never be touched by a generator. Boundaries
    /// are ideal for the outlines of mazes and other important structural features.
    BOUNDARY,
    /// An edge that has not been generated yet by a generator. An unvisited edge
    /// should never be accessible from a passage after generation is complete.
    /// A cell is considered
    UNVISITED,
}

/// The relative position of a wall in an [`InlineCellValue`] along an axis.
///
/// The dimension of an `InlineCellValue` determines how many axes it has. A cell
/// has two walls for each axes. For example, a two-dimensional `InlineCellValue` has
/// two axes: the x-axis and the y-axis. For an inline cell, it will have two walls
/// along the x-axis: one closer to zero and one further away from zero. Being closer-
/// to-zero or further-from-zero is what this enum represents.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum InlineCellValueEdgeSide {
    /// The wall that is furthest from zero along a given axis.
    POSITIVE,
    /// The wall that is closest to zero along a given axis.
    NEGATIVE,
}

impl <const DIMENSION: usize> InlineCellValue<DIMENSION> {
    #[must_use]
    pub fn get_wall(&self, axis: usize, side: InlineCellValueEdgeSide) -> InlineCellValueEdgeType {
        let side_index = match side {
            InlineCellValueEdgeSide::POSITIVE => 1,
            InlineCellValueEdgeSide::NEGATIVE => 0,
        };

        return self.edges[axis][side_index];
    }
}

impl <const DIMENSION: usize> CellValue for InlineCellValue<DIMENSION> {
    fn is_fully_visited(&self) -> bool {
        self.edges.into_iter().flat_map(|dim| dim.into_iter()).all(|edge| edge != InlineCellValueEdgeType::UNVISITED)
    }

    fn is_marked(&self) -> bool {
        self.marked
    }

    fn set_marked(&mut self, marked: bool) {
        self.marked = marked
    }
}

impl <const DIMENSION: usize> Default for InlineCellValue<DIMENSION> {
    #[must_use]
    fn default() -> Self {
        Self { edges: [[InlineCellValueEdgeType::UNVISITED; 2]; DIMENSION], marked: false }
    }
}