use std::iter::FusedIterator;
use crate::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};
use crate::interface::point::CoordinateSpace;
use crate::internal::array_util::Product;

impl<const DIMENSION: usize> IntoIterator for BoxCoordinateSpace<DIMENSION> {
    type Item = CoordinateTuplet<DIMENSION>;
    type IntoIter = BoxCoordinateSpaceIterator<DIMENSION>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const DIMENSION: usize> IntoIterator for &BoxCoordinateSpace<DIMENSION> {
    type Item = CoordinateTuplet<DIMENSION>;
    type IntoIter = BoxCoordinateSpaceIterator<DIMENSION>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct BoxCoordinateSpaceIterator<const DIMENSION: usize> {
    pub(crate) space: BoxCoordinateSpace<DIMENSION>,
    pub(crate) pos: Option<CoordinateTuplet<DIMENSION>>
}

impl <const DIMENSION: usize> Iterator for BoxCoordinateSpaceIterator<DIMENSION> {
    type Item = CoordinateTuplet<DIMENSION>;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment iterator position then return it
        match self.pos {
            // Important: we DON'T unwrap the Option because if the coordinate space is
            // empty, we do NOT want to return a result (CoordinateSpace::origin() returns
            // None if it is empty).
            None => self.pos = self.space.origin(),
            Some(pt) => {
                let mut pt = pt;

                for dim in 0..DIMENSION {
                    if pt[dim] + 1 == self.space[dim] {
                        pt[dim] = 0;
                    } else {
                        pt[dim] += 1;

                        self.pos = Some(pt);

                        return self.pos
                    }
                }

                return None  // All dimensions were saturated  // No self.pos set for iterator fusion
            }
        }

        return self.pos
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let max = self.space.dimensions().product();

        let remaining = match self.pos {
            Some(pos) => {
                let mut running_total = max;

                for dim in DIMENSION..1 {
                    running_total -= pos[dim] * self.space[dim - 1];
                }

                running_total -= pos[0];

                running_total -= 1; // to account for 0-based indexing

                running_total
            },
            None => max
        };

        (remaining, Some(remaining))
    }
}

impl <const DIMENSION: usize> FusedIterator for BoxCoordinateSpaceIterator<DIMENSION> {}