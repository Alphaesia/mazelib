// TODO fix the alignment of all of the annotations in the figures
//! The core abstractions of our maze model.
//!
//! There are a number of different concepts that all contribute to make a maze a maze. It is
//! important to understand them as they are the foundation for this library.
//!
//! Mazes are made up of: a [cell space](#cell), a [cell class](#cell),
//! a [coordinate space](#coordinate-space), a [buffer](#buffer), and a [coordinator](#coordinator)
//! to tie them all together. We'll go through them one-by-one.
//!
//! There are also other entities which, while separate from mazes, interact with them deeply.
//! Some examples include [generators](#generator), [solvers](#solver), and [exporters](#exporter).
//!
//! # Core Maze Concepts
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
//! exporters, which will also be discussed later).
//!
//! Cells are primarily defined by way they connect to other cells.
//! Cells are not restricted to being square. They can be
//! [triangles](https://www.astrolog.org/labyrnth/maze/delta.gif),
//! [hexagons](https://www.astrolog.org/labyrnth/maze/sigma.gif),
//! [circular sectors](https://www.astrolog.org/labyrnth/maze/theta.gif),
//! [multiple shapes](https://www.astrolog.org/labyrnth/maze/upsilon.gif), or
//! [completely irregular](https://www.astrolog.org/labyrnth/maze/crack.gif).
//! Each type of cell is called a *cell class*. Each maze uses a single class.
//!
//! Cell classes are generally designed to be paired up with specific (classes of) coordinate
//! spaces.
//!
//! A cell has three components: a location, an ordinal ID, and a value.
//!
//! ### Identifying Cells
//!
//! Cells can be identified in two ways: by their location, and by their ordinal ID. These two
//! forms of identification are inextricably linked.
//!
//! A cell location is a unique absolute position in the maze. The structure (e.g. 2-tuple, 3-tuple,
//! UUID) of a cell location is specific to each cell class. Cell locations are represented by the
//! [`CellLocation`][cell::CellLocation] trait. The set of all legal cell locations in the context
//! of a maze is known as the maze's *cell space*. The cell space is every position in a maze where
//! it is possible to physically exist. In a way, it is the fabric of reality. The cell space of a
//! maze (and the type of cell location used) is derived from the maze's coordinate space (loosely,
//! how big the maze is) and its cell class (how the cells connect). Because all the information
//! about a cell space can be derived, it is not explicitly represented anywhere in this library.
//!
//! Every cell in a maze is assigned an arbitrary ordinal integer (0, 1, 2, ...) called its
//! *cell id*. It is represented by the [`CellID`][cell::CellID] struct. How locations are mapped
//! to IDs is up to the [maze's coordinator (discussed later)](#coordinator). However, if the cell
//! space has an origin, that origin is always mapped to ID 0.
//!
//! Here's an example of how locations *could* be mapped to IDs:
//!
//! ![The red-outlined maze, but the cells are numbered from left-to-right top-to-bottom starting from zero. Cell 14 is highlighted in green.][example-maze-cell-outlines-with-annotated-ids]
//!
//! ### Cell Values
//!
//! A cell's value determines the [movement rules][cell::ConnectionType] between itself and its
//! neighbours. This may be as blunt as "this cell can be moved through" and "this cell can't", or
//! it may be more granular. The information carried by a cell (and thus the! set of possible
//! values) can differ between cell classes. Cell values are represented by the
//! [`CellValue`][cell::CellValue] trait.
//!
//! ## Point
//!
//! A [point][self::point::Point] is a *potential junction* of a maze. This includes features
//! such as intersections, junctions, and straight hallways. Passageways are simply two connected
//! points with passage cells between them. More generally, a point is a logical location in a maze.
//! They are represented by the [`Point`][point::Point] trait.
//!
//! Here are all the points annotated on the maze from before:
//!
//! ![The pixelated maze, wth a grid of blue dots overlaid.][example-maze-points]
//!
//! The key difference compared to cells is that relationship between points has *semantic meaning*.
//! Moving from one cell to another is like moving from one patch of dirt to another. Moving from
//! point to point is equivalent to moving from junction to junction in a maze. They're the basis
//! for doing anything interesting with mazes (like solving them).
//!
//! It is easier, for both humans and computers, to think about mazes in terms of points instead of
//! cells. For humans, it is easier to consider junctions, intersections, and turns than it is to
//! single out specific cells. For computers, there can be far less cells than points, which makes
//! it easier (and faster) to compute things. Our example maze has 144 cells, but only 16 points
//! --- an order of magnitude less.
//!
//! Unlike cells, which have both a location and a value, points are only a location. Instead points
//! are mapped to specific cells. The value of the point is the value of the cell(s) it maps to.
//! Though as discussed we're more interested in the relationship between points than the points
//! themselves. We look at the cells between and including two points. There does not need to exist
//! a one-to-one mapping between points and cells. A point might be mapped to multiple cells, and a
//! cell might not be mapped to a point at all. Precisely how they are mapped is up to the maze's
//! coordinator (which we'll come to soon).
//!
//! You may notice that on the figure the entrances and exits are not marked as points. This is an
//! example of how the cell space captures nuances of the physical layout that aren't relevant for
//! analysis. In this maze, if you wanted to find a path to the exit, you would just treat the point
//! nearest to the exit as the goal itself.
//!
//! ## Coordinate Space
//!
//! The [coordinate space][point::CoordinateSpace] is the topology the maze. It defines the set of
//! points (potential junctions) that exist in a maze, and how they connect. This dictates the size
//! and shape of the maze.
//!
//! Like how a maze's cell space is the set of all positions where it is possible to physically
//! exist, a maze's coordinate space is the set of all positions we care about (have semantic
//! meaning). The coordinate space lets you traverse the maze without getting bogged down in all
//! the details of "this passage is exactly twelve cells long, but this other passage is eleven".
//! When solving a maze, you just move from point to point until you find the "end point".
//!
//! Specifically, coordinate spaces are
//! [graphs](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)), with points as its
//! vertices, potential passages between those points as its edges, and a few other bits of
//! information (e.g. the origin). They are represented by the
//! [`CoordinateSpace`][point::CoordinateSpace] trait.
//!
//! Here, the turquoise lines between points represent the potential ways one could move between
//! points. Only some of these connections are passages and can be moved through. The others are
//! walls and impassable.
//!
//! ![The pixelated maze with the grid of blue dots, but now with a grid of turquoise lines. The dots lie at all intersections of the lines.][example-maze-points-connected]
//!
//! While the coordinate space defines the connections between points, it does not carry the state
//! of the connection (e.g. if it's a passage or not). That is determined by the cells at the
//! location. You could imagine it as if the coordinate space describes the maze as if there were
//! no walls.
//!
//! ## Buffer
//!
//! The [buffer][self::buffer::MazeBuffer] handles storing mazes while they are being worked on.
//! They carry the raw data of the maze.
//!
//! They only store the mazes cells, indexed by their unique consecutive IDs. They have no knowledge
//! of the higher-order structure of the maze. Typically buffers will just be something akin to a
//! [`Vec`][crate::implm::buffer::VecBuffer] wrapper but other more exotic buffers are possible.
//! Buffers are represented by the [`MazeBuffer`][buffer::MazeBuffer] trait.
//!
//! Buffers do not handle exporting mazes into more permanent forms. That is handled by
//! [exporters](#exporter).
//!
//! ## Coordinator
//!
//! So a maze has both a coordinate space and a cell space. But how do you map one to the other?
//! And if you want to make a passageway from point A to point B, which cells do you have to
//! change? This is the job of the coordinator. The coordinator knows how to convert high-level
//! queries (*"Is there a passageway from point A to B?"*) into low-level checks on specific cells.
//!
//! The coordinator is the glue that ties the maze together. It concretises the abstract coordinate
//! space into physical cells, and stores those cells in the buffer. The coordinator owns all of
//! the components of the maze and is effectively the maze itself. If you ever want to interact
//! with a maze directly, you will almost always do it through the coordinator.
//!
//! In practical terms, the coordinator is the most important concept for using this library. It is
//! represented by the [`MazeCoordinator`][coordinate::MazeCoordinator] trait.
//!
//! # External Concepts
//!
//! ## Generator
//!
//! Generators are responsible for creating the actual layouts of mazes. They turn blank canvases
//! into real challenges. Generators may operate on clean slates, or mazes that already have
//! some parts created. A human could draw the main structural features of the maze and let a
//! generator fill in the gaps. Or the generator could create the whole maze from start to finish.
//! 
//! No two generators are alike. Every generator has its own quirks and identity in the output it
//! produces. They also use a random-number source to ensure that every maze generated is unique.
//! 
//! Generators are represented by the [`MazeGenerator`][generate::MazeGenerator] trait.
//!
//! ## Solver
//!
//! TODO
//!
//! ## Exporter
//!
//! Once a maze has been generated / solved / manipulated / etc., you'll likely want to take it out
//! of memory and turn it into something you can look at. Exporters will take a maze and encode it
//! into a given format. There are no restrictions on what they can produce. It could be anywhere
//! from an ASCII diagram or image to a super-niche binary format for a particular game. Most
//! exporters though are only capable with dealing with with a small number of maze coordinators
//! due to restrictions inherent in the format. You're not exporting a 3D maze to a 2D image after
//! all. Exporters are represented by the [`MazeExporter`][export::MazeExporter] trait.
#![doc = embed_doc_image::embed_image!("example-maze-unannotated", "src/doc/img/maze-model/example-maze-unannotated.png")]
#![doc = embed_doc_image::embed_image!("example-maze-cell-outlines", "src/doc/img/maze-model/example-maze-cell-outlines.png")]
#![doc = embed_doc_image::embed_image!("example-maze-cell-outlines-with-annotated-ids", "src/doc/img/maze-model/example-maze-cell-outlines-with-annotated-ids.png")]
#![doc = embed_doc_image::embed_image!("example-maze-points", "src/doc/img/maze-model/example-maze-points.png")]
#![doc = embed_doc_image::embed_image!("example-maze-points-connected", "src/doc/img/maze-model/example-maze-points-connected.png")]

pub mod buffer;
pub mod cell;
pub mod point;
pub mod coordinate;
pub mod export;
pub mod generate;