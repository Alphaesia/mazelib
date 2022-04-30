use crate::interface::generate::MazeGenerator;
use crate::interface::cell::CellManager;
use rand::Rng;
use rand::seq::IteratorRandom;
use crate::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};

/// A BinaryTreeGenerator, but instead of an arity of 2 it has an arity of `DIMENSION`.
/// Only works with [BoxCoordinateSpace][crate::implm::point::boxy::BoxCoordinateSpace]s.
pub struct NAryTreeGenerator {
    _private: ()
}

impl NAryTreeGenerator {
    pub fn new() -> Self
    {
        Self { _private: () }
    }

    /// Sugar for `NAryTreeGenerator::new().generate(maze)`
    pub fn generate<Maze: CellManager<CoordSpace=BoxCoordinateSpace<DIMENSION>>, const DIMENSION: usize>(maze: &mut Maze) {
        Self::new().generate(maze)
    }
}

impl <Maze: CellManager<CoordSpace=BoxCoordinateSpace<DIMENSION>>, const DIMENSION: usize> MazeGenerator<Maze> for NAryTreeGenerator {
    fn generate_with_rng<R: Rng + ?Sized>(&mut self, maze: &mut Maze, rng: &mut R) {
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
            }).map(|candidate| Into::<CoordinateTuplet<DIMENSION>>::into(candidate))
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