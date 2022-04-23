use crate::interface::cell::CellManager;
use crate::interface::render::MazeRenderer;
use image::RgbaImage;

mod block;

/// Render a two-dimensional maze (i.e. `BoxCoordinateSpace<2>`) into an image and return it.
///
/// Notes:
/// * Exact output may change and should not be depended upon.
pub struct BoxSpaceImageMazeRenderer {
    _private: ()
}

impl BoxSpaceImageMazeRenderer {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

pub trait ImageMazeRenderer<CellSpace: CellManager> : MazeRenderer<CellSpace, Output=RgbaImage> {}