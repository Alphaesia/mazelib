use std::fmt::{Debug, Formatter};
use crate::implm::cell::inline::InlineCellValue;
use crate::implm::cell::inline::value::InlineCellValueWallType;
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::implm::render::text::BoxSpaceTextMazeRenderer;
use crate::interface::buffer::{BufferLocation, MazeBuffer};
use crate::interface::cell::{CellManager, CellConnectionType};
use crate::interface::point::CoordinateSpace;
use crate::interface::render::MazeRenderer;
use crate::internal::abs_util::abs_diff;
use crate::internal::array_util::Product;
use crate::pt;

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

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    /// Set the value of any cell, including ones not mapped by the coordinate space
    ///
    /// Since with a [InlineCellValue] it is impossible to get the maze into an
    /// inconsistent state, this function is not considered `unsafe`.
    ///
    /// In most cases you should use the methods on [CellManager] instead of this.
    pub fn set(&mut self, cell: pt!(), value: <Self as CellManager>::CellVal) {
        self.buffer.set(self.pt_to_buffer_loc(cell), value)
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
            if abs_diff(pt1[i], pt2[i]) == 1 {
                return Some(i)
            }
        }

        return None
    }

    fn pt_to_buffer_loc(&self, pt: pt!()) -> BufferLocation {
        let mut offset = pt[0];

        for i in 1..DIMENSION {
            offset += pt[i] * self.coord_space().dimensions()[i - 1];
        }

        BufferLocation(offset)
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

        let mut from_walls = self.get(from).0;
        let mut to_walls = self.get(to).0;

        let from_pos = from[axis_of_adjacency];
        let to_pos = to[axis_of_adjacency];

        if from_pos < to_pos {
            from_walls[axis_of_adjacency][1] = edge_type;
            to_walls[axis_of_adjacency][0] = edge_type;
        } else {
            from_walls[axis_of_adjacency][0] = edge_type;
            to_walls[axis_of_adjacency][1] = edge_type;
        }

        Self::set_unvisited_edges_to_wall(&mut from_walls);
        Self::set_unvisited_edges_to_wall(&mut to_walls);

        self.set(from, InlineCellValue(from_walls));
        self.set(to, InlineCellValue(to_walls));
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> CellManager for BoxSpaceInlineCellManager<Buffer, DIMENSION> {
    type CoordSpace = BoxCoordinateSpace<DIMENSION>;
    type CellVal = InlineCellValue<DIMENSION>;

    fn coord_space(&self) -> &Self::CoordSpace {
        &self.space
    }

    fn get(&self, pt: pt!()) -> Self::CellVal {
        self.buffer.get(self.pt_to_buffer_loc(pt))
    }

    fn get_connection(&self, from: pt!(), to: pt!()) -> CellConnectionType {
        let axis_of_adjacency = Self::get_axis_of_adjacency(from, to).expect("from and to are not adjacent");

        let (from_wall_side, to_wall_side) = if from[axis_of_adjacency] < to[axis_of_adjacency] {
            (1, 0)
        } else {
            (0, 1)
        };

        let from_wall = self.get(from).0[axis_of_adjacency][from_wall_side];
        let to_wall = self.get(to).0[axis_of_adjacency][to_wall_side];

        return match [from_wall, to_wall] {
            [InlineCellValueWallType::BOUNDARY,  _] | [_, InlineCellValueWallType::BOUNDARY ] => CellConnectionType::BOUNDARY,
            [InlineCellValueWallType::UNVISITED, _] | [_, InlineCellValueWallType::UNVISITED] => CellConnectionType::UNVISITED,
            [InlineCellValueWallType::WALL,      _] | [_, InlineCellValueWallType::WALL     ] => CellConnectionType::WALL,
            [InlineCellValueWallType::PASSAGE, InlineCellValueWallType::PASSAGE]              => CellConnectionType::PASSAGE,
        };
    }

    /// Replace all edges of `pt` that are [InlineCellValueWallType::UNVISITED] with
    /// [InlineCellValueWallType::WALL].
    fn make_passage(&mut self, pt: pt!()) {
        let cell = self.get(pt);

        if cell.is_fully_unvisited() {
            self.set(pt, InlineCellValue([[InlineCellValueWallType::WALL; 2]; DIMENSION]))
        } else {
            let mut walls = cell.0;

            Self::set_unvisited_edges_to_wall(&mut walls);

            self.set(pt, InlineCellValue(walls));
        }
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
        self.set(pt, InlineCellValue([[InlineCellValueWallType::WALL; 2]; DIMENSION]))
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
        self.set(pt, InlineCellValue([[InlineCellValueWallType::BOUNDARY; 2]; DIMENSION]))
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

        return Result::Ok(())
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> Debug for BoxSpaceInlineCellManager<Buffer, DIMENSION> {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_main_dbg_fmt(f)?;
        writeln!(f, "}}")?;

        return Result::Ok(())
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<2>>> Debug for BoxSpaceInlineCellManager<Buffer, 2> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_main_dbg_fmt(f)?;

        writeln!(f)?;

        for line in BoxSpaceTextMazeRenderer::render(self) {
            writeln!(f, "\t{}", line)?;
        };

        writeln!(f, "}}")?;

        return Result::Ok(())
    }
}