use crate::interface::cell::CellManager;
use crate::interface::render::MazeRenderer;

mod block;
mod inline;

/// Render a two-dimensional maze (i.e. `BoxCoordinateSpace<2>`) into text and return it.
///
/// Notes:
/// * Text may contain Unicode characters (not just ASCII).
/// * Exact output may change and should not be depended upon.
pub struct BoxSpaceTextMazeRenderer {
    _private: ()
}

impl BoxSpaceTextMazeRenderer {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

pub trait TextMazeRenderer<CellSpace: CellManager> : MazeRenderer<CellSpace, Output=Vec<String>> {}