use crate::implm::cell::inline::InlineCellValueWallType::UNVISITED;
use crate::interface::cell::CellValue;

/// TODO rewrite
///
/// The first element is the cell in the positive direction,
/// the 2nd is the cell in the negative direction.
/// Even if it is `None` for a particular direction, the
/// neighbouring cell in that direction may have a wall there.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct InlineCellValue<const DIMENSION: usize>(pub [[InlineCellValueWallType; 2]; DIMENSION]);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum InlineCellValueWallType {
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
    UNVISITED
}

impl <const DIMENSION: usize> CellValue for InlineCellValue<DIMENSION> {
    fn is_fully_visited(&self) -> bool {
        self.0.into_iter().flat_map(|dim| dim.into_iter()).all(|edge| edge != UNVISITED)
    }
}

impl <const DIMENSION: usize> InlineCellValue<DIMENSION> {
    pub fn is_fully_unvisited(&self) -> bool {
        self.0.into_iter().flat_map(|dim| dim.into_iter()).all(|edge| edge == UNVISITED)
    }
}

impl <const DIMENSION: usize> Default for InlineCellValue<DIMENSION> {
    fn default() -> Self {
        Self([[InlineCellValueWallType::UNVISITED; 2]; DIMENSION])
    }
}