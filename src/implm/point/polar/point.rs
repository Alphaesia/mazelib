use std::fmt::{Debug, Display, Formatter};
use crate::interface::point::Point;
use crate::internal::util::offset_usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolarCoordinate {
    pub ring: usize,
    pub sector: usize,
}

impl Point for PolarCoordinate {}

impl PolarCoordinate {
    pub fn with_ring(&self, ring: usize) -> Self {
        Self { ring, sector: self.sector }
    }

    pub fn with_sector(&self, sector: usize) -> Self {
        Self { ring: self.ring, sector }
    }

    pub fn offset_ring(&self, offset: isize) -> Self {
        self.with_ring(offset_usize(self.ring, offset))
    }

    pub fn offset_sector(&self, offset: isize) -> Self {
        self.with_sector(offset_usize(self.sector, offset))
    }
}

impl Debug for PolarCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ∠ {})", self.ring, self.sector)
    }
}

impl Display for PolarCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}