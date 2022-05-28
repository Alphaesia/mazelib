use crate::interface::cell::CellValue;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BlockCellValue {
    pub cell_type: BlockCellValueType,
    pub marked: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BlockCellValueType {
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

impl Default for BlockCellValue {
    fn default() -> Self { Self { cell_type: BlockCellValueType::UNVISITED, marked: false } }
}