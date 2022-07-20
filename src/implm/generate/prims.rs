use rand::Rng;
use crate::implm::generate::util::{carve_from_visited_neighbour, get_unvisited_neighbours};
use crate::interface::cell::{CellManager, CellValue};
use crate::interface::generate::MazeGenerator;
use crate::interface::point::CoordinateSpace;

pub struct PrimsGenerator {
    _private: ()
}

impl <Maze: CellManager> MazeGenerator<Maze> for PrimsGenerator {
    fn generate_with_rng<R: Rng + ?Sized>(&mut self, maze: &mut Maze, rng: &mut R) {
        let mut frontier = Vec::new();

        let gen_start = maze.coord_space().choose(rng);

        maze.make_passage(gen_start);

        frontier.append(&mut get_unvisited_neighbours::<Maze, R>(maze, gen_start));

        while frontier.is_empty() == false {
            let pt = {
                let index = rng.gen_range(0..frontier.len());

                // Don't care about the ordering of frontier and is faster than ::remove()
                frontier.swap_remove(index)
            };

            // No longer apart of the frontier so doesn't need to remain marked.
            // We clean up marks as we go.
            maze.get(pt).set_marked(false);

            carve_from_visited_neighbour(maze, rng, pt);

            for neighbour_loc in get_unvisited_neighbours::<Maze, R>(maze, pt) {
                let neighbour_value = maze.get_mut(pt);

                if neighbour_value.is_marked() == false {
                    frontier.push(neighbour_loc);
                    neighbour_value.set_marked(true)
                }
            }
        }
    }
}

impl PrimsGenerator {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for PrimsGenerator {
    fn default() -> Self {
        Self::new()
    }
}