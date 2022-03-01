use rand::RngCore;
use rand::seq::SliceRandom;
use crate::interface::cell::{CellManager, CellValue};
use crate::interface::point::CoordinateSpace;

pub fn carve_to_unvisited_neighbour<Maze: CellManager>(maze: &mut Maze, rng: &mut dyn RngCore, from_pt: <<Maze as CellManager>::CoordSpace as CoordinateSpace>::PtType) -> Option<<<Maze as CellManager>::CoordSpace as CoordinateSpace>::PtType>
{
    // Get unvisited neighbours
    let mut neighbours = maze.coord_space().neighbours_of_pt(from_pt).to_vec();
    neighbours.retain(|&neighbour| maze.get(neighbour).is_fully_visited() == false);

    // Pick a random unvisited neighbouring point
    let next_pt = *neighbours.choose(rng)?;

    // Make a passage from here to there
    maze.make_passage_between(from_pt, next_pt);

    // Yield selected point
    return Some(next_pt);
}