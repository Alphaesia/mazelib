//! Image-related export formats.
#![cfg(any(feature = "img", doc))]

use std::io::Write;

use image;
use image::ImageFormat;

use crate::interface::coordinate::MazeCoordinator;
use crate::interface::export::MazeExporter;

mod block;

/// Export a 2D maze into an image.
///
/// Image writing uses the [`image`] crate. Any format it supports this library supports.
pub trait ImageMazeExporter<M: MazeCoordinator, O: Write> : MazeExporter<M, O> {}

/// An [`ImageMazeExporter`] for mazes that
/// use [`BoxCoordinateSpace`][crate::implm::point::boxy::BoxCoordinateSpace]s.
///
/// Each instance only exports to a given image format, specified on construction.
pub struct BoxSpaceImageMazeExporter {
    format: ImageFormat,
}

impl BoxSpaceImageMazeExporter {
    /// Construct a new instance.
    ///
    /// # Parameters
    ///
    /// `format` --- the image format that mazes will be exported as. [`image`] must support
    ///              encoding in it ([`ImageFormat::can_write`] must return true).
    #[must_use]
    pub fn new(format: ImageFormat) -> Self {
        Self { format }
    }
}