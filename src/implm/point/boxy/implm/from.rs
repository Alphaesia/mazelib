use std::convert::TryInto;

use crate::implm::point::boxy::CoordinateTuplet;

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

impl <const DIMENSION: usize> From<[u32; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [u32; DIMENSION]) -> Self {
        CoordinateTuplet { coords: pt.map(|coord| coord.try_into().expect("coordinate was negative")) }
    }
}

impl <const DIMENSION: usize> From<[i32; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [i32; DIMENSION]) -> Self {
        CoordinateTuplet { coords: pt.map(|coord| coord.try_into().expect("coordinate was negative")) }
    }
}

impl <const DIMENSION: usize> From<[u16; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [u16; DIMENSION]) -> Self {
        CoordinateTuplet { coords: pt.map(|coord| coord.try_into().expect("coordinate was negative")) }
    }
}

impl <const DIMENSION: usize> From<[i16; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [i16; DIMENSION]) -> Self {
        CoordinateTuplet { coords: pt.map(|coord| coord.try_into().expect("coordinate was negative")) }
    }
}

impl <const DIMENSION: usize> From<[u8; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [u8; DIMENSION]) -> Self {
        CoordinateTuplet { coords: pt.map(|coord| coord.try_into().expect("coordinate was negative")) }
    }
}

impl <const DIMENSION: usize> From<[i8; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [i8; DIMENSION]) -> Self {
        CoordinateTuplet { coords: pt.map(|coord| coord.try_into().expect("coordinate was negative")) }
    }
}

impl <const DIMENSION: usize> From<CoordinateTuplet<DIMENSION>> for [usize; DIMENSION] {
    fn from(pt: CoordinateTuplet<DIMENSION>) -> Self {
        pt.coords
    }
}