//! Mazes are inherently complex. However, most programs give you very limited options
//! in dealing with mazes. Whether being restricted to 2D square mazes, or only supporting
//! one type of output. This library aims to unlock and expose mazes fully and completely,
//! allowing you to create, manipulate, analyse, and solve mazes however you desire.
//!
//! # Quick Start
//!
//! For those who want to jump straight into playing around. Do come back and read the docs
//! though!
//!
//! ```
//! use mazelib::implm::buffer::VecBuffer;
//! use mazelib::implm::cell::block::BlockCellValue;
//! use mazelib::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinatorBuilder;
//! use mazelib::implm::generate::HuntAndKillGenerator;
//! use mazelib::implm::point::boxy::BoxCoordinateSpace;
//! use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
//! use mazelib::interface::generate::DefaultMazeGenerator;
//! use mazelib::interface::render::DefaultMazeRenderer;
//! use mazelib::util::apply_solid_border;
//!
//! let [width, height] = [5, 5];
//!
//! // Determine the size of the maze
//! let coord_space = BoxCoordinateSpace::new_checked([width, height]);
//!
//! type Buffer = VecBuffer<BlockCellValue>;
//!
//! // Create the maze "object". The name of the maze type hints at what it supports:
//! // "BoxSpace" means it expects a BoxCoordinateSpace
//! // "BlockCell" means it will use BlockCells for cells
//! // "MazeCoordinator" means it's a maze object (roughly)
//! // 2 means we're creating a 2D maze
//! let mut maze = BoxSpaceBlockCellMazeCoordinatorBuilder::<Buffer, 2>::new(coord_space).build();
//!
//! // Give our maze a nice thick border
//! apply_solid_border(&mut maze);
//!
//! // Generate the maze
//! HuntAndKillGenerator::generate(&mut maze);
//!
//! // Print the maze as text
//! # #[allow(unused_must_use)]
//! BoxSpaceTextMazeRenderer::render(&maze, &mut std::io::stdout());
//! ```
//! produces
//! ```text
//! ███████████████
//! █ █   █       █
//! █ █ █ █ █████ █
//! █   █ █   █   █
//! █████ █████ █ █
//! █   █   █   █ █
//! █ █ ███ █ ███ █
//! █ █ █   █   █ █
//! █ █ █ █████ █ █
//! █ █ █       █ █
//! █ █ █████████ █
//! █ █       █   █
//! █ ███████ █ ███
//! █       █     █
//! ███████████████
//! ```
//!
//! All of the implementations in this example (`BoxCoordinateSpace`, `BlockCellValue`, etc.)
//! can be swapped out for other implementations. Different implementations will affect an aspect
//! of the generated maze in different ways.
//!
//! For example, changing `BlockCellValue` to `InlineCellValue<2>` will change how the cells look.
//! (Note you wll have to replace `BoxSpaceBlockCellMazeBuilder` with `BoxSpaceInlineCellMazeBuilder`).
//!
//! ```
//! # use mazelib::implm::buffer::VecBuffer;
//! # use mazelib::implm::cell::inline::InlineCellValue;
//! # use mazelib::implm::coordinator::inline::BoxSpaceInlineCellMazeCoordinatorBuilder;
//! # use mazelib::implm::point::boxy::BoxCoordinateSpace;
//! #
//! # const DIMENSION: usize = 2;
//! # let coord_space = BoxCoordinateSpace::new_checked([1, 1]);
//! #
//! // 2 for a 2D maze
//! type Buffer = VecBuffer<InlineCellValue<2>>;
//!
//! let mut maze = BoxSpaceInlineCellMazeCoordinatorBuilder::<Buffer, 2>::new(coord_space).build();
//! ```
//! produces
//! ```text
//! ┏━━━━━┯━━━━━━━━━━━┯━━┓
//! ┃     │           │  ┃
//! ┠──┐  └─────╴  ╷  ╵  ┃
//! ┃  │           │     ┃
//! ┃  └───────────┼──╴  ┃
//! ┃              │     ┃
//! ┠──╴  ╷  ┌─────┤  ╶──┨
//! ┃     │  │     │     ┃
//! ┃  ╶──┤  │  ╷  └──┐  ┃
//! ┃     │  │  │     │  ┃
//! ┃  ╷  └──┘  │  ╷  │  ┃
//! ┃  │        │  │  │  ┃
//! ┃  └────────┤  └──┘  ┃
//! ┃           │        ┃
//! ┗━━━━━━━━━━━┷━━━━━━━━┛
//! ```
//!
//! # Architecture
//!
//! The library is split into two halves:
//!
//! [`interface`] holds the key abstractions and defines the model of mazes the library
//! uses. It's very important.
//!
//! [`implm`] has a bunch of built-in implementations of the aforementioned
//! interfaces for your convenience. While you're probably want to use them in most cases,
//! you're not obligated to use all (or any) of them. Feel free to implement any of the
//! interfaces yourself!
//!
//! # Reading Order
//!
//! Firstly, read the previous section in this page on the [crate's architecture](#architecture).
//!
//! Then, you should read the module-level docs of [`interface`]. It explains all of the
//! key interfaces and their role in our maze model.
//!
//! After that, you'll probably want to check out [`interface::coordinator::MazeCoordinator`].
//! This is the central trait which all of the machinery interacts with, and is the main
//! interface between you and the maze. The documentation for
//! [`interface::generate::MazeGenerator`] will likely also be of use. The details of the
//! other interfaces are not too important unless you're implementing those interfaces.
//!
//! Then, explore [`implm`]. It contains standard implementations of all of the
//! interfaces. You'll almost certainly want to use some of them, so have a look around.
//!
//! Finally, take a look at some of the utilities provided in [`util`]. They're fairly simple but
//! it's nice not to have to reimplement them yourself.
//!
//! ## Further Reading
//!
//! If you want to learn more about mazes, like how many of the algorithms included here
//! actually work, Walter D. Pullen and Jamis Buck have some great information on the
//! subject.
//!
//! ### Think Labyrinth!
//!
//! <a href="https://www.astrolog.org/labyrnth.htm"><img src="https://www.astrolog.org/labyrnth/labyrnth.gif"></a>
//!
//! Walter D. Pullen's [Think Labyrinth!](https://www.astrolog.org/labyrnth.htm) has a very
//! comprehensive reference on all aspects of mazes, including the various characteristics
//! of different algorithms and a glossary of maze-related terms.
//!
//! ### Buckblog & Mazes for Programmers
//!
//! <a href="http://www.mazesforprogrammers.com/"><img src="https://imagery.pragprog.com/products/439/jbmaze_largecover.jpg?1421259509"></a>
//!
//! For precise information about how maze generation algorithms actually work and how to
//! implement them yourself, Jamis Buck has some great blog entries on the subject.
//! They all come with visualisers you can run in the browser!
//! <https://weblog.jamisbuck.org/2011/2/7/maze-generation-algorithm-recap>.
//!
//! He has also written a full book on maze algorithms,
//! [Mazes for Programmers](https://pragprog.com/titles/jbmaze/mazes-for-programmers/).
//! It discusses generating mazes, solving mazes, mazes on cubes, spheres, cylinders,
//! and mobius strips, and more.
//! <http://www.mazesforprogrammers.com>.
//!
//! # Glossary
//!
//! [`interface`] defines some core concepts such as "cell" and "point". For all other jargon,
//! [Think Labyrinth!](http://www.astrolog.org/labyrnth.htm>) has a great glossary on technical
//! maze terms that you can find at <http://www.astrolog.org/labyrnth/glossary.htm>.

// Useful unstable features
#![feature(array_try_map)]
#![feature(min_specialization)]
#![feature(doc_auto_cfg)]
#![feature(rustdoc_missing_doc_code_examples)]

// Suspicious documentation
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::unescaped_backticks)]
#![deny(rustdoc::broken_intra_doc_links)]

// Stylistic choices
#![allow(clippy::needless_return, clippy::needless_range_loop, clippy::bool_comparison)]

pub mod interface;
pub mod implm;
mod path; pub use self::path::{PointPath, CellPath};
pub mod util;
pub(crate) mod internal;
#[cfg(test)] mod test;
