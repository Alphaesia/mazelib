use crate::implm::point::boxy::CoordinateTuplet;
use std::convert::TryInto;

impl <const DIMENSION: usize> From<[usize; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(coordinates: [usize; DIMENSION]) -> Self {
        CoordinateTuplet { coords: coordinates }
    }
}

impl <const DIMENSION: usize> From<[isize; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [isize; DIMENSION]) -> Self {
        CoordinateTuplet { coords: pt.map(|coord| coord.try_into().expect("coordinate was negative")) }
    }
}

impl <const DIMENSION: usize> From<[i32; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [i32; DIMENSION]) -> Self {
        CoordinateTuplet { coords: pt.map(|coord| coord.try_into().expect("coordinate was negative")) }
    }
}

impl <const DIMENSION: usize> From<[u32; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [u32; DIMENSION]) -> Self {
        CoordinateTuplet { coords: pt.map(|coord| coord.try_into().expect("coordinate was negative")) }
    }
}

impl <const DIMENSION: usize> From<CoordinateTuplet<DIMENSION>> for [usize; DIMENSION] {
    fn from(pt: CoordinateTuplet<DIMENSION>) -> Self {
        pt.coords
    }
}