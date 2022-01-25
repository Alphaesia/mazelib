use crate::interface::buffer::MazeBuffer;
use crate::interface::template::Template;
use crate::interface::cell::CellManager;
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::interface::point::CoordinateSpace;
use crate::implm::cell::block::{BoxSpaceBlockCellManager, BlockCellValue};

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
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> Template<BoxSpaceBlockCellManager<Buffer, DIMENSION>> for SolidBorderTemplate {
    fn apply(maze: &mut BoxSpaceBlockCellManager<Buffer, DIMENSION>) {
        // TODO this can probably be optimised even further
        //  and rewritten to not be ugly
        let mut cell = [0usize; DIMENSION];

        'outer: loop {
            for (i, dim) in cell.iter().enumerate() {
                if *dim == 0 || *dim == maze.get_full_dimensions()[i] - 1 {
                    maze.set(cell.into(), BlockCellValue::BOUNDARY);
                    break
                }
            }

            #[allow(clippy::needless_range_loop)]
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