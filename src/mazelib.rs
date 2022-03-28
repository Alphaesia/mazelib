//! Mazes are inherently complex. However, most programs give you very limited options
//! in dealing with mazes. Whether being restricted to 2D square mazes, or only supporting
//! one type of output. This library aims to unlock and expose mazes fully and completely,
//! allowing you to create, manipulate, and solve mazes however you desire.
//!
//! # Quick Start
//!
//! For those who want to jump straight into playing around. Do come back and read the docs
//! though!
//!
//! ```
//! # use mazelib::implm::buffer::VecBuffer;
//! # use mazelib::implm::cell::block::{BlockCellValue, BoxSpaceBlockCellManagerBuilder};
//! # use mazelib::implm::cell::inline::{BoxSpaceInlineCellManager, InlineCellValue};
//! # use mazelib::implm::generate::HuntAndKillGenerator;
//! # use mazelib::implm::point::boxy::BoxCoordinateSpace;
//! # use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
//! # use mazelib::implm::template::boxy::SolidBorderTemplate;
//! # use mazelib::interface::render::MazeRenderer;
//! # use mazelib::interface::template::Template;
//! #
//! // 2 for a 2D maze, 3 for a 3D maze, etc.
//! const DIMENSION: usize = 2;
//!
//! let [width, height] = [5, 5];
//!
//! // Determine the size of the maze
//! let coord_space = BoxCoordinateSpace::new([width, height]);
//!
//! type Buffer = VecBuffer<BlockCellValue>;
//!
//! // Create the maze "object"
//! // The "BoxSpace" in BoxSpaceBlockCellManagerBuilder lines up with our BoxCoordinateSpace
//! // "BlockCell" means that this CellManager will use BlockCells
//! let mut maze = BoxSpaceBlockCellManagerBuilder::<Buffer, DIMENSION>::new(coord_space).build();
//!
//! // Give our maze a nice thick border
//! SolidBorderTemplate::apply(&mut maze);
//!
//! // Generate the maze
//! HuntAndKillGenerator::generate(&mut maze);
//!
//! // Print the maze as text
//! let maze_as_text = BoxSpaceTextMazeRenderer::render(&maze);
//! for line in maze_as_text {
//!     println!("{}", line)
//! }
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
//! For example, changing `BlockCellValue` to `InlineCellValue<DIMENSION>` will change how the
//! cells look. (Note you wll have to replace `BoxSpaceBlockCellManagerBuilder` with
//! `BoxSpaceInlineCellManager` and remove the trailing `.build()`).
//!
//! ```
//! # use mazelib::implm::buffer::VecBuffer;
//! # use mazelib::implm::cell::inline::{BoxSpaceInlineCellManager, InlineCellValue};
//! # use mazelib::implm::point::boxy::BoxCoordinateSpace;
//! #
//! # const DIMENSION: usize = 2;
//! # let coord_space = BoxCoordinateSpace::new([1, 1]);
//! #
//! type Buffer = VecBuffer<InlineCellValue<DIMENSION>>;
//!
//! let mut maze = BoxSpaceInlineCellManager::<Buffer, DIMENSION>::new(coord_space);
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
//! [crate::interface] holds the key abstractions and defines the model of mazes the library
//! uses. It's very important.
//!
//! [crate::implm] has a bunch of built-in implementations of the aforementioned
//! interfaces for your convenience. While you're probably want to use them in most cases,
//! you're not obligated to use all (or any) of them. Feel free to implement any of the
//! interfaces yourself!
//!
//! # Reading Order
//!
//! Firstly, read the previous section in this page on the crate [architecture](#architecture).
//!
//! Then, you should read the module-level docs of [crate::interface]. It explains all of the
//! key interfaces and their role in our maze model.
//!
//! After that, you'll probably want to check out [crate::interface::cell::CellManager].
//! This is the central trait which all of the machinery interacts with, and is the main
//! interface between you and the maze. The documentation for
//! [crate::interface::generate::MazeGenerator] will likely be of use. The details of the
//! other interfaces are not too important unless you're implementing an interface.
//!
//! Lastly, explore [crate::implm]. It contains standard implementations of all of the
//! interfaces. You'll almost certainly want to use some of them, so have a look around.
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
//!
//! <https://weblog.jamisbuck.org/2011/2/7/maze-generation-algorithm-recap>.
//!
//! He has also written a full book on maze algorithms,
//! [Mazes for Programmers](https://pragprog.com/titles/jbmaze/mazes-for-programmers/).
//!
//! It discusses generating mazes, solving mazes, mazes on cubes, spheres, cylinders,
//! and mobius strips, and more.
//!
//! <http://www.mazesforprogrammers.com>.
//!
//! # Glossary
//!
//! [crate::implm] defines some core concepts such as "cell" and "point". For all other jargon,
//! [Think Labyrinth!](http://www.astrolog.org/labyrnth.htm>) has a great glossary on technical
//! maze terms that you can find at <http://www.astrolog.org/labyrnth/glossary.htm>.

#![feature(array_zip)]
#![feature(min_specialization)]
#![feature(label_break_value)]

#![deny(rustdoc::broken_intra_doc_links)]

#![allow(clippy::needless_return, clippy::bool_comparison)]  // Stylistic choices

pub mod interface;
pub mod implm;
pub(crate) mod internal;
#[cfg(test)] mod test;