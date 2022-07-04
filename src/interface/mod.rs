// TODO surround the images in <figure>s and <figcaption>s
// TODO fix the alignment of all of the annotations in the figures
//! The core abstractions of our maze model.
//!
//! This crate splits up mazes into distinct parts, all at different levels of abstraction.
//! It is important to understand these, as these divisions are the foundation for the library
//! and will dictate how you call and interact with it.
//!
//! There are five core components that combined make up a maze:
//! 1. [Cells]
//! 2. [Points]
//! 3. [Coordinate Spaces]
//! 4. [Buffers]
//! 5. [Cell Managers]
//!
//! These components are all held together in one bundle by [Maze Carriers].
//!
//! There are also external components that interact with mazes but do not constitute them:
//! * [Renderers]
//! * [Generators]
//! * [Solvers]
//! * [Templates]
//!
//! This page explains the key concepts you will need to understand the library, so you should
//! read this top to bottom completely before reading the rest of the documentation.
//!
//! # Core Components
//!
//! The core components of a maze. It is the combination of all of these components together that
//! constitute a maze.
//!
//! ## Cell
//!
//! Mazes are fundamentally a collection of cells. A cell is the smallest indivisible unit
//! of a maze, like a passage segment or a wall segment.
//!
//! Cells are best understood visually. Consider this maze:
//!
//! ![An image of a pixelated maze.][example-maze-unannotated]
//!
//! Here are all the cells of the maze outlined in red:
//!
//! ![The same maze from before, but all the pixels are outlined in red.][example-maze-cell-outlines]
//!
//! Cells are too small to hold any semantic meaning useful enough for analysis. That job is
//! handled by points, which we'll come to next. Rather, *cells represent the physical structure
//! of the maze*. If you were to build the maze, cells tell you what materials you need to place
//! where --- "Do I need a piece of wall here, or a path?" (And we do indeed build the maze, using
//! renderers, which will also be discussed later).
//!
//! Cells are primarily defined and identified by way they connect to other cells. Each type of
//! cell is called a *cell class*. Each maze only has cells from a single class.
//!
//! Cells are not restricted to being square. They can be
//! [triangles](https://www.astrolog.org/labyrnth/maze/delta.gif),
//! [hexagons](https://www.astrolog.org/labyrnth/maze/sigma.gif),
//! [circular sectors](https://www.astrolog.org/labyrnth/maze/theta.gif),
//! [different shapes in the same maze](https://www.astrolog.org/labyrnth/maze/upsilon.gif),
//! [and worse](https://www.astrolog.org/labyrnth/maze/crack.gif).
//!
//! Cell classes are generally designed to be paired up with specific (classes of) coordinate
//! spaces.
//!
//! A cell has two components: a location and a value.
//!
//! ### Cell Location
//!
//! A cell's location defines its absolute position in the maze and serves as the cell's ID.
//! Each cell has a unique location. The structure (e.g. 2-tuple, 3-tuple, UUID) of a cell
//! location is specific to each cell class. The set of all legal cell locations in the context
//! of a maze is known as the maze's *cell space*. The size of the cell space is proportional
//! to the size of the maze's coordinate space (since the coordinate space must be mapped to the
//! cell space somehow).
//!
//! Cell locations are represented by the [`CellLocation`][self::cell::CellLocation] trait.
//! There is no trait for cell spaces - they are implicitly derived from a maze's
//! cell class-coordinate space pair.
//!
//! ### Cell Value
//!
//! A cell's value determines the [state][self::cell::ConnectionType] of its connections to
//! other cells. The values a cell can take on differ between different cell classes.
//!
//! Cell values are represented by the [`CellValue`][self::cell::CellValue] trait.
//!
//! ## Point
//!
//! A [point][self::point::Point] is a *potential junction* of a maze. This includes features
//! such as intersections, junctions, and straight hallways. Passages are simply two connected
//! points.
//!
//! Here are all the points annotated on the maze from before:
//!
//! ![The pixelated maze, wth a grid of blue dots overlaid.][example-maze-points]
//!
//! The key difference compared to cells is that points have *semantic meaning*. The relationship
//! between points is especially important. Moving from point to point is equivalent to moving from
//! junction to junction in a maze. They're the basis for analysing mazes (such as generating them
//! and solving them).
//!
//! It is easier to think about mazes in terms of points, both for humans and computers. For
//! humans, it is easier to consider junctions, intersections, and turns than it is to single
//! out specific cells. For computers, there can be far less cells than points, which makes
//! it easier (and faster) to compute things. For example, our example maze has 144 cells, but
//! only 16 points -- an order of magnitude less.
//!
//! You may notice that on the figure the entrances and exits are not marked as points. This is
//! because the maze was modified after creation to add gaps in the outer boundary to indicate
//! the entrance and exit. Since these are visual indicators for humans, they're only a part of
//! the cell space. For the purposes of analysis, when start and end points are required, the closest
//! points are used. In this case, they are the upper-left-most and bottom-right-most points
//! respectively.
//!
//! There does not need to exist a one-to-one mapping between points. A point might be mapped to
//! multiple cells, and a cell might not be mapped to a point at all. We'll come to how points
//! influence cells later.
//!
//! Points are represented by the [`Point`][self::point::Point] trait, though it's not particularly
//! useful on its own.
//!
//! ## Coordinate Space
//!
//! The [coordinate space][self::point::CoordinateSpace] sits at the highest level of abstraction.
//! They carry the size and shape of the maze, and know all possible semantic positions (points)
//! within.
//!
//! It is fundamentally a [graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)),
//! with points as its vertices. The edges between vertices represent *possible* passages between
//! points.
//!
//! When generating, solving, or analysing mazes, the coordinate space is used. It declares how
//! points are connected to one another -- the potential passages that could exist between junctions.
//!
//! Here, the turquoise lines between points represent the potential ways one could move between
//! points. Note that however not all of these potential routes end up as passages in the final maze.
//!
//! ![The pixelated maze with the grid of blue dots, but now with a grid of turquoise lines. The dots lie at all intersections of the lines.][example-maze-points-connected]
//!
//! While the coordinate space declares the connections between points, it does not carry the state
//! of the connection (e.g. if it's a passage or not). One way to think about it is that it describes
//! the maze as if there were no walls. Whether a connection is a passage or wall is determined by
//! the cells at the location.
//!
//! Coordinate spaces are represented by the [`CoordinateSpace`][self::point::CoordinateSpace] trait.
//!
//! ## Buffer
//!
//! The [buffer][self::buffer::MazeBuffer] handles storing mazes while they are being worked on.
//! They carry the raw data of the maze.
//!
//! They only store the mazes cells, indexed by their unique sequential IDs. They have no knowledge
//! of the higher-order structure of the maze.
//!
//! Typically buffers will just be something like a [`Vec`][crate::implm::buffer::VecBuffer], though
//! there's no requirement for this to be the case.
//!
//! Buffers are represented by the [`MazeBuffer`][self::buffer::MazeBuffer] trait.
//!
//! ## Cell Manager
//!
//! The [CellManager][self::cell::CellManager] manages the cells.
//!
//! Due to their role in handling conversions to and from cells, CellManager implementations
//! are often specific to only a specific combination of CoordinateSpace and type of cell
//! ([`CellValue`][self::cell::CellValue]).
//!
//! # Maze Carrier
//!
//! A [Maze Carrier][self::maze::Maze] holds the five core components of a maze together.
//! They are represented by the [`Maze`][self::maze::Maze] trait.
//!
//! They are used as references to the maze as a whole.
//!
//! # External Components
//!
//! ## Renderer
//!
//! ...
//!
//! ## Generator
//!
//! ...
//!
//! ## Solver
//!
//! ...
//!
//! ## Template
//!
//! ...
//!
//! [Cells]: #cell
//! [Points]: #points
//! [Coordinate Spaces]: #coordinate-space
//! [Buffers]: #buffer
//! [Cell Managers]: #cell-manager
//! [Maze Carriers]: #maze-carrier
//! [Renderers]: #renderer
//! [Generators]: #generator
//! [Solvers]: #solver
//! [Templates]: #template
#![doc = ::embed_doc_image::embed_image!("example-maze-unannotated", "src/doc/img/example-maze-unannotated.png")]
#![doc = ::embed_doc_image::embed_image!("example-maze-cell-outlines", "src/doc/img/example-maze-cell-outlines.png")]
#![doc = ::embed_doc_image::embed_image!("example-maze-points", "src/doc/img/example-maze-points.png")]
#![doc = ::embed_doc_image::embed_image!("example-maze-points-connected", "src/doc/img/example-maze-points-connected.png")]

pub mod buffer;
pub mod point;
pub mod cell;
pub mod maze;
pub mod render;
pub mod generate;