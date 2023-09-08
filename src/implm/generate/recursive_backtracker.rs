use embed_doc_image::embed_doc_image;
use rand::Rng;

use crate::implm::generate::util::carve_to_unvisited_neighbour;
use crate::interface::generate::MazeGenerator;
use crate::interface::coordinator::MazeCoordinator;
use crate::interface::point::CoordinateSpace;

/// The **Recursive Backtracker** algorithm is a variant of depth-first search which selects
/// the next child at random.
///
/// It produces mazes with a high river factor, without any noticeable visual patterns or
/// artifacts.
///
/// This algorithm is fast. However, it maintains a stack[^name] of the current path, which
/// requires a significant amount of memory (proportional to the size of the maze). In the
/// worst-case, the length of the stack will be equal to the logical size of the coordinate
/// space.
///
/// The [*Hunt-and-Kill* algorithm][crate::implm::generate::HuntAndKillGenerator] produces
/// mazes with a very similar texture to Recursive Backtracker. Given that Hunt-and-Kill
/// requires no memory beyond the maze itself and has comparable speed, there is no reason
/// to favour Recursive Backtracker over Hunt-and-Kill.
///
/// # Examples
///
/// ![A typical output of Recursive Backtracker.][example]
///
/// As you can see, Hunt-and-Kill typically produces mazes with passages that run for a long way
/// before dead-ending. They also tend to be quite twisty. It still features a number of cul-de-sacs
/// though. Also note how this description is exactly the same for
/// [Hunt-and-Kill](crate::implm::generate::HuntAndKillGenerator#example).
///
/// [^name]: While the name implies a recursive implementation, it is much more efficient to
///          implement it imperatively. A recursive implementation will likely overflow the
///          callstack on even medium-sized mazes (ignoring tail-call optimisation).
#[embed_doc_image("example", "src/doc/img/generator/recursive-backtracker/example.png")]
pub struct RecursiveBacktrackerGenerator {
    _private: ()
}

impl <M: MazeCoordinator> MazeGenerator<M> for RecursiveBacktrackerGenerator {
    fn generate_with_rng(&mut self, maze: &mut M, rng: &mut (impl Rng + ?Sized)) {
        // Start at a random point
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

impl Default for RecursiveBacktrackerGenerator {
    fn default() -> Self {
        Self::new()
    }
}