use crate::implm::point::boxy::CoordinateTuplet;
use std::convert::TryInto;
use std::ops::{Index, IndexMut};
use std::fmt::{Debug, Formatter};
use crate::interface::cell::CellLocation;

/// TODO rewrite
///
/// MappedPoint is just a regular CoordinateTuplet.
/// To avoid accidentally passing a non-scaled point into a function
/// expecting a scaled point, and vice versa, we leverage nominal typing
/// to create this new-but-effectively-identical type.
///
/// At compile time, Rust should optimise out this wrapper struct, so
/// there will be no performance penalty.
///
/// *See also: [BlockCellValue][super::value::BlockCellValue]*
#[derive(Copy, Clone)]
pub struct BlockCell<const DIMENSION: usize>(pub(super) CoordinateTuplet<DIMENSION>);

impl <const DIMENSION: usize> CellLocation for BlockCell<DIMENSION> {}

impl <const DIMENSION: usize> BlockCell<DIMENSION> {
    /// Get the point at a given offset from this point
    /// (dimension refers to the direction of the offset - e.g. x-direction is dimension 0).
    pub fn offset(&self, axis: usize, offset: isize) -> Self {
        let mut new = *self;

        if offset >= 0 {
            new[axis] += TryInto::<usize>::try_into(offset).unwrap();
        } else {
            new[axis] -= TryInto::<usize>::try_into(offset.abs()).unwrap();
        }

        return new
    }

    /// Replace the position along the given axis with a new value.
    /// Useful when iterating along an axis.
    /// Suggestions for better names are welcome.
    pub fn at(&self, axis: usize, position: usize) -> Self {
        let mut new: [usize; DIMENSION] = self.0.into();

        new[axis] = position;

        return Self(new.into());
    }
}

impl <const DIMENSION: usize> Index<usize> for BlockCell<DIMENSION> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl <const DIMENSION: usize> IndexMut<usize> for BlockCell<DIMENSION> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl <const DIMENSION: usize> Debug for BlockCell<DIMENSION> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "s{:?}", self.0)
    }
}

impl <const DIMENSION: usize> From<[usize; DIMENSION]> for BlockCell<DIMENSION> {
    fn from(coordinates: [usize; DIMENSION]) -> Self {
        BlockCell(coordinates.into())
    }
}

impl <const DIMENSION: usize> From<[isize; DIMENSION]> for BlockCell<DIMENSION> {
    fn from(pt: [isize; DIMENSION]) -> Self {
        BlockCell(pt.into())
    }
}

impl <const DIMENSION: usize> From<[i32; DIMENSION]> for BlockCell<DIMENSION> {
    fn from(pt: [i32; DIMENSION]) -> Self {
        BlockCell(pt.into())
    }
}

impl <const DIMENSION: usize> From<BlockCell<DIMENSION>> for [usize; DIMENSION] {
    fn from(pt: BlockCell<DIMENSION>) -> Self {
        pt.0.into()
    }
}
