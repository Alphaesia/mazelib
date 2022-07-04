use std::ops::{Index, IndexMut};

use crate::implm::point::boxy::CoordinateTuplet;

impl <const DIMENSION: usize> Index<usize> for CoordinateTuplet<DIMENSION> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coords[index]
    }
}

impl <const DIMENSION: usize> IndexMut<usize> for CoordinateTuplet<DIMENSION> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coords[index]
    }
}