use std::fmt::{Debug, Formatter};

use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::{CellID, CellValue};

/// A [buffer][MazeBuffer] that stores its cells in a [`Vec`] allocated on the heap.
///
/// This is the buffer that you will almost always want to use.
pub struct VecBuffer<CellVal: CellValue> {
    buf: Vec<CellVal>,
}

impl <CellVal: CellValue> MazeBuffer<CellVal> for VecBuffer<CellVal> {
    fn new(cell_count: usize) -> Self {
        Self { buf: vec![CellVal::default(); cell_count] }
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

impl <CellVal: CellValue> Debug for VecBuffer<CellVal> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "VecBuffer(size = {})", self.buf.capacity())
    }
}