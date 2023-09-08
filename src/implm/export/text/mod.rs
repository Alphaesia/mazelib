//! Text-based export formats.

use std::io::Write;
use crate::interface::coordinator::MazeCoordinator;
use crate::interface::export::MazeExporter;

mod block;
mod inline;
mod line_break;

/// Export a 2D maze to text.
///
/// The output may contain any UTF-8 codepoint, not just ASCII.
pub trait TextMazeExporter<M: MazeCoordinator, O: Write> : MazeExporter<M, O> {}

/// A [`TextMazeExporter`] for mazes that
/// use [`BoxCoordinateSpace`][crate::implm::point::boxy::BoxCoordinateSpace]s.
pub struct BoxSpaceTextMazeExporter {
    _private: ()
}

impl BoxSpaceTextMazeExporter {
    /// Construct a new instance.
    ///
    /// Optional, see [`DefaultMazeExporter`][crate::interface::export::DefaultMazeExporter].
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for BoxSpaceTextMazeExporter {
    fn default() -> Self {
        Self::new()
    }
}