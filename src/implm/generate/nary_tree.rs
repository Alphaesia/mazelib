use rand::Rng;
use rand::seq::IteratorRandom;
use crate::interface::generate::MazeGenerator;
use crate::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};
use crate::interface::maze::Maze;

/// A BinaryTreeGenerator, but instead of an arity of 2 it has an arity of `DIMENSION`.
/// Only works with [BoxCoordinateSpace][crate::implm::point::boxy::BoxCoordinateSpace]s.
pub struct NAryTreeGenerator {
    _private: ()
}

impl <M: Maze<CoordSpace=BoxCoordinateSpace<DIMENSION>>, const DIMENSION: usize> MazeGenerator<M> for NAryTreeGenerator {
    fn generate_with_rng<R: Rng + ?Sized>(&mut self, maze: &mut M, rng: &mut R) {
        // For every point in the maze,
        maze.coord_space().into_iter().for_each(|pt| {
            let pt_as_arr: [usize; DIMENSION] = pt.into();

            // find every (valid) neighbour...
            let selected = (0..DIMENSION).filter_map(|dim| {
                let mut candidate = pt_as_arr;

                candidate[dim] += 1;

                // ...that is inside the coordinate space...
                return if candidate[dim] < maze.coord_space().dimensions()[dim] {
                    Some(candidate)
                } else {
                    None
                };
            }).map(Into::<CoordinateTuplet<DIMENSION>>::into)
              // and pick one at random,
              .choose(rng);

            // then make a passage to it.
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