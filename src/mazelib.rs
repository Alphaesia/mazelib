//! # Introduction
//!
//! Mazes are complex.
//!
//! # Library Setup
//!
//! The library has two halves: the [interfaces][self::interface] for mazes, and the
//! [built-in implementations][self::implm] of those interfaces.
//!
//! # Recommended Reading Order
//!
//! It is suggested that you familiarise yourself with how this library has divided
//! mazes (as under the [crate::interface] module) before checking out the built-in
//! implementations in [crate::implm].
//!
//! # Further Reading
//!
//! if you want to learn more about mazes, like how many of the algorithms included here
//! actually work, Walter D. Pullen and Jamis Buck have some great information on the
//! subject.
//!
//! ## Think Labyrinth!
//!
//! Walter D. Pullen's [Think Labyrinth!](http://www.astrolog.org/labyrnth.htm) has a very
//! comprehensive reference on all aspects of mazes, including the various characteristics
//! of different algorithms and a glossary of maze-related terms.
//!
//! ## Buckblog & Mazes for Programmers
//!
//! For precise information about how maze generation algorithms actually work and how to
//! implement them yourself, Jamis Buck has some great blog entries on the subject.
//! They all come with visualisers you can run in the browser!
//!
//! <https://weblog.jamisbuck.org/2011/2/7/maze-generation-algorithm-recap>.
//!
//! He has also written a full book on maze algorithms,
//! [Mazes for Programmers](https://pragprog.com/titles/jbmaze/mazes-for-programmers/).
//!
//! It discusses generating mazes, solving mazes, mazes on cubes, spheres, cylinders,
//! and mobius strips, and more.
//!
//! You can find it at <https://pragprog.com/titles/jbmaze/mazes-for-programmers/>.
//!
//! # Glossary
//!
//! [Think Labyrinth!](http://www.astrolog.org/labyrnth.htm>) has a great glossary on technical
//! maze terms that you can find at <http://www.astrolog.org/labyrnth/glossary.htm>.

#![feature(array_zip)]
#![feature(min_specialization)]

#![allow(clippy::needless_return, clippy::bool_comparison)]  // Stylistic choices

pub mod interface;
pub mod implm;
pub(crate) mod internal;
#[cfg(test)] mod test;