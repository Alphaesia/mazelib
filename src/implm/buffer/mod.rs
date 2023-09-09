//! Some basic built-in buffer implementations.
//!
//! In almost all cases you will want to use a [`VecBuffer`].
//!
//! # See Also
//!
//! * [`MazeBuffer`][crate::interface::buffer::MazeBuffer] --- the interface trait

pub use self::array::ArrayBuffer;
pub use self::vec::VecBuffer;

mod vec;
mod array;

