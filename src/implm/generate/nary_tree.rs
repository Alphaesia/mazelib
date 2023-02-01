use embed_doc_image::embed_doc_image;
use rand::Rng;
use rand::seq::IteratorRandom;

use crate::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};
use crate::interface::generate::MazeGenerator;
use crate::interface::maze::Maze;

/// The ***n*-ary Tree** algorithm is a generalisation of the *Binary Tree* algorithm
/// to higher dimensions.
///
/// Starting from the origin, then iterating sequentially, it connects every cell to
/// a random unvisited neighbour. In [box coordinate spaces][BoxCoordinateSpace]
/// this will produce a tree-like structure, hence the name. In fact, *n* is the
/// dimension of the coordinate space. Due to the fact that randomly selecting
/// neighbours is not guaranteed to produce a tree-like structure on all coordinate
/// spaces, *n*-ary Tree is only implemented on `BoxCoordinateSpace`.
///
/// *n*-ary Tree produces mazes with an exceptionally strong diagonal bias. It also
/// always produces straight passages along axes that intersect the corner opposite
/// the origin (see the example below).
///
/// This trade-off however allows the algorithm to be exceptionally fast --- the
/// fastest among all the algorithms --- and use no extra memory.
///
/// Using this algorithm will require careful consideration of the diagonal bias,
/// as it is extremely easy to traverse the maze along the grain.
///
/// # Example
///
/// ![A typical output of n-ary Tree.][example]
///
/// Notice the long passageways on the bottom and right edges, and how all other passages
/// head towards that bottom-right corner. Passages will never curl in on themselves. Once
/// a passage goes right, it will never go left again (ignoring intersections). Likewise
/// for down/up.
#[embed_doc_image("example", "src/doc/img/generator/nary-tree/example.png")]
pub struct NAryTreeGenerator {
    _private: ()
}

impl <M: Maze<CoordSpace=BoxCoordinateSpace<DIMENSION>>, const DIMENSION: usize> MazeGenerator<M> for NAryTreeGenerator {
    fn generate_with_rng(&mut self, maze: &mut M, rng: &mut (impl Rng + ?Sized)) {
        // For every point in the maze,
        maze.coord_space().into_iter().for_each(|pt| {
            let pt_as_arr: [usize; DIMENSION] = pt.into();

            // find every (valid) neighbour...
            let selected = (0..DIMENSION).filter_map(|dim| {
                let mut candidate = pt_as_arr;

                candidate[dim] += 1;

                // ...that is inside the coordinate space...
                return if candidate[dim] < usize::from(maze.coord_space().dimensions()[dim]) {
                    Some(candidate)
                } else {
                    None
                };
            }).map(Into::<CoordinateTuplet<DIMENSION>>::into)
              // ...and pick one at random,
              .choose(rng);

            // ...then make a passage to it.
            match selected {
                Some(winner) => maze.make_passage_between(pt, winner),
                None => maze.make_passage(pt)
            }
        })
    }
}

impl NAryTreeGenerator {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for NAryTreeGenerator {
    fn default() -> Self {
        Self::new()
    }
}