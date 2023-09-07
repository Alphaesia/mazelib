use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::num::NonZeroUsize;

use crate::implm::cell::block::{BlockCellValue, BlockCellValueType};
use crate::implm::cell::block::BlockCellLocation;
use crate::implm::cell::block::BlockCellValueType::{BOUNDARY, PASSAGE, UNVISITED, WALL};
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::implm::render::text::BoxSpaceTextMazeRenderer;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::{CellID, ConnectionType};
use crate::interface::coordinator::MazeCoordinator;
use crate::interface::point::CoordinateSpace;
use crate::interface::render::MazeRendererNonSeeking;
use crate::internal::array_util::{ArrayZipMap, CheckedProduct, CheckedSum};
use crate::internal::noise_util::pt;
use crate::internal::util::{NONZERO_USIZE_TWO, try_usize_array_to_nonzero_usize_array};

/// TODO write a description
///
/// # Examples
///
/// With scaling and padding:
/// ```
/// use mazelib::implm::point::boxy::TwoDimensionalBoxCoordinateSpace;
/// use mazelib::implm::cell::block::BlockCellValue;
/// use mazelib::implm::buffer::VecBuffer;
/// use mazelib::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinatorBuilder;
///
/// let coord_space = TwoDimensionalBoxCoordinateSpace::new_checked([11, 11]); // Standard 2D coordinate space
/// let scale_factor = [2, 2]; // A simple scale factor of 2 for a clean look
/// let padding = [[1, 1], [1, 1]]; // 1 cell on each side for a border
/// type CellType = BlockCellValue; // Pixelated maze style
///
/// type MazeBuffer = VecBuffer<CellType>;
///
/// let maze = BoxSpaceBlockCellMazeCoordinatorBuilder::<MazeBuffer, 2>::new(coord_space)
///                                             .scale_factor_checked(scale_factor)
///                                             .padding(padding)
///                                             .build();
/// ```
pub struct BoxSpaceBlockCellMazeCoordinator<Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> {
    buffer: Buffer,
    space: BoxCoordinateSpace<DIMENSION>,
    scale_factor: [NonZeroUsize; DIMENSION],
    padding: [[usize; 2]; DIMENSION],
    full_dimensions: [NonZeroUsize; DIMENSION],
}

// Constructor (private - use the builder)
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellMazeCoordinator<Buffer, DIMENSION> {
    /// Construct a new maze from a given coordinate space, scale factor, and padding.
    ///
    /// # Parameters
    ///
    /// `space`        --- the coordinate space for this maze.
    /// `scale_factor` --- the scale factor for points to cells. You can think about it
    ///                    like `cells:points`. A scale factor of 3 will yield 3 cells
    ///                    for every point (a 3:1 ratio). Each axis is scaled independently.
    ///                    The ordering of the scalars is the same as the standard ordering
    ///                    of the coordinate axes.
    /// `padding`      --- How many extra cells to place between edge-adjacent points
    ///                    and the edge of the maze. For example, a value of 1 will yield
    ///                    a 1-cell thick "border" along that edge of the maze. The ordering
    ///                    of the outer elements is the same as the standard ordering of
    ///                    the coordinate axes. For the inner elements, the side closest
    ///                    to zero comes before the side furthest from zero.
    ///
    /// # Type Parameter
    ///
    /// `Buffer` --- the type of buffer to use. A buffer instance will be automatically
    ///              constructed from this type.
    #[must_use]
    fn new(space: BoxCoordinateSpace<DIMENSION>, scale_factor: [NonZeroUsize; DIMENSION], padding: [[usize; 2]; DIMENSION]) -> Self {
        // Arithmetic is so easy and beautiful and succinct
        let full_dimensions = space.dimensions()
            .zip_map(&scale_factor, |dim, scalar| {
                // NonZeroUsize::new only returns None if the scaled dimension == usize::MAX (which I don't
                // think is mathematically even possible, but in such a hypothetical case) the +1 would cause
                // the sum to overflow to zero
                (usize::from(*dim) - 1).checked_mul(usize::from(*scalar))
                    .and_then(|scaled_dim| NonZeroUsize::new(scaled_dim + 1))
                    .expect("The scaled dimensions do not all fit within a usize")
            })
            .zip_map(&padding, |scaled_dim, padding| {
                padding.checked_sum().and_then(|summed_padding| scaled_dim.checked_add(summed_padding)).expect("The full dimensions do not all fit within a usize")
            });

        let cells_required = full_dimensions.checked_product().expect("The full dimensions specified are too large. The number of cells in the maze does not fit within a usize.");

        Self { buffer: Buffer::new(cells_required), space, scale_factor, full_dimensions, padding }
    }
}

// Public functions
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellMazeCoordinator<Buffer, DIMENSION> {
    #[must_use]
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// The dimensions of the coordinate space, scaled by the resolution, plus padding.
    #[must_use]
    pub fn get_full_dimensions(&self) -> [NonZeroUsize; DIMENSION] {
        self.full_dimensions
    }

    /// The number of cells between points, minus one.
    ///
    /// A scale factor of 1 would have each point directly adjacent to one another.
    /// A scale factor of 2 will have 1 cell between each point.
    ///
    /// Does not affect the distance of points from the outer edge of the maze (see
    /// [`padding()`][Self::padding]).
    #[must_use]
    pub fn scale_factor(&self) -> [NonZeroUsize; DIMENSION] {
        self.scale_factor
    }

    /// The number of cells between the edge of the maze and the outermost cell that is mapped to
    /// a point. Useful for borders when the scale factor is greater than one.
    #[must_use]
    pub fn padding(&self) -> [[usize; 2]; DIMENSION] {
        self.padding
    }

    /// Map a point to a cell location.
    #[must_use]
    pub fn map_pt_to_cell_loc(&self, pt: pt!()) -> <Self as MazeCoordinator>::CellLoc {
        let mut pt: [usize; DIMENSION] = pt.into();

        for i in 0..DIMENSION {
            pt[i] *= usize::from(self.scale_factor[i]);
            pt[i] += self.padding[i][0];
        }

        BlockCellLocation(pt.into())
    }

    /// Get the value of the point `pt` for mutation.
    ///
    /// *See also: [`get()`][Self::get].*
    #[must_use]
    pub fn get_mut(&mut self, pt: pt!()) -> &mut <Self as MazeCoordinator>::CellVal {
        self.get_cell_value_mut(self.map_pt_to_cell_loc(pt))
    }

    /// Sugar for
    /// ```
    /// # use mazelib::implm::buffer::VecBuffer;
    /// # use mazelib::implm::cell::block::{BlockCellValue, BlockCellValueType};
    /// # use mazelib::implm::cell::inline::InlineCellValue;
    /// # use mazelib::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinatorBuilder;
    /// # use mazelib::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};
    /// # let mut maze = BoxSpaceBlockCellMazeCoordinatorBuilder::<VecBuffer<BlockCellValue>, 1>::new(BoxCoordinateSpace::new_checked([1])).build();
    /// # let pt: CoordinateTuplet<1> = [0].into();
    /// # let cell_type = BlockCellValueType::PASSAGE;
    /// #
    /// maze.get_mut(pt).cell_type = cell_type;
    /// ```
    ///
    /// Modifies the cell's type but keeps its mark status intact.
    ///
    /// Please see [`get_mut()`][Self::get_mut] for details.
    pub fn set_type(&mut self, pt: pt!(), cell_type: BlockCellValueType) {
        self.get_mut(pt).cell_type = cell_type;
    }

    /// Get the value of any cell.
    ///
    /// In most cases you should use the methods on [`MazeCoordinator::get()`]. The only
    /// reason you should use this method is to access a cell that is not mapped by
    /// the coordinated space.
    #[must_use]
    pub fn get_cell_value(&self, loc: <Self as MazeCoordinator>::CellLoc) -> BlockCellValue {
        self.buffer.get(self.cell_loc_to_id(loc))
    }

    /// Get the value of any cell for mutation.
    ///
    /// In most cases you should use the methods on [`Self::get_mut()`]. The only
    /// reason you should use this method is to access a cell that is not mapped by
    /// the coordinated space.
    ///
    /// See also: [`Self::get_cell_value()`].
    #[must_use]
    pub fn get_cell_value_mut(&mut self, loc: <Self as MazeCoordinator>::CellLoc) -> &mut BlockCellValue {
        self.buffer.get_mut(self.cell_loc_to_id(loc))
    }

    /// Set the value of any cell, including ones not mapped by the coordinate space
    ///
    /// Since with a [BlockCellValue] it is impossible to get the maze into an
    /// inconsistent state, this function is not considered `unsafe`.
    ///
    /// In most cases you should use the methods on [MazeCoordinator]. The only reason
    /// you should use this method is to access a cell that is not mapped by the
    /// coordinated space.
    #[must_use]
    pub fn set_cell_value(&mut self, loc: <Self as MazeCoordinator>::CellLoc, value: BlockCellValue) {
        self.buffer.set(self.cell_loc_to_id(loc), value)
    }

    /// Sugar for
    /// ```
    /// # use mazelib::implm::buffer::VecBuffer;
    /// # use mazelib::implm::cell::block::{BlockCellLocation, BlockCellValue, BlockCellValueType};
    /// # use mazelib::implm::cell::inline::InlineCellValue;
    /// # use mazelib::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinatorBuilder;
    /// # use mazelib::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};
    /// # let mut maze = BoxSpaceBlockCellMazeCoordinatorBuilder::<VecBuffer<BlockCellValue>, 1>::new(BoxCoordinateSpace::new_checked([1])).build();
    /// # let loc: BlockCellLocation<1> = [0].into();
    /// # let cell_type = BlockCellValueType::PASSAGE;
    /// #
    /// maze.get_cell_value_mut(loc).cell_type = cell_type;
    /// ```
    ///
    /// Please see [`Self::get_cell_value_mut()`] for details.
    pub fn set_cell_value_type(&mut self, loc: <Self as MazeCoordinator>::CellLoc, cell_type: BlockCellValueType) {
        self.get_cell_value_mut(loc).cell_type = cell_type;
    }
}

// Internal functions
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellMazeCoordinator<Buffer, DIMENSION> {
    //noinspection RsSelfConvention
    /// Get the axis in which two points are adjacent.
    ///
    /// The number represents the order of the axis (e.g.
    /// x is 0, y is 1, etc.). This is the same as in the index
    /// of the axis for the point (`pt[index]` gives the position of
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

    /// Convert a [`crate::interface::cell::CellLocation`] to a [`CellID`]
    #[must_use]
    fn cell_loc_to_id(&self, cell_loc: <Self as MazeCoordinator>::CellLoc) -> CellID {
        let mut offset = cell_loc[0];

        for i in 1..DIMENSION {
            offset += cell_loc[i] * usize::from(self.full_dimensions[i - 1]);
        }

        CellID(offset)
    }

    fn set_unvisited_neighbours_to_wall(&mut self, cell_loc: <Self as MazeCoordinator>::CellLoc) {
        for i in 0..DIMENSION {
            if cell_loc[i] > 0 {
                let neighbour = self.get_cell_value_mut(cell_loc.offset(i, -1));

                if neighbour.cell_type == UNVISITED {
                    neighbour.cell_type = WALL;
                }
            }

            if cell_loc[i] + 1 < usize::from(self.full_dimensions[i]) {
                let neighbour = self.get_cell_value_mut(cell_loc.offset(i, 1));

                if neighbour.cell_type == UNVISITED {
                    neighbour.cell_type = WALL;
                }
            }
        }
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> MazeCoordinator for BoxSpaceBlockCellMazeCoordinator<Buffer, DIMENSION> {
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

pub struct BoxSpaceBlockCellMazeCoordinatorBuilder<Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> {
    _buffer: PhantomData<Buffer>,  // We're not actually interested in constructing a buffer yet
    space: BoxCoordinateSpace<DIMENSION>,
    scale_factor: [NonZeroUsize; DIMENSION],
    padding: [[usize; 2]; DIMENSION],
}

impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellMazeCoordinatorBuilder<Buffer, DIMENSION> {
    pub fn new(space: BoxCoordinateSpace<DIMENSION>) -> Self {
        Self {
            _buffer: PhantomData,
            space,
            scale_factor: [NONZERO_USIZE_TWO; DIMENSION],
            padding: [[1, 1]; DIMENSION],
        }
    }

    pub fn scale_factor(mut self, scale_factor: [NonZeroUsize; DIMENSION]) -> Self {
        self.scale_factor = scale_factor;

        return self
    }

    pub fn scale_factor_checked(self, scale_factor: [usize; DIMENSION]) -> Self {
        self.scale_factor(try_usize_array_to_nonzero_usize_array(scale_factor).expect("All scalars must be non-zero"))
    }

    pub fn padding(mut self, padding: [[usize; 2]; DIMENSION]) -> Self {
        self.padding = padding;

        return self
    }

    pub fn build(&self) -> BoxSpaceBlockCellMazeCoordinator<Buffer, DIMENSION> {
        BoxSpaceBlockCellMazeCoordinator::new(self.space, self.scale_factor, self.padding)
    }
}

/*
 * We want to show the state of the maze in the debug output for 2D mazes.
 *
 * To this end, we must manually implement Debug for BoxSpaceBlockCellMazeCoordinator,
 * then provide a specialisation where DIMENSION = 2.
 *
 * The reason we must manually implement Debug is because #[derive(Debug)] does
 * not mark its implementation as `default`, which we need in order to specialise.
 */

impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellMazeCoordinator<Buffer, DIMENSION> {
    fn write_main_dbg_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BoxSpaceBlockCellMazeCoordinator {{")?;
        writeln!(f, "\tbuffer: {:?}", self.buffer)?;
        writeln!(f, "\tspace: {:?}", self.space)?;
        writeln!(f, "\tresolution: {:?}", self.scale_factor)?;
        writeln!(f, "\tpadding: {:?}", self.padding)?;
        writeln!(f, "\tfull_dimensions: {:?}", self.full_dimensions)?;

        return Ok(())
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> Debug for BoxSpaceBlockCellMazeCoordinator<Buffer, DIMENSION> {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_main_dbg_fmt(f)?;
        writeln!(f, "}}")?;

        return Ok(())
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>> Debug for BoxSpaceBlockCellMazeCoordinator<Buffer, 2> {
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