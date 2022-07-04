use crate::interface::buffer::MazeBuffer;
use crate::interface::template::Template;
use crate::interface::cell::CellManager;
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::interface::point::CoordinateSpace;
use crate::implm::cell::block::{BlockCellValue, BlockCellValueType};
use crate::implm::cell::inline::{BoxSpaceInlineCellManager, InlineCellValue, InlineCellValueWallType};
use crate::implm::maze::block::BoxSpaceBlockCellMaze;

pub struct SolidBorderTemplate {}

// TODO specialise one day because this is hideously inefficient
impl <Maze: CellManager<CoordSpace=BoxCoordinateSpace<DIMENSION>>, const DIMENSION: usize> Template<Maze> for SolidBorderTemplate {
    default fn apply(maze: &mut Maze) {
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
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> Template<BoxSpaceBlockCellMaze<Buffer, DIMENSION>> for SolidBorderTemplate {
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

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> Template<BoxSpaceInlineCellManager<Buffer, DIMENSION>> for SolidBorderTemplate {
    fn apply(maze: &mut BoxSpaceInlineCellManager<Buffer, DIMENSION>) {
        // TODO this can probably be optimised even further
        //  and rewritten to not be ugly

        // TODO preserve existing walls on the edge (dont overwrite them)
        let mut cell = [0usize; DIMENSION];

        'outer: loop {
            let mut walls = [[InlineCellValueWallType::UNVISITED; 2]; DIMENSION];
            let mut on_boundary = false;

            for (i, dim) in cell.iter().enumerate() {
                if dim == &0 {
                    walls[i][0] = InlineCellValueWallType::BOUNDARY;
                    on_boundary = true;
                }

                if *dim == maze.coord_space().dimensions()[i] - 1 {
                    walls[i][1] = InlineCellValueWallType::BOUNDARY;
                    on_boundary = true;
                }
            }

            if on_boundary {
                maze.set(cell.into(), InlineCellValue { walls, marked: false });
            }

            for i in 0..DIMENSION {
                cell[i] += 1;

                if cell[i] != maze.coord_space().dimensions()[i] {
                    continue 'outer
                } else {
                    cell[i] = 0;
                }
            }

            break 'outer
        }
    }
}