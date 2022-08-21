use std::convert::TryInto;

use crate::interface::point::Point;

/// A tuplet of unsigned integers that uniquely represent a point in n-dimensional space.
///
/// The most minor co-ordinate comes first (e.g. (x, y, z), (width, height, depth)).
///
/// Note: While the name is Coordinate*Tuplet*, this is closer to an array with a
/// generic length than a tuple.
///
/// # Implementing
///
///  While the trait requires an implementation [`Into<[usize; DIMENSION]>`][Into],
/// as per the trait's documentation it is recommended that you implement
/// [`From<YourCoordinateTuplet>`][From] for `[usize; DIMENSION]` instead, as it provides an
/// implementation of [`Into<[usize; DIMENSION]>`][Into] for `YourCoordinateTuplet`.
/// This will save you pain later on.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoordinateTuplet<const DIMENSION: usize> {
    pub(super) coords: [usize; DIMENSION]
}

impl <const DIMENSION: usize> Point for CoordinateTuplet<DIMENSION> {}

impl <const DIMENSION: usize> CoordinateTuplet<DIMENSION> {
    /// Get the point at a given offset from this point
    /// (dimension refers to the direction of the offset - e.g. x-direction is dimension 0).
    pub fn offset(&self, dimension: usize, offset: isize) -> Self {
        let mut new = *self;

        if offset >= 0 {
            new[dimension] += TryInto::<usize>::try_into(offset).unwrap();
        } else {
            new[dimension] -= TryInto::<usize>::try_into(offset.abs()).unwrap();
        }

        return new;
    }

    /// Replace the position along the given axis with a new value.
    /// Useful when iterating along an axis.
    /// Suggestions for better names are welcome.
    pub fn at(&self, axis: usize, position: usize) -> Self {
        let mut new = *self;

        new[axis] = position;

        return new;
    }

    /// Return true if any coordinate is the same as `value`.
    pub fn any(&self, value: usize) -> bool {
        self.coords.contains(&value)
    }
}