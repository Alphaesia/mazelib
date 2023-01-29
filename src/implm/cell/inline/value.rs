use crate::implm::cell::inline::InlineCellValueEdgeType::UNVISITED;
use crate::interface::cell::CellValue;

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
        self.edges.into_iter().flat_map(|dim| dim.into_iter()).all(|edge| edge != UNVISITED)
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
        Self { edges: [[UNVISITED; 2]; DIMENSION], marked: false }
    }
}