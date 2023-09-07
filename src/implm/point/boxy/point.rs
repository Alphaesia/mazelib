use std::convert::TryInto;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};

use crate::interface::point::Point;
use crate::internal::util::offset_usize;

/// A tuplet of unsigned integers that uniquely represent a point in `n`-dimensional space.
///
/// The most minor co-ordinate comes first (e.g. `(x, y, z)`, `(width, height, depth)`).
///
/// Note: While the name is Coordinate**Tuplet**, it is an array not a tuple.
///
/// # Examples
///
/// It's just a regular struct with all fields public, so you can construct it directly.
///
/// ```
/// # use mazelib::implm::point::boxy::CoordinateTuplet;
/// # 
/// CoordinateTuplet { 0: [1, 2, 3 ] };
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoordinateTuplet<const DIMENSION: usize>(pub [usize; DIMENSION]);

impl <const DIMENSION: usize> Point for CoordinateTuplet<DIMENSION> {}

impl <const DIMENSION: usize> CoordinateTuplet<DIMENSION> {
    /// Get the point at a given offset from this point along the given
    /// [axis](super::BoxCoordinateSpace#coordinate-axes).
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
    pub fn offset(&self, axis: usize, offset: isize) -> Self {
        let mut new = *self;

        new[axis] = offset_usize(new[axis], offset);

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
        let mut found_dimension: Option<usize> = None;

        for i in 0..DIMENSION {
            match self[i].abs_diff(other_pt[i]) {
                0 => {},
                1 => match found_dimension {
                    Some(_) => return None,
                    None    => found_dimension = Some(i),
                },
                _ => return None,
            }
        }

        return found_dimension
    }
}

/*
 * Indexing
 */

impl <const DIMENSION: usize> Index<usize> for CoordinateTuplet<DIMENSION> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl <const DIMENSION: usize> IndexMut<usize> for CoordinateTuplet<DIMENSION> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/*
 * Debug/display output
 */

impl <const DIMENSION: usize> Debug for CoordinateTuplet<DIMENSION> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for i in 0..(DIMENSION - 1) {
            write!(f, "{}, ", self[i])?;
        }

        write!(f, "{}", self[DIMENSION - 1])?;

        write!(f, ")")
    }
}

impl <const DIMENSION: usize> Display for CoordinateTuplet<DIMENSION> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/*
 * From/to conversions
 */

impl <const DIMENSION: usize> From<[usize; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [usize; DIMENSION]) -> Self {
        CoordinateTuplet(pt)
    }
}

impl <const DIMENSION: usize> From<[isize; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [isize; DIMENSION]) -> Self {
        CoordinateTuplet(pt.map(|coord| coord.try_into().expect("coordinate was negative")))
    }
}

impl <const DIMENSION: usize> From<[u32; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [u32; DIMENSION]) -> Self {
        CoordinateTuplet(pt.map(|coord| coord.try_into().expect("coordinate was negative")))
    }
}

impl <const DIMENSION: usize> From<[i32; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [i32; DIMENSION]) -> Self {
        CoordinateTuplet(pt.map(|coord| coord.try_into().expect("coordinate was negative")))
    }
}

impl <const DIMENSION: usize> From<[u16; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [u16; DIMENSION]) -> Self {
        CoordinateTuplet(pt.map(|coord| coord.try_into().expect("coordinate was negative")))
    }
}

impl <const DIMENSION: usize> From<[i16; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [i16; DIMENSION]) -> Self {
        CoordinateTuplet(pt.map(|coord| coord.try_into().expect("coordinate was negative")))
    }
}

impl <const DIMENSION: usize> From<[u8; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [u8; DIMENSION]) -> Self {
        CoordinateTuplet(pt.map(|coord| coord.try_into().expect("coordinate was negative")))
    }
}

impl <const DIMENSION: usize> From<[i8; DIMENSION]> for CoordinateTuplet<DIMENSION> {
    fn from(pt: [i8; DIMENSION]) -> Self {
        CoordinateTuplet(pt.map(|coord| coord.try_into().expect("coordinate was negative")))
    }
}

impl <const DIMENSION: usize> From<CoordinateTuplet<DIMENSION>> for [usize; DIMENSION] {
    fn from(pt: CoordinateTuplet<DIMENSION>) -> Self {
        pt.0
    }
}

impl From<(i32, i32)> for CoordinateTuplet<2> {
    fn from(pt: (i32, i32)) -> Self {
        CoordinateTuplet([pt.0 as usize, pt.1 as usize])
    }
}

impl From<(usize, usize)> for CoordinateTuplet<2> {
    fn from(pt: (usize, usize)) -> Self {
        CoordinateTuplet([pt.0, pt.1])
    }
}

impl From<(i32, i32, i32)> for CoordinateTuplet<3> {
    fn from(pt: (i32, i32, i32)) -> Self {
        CoordinateTuplet([pt.0 as usize, pt.1 as usize, pt.2 as usize])
    }
}

impl From<(usize, usize, usize)> for CoordinateTuplet<3> {
    fn from(pt: (usize, usize, usize)) -> Self {
        CoordinateTuplet([pt.0, pt.1, pt.2])
    }
}