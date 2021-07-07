use std::fmt::Debug;
use crate::geometry::space::CoordinateSpace;
use crate::cell::data::CellData;

// Cleaner type signatures TM
macro_rules! pt_type {
    () => { <<Self as LogicalPointMapper>::CoordSpace as CoordinateSpace>::PtType };
}

/// Maps a coordinate space onto a cell space.
/// It determines what cell(s) a logical point maps to.
/// It does not mutate the buffer directly itself, rather it delegates that to
/// a [crate::cell::space::CellSpace].
pub trait SpaceMapper: Send + Sync + Debug {
    type CoordSpace: CoordinateSpace;
    type CellType: CellData;

    /*
     * Getters
     */

    /// Get the value of a given point
    fn get_pt(&self, pt: pt_type!()) -> Self::CellType;

    /// Helper function - equivalent to ``get_pt(pt).is_passage()``
    fn is_passage(&self, pt: pt_type!()) -> bool {
        self.get_pt(pt).is_passage()
    }

    /// Helper function - equivalent to ``get_pt(pt).is_wall()``
    fn is_wall(&self, pt: pt_type!()) -> bool {
        self.get_pt(pt).is_wall()
    }

    /// Helper function - equivalent to ``get_pt(pt).Helper()``
    fn is_boundary(&self, pt: pt_type!()) -> bool {
        self.get_pt(pt).is_boundary()
    }

    /// Helper function - equivalent to ``get_pt(pt).is_unvisited()``
    fn is_unvisited(&self, pt: pt_type!()) -> bool {
        self.get_pt(pt).is_unvisited()
    }

    /*
     * Setters
     */

    fn make_passage(&mut self, pt: pt_type!());

    /// Note: from and to must be adjacent
    fn make_passage_between(&mut self, from: pt_type!(), to: pt_type!());

    fn make_wall(&mut self, pt: pt_type!());

    /// Note: from and to must be adjacent
    fn make_wall_between(&mut self, from: pt_type!(), to: pt_type!());

    fn make_boundary(&mut self, pt: pt_type!());

    /// Note: from and to must be adjacent
    fn make_boundary_between(&mut self, from: pt_type!(), to: pt_type!());

    /*
     * Other
     */

    /// The maximum number of physical cells required (e.g. including passage tiles between intersections)
    fn cells_required(space: &Self::CoordSpace) -> usize;
}

/// A LogicalPointMapper that treats walls and passages as separate cells.
/// Only applies to [crate::geometry::space::BoxCoordinateSpace]s.
///
/// As an analogy, imagine a black-and-white pixel image of a self.maze,
/// where all white pixels are passages and all black pixels are
/// walls. That is what a BitmapMaze is.
///
/// If you have a better name please let me know.
pub trait BitmapMaze<CoordSpace, const DIMENSION: usize>: SpaceMapper where
    CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
    <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
    <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
    <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]> {
    /// When converting points to cells, how much to scale distances in each axis.
    fn scale() -> [usize; DIMENSION];

    // TODO need better name than "extended"
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_passage_between(&mut self, from: pt_type!(), to: pt_type!());
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_wall_between(&mut self, from: pt_type!(), to: pt_type!());
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_boundary_between(&mut self, from: pt_type!(), to: pt_type!());

    // TODO might take these out / move them somewhere else
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_unaligned_extended_passage_between(&mut self, from: pt_type!(), to: pt_type!());
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_unaligned_extended_wall_between(&mut self, from: pt_type!(), to: pt_type!());
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_unaligned_extended_boundary_between(&mut self, from: pt_type!(), to: pt_type!());
}