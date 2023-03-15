//! Common coordinate spaces and their points.
//!
//! If you are looking for [two-dimensional][self::boxy::TwoDimensionalBoxCoordinateSpace]
//! or [three-dimensional][self::boxy::ThreeDimensionalBoxCoordinateSpace] coordinate spaces,
//! check out [`boxy`].
//!
//! # See Also
//! * [`CoordinateSpace`][crate::interface::point::CoordinateSpace], and
//! * [`Point`][crate::interface::point::Point] --- the interface traits.

pub mod boxy;
pub mod polar;