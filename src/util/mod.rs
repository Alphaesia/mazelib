//! A variety of basic helper functions provided for your convenience.

use crate::interface::cell::CellValue;
use crate::interface::coordinator::MazeCoordinator;
use crate::interface::point::CoordinateSpace;

pub use self::solid_border::apply_solid_border;

mod solid_border;

/// Convert all unvisited *points* (not cells) in a maze into wall cells.
///
/// ```text
/// for point in maze:
///     if point is unvisited:
///         maze.make_wall(point)
/// ```
pub fn convert_unvisited_points_to_walls(maze: &mut impl MazeCoordinator) {
    for pt in maze.coord_space().iter() {
        if maze.get(pt).is_fully_visited() == false {
            maze.make_wall(pt)
        }
    }
}