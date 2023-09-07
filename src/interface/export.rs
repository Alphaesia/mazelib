//! Exporting mazes to other formats.
//!
//! # Recommended Reading
//! 1. [`MazeExporter`] --- the exporter interface.

use std::io;
use std::io::Write;

use crate::interface::coordinator::MazeCoordinator;

/// Export a maze into another, usually persistent, format.
///
/// [Maze coordinator][crate::interface::coordinator::MazeCoordinator]s are only useful within the
/// confines of this library. If you want to visualise your handiwork you'll want to export it to
/// something you can [see][crate::implm::export::img::ImageMazeExporter]. If you want to use
/// the maze in another program, you'll want to write your own exporter that exports the maze to
/// that program's datastructures.
///
/// Exporters are wild and varied in their produce. What an exporter produces is completely up to
/// them so make sure to consult your chosen implementation's documentation.
///
/// # Stability
///
/// For all implementations, unless explicitly documented otherwise,
/// *the exact details of the output should not be depended upon*. For example, the
/// [implementation that exports to text][crate::implm::export::text::BoxSpaceTextMazeExporter]
/// may change the characters it outputs or the spacing. Generally you should not be perform any
/// operations on an exported maze that can be done before export. If for some reason you do need
/// the output to never change, just copy-paste the exporter into your own project
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
/// use mazelib::implm::export::text::BoxSpaceTextMazeExporter;
/// use mazelib::interface::export::MazeExporter;
/// #
/// # fn test() -> Result<()> {
/// #
/// # let maze = BoxSpaceInlineCellMazeCoordinatorBuilder::<VecBuffer<InlineCellValue<2>>, 2>::new(BoxCoordinateSpace::new_checked([5, 5])).build();
///
/// // Buffer writes for performance
/// let mut output = BufWriter::new(std::io::stdout());
///
/// BoxSpaceTextMazeExporter::new().export(&maze, &mut output)
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
/// use mazelib::implm::export::text::BoxSpaceTextMazeExporter;
/// use mazelib::interface::export::MazeExporter;
/// #
/// # fn example() -> std::io::Result<()> {
/// #
/// # let maze = BoxSpaceInlineCellMazeCoordinatorBuilder::<VecBuffer<InlineCellValue<2>>, 2>::new(BoxCoordinateSpace::new_checked([5, 5])).build();
///
/// // Buffer writes for performance
/// let mut output = BufWriter::new(File::create("maze.txt")?);
///
/// BoxSpaceTextMazeExporter::new().export(&maze, &mut output)?;
/// #
/// # return Ok(());
/// # }
/// ```
pub trait MazeExporter<M: MazeCoordinator, O: Write> {
    /// Export the maze `maze` to writer `output`.
    ///
    /// The format the maze is exported to (and thus the type of data that
    /// is written to `output`) is defined by and dependent on the
    /// implementation.
    ///
    /// Returns an IO error if one was encountered while writing to `output`.
    ///
    /// # Usage Notes
    ///
    /// It is recommended to wrap `output` in a [`BufWriter`][std::io::BufWriter]
    /// if it not already buffered.
    fn export(&self, maze: &M, output: &mut O) -> io::Result<()>;
}

/// Simple sugar for [`MazeExporter`]s.
///
/// Lets you elide constructing exporters when they implement [`Default`].
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
/// # use mazelib::implm::export::text::BoxSpaceTextMazeExporter;
/// # use mazelib::interface::export::MazeExporter;
/// #
/// # fn test() -> std::io::Result<()> {
/// #
/// # let mut maze = BoxSpaceBlockCellMazeCoordinatorBuilder::<VecBuffer<BlockCellValue>, 2>::new(BoxCoordinateSpace::new_checked([1, 1])).build();
/// # let mut output = File::create(Path::new("")).unwrap();
/// #
/// BoxSpaceTextMazeExporter::new().export(&mut maze, &mut output)
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
/// # use mazelib::implm::export::text::BoxSpaceTextMazeExporter;
/// # use mazelib::interface::export::DefaultMazeExporter;
/// #
/// # fn test() -> std::io::Result<()> {
/// #
/// # let mut maze = BoxSpaceBlockCellMazeCoordinatorBuilder::<VecBuffer<BlockCellValue>, 2>::new(BoxCoordinateSpace::new_checked([1, 1])).build();
/// # let mut output = File::create(Path::new("")).unwrap();
/// #
/// BoxSpaceTextMazeExporter::export(&mut maze, &mut output)
/// # }
/// ```
pub trait DefaultMazeExporter<M: MazeCoordinator, O: Write>: MazeExporter<M, O> {
    /// *See [`MazeExporter::export()`].*
    fn export(maze: &M, output: &mut O) -> io::Result<()>;
}

impl <M: MazeCoordinator, O: Write, T: MazeExporter<M, O> + Default> DefaultMazeExporter<M, O> for T {
    fn export(maze: &M, output: &mut O) -> io::Result<()> {
        Self::default().export(maze, output)
    }
}