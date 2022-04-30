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
/// # Implementing
///
/// If an implementation does not require a writer to support seeking, it
/// should implement [MazeRendererNonSeeking] instead. Implementing that
/// trait will automatically also implement this one.
pub trait MazeRenderer<CellSpace: CellManager> {
    fn render<Output: io::Write + io::Seek>(&self, maze: &CellSpace, output: &mut Output) -> io::Result<()>;
}

/// Like [MazeRenderer], but supports writers that do not support seeking.
///
/// See [MazeRenderer] for details on this trait.
pub trait MazeRendererNonSeeking<CellSpace: CellManager>: MazeRenderer<CellSpace> {
    fn render<Output: io::Write>(&self, maze: &CellSpace, output: &mut Output) -> io::Result<()>;
}

// Blanket implementation of MazeRenderer for all MazeRendererNonSeekings
impl <CellSpace: CellManager, T: MazeRendererNonSeeking<CellSpace>> MazeRenderer<CellSpace> for T {
    fn render<Output: io::Write + io::Seek>(&self, maze: &CellSpace, output: &mut Output) -> io::Result<()> {
        MazeRendererNonSeeking::<CellSpace>::render(self, maze, output)
    }
}