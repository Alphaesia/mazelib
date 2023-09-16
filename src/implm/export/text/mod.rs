//! Text-based export formats.

use std::io::Write;

use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::interface::coordinate::MazeCoordinator;
use crate::interface::export::MazeExporter;

pub use self::block::{BoxSpaceBlockCellTextMazeExporter, BoxSpaceBlockCellTextMazeExporterBuilder};
pub use self::inline::{BoxSpaceInlineCellTextMazeExporter, BoxSpaceInlineCellTextMazeExporterBuilder};

mod block;
mod inline;

/// Export a 2D maze to text.
///
/// The output may contain any UTF-8 codepoint, not just ASCII.
pub trait TextMazeExporter<M: MazeCoordinator, O: Write> : MazeExporter<M, O> {}

/// A [`TextMazeExporter`] for 2D mazes that use [`BoxCoordinateSpace`]s.
pub trait BoxSpaceTextMazeExporter<M: MazeCoordinator<CoordSpace=BoxCoordinateSpace<2>>, O: Write> : TextMazeExporter<M, O> {}