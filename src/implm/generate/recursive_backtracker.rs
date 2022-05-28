use rand::Rng;
use crate::implm::generate::util::carve_to_unvisited_neighbour;
use crate::interface::cell::CellManager;
use crate::interface::generate::MazeGenerator;
use crate::interface::point::CoordinateSpace;

pub struct RecursiveBacktrackerGenerator {
    _private: ()
}

impl <Maze: CellManager> MazeGenerator<Maze> for RecursiveBacktrackerGenerator {
    fn generate_with_rng<R: Rng + ?Sized>(&mut self, maze: &mut Maze, rng: &mut R) {
        // Start at the origin
        let mut current_pt = maze.coord_space().choose(rng);

        maze.make_passage(current_pt);

        let mut stack = vec![current_pt];

        while !stack.is_empty() {
            match carve_to_unvisited_neighbour(maze, rng, current_pt) {
                Some(pt) => {
                    stack.push(pt);
                    current_pt = pt;
                }
                None => current_pt = stack.pop().unwrap()
            }
        }
    }
}

impl RecursiveBacktrackerGenerator {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for RecursiveBacktrackerGenerator {
    fn default() -> Self {
        Self::new()
    }
}