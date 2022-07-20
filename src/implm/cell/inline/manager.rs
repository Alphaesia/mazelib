use std::fmt::{Debug, Formatter};

use crate::implm::cell::inline::InlineCellValue;
use crate::implm::cell::inline::value::InlineCellValueWallType;
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::implm::render::text::BoxSpaceTextMazeRenderer;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::{CellID, CellManager, ConnectionType};
use crate::interface::point::CoordinateSpace;
use crate::interface::render::MazeRendererNonSeeking;
use crate::internal::array_util::Product;
use crate::{CellPath, PointPath, pt};
use crate::implm::cell::block::BlockCell;

/// As this manager implements a one-to-one mapping between points and cells, there is
/// no separate [`CellLocation`][crate::interface::cell::CellLocation] struct.
/// [`CoordinateTuplet`][crate::implm::point::boxy::CoordinateTuplet]s are converted directly
/// into [`CellID`]s.
pub struct BoxSpaceInlineCellManager<Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> {
    buffer: Buffer,
    space: BoxCoordinateSpace<DIMENSION>,
}

// Public functions
impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> BoxSpaceInlineCellManager<Buffer, DIMENSION> {
    /// Construct a new maze from a given coordinate space.
    /// A [crate::interface::buffer::MazeBuffer] will be created from the value of type parameter `Buffer`.
    pub fn new(space: BoxCoordinateSpace<DIMENSION>) -> Self {
        Self { buffer: Buffer::new(space.dimensions().product()), space }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Set the value of any cell, including ones not mapped by the coordinate space
    ///
    /// Since with a [InlineCellValue] it is impossible to get the maze into an
    /// inconsistent state, this function is not considered `unsafe`.
    ///
    /// In most cases you should use the methods on [CellManager] instead of this.
    pub fn set(&mut self, cell: pt!(), value: <Self as CellManager>::CellVal) {
        self.buffer.set(self.pt_to_cell_id(cell), value)
    }
}

// Internal functions
impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> BoxSpaceInlineCellManager<Buffer, DIMENSION> {
    //noinspection RsSelfConvention
    /// Get the axis in which two points are adjacent.
    ///
    /// The number represents the order of the axis (e.g.
    /// x is 0, y is 1, etc.). This is the same as in the index
    /// of the axis for the point (pt[index] gives the position of
    /// the point along that axis).
    ///
    /// Returns None if the points are identical or not adjacent.
    fn get_axis_of_adjacency(pt1: pt!(), pt2: pt!()) -> Option<usize> {
        for i in 0..DIMENSION {
            if pt1[i].abs_diff(pt2[i]) == 1 {
                return Some(i)
            }
        }

        return None
    }

    fn pt_to_cell_id(&self, pt: pt!()) -> CellID {
        let mut offset = pt[0];

        for i in 1..DIMENSION {
            offset += pt[i] * self.coord_space().dimensions()[i - 1];
        }

        CellID(offset)
    }

    fn get_mut(&mut self, pt: pt!()) -> &mut <Self as CellManager>::CellVal {
        self.buffer.get_mut(self.pt_to_cell_id(pt))
    }

    fn set_unvisited_edges_to_wall(cell: &mut [[InlineCellValueWallType; 2]; DIMENSION]) {
        for i in 0..DIMENSION {
            let dim = &mut cell[i];

            if dim[0] == InlineCellValueWallType::UNVISITED {
                dim[0] = InlineCellValueWallType::WALL;
            }

            if dim[1] == InlineCellValueWallType::UNVISITED {
                dim[1] = InlineCellValueWallType::WALL;
            }
        }
    }

    /// Set the edge between the two cells to `edge_type`, for both cells.
    ///
    /// All [InlineCellValueWallType::UNVISITED] edges will be replaced with
    /// [InlineCellValueWallType::WALL].
    fn make_between(&mut self, from: pt!(), to: pt!(), edge_type: InlineCellValueWallType) {
        let axis_of_adjacency = Self::get_axis_of_adjacency(from, to).expect("from and to are not adjacent");

        let from_pos = from[axis_of_adjacency];
        let to_pos = to[axis_of_adjacency];

        let from_before_to = from_pos < to_pos;

        let mut from_existing = self.get_mut(from);

        if from_before_to {
            from_existing.walls[axis_of_adjacency][1] = edge_type;
        } else {
            from_existing.walls[axis_of_adjacency][0] = edge_type;
        }

        Self::set_unvisited_edges_to_wall(&mut from_existing.walls);

        let mut to_existing = self.get_mut(to);

        if from_before_to {
            to_existing.walls[axis_of_adjacency][0] = edge_type;
        } else {
            to_existing.walls[axis_of_adjacency][1] = edge_type;
        }

        Self::set_unvisited_edges_to_wall(&mut to_existing.walls);
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> CellManager for BoxSpaceInlineCellManager<Buffer, DIMENSION> {
    type CoordSpace = BoxCoordinateSpace<DIMENSION>;
    type CellLoc = BlockCell<DIMENSION>;
    type CellVal = InlineCellValue<DIMENSION>;

    fn coord_space(&self) -> &Self::CoordSpace {
        &self.space
    }

    fn get(&self, pt: pt!()) -> Self::CellVal {
        self.buffer.get(self.pt_to_cell_id(pt))
    }

    fn get_connection(&self, from: pt!(), to: pt!()) -> ConnectionType {
        let axis_of_adjacency = Self::get_axis_of_adjacency(from, to).expect("from and to are not adjacent");

        let (from_wall_side, to_wall_side) = if from[axis_of_adjacency] < to[axis_of_adjacency] {
            (1, 0)
        } else {
            (0, 1)
        };

        let from_wall = self.get(from).walls[axis_of_adjacency][from_wall_side];
        let to_wall = self.get(to).walls[axis_of_adjacency][to_wall_side];

        return match [from_wall, to_wall] {
            [InlineCellValueWallType::BOUNDARY,  _] | [_, InlineCellValueWallType::BOUNDARY ] => ConnectionType::BOUNDARY,
            [InlineCellValueWallType::UNVISITED, _] | [_, InlineCellValueWallType::UNVISITED] => ConnectionType::UNVISITED,
            [InlineCellValueWallType::WALL,      _] | [_, InlineCellValueWallType::WALL     ] => ConnectionType::WALL,
            [InlineCellValueWallType::PASSAGE, InlineCellValueWallType::PASSAGE]              => ConnectionType::PASSAGE,
        };
    }

    /// Replace all edges of `pt` that are [InlineCellValueWallType::UNVISITED] with
    /// [InlineCellValueWallType::WALL].
    fn make_passage(&mut self, pt: pt!()) {
        Self::set_unvisited_edges_to_wall(&mut self.get_mut(pt).walls);
    }

    /// Set the edge between the two cells to [InlineCellValueWallType::PASSAGE],
    /// for both cells.
    ///
    /// All [InlineCellValueWallType::UNVISITED] edges will be replaced with
    /// [InlineCellValueWallType::WALL].
    fn make_passage_between(&mut self, from: pt!(), to: pt!()) {
        self.make_between(from, to, InlineCellValueWallType::PASSAGE)
    }

    /// Set all edges of `pt` to [InlineCellValueWallType::WALL].
    fn make_wall(&mut self, pt: pt!()) {
        let cell_value = self.get_mut(pt);

        cell_value.walls = [[InlineCellValueWallType::WALL; 2]; DIMENSION];
    }

    /// Set the edge between the two cells to [InlineCellValueWallType::WALL],
    /// for both cells.
    ///
    /// All [InlineCellValueWallType::UNVISITED] edges will be replaced with
    /// [InlineCellValueWallType::WALL].
    fn make_wall_between(&mut self, from: pt!(), to: pt!()) {
        self.make_between(from, to, InlineCellValueWallType::WALL)
    }

    /// Set all edges of `pt` to [InlineCellValueWallType::BOUNDARY].
    fn make_boundary(&mut self, pt: pt!()) {
        let cell_value = self.get_mut(pt);

        cell_value.walls = [[InlineCellValueWallType::BOUNDARY; 2]; DIMENSION];
    }

    // TODO what should we do with unvisited edges here? set to wall, boundary, or ignore?
    /// Set the edge between the two cells to [InlineCellValueWallType::BOUNDARY],
    /// for both cells.
    ///
    /// All [InlineCellValueWallType::UNVISITED] edges will be replaced with
    /// [InlineCellValueWallType::WALL].
    fn make_boundary_between(&mut self, from: pt!(), to: pt!()) {
        self.make_between(from, to, InlineCellValueWallType::BOUNDARY)
    }

    fn map_pt_path_to_cell_path(&self, path: &PointPath<Self::CoordSpace>) -> CellPath<Self::CellLoc> {
        drop(path);
        todo!()
    }
}

/*
 * We want to show the state of the maze in the debug output for 2D mazes.
 *
 * To this end, we must manually implement Debug for BoxSpaceInlineCellManager,
 * then provide a specialisation where DIMENSION = 2.
 *
 * The reason we must manually implement Debug is because #[derive(Debug)] does
 * not mark its implementation as `default`, which we need in order to specialise.
 */

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> BoxSpaceInlineCellManager<Buffer, DIMENSION> {
    fn write_main_dbg_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BoxSpaceBlockCellManager {{")?;
        writeln!(f, "\tbuffer: {:?}", self.buffer)?;
        writeln!(f, "\tspace: {:?}", self.space)?;

        return Ok(())
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> Debug for BoxSpaceInlineCellManager<Buffer, DIMENSION> {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_main_dbg_fmt(f)?;
        writeln!(f, "}}")?;

        return Ok(())
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<2>>> Debug for BoxSpaceInlineCellManager<Buffer, 2> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_main_dbg_fmt(f)?;

        writeln!(f)?;

        let mut render = Vec::<u8>::new();

        if let Err(err) = BoxSpaceTextMazeRenderer::new().render(self, &mut render) {
            panic!("{}", err)
        }

        let render = std::str::from_utf8(&render).expect("BoxSpaceTextMazeRenderer did not produce valid UTF-8");

        for line in render.lines() {
            writeln!(f, "\t{}", line)?;
        };

        writeln!(f, "}}")?;

        return Ok(())
    }
}