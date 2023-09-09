use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use crate::implm::cell::block::BlockCellLocation;
use crate::implm::cell::inline::InlineCellValue;
use crate::implm::cell::inline::InlineCellValueEdgeType;
use crate::implm::export::text::BoxSpaceInlineCellTextMazeExporter;
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::{CellID, ConnectionType};
use crate::interface::coordinator::MazeCoordinator;
use crate::interface::export::MazeExporter;
use crate::interface::point::CoordinateSpace;
use crate::internal::array_util::Product;
use crate::pt;

/// As this coordinator implements a one-to-one mapping between points and cells, there is
/// no separate [`CellLocation`][crate::interface::cell::CellLocation] struct.
/// [`CoordinateTuplet`][crate::implm::point::boxy::CoordinateTuplet]s are converted directly
/// into [`CellID`]s.
pub struct BoxSpaceInlineCellMazeCoordinator<Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> {
    buffer: Buffer,
    space: BoxCoordinateSpace<DIMENSION>,
}

// Constructor (private - use the builder)
impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> BoxSpaceInlineCellMazeCoordinator<Buffer, DIMENSION> {
    /// Construct a new maze from a given coordinate space.
    /// A [`MazeBuffer`] will be created from the value of type parameter `Buffer`.
    #[must_use]
    fn new(space: BoxCoordinateSpace<DIMENSION>) -> Self {
        Self { buffer: Buffer::new(space.dimensions().product()), space }
    }
}

// Public functions
impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> BoxSpaceInlineCellMazeCoordinator<Buffer, DIMENSION> {
    #[must_use]
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Set the value of any cell, including ones not mapped by the coordinate space
    ///
    /// Since with a [InlineCellValue] it is impossible to get the maze into an
    /// inconsistent state, this function is not considered `unsafe`.
    ///
    /// In most cases you should use the methods on [MazeCoordinator] instead of this.
    pub fn set(&mut self, cell: pt!(), value: <Self as MazeCoordinator>::CellVal) {
        self.buffer.set(self.pt_to_cell_id(cell), value)
    }
}

// Internal functions
impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> BoxSpaceInlineCellMazeCoordinator<Buffer, DIMENSION> {
    //noinspection RsSelfConvention
    /// Get the axis in which two points are adjacent.
    ///
    /// The number represents the order of the axis (e.g.
    /// x is 0, y is 1, etc.). This is the same as in the index
    /// of the axis for the point (pt[index] gives the position of
    /// the point along that axis).
    ///
    /// Returns None if the points are identical or not adjacent.
    #[must_use]
    fn get_axis_of_adjacency(pt1: pt!(), pt2: pt!()) -> Option<usize> {
        for i in 0..DIMENSION {
            if pt1[i].abs_diff(pt2[i]) == 1 {
                return Some(i)
            }
        }

        return None
    }

    #[must_use]
    fn pt_to_cell_id(&self, pt: pt!()) -> CellID {
        let mut offset = pt[0];

        for i in 1..DIMENSION {
            offset += pt[i] * usize::from(self.coord_space().dimensions()[i - 1]);
        }

        CellID(offset)
    }

    #[must_use]
    fn get_mut(&mut self, pt: pt!()) -> &mut <Self as MazeCoordinator>::CellVal {
        self.buffer.get_mut(self.pt_to_cell_id(pt))
    }

    fn set_unvisited_edges_to_wall(cell: &mut [[InlineCellValueEdgeType; 2]; DIMENSION]) {
        for i in 0..DIMENSION {
            let dim = &mut cell[i];

            if dim[0] == InlineCellValueEdgeType::UNVISITED {
                dim[0] = InlineCellValueEdgeType::WALL;
            }

            if dim[1] == InlineCellValueEdgeType::UNVISITED {
                dim[1] = InlineCellValueEdgeType::WALL;
            }
        }
    }

    /// Set the edge between the two cells to `edge_type`, for both cells.
    ///
    /// All [InlineCellValueEdgeType::UNVISITED] edges will be replaced with
    /// [InlineCellValueEdgeType::WALL].
    fn make_between(&mut self, from: pt!(), to: pt!(), edge_type: InlineCellValueEdgeType) {
        let axis_of_adjacency = Self::get_axis_of_adjacency(from, to).expect("from and to are not adjacent");

        let from_pos = from[axis_of_adjacency];
        let to_pos = to[axis_of_adjacency];

        let from_before_to = from_pos < to_pos;

        let from_existing = self.get_mut(from);

        if from_before_to {
            from_existing.edges[axis_of_adjacency][1] = edge_type;
        } else {
            from_existing.edges[axis_of_adjacency][0] = edge_type;
        }

        Self::set_unvisited_edges_to_wall(&mut from_existing.edges);

        let to_existing = self.get_mut(to);

        if from_before_to {
            to_existing.edges[axis_of_adjacency][0] = edge_type;
        } else {
            to_existing.edges[axis_of_adjacency][1] = edge_type;
        }

        Self::set_unvisited_edges_to_wall(&mut to_existing.edges);
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> MazeCoordinator for BoxSpaceInlineCellMazeCoordinator<Buffer, DIMENSION> {
    type CoordSpace = BoxCoordinateSpace<DIMENSION>;
    type CellLoc = BlockCellLocation<DIMENSION>;
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

        let from_wall = self.get(from).edges[axis_of_adjacency][from_wall_side];
        let to_wall = self.get(to).edges[axis_of_adjacency][to_wall_side];

        return match [from_wall, to_wall] {
            [InlineCellValueEdgeType::BOUNDARY,  _] | [_, InlineCellValueEdgeType::BOUNDARY ] => ConnectionType::BOUNDARY,
            [InlineCellValueEdgeType::UNVISITED, _] | [_, InlineCellValueEdgeType::UNVISITED] => ConnectionType::UNVISITED,
            [InlineCellValueEdgeType::WALL,      _] | [_, InlineCellValueEdgeType::WALL     ] => ConnectionType::WALL,
            [InlineCellValueEdgeType::PASSAGE, InlineCellValueEdgeType::PASSAGE]              => ConnectionType::PASSAGE,
        };
    }

    /// Replace all edges of `pt` that are [InlineCellValueEdgeType::UNVISITED] with
    /// [InlineCellValueEdgeType::WALL].
    fn make_passage(&mut self, pt: pt!()) {
        Self::set_unvisited_edges_to_wall(&mut self.get_mut(pt).edges);
    }

    /// Set the edge between the two cells to [InlineCellValueEdgeType::PASSAGE],
    /// for both cells.
    ///
    /// All [InlineCellValueEdgeType::UNVISITED] edges will be replaced with
    /// [InlineCellValueEdgeType::WALL].
    fn make_passage_between(&mut self, from: pt!(), to: pt!()) {
        self.make_between(from, to, InlineCellValueEdgeType::PASSAGE)
    }

    /// Set all edges of `pt` to [InlineCellValueEdgeType::WALL].
    fn make_wall(&mut self, pt: pt!()) {
        let cell_value = self.get_mut(pt);

        cell_value.edges = [[InlineCellValueEdgeType::WALL; 2]; DIMENSION];
    }

    /// Set the edge between the two cells to [InlineCellValueEdgeType::WALL],
    /// for both cells.
    ///
    /// All [InlineCellValueEdgeType::UNVISITED] edges will be replaced with
    /// [InlineCellValueEdgeType::WALL].
    fn make_wall_between(&mut self, from: pt!(), to: pt!()) {
        self.make_between(from, to, InlineCellValueEdgeType::WALL)
    }

    /// Set all edges of `pt` to [InlineCellValueEdgeType::BOUNDARY].
    fn make_boundary(&mut self, pt: pt!()) {
        let cell_value = self.get_mut(pt);

        cell_value.edges = [[InlineCellValueEdgeType::BOUNDARY; 2]; DIMENSION];
    }

    // TODO what should we do with unvisited edges here? set to wall, boundary, or ignore?
    /// Set the edge between the two cells to [InlineCellValueEdgeType::BOUNDARY],
    /// for both cells.
    ///
    /// All [InlineCellValueEdgeType::UNVISITED] edges will be replaced with
    /// [InlineCellValueEdgeType::WALL].
    fn make_boundary_between(&mut self, from: pt!(), to: pt!()) {
        self.make_between(from, to, InlineCellValueEdgeType::BOUNDARY)
    }
}

#[must_use]
pub struct BoxSpaceInlineCellMazeCoordinatorBuilder<Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> {
    _buffer: PhantomData<Buffer>,
    space: BoxCoordinateSpace<DIMENSION>,
}

impl<Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> BoxSpaceInlineCellMazeCoordinatorBuilder<Buffer, DIMENSION> {
    pub fn new(space: BoxCoordinateSpace<DIMENSION>) -> Self {
        Self {
            _buffer: PhantomData,
            space,
        }
    }

    #[must_use]
    pub fn build(&self) -> BoxSpaceInlineCellMazeCoordinator<Buffer, DIMENSION> {
        BoxSpaceInlineCellMazeCoordinator::new(self.space)
    }
}

/*
 * We want to show the state of the maze in the debug output for 2D mazes.
 *
 * To this end, we must manually implement Debug for BoxSpaceInlineCellMazeCoordinator,
 * then provide a specialisation where DIMENSION = 2.
 *
 * The reason we must manually implement Debug is because #[derive(Debug)] does
 * not mark its implementation as `default`, which we need in order to specialise.
 */

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> BoxSpaceInlineCellMazeCoordinator<Buffer, DIMENSION> {
    fn write_main_dbg_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BoxSpaceInlineCellMazeCoordinator {{")?;
        writeln!(f, "\tbuffer: {:?}", self.buffer)?;
        writeln!(f, "\tspace: {:?}", self.space)?;

        return Ok(())
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> Debug for BoxSpaceInlineCellMazeCoordinator<Buffer, DIMENSION> {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_main_dbg_fmt(f)?;
        writeln!(f, "}}")?;

        return Ok(())
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<2>>> Debug for BoxSpaceInlineCellMazeCoordinator<Buffer, 2> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_main_dbg_fmt(f)?;

        writeln!(f)?;

        let mut text_export = Vec::<u8>::new();

        if let Err(err) = BoxSpaceInlineCellTextMazeExporter::default().export(self, &mut text_export) {
            panic!("{}", err)
        }

        let text_export = std::str::from_utf8(&text_export).expect("BoxSpaceTextMazeExporter did not produce valid UTF-8");

        for line in text_export.lines() {
            writeln!(f, "\t{}", line)?;
        };

        writeln!(f, "}}")?;

        return Ok(())
    }
}