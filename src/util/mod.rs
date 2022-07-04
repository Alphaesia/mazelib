mod solid_border;

pub use self::solid_border::solid_border;

use crate::interface::cell::CellValue;
use crate::interface::maze::Maze;

pub fn convert_unvisited_to_walls<M: Maze>(maze: &mut M) {
    for pt in maze.coord_space().into_iter() {
        if maze.get(pt).is_fully_visited() == false {
            maze.make_wall(pt)
        }
    }
}