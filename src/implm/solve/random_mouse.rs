use rand::{Rng, thread_rng};
use rand::seq::IteratorRandom;
use crate::interface::cell::CellManager;
use crate::interface::maze::Maze;
use crate::interface::point::CoordinateSpace;
use crate::interface::solve::MazeSolver;
use crate::PointPath;

pub struct RandomMouse {
    _private: (),
}

impl RandomMouse {
    pub fn new() -> Self {
        Self { _private: () }
    }

    fn pathfind_with_rng<M: Maze, R: Rng + ?Sized>(&self, maze: &M, start: <<M as CellManager>::CoordSpace as CoordinateSpace>::PtType, end: <<M as CellManager>::CoordSpace as CoordinateSpace>::PtType, rng: &mut R) -> Option<PointPath<M::CoordSpace>> {
        let mut path: PointPath<M::CoordSpace> = PointPath::<M::CoordSpace>::starting_at(start);

        let mut pos = start;
        let mut previous_pos = start;

        while pos != end {
            (pos, previous_pos) = (match maze.coord_space().neighbours_of_pt(pos).iter()
                        .filter(|neighbour| **neighbour != previous_pos)
                        .filter(|neighbour| maze.is_passage_between(pos, **neighbour))
                        .choose(rng) {
                Some(pt) => *pt,
                None => previous_pos,
            }, pos);

            path.push(pos);
        }

        return Some(path);
    }
}

impl <M: Maze> MazeSolver<M> for RandomMouse {
    fn pathfind(&mut self, maze: &M, start: <<M as CellManager>::CoordSpace as CoordinateSpace>::PtType, end: <<M as CellManager>::CoordSpace as CoordinateSpace>::PtType) -> Option<PointPath<M::CoordSpace>> {
        Self::pathfind_with_rng(self, maze, start, end, &mut thread_rng())
    }
}