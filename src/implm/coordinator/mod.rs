//! Coordinators for the other built-in components.
//!
//! There isn't a coordinator for every built-in (`CellValue`-`CoordinateSpace`) pair but the
//! common cases are covered (or at least, will be in future). If you desire a coordinator for a
//! pair not implemented here you should be able to use the existing implementations as a
//! reference.
//! 
//! # Comparison by Example
//! 
//! Here is a comparison of a typical maze of each coordinator. You can see commentary on each
//! example on their individual pages.
//! 
//! [`BoxSpaceBlockCellMazeCoordinator`][self::block::BoxSpaceBlockCellMazeCoordinator]:
//! 
//! ![A pixellated-looking maze, where every cell is one pixel][box-space-block-cell-coordinator-example]
//! 
//! [`BoxSpaceInlineCellMazeCoordinator`][self::inline::BoxSpaceInlineCellMazeCoordinator]:
//! 
//! TODO
//!
//! # See Also
//!
//! * [`MazeCoordinator`][crate::interface::coordinator::MazeCoordinator] --- the interface trait
#![doc = embed_doc_image::embed_image!("box-space-block-cell-coordinator-example", "src/doc/img/coordinator/box-space-block-cell/example-large.png")]

pub mod block;
pub mod inline;
