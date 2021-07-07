use crate::geometry::space::{BoxCoordinateSpace, CoordinateSpace};
use crate::cell;
use crate::geometry::node::CoordinateTuplet;
use crate::cell::manager::BoxyCellSpace;

pub trait Template<CellSpace: cell::manager::CellManager> {
    fn apply(maze: &mut CellSpace);
}

pub struct SolidBorderTemplate {}

impl <CellSpace: cell::manager::CellManager> Template<CellSpace> for SolidBorderTemplate where
        <CellSpace as cell::manager::CellManager>::CoordSpace: BoxCoordinateSpace<2>,
        CellSpace: BoxyCellSpace<<CellSpace as cell::manager::CellManager>::CoordSpace, 2>,
        <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType: CoordinateTuplet<2>,
        <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType: Into<[usize; 2]>,
        <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType: From<[usize; 2]> {
    fn apply(maze: &mut CellSpace) {
        let dimensions = maze.space().dimensions();

        let top_left: <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType = [0, 0].into();
        let top_right: <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType = [dimensions[0] * CellSpace::scale() - 1, 0].into();
        let bottom_left: <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType = [0, dimensions[1] * CellSpace::scale() - 1].into();
        let bottom_right: <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType = [dimensions[0] * CellSpace::scale() - 1, dimensions[1] * CellSpace::scale() - 1].into();

        maze.make_unaligned_extended_boundary_between(top_left, top_right);
        maze.make_unaligned_extended_boundary_between(bottom_left, bottom_right);
        maze.make_unaligned_extended_boundary_between(top_left, bottom_left);
        maze.make_unaligned_extended_boundary_between(top_right, bottom_right);
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