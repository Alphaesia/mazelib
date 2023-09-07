use std::io::Write;
use crate::interface::coordinator::MazeCoordinator;
use crate::interface::render::MazeRenderer;

mod block;
mod inline;
mod line_break;

/// Render a two-dimensional maze (i.e. `BoxCoordinateSpace<2>`) into text and return it.
///
/// The output may contain any UTF-8 codepoint, not just ASCII.
pub struct BoxSpaceTextMazeRenderer {
    _private: ()
}

pub trait TextMazeRenderer<M: MazeCoordinator, O: Write> : MazeRenderer<M, O> {}

impl BoxSpaceTextMazeRenderer {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for BoxSpaceTextMazeRenderer {
    fn default() -> Self {
        Self::new()
    }
}