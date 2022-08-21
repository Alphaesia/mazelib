use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use crate::implm::point::boxy::CoordinateTuplet;
use crate::interface::cell::CellLocation;

/// [`CellLocation`] for inline cells.
///
/// *See also: [InlineCellValue][super::InlineCellValue]*
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InlineCellLocation<const DIMENSION: usize>(pub(super) CoordinateTuplet<DIMENSION>);

impl <const DIMENSION: usize> CellLocation for InlineCellLocation<DIMENSION> {}

impl <const DIMENSION: usize> InlineCellLocation<DIMENSION> {
    /// Get the point at a given offset from this point
    /// (dimension refers to the direction of the offset - e.g. x-direction is dimension 0).
    pub fn offset(&self, axis: usize, offset: isize) -> Self {
        Self(self.0.offset(axis, offset))
    }

    /// Replace the position along the given axis with a new value.
    /// Useful when iterating along an axis.
    /// Suggestions for better names are welcome.
    pub fn at(&self, axis: usize, position: usize) -> Self {
        Self(self.0.at(axis, position))
    }
}

impl <const DIMENSION: usize> Index<usize> for InlineCellLocation<DIMENSION> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl <const DIMENSION: usize> IndexMut<usize> for InlineCellLocation<DIMENSION> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl <const DIMENSION: usize> Debug for InlineCellLocation<DIMENSION> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cl{:?}", self.0)
    }
}

impl <const DIMENSION: usize> From<[usize; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(coordinates: [usize; DIMENSION]) -> Self {
        InlineCellLocation(coordinates.into())
    }
}

impl <const DIMENSION: usize> From<[isize; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [isize; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[u32; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [u32; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[i32; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [i32; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[u16; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [u16; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[i16; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [i16; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[u8; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [u8; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<[i8; DIMENSION]> for InlineCellLocation<DIMENSION> {
    fn from(pt: [i8; DIMENSION]) -> Self {
        InlineCellLocation(pt.into())
    }
}

impl <const DIMENSION: usize> From<InlineCellLocation<DIMENSION>> for [usize; DIMENSION] {
    fn from(pt: InlineCellLocation<DIMENSION>) -> Self {
        pt.0.into()
    }
}