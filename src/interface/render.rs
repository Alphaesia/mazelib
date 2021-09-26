use crate::interface::point::CoordinateSpace;
use crate::interface::cell::CellManager;

pub trait MazeRenderer<CellSpace: CellManager> {
    type Output;

    fn render(maze: &CellSpace) -> Self::Output;
}

pub trait MazeRendererWithMarker<CellSpace: CellManager> : MazeRenderer<CellSpace> {
    fn render_with_marker(maze: &CellSpace, marker: <<CellSpace as CellManager>::CoordSpace as CoordinateSpace>::PtType) -> Self::Output;
}