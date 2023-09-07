//! Rendering (exporting) mazes to other formats.
//!
//! # Recommended Reading
//! 1. [`MazeRenderer`] --- the main rendering interface.
//! 2. [`MazeRendererNonSeeking`] --- a commonly used variant of the rendering interface.

use std::io;
use std::io::{Seek, Write};

use crate::interface::coordinator::MazeCoordinator;

/// Render (export) a maze into another format.
///
/// Render a maze into another, usually non-volatile format. Each
/// implementation of this trait will render mazes into different formats.
///
/// # Stability
///
/// For all implementations, unless explicitly documented otherwise,
/// *the exact formatting of the render should not be depended upon*.
/// For example, the [implementation that renders to text][crate::implm::render::text::BoxSpaceTextMazeRenderer]
/// may change the characters it outputs or the spacing.
///
/// Generally you should not be perform any operations on a rendered maze that
/// can be done before rendering.
///
/// # Examples
///
/// Assuming you already have a maze:
/// ```
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::inline::InlineCellValue;
/// # use mazelib::implm::coordinator::inline::BoxSpaceInlineCellMazeCoordinator;
/// #
/// let maze: BoxSpaceInlineCellMazeCoordinator<VecBuffer<InlineCellValue<2>>, 2> /* = ... */;
/// ```
///
/// Write a maze to stdout:
/// ```no_run
/// use std::io::BufWriter;
/// # use std::io::Result;
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::inline::InlineCellValue;
/// # use mazelib::implm::coordinator::inline::BoxSpaceInlineCellMazeCoordinatorBuilder;
/// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
/// use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
/// use mazelib::interface::render::MazeRendererNonSeeking;
/// #
/// # fn test() -> Result<()> {
/// #
/// # let maze = BoxSpaceInlineCellMazeCoordinatorBuilder::<VecBuffer<InlineCellValue<2>>, 2>::new(BoxCoordinateSpace::new_checked([5, 5])).build();
///
/// // Buffer writes for performance
/// let mut output = BufWriter::new(std::io::stdout());
///
/// BoxSpaceTextMazeRenderer::new().render(&maze, &mut output)
/// # }
/// ```
///
/// Write a maze to a file:
/// ```no_run
/// use std::fs::File;
/// use std::io::BufWriter;
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::inline::InlineCellValue;
/// # use mazelib::implm::coordinator::inline::BoxSpaceInlineCellMazeCoordinatorBuilder;
/// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
/// use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
/// use mazelib::interface::render::MazeRendererNonSeeking;
/// #
/// # fn example() -> std::io::Result<()> {
/// #
/// # let maze = BoxSpaceInlineCellMazeCoordinatorBuilder::<VecBuffer<InlineCellValue<2>>, 2>::new(BoxCoordinateSpace::new_checked([5, 5])).build();
///
/// // Buffer writes for performance
/// let mut output = BufWriter::new(File::create("maze.txt")?);
///
/// BoxSpaceTextMazeRenderer::new().render(&maze, &mut output)?;
/// #
/// # return Ok(());
/// # }
/// ```
///
/// # Implementing
///
/// If an implementation does not require a writer to support seeking, it
/// should implement [`MazeRendererNonSeeking`] instead. Implementing that
/// trait will also automatically implement this one.
pub trait MazeRenderer<M: MazeCoordinator> {
    /// Render the maze `maze` to writer `output`.
    ///
    /// The format the maze is rendered to (and thus the type of data that
    /// is written to `output`) is defined by and dependent on the
    /// implementation.
    ///
    /// Returns an IO error if one was encountered while writing to `output`.
    ///
    /// # Usage Notes
    ///
    /// It is recommended to wrap `output` in a [`BufWriter`][std::io::BufWriter]
    /// if it not already buffered.
    fn render<Output: Write + Seek>(&self, maze: &M, output: &mut Output) -> io::Result<()>;
}

/// Like [`MazeRenderer`], but supports writers that do not support seeking.
///
/// See [`MazeRenderer`] for details on this trait.
///
/// If you have a writer that requires this trait, make sure you import
/// this trait and not `MazeRenderer`.
pub trait MazeRendererNonSeeking<M: MazeCoordinator>: MazeRenderer<M> {
    fn render<Output: Write>(&self, maze: &M, output: &mut Output) -> io::Result<()>;
}

// Blanket implementation of MazeRenderer for all MazeRendererNonSeeking-s
impl <M: MazeCoordinator, T: MazeRendererNonSeeking<M>> MazeRenderer<M> for T {
    fn render<Output: Write + Seek>(&self, maze: &M, output: &mut Output) -> io::Result<()> {
        MazeRendererNonSeeking::<M>::render(self, maze, output)
    }
}

/// Simple sugar for [`MazeRenderer`]s.
///
/// Lets you elide constructing renderers with parameterless constructors (specifically,
/// renderers that implement [`Default`]).
///
/// ```no_run
/// # use std::fs::File;
/// # use std::path::Path;
/// # use mazelib::interface::generate::MazeGenerator;
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::block::BlockCellValue;
/// # use mazelib::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinatorBuilder;
/// # use mazelib::implm::generate::HuntAndKillGenerator;
/// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
/// # use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
/// # use mazelib::interface::render::MazeRenderer;
/// #
/// # fn test() -> std::io::Result<()> {
/// #
/// # let mut maze = BoxSpaceBlockCellMazeCoordinatorBuilder::<VecBuffer<BlockCellValue>, 2>::new(BoxCoordinateSpace::new_checked([1, 1])).build();
/// # let mut output = File::create(Path::new("")).unwrap();
/// #
/// BoxSpaceTextMazeRenderer::new().render(&mut maze, &mut output)
/// # }
/// ```
/// becomes
/// ```no_run
/// # use std::fs::File;
/// # use std::path::Path;
/// # use mazelib::interface::generate::MazeGenerator;
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::block::BlockCellValue;
/// # use mazelib::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinatorBuilder;
/// # use mazelib::implm::generate::HuntAndKillGenerator;
/// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
/// # use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
/// # use mazelib::interface::render::DefaultMazeRenderer;
/// #
/// # fn test() -> std::io::Result<()> {
/// #
/// # let mut maze = BoxSpaceBlockCellMazeCoordinatorBuilder::<VecBuffer<BlockCellValue>, 2>::new(BoxCoordinateSpace::new_checked([1, 1])).build();
/// # let mut output = File::create(Path::new("")).unwrap();
/// #
/// BoxSpaceTextMazeRenderer::render(&mut maze, &mut output)
/// # }
/// ```
pub trait DefaultMazeRenderer<M: MazeCoordinator>: MazeRenderer<M> {
    /// *See [`MazeRenderer::render()`].*
    fn render<Output: Write + Seek>(maze: &M, output: &mut Output) -> io::Result<()>;
}

impl <M: MazeCoordinator, T: MazeRenderer<M> + Default> DefaultMazeRenderer<M> for T {
    fn render<Output: Write + Seek>(maze: &M, output: &mut Output) -> io::Result<()> {
        Self::default().render(maze, output)
    }
}

/// Simple sugar for [`MazeRendererNonSeeking`]s.
///
/// Lets you elide constructing renderers with parameterless constructors (specifically,
/// renderers that implement [`Default`]).
///
/// ```no_run
/// # use std::fs::File;
/// # use std::path::Path;
/// # use mazelib::interface::generate::MazeGenerator;
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::block::BlockCellValue;
/// # use mazelib::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinatorBuilder;
/// # use mazelib::implm::generate::HuntAndKillGenerator;
/// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
/// # use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
/// # use mazelib::interface::render::MazeRendererNonSeeking;
/// #
/// # fn test() -> std::io::Result<()> {
/// #
/// # let mut maze = BoxSpaceBlockCellMazeCoordinatorBuilder::<VecBuffer<BlockCellValue>, 2>::new(BoxCoordinateSpace::new_checked([1, 1])).build();
/// # let mut output = File::create(Path::new("")).unwrap();
/// #
/// BoxSpaceTextMazeRenderer::new().render(&mut maze, &mut output)
/// # }
/// ```
/// becomes
/// ```no_run
/// # use std::fs::File;
/// # use std::path::Path;
/// # use mazelib::interface::generate::MazeGenerator;
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::block::BlockCellValue;
/// # use mazelib::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinatorBuilder;
/// # use mazelib::implm::generate::HuntAndKillGenerator;
/// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
/// # use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
/// # use mazelib::interface::render::DefaultMazeRendererNonSeeking;
/// #
/// # fn test() -> std::io::Result<()> {
/// #
/// # let mut maze = BoxSpaceBlockCellMazeCoordinatorBuilder::<VecBuffer<BlockCellValue>, 2>::new(BoxCoordinateSpace::new_checked([1, 1])).build();
/// # let mut output = File::create(Path::new("")).unwrap();
/// #
/// BoxSpaceTextMazeRenderer::render(&mut maze, &mut output)
/// # }
/// ```
pub trait DefaultMazeRendererNonSeeking<M: MazeCoordinator>: MazeRendererNonSeeking<M> {
    /// *See [`MazeRenderer::render()`].*
    fn render<Output: Write>(maze: &M, output: &mut Output) -> io::Result<()>;
}

impl <M: MazeCoordinator, T: MazeRendererNonSeeking<M> + Default> DefaultMazeRendererNonSeeking<M> for T {
    fn render<Output: Write>(maze: &M, output: &mut Output) -> io::Result<()> {
        MazeRendererNonSeeking::<M>::render(&Self::default(), maze, output)
    }
}