#![cfg(any(feature = "img", doc))]

use image::ImageFormat;

use crate::interface::maze::Maze;
use crate::interface::render::MazeRenderer;

mod block;

/// Render a two-dimensional maze (i.e. `BoxCoordinateSpace<2>`) into an image and return it.
pub struct BoxSpaceImageMazeRenderer {
    format: ImageFormat,
}

impl BoxSpaceImageMazeRenderer {
    pub fn new(format: ImageFormat) -> Self {
        Self { format }
    }
}

pub trait ImageMazeRenderer<M: Maze> : MazeRenderer<M> {}