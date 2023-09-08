use std::iter::FusedIterator;
use crate::implm::point::polar::{PolarCoordinate, PolarCoordinateSpace};

/// An iterator for iterating over all of the points in a boxy coordinate space.
///
/// Points are yielded by sector, then by ring, in ascending order.
///
/// Can only be obtained by calling
/// [`PolarCoordinateSpace::iter()`][crate::interface::point::CoordinateSpace::iter] or
/// [`PolarCoordinateSpace::iter_from()`][crate::interface::point::CoordinateSpace::iter_from].
///
/// # Examples
///
/// ```
/// # use mazelib::implm::point::polar::{PolarCoordinate, PolarCoordinateSpace};
/// # use mazelib::interface::point::CoordinateSpace;
/// #
/// let mut iter = PolarCoordinateSpace::new_checked(2, 2).iter();
///
/// assert_eq!(Some(PolarCoordinate { ring: 0, sector: 0 }), iter.next());
/// assert_eq!(Some(PolarCoordinate { ring: 0, sector: 1 }), iter.next());
/// assert_eq!(Some(PolarCoordinate { ring: 1, sector: 0 }), iter.next());
/// assert_eq!(Some(PolarCoordinate { ring: 1, sector: 1 }), iter.next());
/// assert_eq!(None, iter.next());
/// assert_eq!(None, iter.next());
/// ```
pub struct PolarCoordinateSpaceIterator {
    space: PolarCoordinateSpace,
    pos: Option<PolarCoordinate>,
}

// Constructor
impl PolarCoordinateSpaceIterator {
    #[must_use]
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
                if pt.sector + 1 == usize::from(self.space.sectors()) {
                    if pt.ring + 1 == usize::from(self.space.rings()) {
                        return None  // Iterator is done
                    } else {
                        pt.sector = 0;
                        pt.ring += 1;
                    }
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