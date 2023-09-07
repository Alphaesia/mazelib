//! Coordinators for the other built-in components.
//!
//! There isn't a coordinator for every built-in (`CellValue`-`CoordinateSpace`) pair but the
//! common cases are covered (or at least, will be in future). If you desire a coordinator for a
//! pair not implemented here you should be able to use the existing implementations as a
//! reference.
//!
//! # See Also
//!
//! * [`MazeCoordinator`][crate::interface::coordinator::MazeCoordinator] --- the interface trait

pub mod block;
pub mod inline;
