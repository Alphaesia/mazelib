use crate::interface::cell::{CellManager, CellValue};

pub fn convert_unvisited_to_passages<Maze: CellManager>(maze: &mut Maze) {
    for pt in maze.coord_space().into_iter() {
        if maze.get(pt).is_fully_visited() == false {
            maze.make_passage(pt)
        }
    }
}

pub fn convert_unvisited_to_walls<Maze: CellManager>(maze: &mut Maze) {
    for pt in maze.coord_space().into_iter() {
        if maze.get(pt).is_fully_visited() == false {
            maze.make_wall(pt)
        }
    }
}

pub fn convert_unvisited_to_boundaries<Maze: CellManager>(maze: &mut Maze) {
    for pt in maze.coord_space().into_iter() {
        if maze.get(pt).is_fully_visited() == false {
            maze.make_boundary(pt)
        }
    }
}