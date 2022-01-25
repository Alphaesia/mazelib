use crate::interface::buffer::{MazeBuffer, BufferLocation};
use crate::interface::cell::{CellManager, CellValue};
use crate::implm::point::boxy::{BoxCoordinateSpace};
use crate::internal::noise_util::pt;
use crate::interface::point::CoordinateSpace;
use std::fmt::{Debug, Formatter};
use crate::internal::abs_util::abs_diff;
use crate::internal::array_util::Product;
use crate::implm::render::text::BoxSpaceBlockCellTextMazeRenderer;
use crate::interface::render::MazeRenderer;
use crate::implm::cell::block::BlockCellValue;
use crate::implm::cell::block::scaled_pt::MappedPoint;
use std::marker::PhantomData;

/// Each maze has a scale factor. This determines the cellular distance between points.
/// With a scale factor of 1 in a direction, points along that axis will map to adjacent
/// cells. With a factor of 2, there will be one cell in between each point. With a factor
/// of 3, there will be two cells between each point, etc.
///
/// # Examples
///
/// With scaling and padding:
/// ```
/// use mazelib::implm::point::boxy::TwoDimensionalBoxCoordinateSpace;
/// use mazelib::implm::cell::block::{BlockCellValue, BoxSpaceBlockCellManagerBuilder};
/// use mazelib::implm::buffer::VecBuffer;
///
/// let coord_space = TwoDimensionalBoxCoordinateSpace::new([11, 11]); // Standard 2D coordinate space
/// let scale_factor = [2, 2]; // A simple scale factor of 2 for a clean look
/// let padding = [[1, 1], [1, 1]]; // 1 cell on each side for a border
/// type CellType = BlockCellValue; // Pixelated maze style
///
/// type MazeBuffer = VecBuffer<CellType>;
///
/// let maze = BoxSpaceBlockCellManagerBuilder::<MazeBuffer, 2>::new(coord_space)
///                                             .scale_factor(scale_factor)
///                                             .padding(padding)
///                                             .build();
/// ```
pub struct BoxSpaceBlockCellManager<Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> {
    buffer: Buffer,
    space: BoxCoordinateSpace<DIMENSION>,
    scale_factor: [usize; DIMENSION],
    scaled_dimensions: [usize; DIMENSION],
    padding: [[usize; 2]; DIMENSION],
}

// Public functions
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellManager<Buffer, DIMENSION> {
    /// Construct a new maze from a given coordinate space and scale factor.
    /// A [crate::interface::buffer::MazeBuffer] will be created from the value of type parameter `Buffer`.
    pub fn new(space: BoxCoordinateSpace<DIMENSION>, scale_factor: [usize; DIMENSION], padding: [[usize; 2]; DIMENSION]) -> Self {
        let cells_required = Self::scale_dimensions(space.dimensions(), scale_factor).zip(padding).map(|(scaled_dim, padding)| scaled_dim + padding[0] + padding[1]).product();

        let scaled_dimensions = Self::scale_dimensions(space.dimensions(), scale_factor);

        Self { buffer: Buffer::new(cells_required), space, scale_factor, scaled_dimensions, padding }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// The dimensions of the coordinate space, scaled by the resolution, plus padding
    pub fn get_full_dimensions(&self) -> [usize; DIMENSION] {
        self.scaled_dimensions.zip(self.padding).map(|(dim, padding)| dim + padding[0] + padding[1])
    }

    /// The number of cells between the edge of the maze and the outermost cell that is mapped to
    /// a point. Useful for borders when the scale factor is greater than one.
    pub fn padding(&self) -> [[usize; 2]; DIMENSION] {
        self.padding
    }

    /// The dimensions of the coordinate space, scaled by the resolution
    pub fn scale_dimensions(dimensions: [usize; DIMENSION], resolution: [usize; DIMENSION]) -> [usize; DIMENSION] {
        dimensions.zip(resolution)
            .map(|(dim, res)| {
                if dim != 0 {
                    (dim - 1) * res + 1
                } else {
                    0
                }
            })
    }

    /// Map a point to a cell.
    pub fn map_pt(&self, pt: pt!()) -> MappedPoint<DIMENSION> {
        let mut pt: [usize; DIMENSION] = pt.into();

        #[allow(clippy::needless_range_loop)]
        for i in 0..DIMENSION {
            pt[i] *= self.scale_factor[i];
            pt[i] += self.padding[i][0];
        }

        MappedPoint(pt.into())
    }

    /// Get the value of any cell.
    ///
    /// In most cases you should use the methods on [CellManager::get()]. The only
    /// reason you should use this method is to access a cell that is not mapped by
    /// the coordinated space.
    pub fn get_cell(&self, cell: MappedPoint<DIMENSION>) -> BlockCellValue {
        self.buffer.get(self.cell_to_buffer_loc(cell))
    }

    /// Set the value of any cell, including ones not mapped by the coordinate space
    ///
    /// Since with a [BlockCellValue] it is impossible to get the maze into an
    /// inconsistent state, this function is not considered `unsafe`.
    ///
    /// In most cases you should use the methods on [CellManager]. The only reason
    /// you should use this method is to access a cell that is not mapped by the
    /// coordinated space.
    pub fn set(&mut self, cell: MappedPoint<DIMENSION>, value: BlockCellValue) {
        self.buffer.set(self.cell_to_buffer_loc(cell), value)
    }
}

// Internal functions
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellManager<Buffer, DIMENSION> {
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

    fn cell_to_buffer_loc(&self, pt: MappedPoint<DIMENSION>) -> BufferLocation {
        let mut offset = pt[0];

        for i in 1..DIMENSION {
            offset += pt[i] * (self.scaled_dimensions[i - 1] + self.padding[i - 1][0] + self.padding[i - 1][1]);
        }

        BufferLocation(offset)
    }

    fn set_unvisited_neighbours_to_wall(&mut self, pt: MappedPoint<DIMENSION>) {
        for i in 0..DIMENSION {
            if pt[i] > 0 {
                let neighbour = pt.offset(i, -1);

                // We don't overwrite boundaries
                if self.get_cell(neighbour).is_unvisited() {
                    self.set(neighbour, BlockCellValue::WALL)
                }
            }

            if pt[i] + 1 < self.scaled_dimensions[i] {
                let neighbour = pt.offset(i, 1);

                if self.get_cell(neighbour).is_unvisited() {
                    self.set(neighbour, BlockCellValue::WALL)
                }
            }
        }
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> CellManager for BoxSpaceBlockCellManager<Buffer, DIMENSION> {
    type CoordSpace = BoxCoordinateSpace<DIMENSION>;
    type CellVal = BlockCellValue;

    fn coord_space(&self) -> &Self::CoordSpace {
        &self.space
    }

    fn get(&self, pt: pt!()) -> Self::CellVal {
        self.get_cell(self.map_pt(pt))
    }

    /// Set `pt` to [super::BlockCellValue::PASSAGE].
    fn make_passage(&mut self, pt: pt!()) {
        let pt = self.map_pt(pt);

        self.set(pt, BlockCellValue::PASSAGE);
        self.set_unvisited_neighbours_to_wall(pt);
    }

    /// Set `from` and `to` to [super::BlockCellValue::PASSAGE]. If the resolution is greater than
    /// one along the axis of adjacency, then all intermediate cells will be set to walls too.
    ///
    /// All cells that are adjacent to from, or any of the intermediate cells, and are unvisited,
    /// will be set to [super::BlockCellValue::WALL]. Note that this excludes `to`, so that
    /// maze carvers will be able to progress. If you wish for `to` to also be surrounded by
    /// walls, simply call [Self::make_passage] on `to` as well.
    fn make_passage_between(&mut self, from: pt!(), to: pt!()) {
        let axis_of_adjacency = Self::get_axis_of_adjacency(from, to).expect("from and to are not adjacent");

        let from = self.map_pt(from);
        let to = self.map_pt(to);

        let from_pos = from[axis_of_adjacency];
        let to_pos = to[axis_of_adjacency];

        if from_pos < to_pos {
            // Skip out on the end so we don't add walls around it
            for i in from_pos..to_pos {
                let pt = from.at(axis_of_adjacency, i);

                self.set(pt, BlockCellValue::PASSAGE);
                self.set_unvisited_neighbours_to_wall(pt);
            }
        } else {
            // Skip out on the end so we don't add walls around it
            for i in (to_pos + 1)..=from_pos {
                let pt = from.at(axis_of_adjacency, i);

                self.set(pt, BlockCellValue::PASSAGE);
                self.set_unvisited_neighbours_to_wall(pt);
            }
        }

        self.set(to, BlockCellValue::PASSAGE);
    }

    /// Set `pt` to [super::BlockCellValue::WALL].
    fn make_wall(&mut self, pt: pt!()) {
        self.set(self.map_pt(pt), BlockCellValue::WALL)
    }

    /// Set `from` and `to` to [super::BlockCellValue::WALL]. If the resolution is greater than
    /// one along the axis of adjacency, then all intermediate cells will be set to walls too.
    fn make_wall_between(&mut self, from: pt!(), to: pt!()) {
        let axis_of_adjacency = Self::get_axis_of_adjacency(from, to).expect("from and to are not adjacent");

        let from = self.map_pt(from);
        let to = self.map_pt(to);

        let from_pos = from[axis_of_adjacency];
        let to_pos = to[axis_of_adjacency];

        if from_pos < to_pos {
            for i in from_pos..=to_pos {
                self.set(from.at(axis_of_adjacency, i), BlockCellValue::WALL);
            }
        } else {
            for i in to_pos..=from_pos {
                self.set(from.at(axis_of_adjacency, i), BlockCellValue::WALL);
            }
        }
    }

    /// Set `pt` to [super::BlockCellValue::BOUNDARY].
    fn make_boundary(&mut self, pt: pt!()) {
        self.set(self.map_pt(pt), BlockCellValue::BOUNDARY)
    }

    /// Set `from` and `to` to [super::BlockCellValue::BOUNDARY]. If the resolution is greater than
    /// one along the axis of adjacency, then all intermediate cells will be set to boundaries too.
    fn make_boundary_between(&mut self, from: pt!(), to: pt!()) {
        let axis_of_adjacency = Self::get_axis_of_adjacency(from, to).expect("from and to are not adjacent");

        let from = self.map_pt(from);
        let to = self.map_pt(to);

        let from_pos = from[axis_of_adjacency];
        let to_pos = to[axis_of_adjacency];

        if from_pos < to_pos {
            for i in from_pos..=to_pos {
                self.set(from.at(axis_of_adjacency, i), BlockCellValue::BOUNDARY);
            }
        } else {
            for i in to_pos..=from_pos {
                self.set(from.at(axis_of_adjacency, i), BlockCellValue::BOUNDARY);
            }
        }
    }
}

pub struct BoxSpaceBlockCellManagerBuilder<Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> {
    _buffer: PhantomData<Buffer>,  // We're not actually interested in constructing a buffer yet
    space: BoxCoordinateSpace<DIMENSION>,
    scale_factor: [usize; DIMENSION],
    padding: [[usize; 2]; DIMENSION],
}

impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellManagerBuilder<Buffer, DIMENSION> {
    pub fn new(space: BoxCoordinateSpace<DIMENSION>) -> Self {
        Self {
            _buffer: PhantomData,
            space,
            scale_factor: [1; DIMENSION],
            padding: [[0, 0]; DIMENSION],
        }
    }

    pub fn scale_factor(mut self, scale_factor: [usize; DIMENSION]) -> Self {
        self.scale_factor = scale_factor;

        return self
    }

    pub fn padding(mut self, padding: [[usize; 2]; DIMENSION]) -> Self {
        self.padding = padding;

        return self
    }

    pub fn build(self) -> BoxSpaceBlockCellManager<Buffer, DIMENSION> {
        BoxSpaceBlockCellManager::new(self.space, self.scale_factor, self.padding)
    }
}

/*
 * We want to show the state of the maze in the debug output for 2D mazes.
 *
 * To this end, we must manually implement Debug for BoxSpaceBlockCellManager,
 * then provide a specialisation where DIMENSION = 2.
 *
 * The reason we must manually implement Debug is because #[derive(Debug)] does
 * not mark its implementation as `default`, which we need in order to specialise.
 */

impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellManager<Buffer, DIMENSION> {
    fn write_main_dbg_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BoxSpaceBlockCellManager {{")?;
        writeln!(f, "\tbuffer: {:?}", self.buffer)?;
        writeln!(f, "\tspace: {:?}", self.space)?;
        writeln!(f, "\tresolution: {:?}", self.scale_factor)?;
        writeln!(f, "\tscaled_dimensions: {:?}", self.scaled_dimensions)?;

        return Result::Ok(())
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> Debug for BoxSpaceBlockCellManager<Buffer, DIMENSION> {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_main_dbg_fmt(f)?;
        writeln!(f, "}}")?;

        return Result::Ok(())
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>> Debug for BoxSpaceBlockCellManager<Buffer, 2> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_main_dbg_fmt(f)?;

        writeln!(f)?;

        for line in BoxSpaceBlockCellTextMazeRenderer::render(self) {
            writeln!(f, "\t{}", line)?;
        };

        writeln!(f, "}}")?;

        return Result::Ok(())
    }
}