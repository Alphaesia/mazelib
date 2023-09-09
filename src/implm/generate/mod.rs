//! A variety of different maze generators built-in.
//!
//! The documentation for each algorithm contains a brief overview of
//! the algorithm, the texture it produces, and some notable performance
//! characteristics to be aware of.
//!
//! [ThinkLabyrinth has a page that compares all the algorithms featured here and provides more detail and examples.](https://www.astrolog.org/labyrnth/algrithm.htm#perfect)
//!
//! The [Hunt-and-Kill algorithm][HuntAndKillGenerator] is **recommended
//! as the default algorithm**. It produces high-quality mazes while
//! maintaining high performance.
//!
//! # Comparison by Example
//!
//! Here is a comparison of the typical output of each generator. You can
//! see commentary on each example on their individual pages.
//!
//! [*Hunt-and-Kill*][HuntAndKillGenerator]:
//!
//! ![A typical output of Hunt-and-Kill.][example-hunt-and-kill]
//!
//! [*Recursive Backtracker*][RecursiveBacktrackerGenerator]:
//!
//! ![A typical output of Recursive Backtracker.][example-recursive-backtracker]
//!
//! [*n-ary Tree*][NAryTreeGenerator]:
//!
//! ![A typical output of n-ary Tree.][example-nary-tree]
//!
//! # See Also
//!
//! * [`MazeGenerator`][crate::interface::generate::MazeGenerator] --- the interface trait
#![doc = ::embed_doc_image::embed_image!("example-hunt-and-kill", "src/doc/img/generate/hunt-and-kill/example.png")]
#![doc = ::embed_doc_image::embed_image!("example-recursive-backtracker", "src/doc/img/generate/recursive-backtracker/example.png")]
#![doc = ::embed_doc_image::embed_image!("example-nary-tree", "src/doc/img/generate/nary-tree/example.png")]

pub use self::hunt_and_kill::HuntAndKillGenerator;
pub use self::nary_tree::NAryTreeGenerator;
pub use self::recursive_backtracker::RecursiveBacktrackerGenerator;

mod nary_tree;
mod hunt_and_kill;
mod recursive_backtracker;
mod util;

