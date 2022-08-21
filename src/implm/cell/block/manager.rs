use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use crate::implm::cell::block::{BlockCellValue, BlockCellValueType};
use crate::implm::cell::block::location::BlockCellLocation;
use crate::implm::cell::block::value::BlockCellValueType::{BOUNDARY, PASSAGE, UNVISITED, WALL};
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::implm::render::text::BoxSpaceTextMazeRenderer;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::{CellID, CellManager, ConnectionType};
use crate::interface::point::CoordinateSpace;
use crate::interface::render::MazeRendererNonSeeking;
use crate::internal::array_util::{Product, Sum};
use crate::internal::noise_util::pt;

/// TODO write a description
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
    padding: [[usize; 2]; DIMENSION],
    full_dimensions: [usize; DIMENSION],
}

// Constructor (private - use the builder)
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellManager<Buffer, DIMENSION> {
    /// Construct a new maze from a given coordinate space, scale factor, and padding.
    /// A [crate::interface::buffer::MazeBuffer] will be created from the value of type parameter `Buffer`.
    fn new(space: BoxCoordinateSpace<DIMENSION>, scale_factor: [usize; DIMENSION], padding: [[usize; 2]; DIMENSION]) -> Self {
        let full_dimensions = Self::scale_dimensions(space.dimensions(), scale_factor).zip(padding).map(|(scaled_dim, padding)| scaled_dim + padding.sum());

        let cells_required = full_dimensions.product();

        Self { buffer: Buffer::new(cells_required), space, scale_factor, full_dimensions, padding }
    }
}

// Public functions
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellManager<Buffer, DIMENSION> {
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// The dimensions of the coordinate space, scaled by the resolution, plus padding.
    pub fn get_full_dimensions(&self) -> [usize; DIMENSION] {
        self.full_dimensions
    }

    /// The number of cells between points, minus one.
    ///
    /// A scale factor of 1 would have each point directly adjacent to one another.
    /// A scale factor of 2 will have 1 cell between each point.
    ///
    /// Does not affect the distance of points from the outer edge of the maze (see
    /// [`padding()`][Self::padding]).
    pub fn scale_factor(&self) -> [usize; DIMENSION] {
        self.scale_factor
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

    /// Map a point to a cell location.
    pub fn map_pt_to_cell_loc(&self, pt: pt!()) -> <Self as CellManager>::CellLoc {
        let mut pt: [usize; DIMENSION] = pt.into();

        for i in 0..DIMENSION {
            pt[i] *= self.scale_factor[i];
            pt[i] += self.padding[i][0];
        }

        BlockCellLocation(pt.into())
    }

    /// Get the value of the point `pt` for mutation.
    ///
    /// See also: [`Self::get()`].
    pub fn get_mut(&mut self, pt: pt!()) -> &mut <Self as CellManager>::CellVal {
        self.get_cell_value_mut(self.map_pt_to_cell_loc(pt))
    }

    /// Sugar for
    /// ```
    /// # use mazelib::implm::buffer::VecBuffer;
    /// # use mazelib::implm::cell::block::{BlockCellValue, BlockCellValueType, BoxSpaceBlockCellManagerBuilder};
    /// # use mazelib::implm::cell::inline::InlineCellValue;
    /// # use mazelib::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};
    /// # let mut maze = BoxSpaceBlockCellManagerBuilder::<VecBuffer<BlockCellValue>, 1>::new(BoxCoordinateSpace::new([1])).build();
    /// # let pt: CoordinateTuplet<1> = [0].into();
    /// # let cell_type = BlockCellValueType::PASSAGE;
    /// #
    /// maze.get_mut(pt).cell_type = cell_type;
    /// ```
    ///
    /// Modifies the cell's type but keeps its mark status intact.
    ///
    /// Please see [`Self::get_mut()`] for details.
    pub fn set_type(&mut self, pt: pt!(), cell_type: BlockCellValueType) {
        self.get_mut(pt).cell_type = cell_type;
    }

    /// Get the value of any cell.
    ///
    /// In most cases you should use the methods on [`CellManager::get()`]. The only
    /// reason you should use this method is to access a cell that is not mapped by
    /// the coordinated space.
    pub fn get_cell_value(&self, loc: <Self as CellManager>::CellLoc) -> BlockCellValue {
        self.buffer.get(self.cell_loc_to_id(loc))
    }

    /// Get the value of any cell for mutation.
    ///
    /// In most cases you should use the methods on [`Self::get_mut()`]. The only
    /// reason you should use this method is to access a cell that is not mapped by
    /// the coordinated space.
    ///
    /// See also: [`Self::get_cell_value()`].
    pub fn get_cell_value_mut(&mut self, loc: <Self as CellManager>::CellLoc) -> &mut BlockCellValue {
        self.buffer.get_mut(self.cell_loc_to_id(loc))
    }

    /// Set the value of any cell, including ones not mapped by the coordinate space
    ///
    /// Since with a [BlockCellValue] it is impossible to get the maze into an
    /// inconsistent state, this function is not considered `unsafe`.
    ///
    /// In most cases you should use the methods on [CellManager]. The only reason
    /// you should use this method is to access a cell that is not mapped by the
    /// coordinated space.
    pub fn set_cell_value(&mut self, loc: <Self as CellManager>::CellLoc, value: BlockCellValue) {
        self.buffer.set(self.cell_loc_to_id(loc), value)
    }

    /// Sugar for
    /// ```
    /// # use mazelib::implm::buffer::VecBuffer;
    /// # use mazelib::implm::cell::block::{BlockCellLocation, BlockCellValue, BlockCellValueType, BoxSpaceBlockCellManagerBuilder};
    /// # use mazelib::implm::cell::inline::InlineCellValue;
    /// # use mazelib::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};
    /// # let mut maze = BoxSpaceBlockCellManagerBuilder::<VecBuffer<BlockCellValue>, 1>::new(BoxCoordinateSpace::new([1])).build();
    /// # let loc: BlockCellLocation<1> = [0].into();
    /// # let cell_type = BlockCellValueType::PASSAGE;
    /// #
    /// maze.get_cell_value_mut(loc).cell_type = cell_type;
    /// ```
    ///
    /// Please see [`Self::get_cell_value_mut()`] for details.
    pub fn set_cell_value_type(&mut self, loc: <Self as CellManager>::CellLoc, cell_type: BlockCellValueType) {
        self.get_cell_value_mut(loc).cell_type = cell_type;
    }
}

// Internal functions
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellManager<Buffer, DIMENSION> {
    //noinspection RsSelfConvention
    /// Get the axis in which two points are adjacent.
    ///
    /// The number represents the order of the axis (e.g.
    /// x is 0, y is 1, etc.). This is the same as in the index
    /// of the axis for the point (`pt[index]` gives the position of
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

    /// Convert a [`crate::interface::cell::CellLocation`] to a [`crate::interface::cell::CellID`]
    fn cell_loc_to_id(&self, cell_loc: <Self as CellManager>::CellLoc) -> CellID {
        let mut offset = cell_loc[0];

        for i in 1..DIMENSION {
            offset += cell_loc[i] * self.full_dimensions[i - 1];
        }

        CellID(offset)
    }

    fn set_unvisited_neighbours_to_wall(&mut self, cell_loc: <Self as CellManager>::CellLoc) {
        for i in 0..DIMENSION {
            if cell_loc[i] > 0 {
                let neighbour = self.get_cell_value_mut(cell_loc.offset(i, -1));

                if neighbour.cell_type == UNVISITED {
                    neighbour.cell_type = WALL;
                }
            }

            if cell_loc[i] + 1 < self.full_dimensions[i] {
                let neighbour = self.get_cell_value_mut(cell_loc.offset(i, 1));

                if neighbour.cell_type == UNVISITED {
                    neighbour.cell_type = WALL;
                }
            }
        }
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> CellManager for BoxSpaceBlockCellManager<Buffer, DIMENSION> {
    type CoordSpace = BoxCoordinateSpace<DIMENSION>;
    type CellLoc = BlockCellLocation<DIMENSION>;
    type CellVal = BlockCellValue;

    fn coord_space(&self) -> &Self::CoordSpace {
        &self.space
    }

    fn get(&self, pt: pt!()) -> Self::CellVal {
        self.get_cell_value(self.map_pt_to_cell_loc(pt))
    }

    fn get_connection(&self, from: pt!(), to: pt!()) -> ConnectionType {
        match [self.get(from).cell_type, self.get(to).cell_type] {
            [BOUNDARY,  _] | [_, BOUNDARY ] => ConnectionType::BOUNDARY,
            [UNVISITED, _] | [_, UNVISITED] => ConnectionType::UNVISITED,
            [WALL,      _] | [_, WALL     ] => ConnectionType::WALL,
            [PASSAGE,            PASSAGE  ] => ConnectionType::PASSAGE,
        }
    }

    /// Set `pt` to [`BlockCellValueType::PASSAGE`].
    fn make_passage(&mut self, pt: pt!()) {
        let cell_loc = self.map_pt_to_cell_loc(pt);

        self.set_cell_value_type(cell_loc, PASSAGE);
        self.set_unvisited_neighbours_to_wall(cell_loc);
    }

    /// Set `from` and `to` to [`BlockCellValueType::PASSAGE`]. If the resolution is greater than
    /// one along the axis of adjacency, then all intermediate cells will be set to walls too.
    ///
    /// All cells that are adjacent to from, or any of the intermediate cells, and are unvisited,
    /// will be set to [`BlockCellValueType::WALL`]. Note that this excludes `to`, so that
    /// maze carvers will be able to progress. If you wish for `to` to also be surrounded by
    /// walls, simply call [`Self::make_passage()`] on `to` as well.
    fn make_passage_between(&mut self, from: pt!(), to: pt!()) {
        let axis_of_adjacency = Self::get_axis_of_adjacency(from, to).expect("from and to are not adjacent");

        let from = self.map_pt_to_cell_loc(from);
        let to = self.map_pt_to_cell_loc(to);

        let from_pos = from[axis_of_adjacency];
        let to_pos = to[axis_of_adjacency];

        if from_pos < to_pos {
            // Skip out on the end so we don't add walls around it
            for i in from_pos..to_pos {
                let cell = from.at(axis_of_adjacency, i);

                self.set_cell_value_type(cell, PASSAGE);
                self.set_unvisited_neighbours_to_wall(cell);
            }
        } else {
            // Skip out on the end so we don't add walls around it
            for i in (to_pos + 1)..=from_pos {
                let cell = from.at(axis_of_adjacency, i);

                self.set_cell_value_type(cell, PASSAGE);
                self.set_unvisited_neighbours_to_wall(cell);
            }
        }

        self.set_cell_value_type(to, PASSAGE);
    }

    /// Set `pt` to [`BlockCellValueType::WALL`].
    fn make_wall(&mut self, pt: pt!()) {
        self.set_type(pt, WALL);
    }

    /// Set `from` and `to` to [`BlockCellValueType::WALL`]. If the resolution is greater than
    /// one along the axis of adjacency, then all intermediate cells will be set to walls too.
    fn make_wall_between(&mut self, from: pt!(), to: pt!()) {
        let axis_of_adjacency = Self::get_axis_of_adjacency(from, to).expect("from and to are not adjacent");

        let from = self.map_pt_to_cell_loc(from);
        let to = self.map_pt_to_cell_loc(to);

        let from_pos = from[axis_of_adjacency];
        let to_pos = to[axis_of_adjacency];

        if from_pos < to_pos {
            for i in from_pos..=to_pos {
                self.set_cell_value_type(from.at(axis_of_adjacency, i), WALL);
            }
        } else {
            for i in to_pos..=from_pos {
                self.set_cell_value_type(from.at(axis_of_adjacency, i), WALL);
            }
        }
    }

    /// Set `pt` to [`BlockCellValueType::BOUNDARY`].
    fn make_boundary(&mut self, pt: pt!()) {
        self.set_type(pt, BOUNDARY);
    }

    /// Set `from` and `to` to [`BlockCellValueType::BOUNDARY`]. If the resolution is greater than
    /// one along the axis of adjacency, then all intermediate cells will be set to boundaries too.
    fn make_boundary_between(&mut self, from: pt!(), to: pt!()) {
        let axis_of_adjacency = Self::get_axis_of_adjacency(from, to).expect("from and to are not adjacent");

        let from = self.map_pt_to_cell_loc(from);
        let to = self.map_pt_to_cell_loc(to);

        let from_pos = from[axis_of_adjacency];
        let to_pos = to[axis_of_adjacency];

        if from_pos < to_pos {
            for i in from_pos..=to_pos {
                self.set_cell_value_type(from.at(axis_of_adjacency, i), BOUNDARY);
            }
        } else {
            for i in to_pos..=from_pos {
                self.set_cell_value_type(from.at(axis_of_adjacency, i), BOUNDARY);
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
            scale_factor: [2; DIMENSION],
            padding: [[1, 1]; DIMENSION],
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
        writeln!(f, "\tpadding: {:?}", self.padding)?;
        writeln!(f, "\tfull_dimensions: {:?}", self.full_dimensions)?;

        return Ok(())
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> Debug for BoxSpaceBlockCellManager<Buffer, DIMENSION> {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_main_dbg_fmt(f)?;
        writeln!(f, "}}")?;

        return Ok(())
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>> Debug for BoxSpaceBlockCellManager<Buffer, 2> {
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