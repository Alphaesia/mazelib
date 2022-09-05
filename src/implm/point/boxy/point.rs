use std::convert::TryInto;

use crate::interface::point::Point;

/// A tuplet of unsigned integers that uniquely represent a point in `n`-dimensional space.
///
/// The most minor co-ordinate comes first (e.g. `(x, y, z)`, `(width, height, depth)`).
///
/// Note: While the name is Coordinate**Tuplet**, it is an array not a tuple.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoordinateTuplet<const DIMENSION: usize>(pub [usize; DIMENSION]);

impl <const DIMENSION: usize> Point for CoordinateTuplet<DIMENSION> {}

impl <const DIMENSION: usize> CoordinateTuplet<DIMENSION> {
    /// Get the point at a given offset from this point along the given [axis].
    ///
    /// # Examples
    ///
    /// ```
    /// use mazelib::implm::point::boxy::CoordinateTuplet;
    ///
    /// let pt = CoordinateTuplet([2, 2, 2]);
    ///
    /// assert_eq!(CoordinateTuplet([2, 5, 2]), pt.offset(1, 3));
    /// assert_eq!(CoordinateTuplet([1, 2, 2]), pt.offset(0, -1));
    /// ```
    ///
    /// [axis]: self::super::BoxCoordinateSpace#coordinate-axes
    pub fn offset(&self, axis: usize, offset: isize) -> Self {
        let mut new = *self;

        if offset >= 0 {
            new[axis] += TryInto::<usize>::try_into(offset).unwrap();
        } else {
            new[axis] -= TryInto::<usize>::try_into(offset.abs()).unwrap();
        }

        return new;
    }

    /// Replace the position along the given [axis] with a new value.
    ///
    /// Useful when iterating along an axis.
    ///
    /// Suggestions for better names are welcome.
    ///
    /// # Examples
    ///
    /// ```
    /// use mazelib::implm::point::boxy::CoordinateTuplet;
    ///
    /// let pt = CoordinateTuplet([1, 1, 1]);
    ///
    /// assert_eq!(CoordinateTuplet([1, 1, 0]), pt.at(2, 0));
    /// ```
    ///
    /// [axis]: self::super::BoxCoordinateSpace#coordinate-axes
    pub fn at(&self, axis: usize, position: usize) -> Self {
        let mut new = *self;

        new[axis] = position;

        return new;
    }

    /// Return true if any coordinate of this tuplet is the same as `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mazelib::implm::point::boxy::CoordinateTuplet;
    ///
    /// assert!(CoordinateTuplet([7, 8, 9]).any_coordinate_is(8));
    /// assert!(CoordinateTuplet([0, 0, 1]).any_coordinate_is(0));
    ///
    /// assert_eq!(false, CoordinateTuplet([7, 8, 9]).any_coordinate_is(1));
    /// assert_eq!(false, CoordinateTuplet([0, 0, 1]).any_coordinate_is(2));
    /// ```
    pub fn any_coordinate_is(&self, value: usize) -> bool {
        self.0.contains(&value)
    }

    /// Return the [axis] upon which this point is [directly adjacent] with another point.
    ///
    /// # Examples
    ///
    /// ```
    /// use mazelib::implm::point::boxy::CoordinateTuplet;
    ///
    /// assert_eq!(Some(1), CoordinateTuplet([5, 5, 5]).axis_of_adjacency_with(CoordinateTuplet([5, 4, 5])));
    /// assert_eq!(Some(0), CoordinateTuplet([14, 2, 8]).axis_of_adjacency_with(CoordinateTuplet([13, 2, 8])));
    /// assert_eq!(None, CoordinateTuplet([2, 2, 2]).axis_of_adjacency_with(CoordinateTuplet([2, 3, 3])));
    /// assert_eq!(None, CoordinateTuplet([1, 1, 1]).axis_of_adjacency_with(CoordinateTuplet([1, 1, 1])));
    /// ```
    ///
    /// [axis]: self::super::BoxCoordinateSpace#coordinate-axes
    /// [directly adjacent]: self::super::BoxCoordinateSpace#direct-adjacency
    pub fn axis_of_adjacency_with(&self, other_pt: CoordinateTuplet<DIMENSION>) -> Option<usize> {
        for i in 0..DIMENSION {
            return match self[i].abs_diff(other_pt[i]) {
                0 => continue,
                1 => Some(i),
                _ => None,
            }
        }

        return None
    }
}