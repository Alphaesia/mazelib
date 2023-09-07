//! Rendering (exporting) mazes to other formats.
//!
//! # Recommended Reading
//! 1. [`MazeRenderer`] --- the rendering interface.

use std::io;
use std::io::Write;

use crate::interface::coordinator::MazeCoordinator;

/// Export a maze into another, usually persistent, format.
///
/// # Stability
///
/// For all implementations, unless explicitly documented otherwise,
/// *the exact details of the output should not be depended upon*. For example, the
/// [implementation that renders to text][crate::implm::render::text::BoxSpaceTextMazeRenderer]
/// may change the characters it outputs or the spacing. Generally you should not be perform any
/// operations on an exported maze that can be done before export. If for some reason you do need
/// the output to never change, just copy-paste the renderer into your own project
/// (license-permitting). Or just never update the library.
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
/// use mazelib::interface::render::MazeRenderer;
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
/// use mazelib::interface::render::MazeRenderer;
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
pub trait MazeRenderer<M: MazeCoordinator, O: Write> {
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
    fn render(&self, maze: &M, output: &mut O) -> io::Result<()>;
}

/// Simple sugar for [`MazeRenderer`]s.
///
/// Lets you elide constructing renderers when they implement [`Default`].
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
pub trait DefaultMazeRenderer<M: MazeCoordinator, O: Write>: MazeRenderer<M, O> {
    /// *See [`MazeRenderer::render()`].*
    fn render(maze: &M, output: &mut O) -> io::Result<()>;
}

impl <M: MazeCoordinator, O: Write, T: MazeRenderer<M, O> + Default> DefaultMazeRenderer<M, O> for T {
    fn render(maze: &M, output: &mut O) -> io::Result<()> {
        Self::default().render(maze, output)
    }
}