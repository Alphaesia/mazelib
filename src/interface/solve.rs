use crate::interface::cell::CellManager;
use crate::interface::maze::Maze;
use crate::interface::point::CoordinateSpace;
use crate::PointPath;

pub trait MazeSolver<M: Maze> {
    fn pathfind(&mut self, maze: &M, start: <<M as CellManager>::CoordSpace as CoordinateSpace>::PtType, end: <<M as CellManager>::CoordSpace as CoordinateSpace>::PtType) -> Option<PointPath<M::CoordSpace>>;
}