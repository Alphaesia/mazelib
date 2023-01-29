use std::fmt::{Debug, Formatter};

use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::{CellID, CellValue};

/// A [buffer][MazeBuffer] that stores its cells in a fixed-size array.
///
/// `MAX_CELLS` is the capacity of the backing array.
///
/// You should probably use [`VecBuffer`][crate::implm::buffer::VecBuffer] instead.
pub struct ArrayBuffer<CellVal: CellValue, const MAX_CELLS: usize> {
    buf: [CellVal; MAX_CELLS],
}

impl <CellVal: CellValue, const CELLS: usize> MazeBuffer<CellVal> for ArrayBuffer<CellVal, CELLS> {
    fn new(cell_count: usize) -> Self {
        assert_eq!(cell_count, CELLS, "Maze requires a different amount of cells than specified");

        Self { buf: [CellVal::default(); CELLS] }
    }

    fn get(&self, cell: CellID) -> CellVal {
        self.buf[cell.0]
    }

    fn get_mut(&mut self, cell: CellID) -> &mut CellVal {
        &mut self.buf[cell.0]
    }

    fn set(&mut self, cell: CellID, new_value: CellVal) {
        self.buf[cell.0] = new_value
    }
}

impl <CellVal: CellValue, const CELLS: usize> Debug for ArrayBuffer<CellVal, CELLS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArrayBuffer(size = {})", CELLS)
    }
}