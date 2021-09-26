use crate::interface::cell::{CellValue, CellValueType};

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
    fn get_type(&self) -> CellValueType {
        match self {
            Self::PASSAGE => CellValueType::PASSAGE,
            Self::WALL => CellValueType::WALL,
            Self::BOUNDARY => CellValueType::BOUNDARY,
            Self::UNVISITED => CellValueType::UNVISITED,
        }
    }
}

impl Default for BlockCellValue {
    fn default() -> Self { Self::UNVISITED }
}