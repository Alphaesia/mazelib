use embed_doc_image::embed_doc_image;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::implm::generate::util::carve_to_unvisited_neighbour;
use crate::interface::cell::CellValue;
use crate::interface::coordinator::MazeCoordinator;
use crate::interface::generate::MazeGenerator;
use crate::interface::point::CoordinateSpace;

/// The **Hunt-and-Kill** algorithm links random walks together to generate a maze.
///
/// It produces mazes with a high river factor, without any noticeable visual patterns or artifacts.
///
/// It runs relatively quickly, visiting every cell exactly twice, and uses no extra memory.
///
/// Hunt-and-Kill is a solid all-round algorithm, and the suggested default algorithm for users.
///
/// It was invented by [Walter D. Pullen](https://www.astrolog.org/home.htm).
///
/// # Examples
///
/// ![A typical output of Hunt-and-Kill.][example]
///
/// As you can see, Hunt-and-Kill typically produces mazes with passages that run for a long way
/// before dead-ending. They also tend to be quite twisty. It still features a number of cul-de-sacs
/// though.
#[embed_doc_image("example", "src/doc/img/generate/hunt-and-kill/example.png")]
pub struct HuntAndKillGenerator {
    _private: ()
}

impl <M: MazeCoordinator> MazeGenerator<M> for HuntAndKillGenerator {
    fn generate_with_rng(&mut self, maze: &mut M, rng: &mut (impl Rng + ?Sized)) {
        'hunt: for pt in maze.coord_space().iter() {
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
    /// Construct a new generator instance.
    /// 
    /// This doesn't take any parameters, so if you're just immediately going to call
    /// [`generate()`][crate::interface::generate::MazeGenerator::generate], you may wish to use
    /// [`DefaultMazeGenerator::generate()`][crate::interface::generate::DefaultMazeGenerator::generate]
    /// instead.
    ///
    /// Equivalent to [`Self::default()`].
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for HuntAndKillGenerator {
    fn default() -> Self {
        Self::new()
    }
}