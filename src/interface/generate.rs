use rand::RngCore;
use crate::interface::cell::CellManager;

pub trait MazeGenerator<CellSpace: CellManager> {
    fn generate(&mut self, maze: &mut CellSpace, rng: &mut dyn RngCore);
}