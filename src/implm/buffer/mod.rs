//! Some basic built-in buffer implementations.
//!
//! In almost all cases you will want to use a [`VecBuffer`].
//!
//! # See Also
//!
//! * [`MazeBuffer`](crate::interface::buffer::MazeBuffer) --- the interface trait

mod vec;
mod array;

pub use self::vec::VecBuffer;
pub use self::array::ArrayBuffer;