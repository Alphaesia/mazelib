use std::ops::{Add, AddAssign};
use std::fmt::{Debug, Formatter};
use core::fmt;

pub trait Point: Copy + Clone + Sized + PartialEq + Eq + Debug {}

/// A series of numbers that uniquely represent a point in n-dimensional space.
///
/// The most minor co-ordinate comes first (e.g. (x, y, z), (width, height, depth)).
///
/// Note: While the trait requires an implementation Into<[usize; DIMENSION]>, it is HIGHLY recommended
/// that you implement From<YourCoordinateTuplet> for [usize; DIMENSION] (which provides an implementation
/// of Into<[usize; DIMENSION]> for YourCoordinateTuplet). This will save you pain later on.
pub trait CoordinateTuplet<const DIMENSION: usize>: Point + From<[usize; DIMENSION]> + From<[isize; DIMENSION]> + From<[i32; DIMENSION]> + Into<[usize; DIMENSION]> {
    /// Get the point at a given offset from this point
    /// (dimension refers to the direction of the offset - e.g. x-direction is dimension 0).
    fn offset(&self, dimension: usize, offset: usize) -> Self {
        let array: [usize; DIMENSION] = self.into();

        let mut array = array.clone();

        array[dimension] += offset;

        return array
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoordinatePair {
    pub x: usize,
    pub y: usize
}

impl Point for CoordinatePair {}
impl CoordinateTuplet<2> for CoordinatePair {}

//region CoordinatePair from tuple/array

impl From<(usize, usize)> for CoordinatePair {
    fn from(coords: (usize, usize)) -> Self {
        CoordinatePair { x: coords.0, y: coords.1 }
    }
}

impl From<(isize, isize)> for CoordinatePair {
    fn from(coords: (isize, isize)) -> Self {
        CoordinatePair { x: coords.0 as usize, y: coords.1 as usize }
    }
}

impl From<(i32, i32)> for CoordinatePair {
    fn from(coords: (i32, i32)) -> Self {
        CoordinatePair { x: coords.0 as usize, y: coords.1 as usize }
    }
}

impl From<[usize; 2]> for CoordinatePair {
    fn from(coords: [usize; 2]) -> Self {
        CoordinatePair { x: coords[0], y: coords[1] }
    }
}

impl From<[isize; 2]> for CoordinatePair {
    fn from(coords: [isize; 2]) -> Self {
        CoordinatePair { x: coords[0] as usize, y: coords[1] as usize }
    }
}

impl From<[i32; 2]> for CoordinatePair {
    fn from(coords: [i32; 2]) -> Self {
        CoordinatePair { x: coords[0] as usize, y: coords[1] as usize }
    }
}

impl From<CoordinatePair> for [usize; 2] {
    fn from(pair: CoordinatePair) -> Self {
        [pair.x, pair.y]
    }
}

//endregion

//region CoordinatePair addition

impl Add<CoordinatePair> for CoordinatePair {
    type Output = CoordinatePair;

    fn add(self, rhs: CoordinatePair) -> CoordinatePair {
        CoordinatePair::from((self.x + rhs.x, self.y + rhs.y))
    }
}

impl Add<(usize, usize)> for CoordinatePair {
    type Output = CoordinatePair;

    fn add(self, rhs: (usize, usize)) -> CoordinatePair {
        self + CoordinatePair::from(rhs)
    }
}

// Allows you to "subtract" from a point
impl Add<(isize, isize)> for CoordinatePair {
    type Output = CoordinatePair;

    fn add(self, rhs: (isize, isize)) -> CoordinatePair {
        CoordinatePair::from((self.x as isize + rhs.0, self.y as isize + rhs.1))
    }
}

// Used for addition literals (pt + (1, 0)) since numeric literals default to i32
impl Add<(i32, i32)> for CoordinatePair {
    type Output = CoordinatePair;

    fn add(self, rhs: (i32, i32)) -> CoordinatePair {
        CoordinatePair::from((self.x as isize + rhs.0 as isize, self.y as isize + rhs.1 as isize))
    }
}

impl AddAssign<CoordinatePair> for CoordinatePair {
    fn add_assign(&mut self, rhs: CoordinatePair) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign<(usize, usize)> for CoordinatePair {
    fn add_assign(&mut self, rhs: (usize, usize)) {
        *self = Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1
        }
    }
}

impl AddAssign<(isize, isize)> for CoordinatePair {
    fn add_assign(&mut self, rhs: (isize, isize)) {
        *self = Self {
            x: (self.x as isize + rhs.0) as usize,
            y: (self.y as isize + rhs.1) as usize
        }
    }
}

impl AddAssign<(i32, i32)> for CoordinatePair {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        *self = Self {
            x: (self.x as isize + rhs.0 as isize) as usize,
            y: (self.y as isize + rhs.1 as isize) as usize
        }
    }
}

//endregion

impl Debug for CoordinatePair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoordinateTriplet {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl Point for CoordinateTriplet {}
impl CoordinateTuplet<3> for CoordinateTriplet {}

//region CoordinateTriplet from tuple/array

impl From<(usize, usize, usize)> for CoordinateTriplet {
    fn from(coords: (usize, usize, usize)) -> Self {
        CoordinateTriplet { x: coords.0, y: coords.1, z: coords.2 }
    }
}

impl From<(isize, isize, isize)> for CoordinateTriplet {
    fn from(coords: (isize, isize, isize)) -> Self {
        CoordinateTriplet { x: coords.0 as usize, y: coords.1 as usize, z: coords.2 as usize }
    }
}

impl From<(i32, i32, i32)> for CoordinateTriplet {
    fn from(coords: (i32, i32, i32)) -> Self {
        CoordinateTriplet { x: coords.0 as usize, y: coords.1 as usize, z: coords.2 as usize }
    }
}

impl From<[usize; 3]> for CoordinateTriplet {
    fn from(coords: [usize; 3]) -> Self {
        CoordinateTriplet { x: coords[0], y: coords[1], z: coords[2] }
    }
}

impl From<[isize; 3]> for CoordinateTriplet {
    fn from(coords: [isize; 3]) -> Self {
        CoordinateTriplet { x: coords[0] as usize, y: coords[1] as usize, z: coords[2] as usize }
    }
}

impl From<[i32; 3]> for CoordinateTriplet {
    fn from(coords: [i32; 3]) -> Self {
        CoordinateTriplet { x: coords[0] as usize, y: coords[1] as usize, z: coords[2] as usize }
    }
}

impl From<CoordinateTriplet> for [usize; 3] {
    fn from(pair: CoordinateTriplet) -> Self {
        [pair.x, pair.y, pair.z]
    }
}

//endregion

//region CoordinateTriplet addition

impl Add<CoordinateTriplet> for CoordinateTriplet {
    type Output = CoordinateTriplet;

    fn add(self, rhs: CoordinateTriplet) -> CoordinateTriplet {
        CoordinateTriplet::from((self.x + rhs.x, self.y + rhs.y, self.z + rhs.z))
    }
}

impl Add<(usize, usize, usize)> for CoordinateTriplet {
    type Output = CoordinateTriplet;

    fn add(self, rhs: (usize, usize, usize)) -> CoordinateTriplet {
        self + CoordinateTriplet::from(rhs)
    }
}

impl Add<(isize, isize, isize)> for CoordinateTriplet {
    type Output = CoordinateTriplet;

    fn add(self, rhs: (isize, isize, isize)) -> CoordinateTriplet {
        CoordinateTriplet::from((self.x as isize + rhs.0, self.y as isize + rhs.1, self.z as isize + rhs.2))
    }
}

impl Add<(i32, i32, i32)> for CoordinateTriplet {
    type Output = CoordinateTriplet;

    fn add(self, rhs: (i32, i32, i32)) -> CoordinateTriplet {
        CoordinateTriplet::from((self.x as isize + rhs.0 as isize, self.y as isize + rhs.1 as isize, self.z as isize + rhs.2 as isize))
    }
}

impl AddAssign<CoordinateTriplet> for CoordinateTriplet {
    fn add_assign(&mut self, rhs: CoordinateTriplet) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl AddAssign<(usize, usize, usize)> for CoordinateTriplet {
    fn add_assign(&mut self, rhs: (usize, usize, usize)) {
        *self = Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2
        }
    }
}

impl AddAssign<(isize, isize, isize)> for CoordinateTriplet {
    fn add_assign(&mut self, rhs: (isize, isize, isize)) {
        *self = Self {
            x: (self.x as isize + rhs.0) as usize,
            y: (self.y as isize + rhs.1) as usize,
            z: (self.z as isize + rhs.2) as usize
        }
    }
}

impl AddAssign<(i32, i32, i32)> for CoordinateTriplet {
    fn add_assign(&mut self, rhs: (i32, i32, i32)) {
        *self = Self {
            x: (self.x as isize + rhs.0 as isize) as usize,
            y: (self.y as isize + rhs.1 as isize) as usize,
            z: (self.z as isize + rhs.2 as isize) as usize
        }
    }
}

//endregion

impl Debug for CoordinateTriplet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}