use crate::implm::cell::block::{BlockCellValue, BlockCellValueType};
use crate::implm::cell::inline::{BoxSpaceInlineCellManager, InlineCellValue, InlineCellValueEdgeType};
use crate::implm::maze::block::BoxSpaceBlockCellMaze;
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::CellManager;
use crate::interface::point::CoordinateSpace;

/// Convert all *cells* (not points) adjacent to the edge of a box maze into border cells.
pub fn solid_border<M: CellManager<CoordSpace=BoxCoordinateSpace<DIMENSION>>, const DIMENSION: usize>(maze: &mut M) {
    <FixSpecialisationPls as SolidBorder<M>>::apply(maze);
}

// Let's play Twister® because we can't specialise standalone functions

trait SolidBorder<M: CellManager> {
    fn apply(maze: &mut M);
}

struct FixSpecialisationPls {}

// TODO specialise one day because this is hideously inefficient
impl <M: CellManager<CoordSpace=BoxCoordinateSpace<DIMENSION>>, const DIMENSION: usize> SolidBorder<M> for FixSpecialisationPls {
    default fn apply(maze: &mut M) {
        for pt in maze.coord_space() {
            if maze.coord_space().is_adjacent_to_edge(pt) == false {
                continue
            }

            for neighbour in maze.coord_space().neighbours_of_pt(pt) {
                if maze.coord_space().is_adjacent_to_edge(neighbour) {
                    maze.make_boundary_between(pt, neighbour)
                }
            }
        }
    }
}

// We operate on cells directly here so we can ignore any padding
// or scaling effects.
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> SolidBorder<BoxSpaceBlockCellMaze<Buffer, DIMENSION>> for FixSpecialisationPls {
    fn apply(maze: &mut BoxSpaceBlockCellMaze<Buffer, DIMENSION>) {
        // TODO this can probably be optimised even further
        //  and rewritten to not be ugly
        let mut cell = [0usize; DIMENSION];

        'outer: loop {
            for (i, dim) in cell.iter().enumerate() {
                if *dim == 0 || *dim == maze.get_full_dimensions()[i] - 1 {
                    maze.set_cell_value_type(cell.into(), BlockCellValueType::BOUNDARY);
                    break
                }
            }

            for i in 0..DIMENSION {
                cell[i] += 1;

                if cell[i] != maze.get_full_dimensions()[i] {
                    continue 'outer
                } else {
                    cell[i] = 0;
                }
            }

            break 'outer
        }
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> SolidBorder<BoxSpaceInlineCellManager<Buffer, DIMENSION>> for FixSpecialisationPls {
    fn apply(maze: &mut BoxSpaceInlineCellManager<Buffer, DIMENSION>) {
        // TODO this can probably be optimised even further
        //  and rewritten to not be ugly

        // TODO preserve existing walls on the edge (dont overwrite them)
        let mut cell = [0usize; DIMENSION];

        'outer: loop {
            let mut walls = [[InlineCellValueEdgeType::UNVISITED; 2]; DIMENSION];
            let mut on_boundary = false;

            for (i, dim) in cell.iter().enumerate() {
                if dim == &0 {
                    walls[i][0] = InlineCellValueEdgeType::BOUNDARY;
                    on_boundary = true;
                }

                if *dim == usize::from(maze.coord_space().dimensions()[i]) - 1 {
                    walls[i][1] = InlineCellValueEdgeType::BOUNDARY;
                    on_boundary = true;
                }
            }

            if on_boundary {
                maze.set(cell.into(), InlineCellValue { edges: walls, marked: false });
            }

            for i in 0..DIMENSION {
                cell[i] += 1;

                if cell[i] != usize::from(maze.coord_space().dimensions()[i]) {
                    continue 'outer
                } else {
                    cell[i] = 0;
                }
            }

            break 'outer
        }
    }
}