//! Common cell classes.
//!
//! Pick your favourite.
//!
//! # Comparison by Example
//!
//! [Block cell][self::block]:
//! <!-- why yes, I have used this exact example in five different places -->
//! ![A pixellated-looking maze, where every cell is one pixel][box-space-block-cell-coordinator-example]
//! 
//! [Inline cell][self::inline]:
//! TODO
#![doc = embed_doc_image::embed_image!("box-space-block-cell-coordinator-example", "src/doc/img/coordinate/box-space-block-cell/example-large.png")]

pub mod block;
pub mod inline;