use std::fmt::{Debug, Display, Formatter};

use crate::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};

impl <const DIMENSION: usize> Debug for CoordinateTuplet<DIMENSION> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for i in 0..(DIMENSION - 1) {
            write!(f, "{}, ", self[i])?;
        }

        write!(f, "{}", self[DIMENSION - 1])?;

        write!(f, ")")
    }
}

impl <const DIMENSION: usize> Display for CoordinateTuplet<DIMENSION> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl <const DIMENSION: usize> Debug for BoxCoordinateSpace<DIMENSION> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BoxCoordinateSpace(dimensions = ")?;

        for i in 0..(DIMENSION - 1) {
            write!(f, "{}x", self[i])?;
        }

        write!(f, "{}", self[DIMENSION - 1])?;

        write!(f, ")")
    }
}