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

/// An iterator for iterating over all of the points in a polar coordinate space.
///
/// Points are yielded by their coordinate in ascending order, with more minor coordinates having
/// precedence.
///
/// Can only be obtained by calling
/// [`BoxCoordinateSpace::iter()`][crate::interface::point::CoordinateSpace::iter] or
/// [`BoxCoordinateSpace::iter_from()`][crate::interface::point::CoordinateSpace::iter_from].
///
/// # Examples
///
/// ```
/// # use mazelib::implm::point::boxy::{BoxCoordinateSpace, CoordinatePair};
/// # use mazelib::interface::point::CoordinateSpace;
/// #
/// let mut iter = BoxCoordinateSpace::new_checked([2, 2]).iter();
///
/// assert_eq!(Some(CoordinatePair { 0: [0, 0] }), iter.next());
/// assert_eq!(Some(CoordinatePair { 0: [1, 0] }), iter.next());
/// assert_eq!(Some(CoordinatePair { 0: [0, 1] }), iter.next());
/// assert_eq!(Some(CoordinatePair { 0: [1, 1] }), iter.next());
/// assert_eq!(None, iter.next());
/// assert_eq!(None, iter.next());
/// ```
pub struct BoxCoordinateSpaceIterator<const DIMENSION: usize> {
    space: BoxCoordinateSpace<DIMENSION>,
    pos: Option<CoordinateTuplet<DIMENSION>>
}

// Constructor
impl <const DIMENSION: usize> BoxCoordinateSpaceIterator<DIMENSION> {
    pub(crate) fn new(space: BoxCoordinateSpace<DIMENSION>, starting_pos: Option<CoordinateTuplet<DIMENSION>>) -> Self {
        Self { space, pos: starting_pos }
    }
}

impl <const DIMENSION: usize> Iterator for BoxCoordinateSpaceIterator<DIMENSION> {
    type Item = CoordinateTuplet<DIMENSION>;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment iterator position then return it
        match self.pos {
            None => self.pos = Some([0; DIMENSION].into()),
            Some(mut pt) => {
                for dim in 0..DIMENSION {
                    if pt[dim] + 1 == usize::from(self.space[dim]) {
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
        let max = usize::from(self.space.dimensions().product());

        let remaining = match self.pos {
            Some(pos) => {
                let mut running_total = max;

                for dim in DIMENSION..1 {
                    running_total -= pos[dim] * usize::from(self.space[dim - 1]);
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