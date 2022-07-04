use crate::interface::maze::Maze;

pub trait Template<M: Maze> {
    fn apply(maze: &mut M);
}