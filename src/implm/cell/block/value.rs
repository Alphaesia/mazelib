use crate::interface::cell::CellValue;

/// A cell where walls and passages do not overlap,
/// and instead are separate cells.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BlockCellValue {
    UNVISITED,
    BOUNDARY,
    WALL,
    PASSAGE
}

impl CellValue for BlockCellValue {
    fn is_fully_visited(&self) -> bool {
        self != &Self::UNVISITED
    }
}

impl Default for BlockCellValue {
    fn default() -> Self { Self::UNVISITED }
}