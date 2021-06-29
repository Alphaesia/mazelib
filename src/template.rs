use crate::geometry::space::{BoxCoordinateSpace, CoordinateSpace};
use crate::{cell, maze};
use crate::maze::Maze;
use crate::geometry::node::CoordinateTuplet;
use crate::cell::space::BoxyCellSpace;

pub trait Template<Maze: maze::Maze<CellSpace>, CellSpace: cell::space::CellSpace<Maze>> {
    fn apply(maze: &mut Maze);
}

pub struct SolidBorderTemplate {}

impl <Maze, CoordSpace, CellSpace> Template<Maze, CellSpace> for SolidBorderTemplate where
        Maze: maze::Maze<CellSpace>,
        CoordSpace: BoxCoordinateSpace<2>,
        CellSpace: BoxyCellSpace<Maze, CoordSpace, 2, CoordSpace=CoordSpace>,
        <CoordSpace as CoordinateSpace>::PtType: CoordinateTuplet<2>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; 2]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; 2]> {
    fn apply(maze: &mut Maze) {
        let dimensions = maze.space().dimensions();

        let top_left: <CoordSpace as CoordinateSpace>::PtType = [0, 0].into();
        let top_right: <CoordSpace as CoordinateSpace>::PtType = [dimensions[0] * CellSpace::scale(), 0].into();
        let bottom_left: <CoordSpace as CoordinateSpace>::PtType = [0, dimensions[1] * CellSpace::scale()].into();
        let bottom_right: <CoordSpace as CoordinateSpace>::PtType = [dimensions[0] * CellSpace::scale(), dimensions[1] * CellSpace::scale()].into();

        CellSpace::make_unaligned_extended_boundary_between(maze, top_left, top_right);
        CellSpace::make_unaligned_extended_boundary_between(maze, bottom_left, bottom_right);
        CellSpace::make_unaligned_extended_boundary_between(maze, top_left, bottom_left);
        CellSpace::make_unaligned_extended_boundary_between(maze, top_right, bottom_right);
    }
}

// TODO figure out a general impl
// impl <CellSpace, const DIMENSION: usize> Template<CellSpace> for SolidBorderTemplate<CellSpace::CoordSpace, DIMENSION> where
//         CellSpace: cell::space::CellSpace,
//         CellSpace::CoordSpace: BoxCoordinateSpace<{ DIMENSION }> {
//     fn apply(maze: &mut Maze<CellSpace>) {
//         let vertices = [[0usize; DIMENSION]; 2 ** DIMENSION];
//
//         for pt in maze.space() {
//             let pt = unsafe { *coerce!(<<CellSpace as cell::space::CellSpace>::CoordSpace as IntoIterator>::Item, <<CellSpace as cell::space::CellSpace>::CoordSpace as CoordinateSpace>::PtType, &pt) };
//
//             debug_maze(maze, pt);
//
//             if maze.space().is_adjacent_to_edge(pt) {
//                 CellSpace::make_boundary(maze, pt);
//             }
//         }
//     }
// }