use rand::Rng;
use rand::seq::SliceRandom;

use crate::implm::generate::util::carve_to_unvisited_neighbour;
use crate::interface::cell::CellValue;
use crate::interface::generate::MazeGenerator;
use crate::interface::maze::Maze;
use crate::interface::point::CoordinateSpace;

pub struct HuntAndKillGenerator {
    _private: ()
}

impl <M: Maze> MazeGenerator<M> for HuntAndKillGenerator {
    fn generate_with_rng<R: Rng + ?Sized>(&mut self, maze: &mut M, rng: &mut R) {
        'hunt: for pt in maze.coord_space().into_iter() {
            // Look for an unvisited point

            if maze.get(pt).is_fully_visited() == false {
                // Connect the new kill path to a previous path
                {
                    // Get neighbouring visited points

                    let mut neighbours = maze.coord_space().neighbours_of_pt(pt).to_vec();

                    neighbours.retain(|&neighbour| maze.get(neighbour).is_fully_visited());

                    if neighbours.is_empty() == false {
                        // unwrap() is safe by virtue of the algorithm
                        let selected_pt = *neighbours.choose(rng).unwrap();  // Already checked length

                        // The path is going from there to our hunt end position
                        maze.make_passage_between(selected_pt, pt);
                    }
                }

                #[allow(unused_labels)]  // So we can label the kill phase
                'kill: {
                    let mut current_pt = pt;

                    loop {
                        match carve_to_unvisited_neighbour(maze, rng, current_pt) {
                            Some(pt) => current_pt = pt,
                            None => continue 'hunt  // If we're in a dead end, revert to the hunt phase
                        }
                    }
                }
            }
        }
    }
}

impl HuntAndKillGenerator {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for HuntAndKillGenerator {
    fn default() -> Self {
        Self::new()
    }
}