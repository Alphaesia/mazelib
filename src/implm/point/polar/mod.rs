//! Polar (circular) coordinate spaces.
//!
//! For more information on what polar coordinates are, see [`PolarCoordinate`].

mod space;
mod point;
mod iterator;

pub use self::space::PolarCoordinateSpace;
pub use self::point::PolarCoordinate;
pub use self::iterator::PolarCoordinateSpaceIterator;