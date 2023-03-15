use std::iter::FusedIterator;
use crate::implm::point::polar::{PolarCoordinate, PolarCoordinateSpace};

pub struct PolarCoordinateSpaceIterator {
    space: PolarCoordinateSpace,
    pos: Option<PolarCoordinate>,
}

// Constructor
impl PolarCoordinateSpaceIterator {
    pub(crate) fn new(space: PolarCoordinateSpace, starting_pos: Option<PolarCoordinate>) -> Self {
        Self { space, pos: starting_pos }
    }
}

impl Iterator for PolarCoordinateSpaceIterator {
    type Item = PolarCoordinate;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pos {
            None => self.pos = Some(PolarCoordinate { ring: 0, sector: 0 }),
            Some(mut pt) => {
                if pt.ring + 1 == usize::from(self.space.rings()) {
                    return None  // Iterator is done
                } else if pt.sector + 1 == usize::from(self.space.sectors()) {
                    pt.sector = 0;
                    pt.ring += 1;
                } else {
                    pt.sector += 1;
                }

                self.pos = Some(pt);
            }
        }

        return self.pos
    }
}

impl FusedIterator for PolarCoordinateSpaceIterator {}