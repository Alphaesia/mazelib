//! Some basic built-in [MazeBuffer][crate::interface::buffer::MazeBuffer] implementations.
//!
//! In almost all cases, you will want to use a [VecBuffer][self::vec::VecBuffer].

mod vec;
mod array;

pub use self::vec::VecBuffer;
pub use self::array::ArrayBuffer;