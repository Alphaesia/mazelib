use crate::cell;
use crate::buffer::MazeBuffer;
use std::fmt::Debug;

pub trait Maze<CellSpace: cell::space::CellSpace<Self> + ?Sized>: Send + Sync + Debug {
    fn space(&self) -> CellSpace::CoordSpace;

    fn buffer(&self) -> &dyn MazeBuffer<Self, CellSpace>;

    fn mut_buffer<'a>(&'a mut self) -> &'a mut dyn MazeBuffer<Self, CellSpace>;
}