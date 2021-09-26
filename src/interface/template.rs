use crate::interface::cell::CellManager;

pub trait Template<Maze: CellManager> {
    fn apply(maze: &mut Maze);
}