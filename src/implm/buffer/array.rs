use std::fmt::{Debug, Formatter};
use std::num::NonZeroUsize;

use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::{CellID, CellValue};

/// A [buffer][MazeBuffer] that stores its cells in a fixed-size array.
///
/// `MAX_CELLS` is the capacity of the backing array.
///
/// You should probably use [`VecBuffer`][crate::implm::buffer::VecBuffer] instead,
/// which is a more flexible and ergonomic alternative. `ArrayBuffer` is only warranted
/// in specific niche scenarios. It primarily exists to provide a second, contrasting
/// implementation of `MazeBuffer`.
pub struct ArrayBuffer<CellVal: CellValue, const MAX_CELLS: usize> {
    size: NonZeroUsize,
    buf: [CellVal; MAX_CELLS],
}

impl <CellVal: CellValue, const CELLS: usize> MazeBuffer<CellVal> for ArrayBuffer<CellVal, CELLS> {
    fn new(cell_count: NonZeroUsize) -> Self {
        assert!(usize::from(cell_count) <= CELLS, "cell_count is greater than the buffer's capacity");

        Self { size: cell_count, buf: [CellVal::default(); CELLS] }
    }

    fn get(&self, cell: CellID) -> CellVal {
        if cell.0 >= usize::from(self.size) {
            panic!("Cell is out of bounds");
        }

        return self.buf[cell.0];
    }

    fn get_mut(&mut self, cell: CellID) -> &mut CellVal {
        if cell.0 >= usize::from(self.size) {
            panic!("Cell is out of bounds");
        }

        return &mut self.buf[cell.0];
    }

    fn set(&mut self, cell: CellID, new_value: CellVal) {
        if cell.0 >= usize::from(self.size) {
            panic!("Cell is out of bounds");
        }

        return self.buf[cell.0] = new_value;
    }
}

impl <CellVal: CellValue, const CELLS: usize> Debug for ArrayBuffer<CellVal, CELLS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArrayBuffer(size = {})", CELLS)
    }
}