use std::num::NonZeroUsize;
use rand::Rng;
use crate::implm::point::polar::point::PolarCoordinate;
use crate::implm::point::polar::PolarCoordinateSpaceIterator;
use crate::interface::point::CoordinateSpace;

#[derive(Copy, Clone, Debug)]
pub struct PolarCoordinateSpace {
    rings: NonZeroUsize,
    sectors: NonZeroUsize,
    size: NonZeroUsize
}

impl PolarCoordinateSpace {
    pub fn new(rings: NonZeroUsize, sectors: NonZeroUsize) -> Self {
        let size = rings.checked_mul(sectors).expect("The dimensions specified are too large. The number of points in the space does not fit within a usize.");

        Self { rings, sectors, size }
    }

    pub fn new_checked(rings: usize, sectors: usize) -> Self {
        Self::new(NonZeroUsize::new(rings).expect("rings must be non-zero"), NonZeroUsize::new(sectors).expect("sectors must be non-zero"))
    }

    pub fn rings(&self) -> NonZeroUsize {
        self.rings
    }

    pub fn sectors(&self) -> NonZeroUsize {
        self.sectors
    }
}

impl CoordinateSpace for PolarCoordinateSpace {
    type PtType = PolarCoordinate;
    type Iter = PolarCoordinateSpaceIterator;

    fn logical_size(&self) -> NonZeroUsize {
        self.size
    }

    fn neighbours_of_pt(&self, pt: Self::PtType) -> Vec<Self::PtType> {
        let mut neighbours = Vec::with_capacity(4);

        let sectors_minus_one = usize::from(self.sectors) - 1;

        if pt.ring > 0 {
            neighbours.push(pt.offset_ring(-1))
        }

        if pt.ring < sectors_minus_one {
            neighbours.push(pt.offset_ring(1))
        }

        if pt.sector == 0 {
            neighbours.push(pt.with_sector(sectors_minus_one))
        } else {
            neighbours.push(pt.offset_sector(-1))
        }

        if pt.sector == sectors_minus_one {
            neighbours.push(pt.with_sector(0))
        } else {
            neighbours.push(pt.offset_sector(1))
        }

        return neighbours
    }

    fn are_adjacent(&self, pt1: Self::PtType, pt2: Self::PtType) -> bool {
        // Only sectors wrap - rings don't

        return if pt1.ring == pt2.ring {
            pt1.sector.abs_diff(pt2.sector) == 1
                || pt1.sector == 0 && pt2.sector == usize::from(self.sectors) - 1
                || pt1.sector == usize::from(self.sectors) - 1 && pt2.sector == 0
        } else if pt1.sector == pt2.sector {
            pt1.ring.abs_diff(pt2.ring) == 1
        } else {
            false
        }
    }

    fn iter(&self) -> Self::Iter {
        PolarCoordinateSpaceIterator::new(*self, None)
    }

    fn iter_from(&self, pt: Self::PtType) -> Self::Iter {
        PolarCoordinateSpaceIterator::new(*self, Some(pt))
    }

    fn choose(&self, rng: &mut (impl Rng + ?Sized)) -> Self::PtType {
        let ring = rng.gen_range(0..self.rings.into());

        let sector = rng.gen_range(0..self.sectors.into());

        return Self::PtType { ring, sector }
    }
}