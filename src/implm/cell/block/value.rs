use crate::interface::cell::CellValue;

/// A cell type where cells are either passage cells or wall
/// wall cells, with no in between. They are called Block Cells
/// because the resulting mazes look blocky / pixellated.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BlockCellValue {
    /// The specific type or value of the cell. For more information
    /// see [`BlockCellValueType`].
    pub cell_type: BlockCellValueType,

    /// Whether this cell has been marked or flagged. This is a
    /// general-use field, with no specific meaning.
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
    #[must_use]
    fn default() -> Self { Self { cell_type: BlockCellValueType::UNVISITED, marked: false } }
}