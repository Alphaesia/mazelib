//! Polar (circular) coordinate spaces.
//!
//! For more information on what polar coordinates are, see [`PolarCoordinate`].

pub use self::iterator::PolarCoordinateSpaceIterator;
pub use self::point::PolarCoordinate;
pub use self::space::PolarCoordinateSpace;

mod space;
mod point;
mod iterator;

