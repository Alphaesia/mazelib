// TODO surround the images in <figure>s and <figcaption>s
// TODO fix the alignment of all of the annotations in the figures
//! The core abstractions of our maze model.
//!
//! This crate splits up mazes into six distinct parts, all at different levels of abstraction.
//! It is important to understand these, as these divisions are the foundation for the library
//! and will dictate how you call and interact with it.
//!
//! This page explains the key concepts you will need to understand the library, so you should read
//! this top to bottom completely before reading the rest of the documentation.
//!
//! # Cell
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
//! Cells are too small to hold any semantic meaning useful enough for analysis. This job is
//! handled by points, which we'll come to next. Rather, *cells represent the physical structure
//! of the maze*. If you were to build the maze, cells tell you what materials you need to place
//! where --- "Do I need a piece of wall here, or a path?" (And we do indeed build the maze, using
//! Renderers, which will also be discussed later).
//!
//! Cells do not have to be squares. They can be
//! [triangles](https://www.astrolog.org/labyrnth/maze/delta.gif),
//! [hexagons](https://www.astrolog.org/labyrnth/maze/sigma.gif),
//! [circular sectors](https://www.astrolog.org/labyrnth/maze/theta.gif),
//! [different shapes in the same maze](https://www.astrolog.org/labyrnth/maze/upsilon.gif),
//! [and worse](https://www.astrolog.org/labyrnth/maze/crack.gif). However square cells are the
//! easiest to reason about and deal with programmatically, so we'll stick to them for our examples.
//!
//! In this library, a cell is represented by two components: a CellLocation and a CellValue.
//! A [CellLocation][self::cell::CellLocation] represents a cell as a position.
//! A [CellValue][self::cell::CellValue] represents the actual contents of the cell, such
//! as a passage segment or wall segment. CellLocations do not hold any information about the
//! values of cells. Instead you query their value from CellManagers (which we will also get to
//! later).
//!
//! # Point
//!
//! A [Point][self::point::Point] is a *potential junction* of a maze. This includes features
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
//! the . For the purposes of analysis, when start and end points are required, the closest points
//! are used. In this case, they are the upper-left-most and bottom-right-most points respectively.
//!
//! There does not need to exist a one-to-one mapping between points. A point might be mapped to
//! multiple cells, and a cell might not be mapped to a point at all. We'll come to how points
//! influence cells later.
//!
//! # Coordinate Space and Point
//!
//! The [CoordinateSpace][self::point::CoordinateSpace] sits at the highest level of abstraction.
//! They carry the size and shape of the maze, and know all possible positions
//! ([Point][self::point::Point]s) within.
//!
//! It is fundamentally a graph, with each [Point][self::point::Point] (vertex) on the graph
//! representing a "room" in a maze, or a potential intersection of passages. When thinking
//! about generating or solving mazes, [Point][self::point::Point]s are the basic logical unit.
//! The [CoordinateSpace][self::point::CoordinateSpace] dictates how [Point][self::point::Point]s
//! are connected to one another - the potential passages between rooms in a maze.
//!
//! ![The pixelated maze with the grid of blue dots, but now with a grid of turquoise lines. The dots lie at all intersections of the lines.][example-maze-points-connected]
//!
//! [Maze generators][self::generate::MazeGenerator] and solvers also operate at this level.
//!
//! Note that the [CoordinateSpace][self::point::CoordinateSpace] does not carry any information
//! about the placement of walls and passages. Rather, it describes the maze as if there were
//! no walls. Passage and wall placement is handled at other levels of the abstraction stack.
//!
//! If you think about mazes as
//! [mathematical graphs](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)), points
//! are nodes in the graph (vertices).
//!
//! # Buffer
//!
//! The [Buffer][self::buffer::MazeBuffer] handles storing mazes while they are being worked on.
//! They carry the raw data of the maze. They only know cells by their unique sequential IDs
//! ([BufferLocation][self::buffer::BufferLocation]), and have no knowledge of the higher-order
//! structure of the maze.
//!
//! # Cell Manager
//!
//! The [CellManager][self::cell::CellManager] manages the cells.
//!
//! Due to their role in handling conversions to and from cells, CellManager implementations
//! are often specific to only a specific combination of CoordinateSpace and type of cell
//! ([CellValue][self::cell::CellValue]).
//!
//! # Renderer
//!
//! ...
#![doc = ::embed_doc_image::embed_image!("example-maze-unannotated", "src/doc/img/example-maze-unannotated.png")]
#![doc = ::embed_doc_image::embed_image!("example-maze-cell-outlines", "src/doc/img/example-maze-cell-outlines.png")]
#![doc = ::embed_doc_image::embed_image!("example-maze-points", "src/doc/img/example-maze-points.png")]
#![doc = ::embed_doc_image::embed_image!("example-maze-points-connected", "src/doc/img/example-maze-points-connected.png")]

pub mod buffer;
pub mod point;
pub mod cell;
pub mod render;
pub mod template;
pub mod generate;