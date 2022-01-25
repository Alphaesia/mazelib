use rand::RngCore;
use rand::seq::SliceRandom;
use crate::interface::cell::{CellManager, CellValue};
use crate::interface::generate::MazeGenerator;
use crate::interface::point::CoordinateSpace;

pub struct HuntAndKillGenerator {
    _private: ()
}

impl HuntAndKillGenerator {
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Sugar for `HuntAndKillGenerator::new().generate(maze, rng)`
    pub fn generate<Maze: CellManager>(maze: &mut Maze, rng: &mut dyn RngCore) {
        Self::new().generate(maze, rng)
    }
}

impl <Maze: CellManager> MazeGenerator<Maze> for HuntAndKillGenerator {
    fn generate(&mut self, maze: &mut Maze, rng: &mut dyn RngCore) {
        'hunt: for pt in maze.coord_space().into_iter() {
            // Look for an unvisited point

            if maze.get(pt).is_unvisited() {
                // Connect the new kill path to a previous path
                {
                    // Get neighbouring visited points

                    let mut neighbours = maze.coord_space().neighbours_of_pt(pt).to_vec();

                    neighbours.retain(|&neighbour| !maze.get(neighbour).is_unvisited());

                    if neighbours.len() > 0 {
                        // unwrap() is safe by virtue of the algorithm
                        let selected_pt = *neighbours.choose(rng).unwrap();  // Already checked length

                        // The path is going from there to our hunt end position
                        maze.make_passage_between(selected_pt, pt);
                    }
                }

                #[allow(unused_labels)]  // So we can label the kill phase
                'kill: {
                    let mut current_pt = pt;

                    // Carve a random path
                    loop {
                        // Get unvisited neighbours

                        let mut neighbours = maze.coord_space().neighbours_of_pt(current_pt).to_vec();

                        neighbours.retain(|&neighbour| maze.get(neighbour).is_unvisited());

                        // If we're in a dead end, revert to the hunt phase
                        if neighbours.len() == 0 {
                            continue 'hunt
                        }

                        // Pick a random unvisited neighbouring point

                        let next_pt = *neighbours.choose(rng).unwrap();  // Already checked length

                        // Make a passage from here to there

                        maze.make_passage_between(current_pt, next_pt);

                        // Move to the selected point

                        current_pt = next_pt;
                    }
                }
            }
        }
    }
}