//! Rendering (exporting) mazes to other formats.
//!
//! # Recommended Reading
//! 1. [`MazeRenderer`] --- the main rendering interface.
//! 2. [`MazeRendererNonSeeking`] --- a commonly used variant of the rendering interface.

use std::io;
use crate::interface::cell::CellManager;

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
/// # use mazelib::implm::cell::inline::{BoxSpaceInlineCellManager, InlineCellValue};
/// #
/// let maze: BoxSpaceInlineCellManager<VecBuffer<InlineCellValue<2>>, 2> /* = ... */;
/// ```
///
/// Write a maze to stdout:
/// ```no_run
/// use std::io::BufWriter;
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::inline::{BoxSpaceInlineCellManager, InlineCellValue};
/// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
/// use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
/// use mazelib::interface::render::MazeRendererNonSeeking;
/// #
/// # let maze = BoxSpaceInlineCellManager::<VecBuffer<InlineCellValue<2>>, 2>::new(BoxCoordinateSpace::new([5, 5]));
///
/// // Buffer writes for performance
/// let mut output = BufWriter::new(std::io::stdout());
///
/// BoxSpaceTextMazeRenderer::new().render(&maze, &mut output);
/// ```
///
/// Write a maze to a file:
/// ```no_run
/// use std::fs::File;
/// use std::io::BufWriter;
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::inline::{BoxSpaceInlineCellManager, InlineCellValue};
/// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
/// use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
/// use mazelib::interface::render::MazeRendererNonSeeking;
/// #
/// # fn example() -> std::io::Result<()> {
/// #
/// # let maze = BoxSpaceInlineCellManager::<VecBuffer<InlineCellValue<2>>, 2>::new(BoxCoordinateSpace::new([5, 5]));
///
/// // Buffer writes for performance
/// let mut output = BufWriter::new(File::create("maze.txt")?);
///
/// BoxSpaceTextMazeRenderer::new().render(&maze, &mut output);
/// #
/// # return Ok(())
/// # }
/// #
/// # example();
/// ```
///
/// # Implementing
///
/// If an implementation does not require a writer to support seeking, it
/// should implement [`MazeRendererNonSeeking`] instead. Implementing that
/// trait will automatically also implement this one.
pub trait MazeRenderer<CellSpace: CellManager> {
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
    fn render<Output: io::Write + io::Seek>(&self, maze: &CellSpace, output: &mut Output) -> io::Result<()>;
}

/// Like [`MazeRenderer`], but supports writers that do not support seeking.
///
/// See [`MazeRenderer`] for details on this trait.
///
/// If you have a writer that requires this trait, make sure you import
/// this trait and not `MazeRenderer`.
pub trait MazeRendererNonSeeking<CellSpace: CellManager>: MazeRenderer<CellSpace> {
    fn render<Output: io::Write>(&self, maze: &CellSpace, output: &mut Output) -> io::Result<()>;
}

// Blanket implementation of MazeRenderer for all MazeRendererNonSeekings
impl <CellSpace: CellManager, T: MazeRendererNonSeeking<CellSpace>> MazeRenderer<CellSpace> for T {
    fn render<Output: io::Write + io::Seek>(&self, maze: &CellSpace, output: &mut Output) -> io::Result<()> {
        MazeRendererNonSeeking::<CellSpace>::render(self, maze, output)
    }
}