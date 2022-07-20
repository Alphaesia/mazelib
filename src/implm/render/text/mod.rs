use crate::interface::maze::Maze;
use crate::interface::render::MazeRendererNonSeeking;
use crate::PointPath;

mod block;
mod inline;
mod line_break;

/// Render a two-dimensional maze (i.e. `BoxCoordinateSpace<2>`) into text and return it.
///
/// The output may contain any UTF-8 codepoint, not just ASCII.
#[derive(Debug)]
pub struct BoxSpaceTextMazeRenderer<M: Maze> {
    path: Option<PointPath<M::CoordSpace>>
}

pub trait TextMazeRenderer<M: Maze> : MazeRendererNonSeeking<M> {}

impl <M: Maze> BoxSpaceTextMazeRenderer<M> {
    pub fn new() -> Self {
        Self { path: None }
    }

    pub fn with_path(path: PointPath<M::CoordSpace>) -> Self {
        Self { path: Some(path) }
    }
}

impl <M: Maze> Default for BoxSpaceTextMazeRenderer<M> {
    fn default() -> Self {
        Self::new()
    }
}