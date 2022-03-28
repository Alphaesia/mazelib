//! A variety of built-in implementations of the interfaces defined in [crate::interface].
//!
//! If you are not doing anything special, and just generating/solving regular mazes,
//! you can use the implementations in this crate instead of rolling your own.
//!
//! Despite what the name may otherwise imply, public items found in this module are
//! considered API and are safe to use.

pub mod buffer;
pub mod point;
pub mod cell;
pub mod render;
pub mod template;
pub mod generate;