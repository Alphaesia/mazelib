use crate::interface::cell::CellValue;
use crate::interface::buffer::{MazeBuffer, BufferLocation};
use std::fmt::{Debug, Formatter};

/// A [MazeBuffer] that stores its cells in a fixed-size array.
///
/// `MAX_CELLS` is the maximum capacity of this array.
///
///
/// You should probably use [VecBuffer][super::VecBuffer] instead.
pub struct ArrayBuffer<CellVal: CellValue, const MAX_CELLS: usize> {
    buf: [CellVal; MAX_CELLS],
}

impl <CellVal: CellValue, const CELLS: usize> MazeBuffer<CellVal> for ArrayBuffer<CellVal, CELLS> {
    fn new(cell_count: usize) -> Self {
        assert_eq!(cell_count, CELLS, "Maze requires a different amount of cells than specified");

        Self { buf: [CellVal::default(); CELLS] }
    }

    fn get(&self, loc: BufferLocation) -> CellVal {
        self.buf[loc.0]
    }

    fn set(&mut self, loc: BufferLocation, new_value: CellVal) {
        self.buf[loc.0] = new_value
    }
}

impl <CellVal: CellValue, const CELLS: usize> Debug for ArrayBuffer<CellVal, CELLS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArrayBuffer(size = {})", CELLS)
    }
}