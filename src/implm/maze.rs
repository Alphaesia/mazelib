//! Some carriers for common maze types.
//!
//! As discussed in [`Maze`][crate::interface::maze::Maze], `Maze`
//! implementors are just [`CellManager`][crate::interface::cell::CellManager]s.
//! Here, we re-export the builtin cell managers under new names,
//! replacing "CellManager" with "Maze". This is to match the `CellManager`/
//! `Maze` split discussed in [`Maze`][crate::interface::maze::Maze] for
//! `CellManager` implementations.

/// Re-exports of [`crate::implm::cell::block`] under different names.
///
/// Use these names when modelling cell managers as maze carriers. Use
/// their other names when modelling them as maze components.
pub mod block {
    pub use super::super::cell::block::BoxSpaceBlockCellManager as BoxSpaceBlockCellMaze;
    pub use super::super::cell::block::BoxSpaceBlockCellManagerBuilder as BoxSpaceBlockCellMazeBuilder;
}

/// Re-exports of [`crate::implm::cell::inline`] under different names.
///
/// Use these names when modelling cell managers as maze carriers. Use
/// their other names when modelling them as maze components.
pub mod inline {
    pub use super::super::cell::inline::BoxSpaceInlineCellManager as BoxSpaceInlineCellMaze;
    pub use super::super::cell::inline::BoxSpaceInlineCellManagerBuilder as BoxSpaceInlineCellMazeBuilder;
}