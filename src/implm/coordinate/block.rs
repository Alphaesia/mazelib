//! Coordinators for [block cells][crate::implm::cell::block].
//!
//! # Comparison by Example
//!
//! Here is a comparison of a typical maze of each coordinator. You can see commentary on each
//! example on their individual pages.
//!
//! [`BoxSpaceBlockCellMazeCoordinator`]:
//!
//! ![A pixellated-looking maze, where every cell is one pixel][box-space-block-cell-coordinator-example]
#![doc = embed_doc_image::embed_image!("box-space-block-cell-coordinator-example", "src/doc/img/coordinate/box-space-block-cell/example-large.png")]

use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::num::NonZeroUsize;
use embed_doc_image::embed_doc_image;

use crate::implm::cell::block::{BlockCellValue, BlockCellPrimaryValue};
use crate::implm::cell::block::BlockCellLocation;
use crate::implm::cell::block::BlockCellPrimaryValue::{BOUNDARY, PASSAGE, UNVISITED, WALL};
use crate::implm::export::text::BoxSpaceBlockCellTextMazeExporter;
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::{CellID, ConnectionType};
use crate::interface::coordinate::MazeCoordinator;
use crate::interface::export::MazeExporter;
use crate::interface::point::CoordinateSpace;
use crate::internal::array_util::{ArrayZipMap, CheckedProduct, CheckedSum};
use crate::internal::noise_util::pt;
use crate::internal::util::{NONZERO_USIZE_TWO, try_usize_array_to_nonzero_usize_array};

/// A maze coordinator that maps a box-like coordinate space to box-like cells.
/// 
/// ![A pixellated-looking maze, where every cell is one pixel][box-space-block-cell-coordinator-example]
/// 
/// It produces mazes with a distinct pixellated look.
/// 
/// Every point is mapped to a box of cells (a square, cube, etc.). The dimensionality of the box is
/// the same as the coordinate space, so for a 2D maze they would be squares. Every box is exactly
/// the same size. The width, height, depth, etc. of these boxes is controlled by the coordinator's
/// *scale factors*. Every scale factor must be at least 1 (so every point maps to at least one
/// cell). Additionally, there may be some *padding* cells. These exist on the edge of the cell
/// space and are not mapped by any point. Each edge may have zero or more padding cells, and
/// different edges can have different amounts. They're useful for adding borders to mazes.
/// 
/// TODO insert annotated diagram
///
/// # Examples
///
/// With scaling and padding:
/// ```
/// use mazelib::implm::point::boxy::TwoDimensionalBoxCoordinateSpace;
/// use mazelib::implm::cell::block::BlockCellValue;
/// use mazelib::implm::buffer::VecBuffer;
/// use mazelib::implm::coordinate::block::BoxSpaceBlockCellMazeCoordinator;
///
/// let coord_space = TwoDimensionalBoxCoordinateSpace::new_checked([11, 11]); // Standard 2D coordinate space
/// let scale_factors = [2, 2]; // A simple scale factor of 2 for a clean look
/// let padding = [[1, 1], [1, 1]]; // 1 cell on each side for a border
/// type CellType = BlockCellValue; // Pixelated maze style
///
/// type MazeBuffer = VecBuffer<CellType>;
///
/// let maze = BoxSpaceBlockCellMazeCoordinator::<MazeBuffer, 2>::builder(coord_space)
///     .scale_factors_checked(scale_factors)
///     .padding(padding)
///     .build();
/// ```
/// 
/// TODO add image showing result
#[embed_doc_image("box-space-block-cell-coordinator-example", "src/doc/img/coordinate/box-space-block-cell/example-large.png")]
pub struct BoxSpaceBlockCellMazeCoordinator<Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> {
    /// The maze buffer the maze is stored in.
    buffer: Buffer,
    /// The maze's coordinate space.
    space: BoxCoordinateSpace<DIMENSION>,
    /// The number of cells a point corresponds to, for each axis.
    /// 
    /// The scale factors are ordered from most minor axis to most major.
    scale_factors: [NonZeroUsize; DIMENSION],
    /// The number of cells on the edge of the maze that are not mapped to any point, for each
    /// direction.
    ///
    /// The scale factor pairs are ordered from most minor axis to most major. The number of cells
    /// on the negative edge is the first in each pair, and the number on the positive edge is
    /// second.
    padding: [[usize; 2]; DIMENSION],
    /// The dimensions of the cell space, derived from the coordinate space's dimensions, scaled,
    /// and padded. Cached for performance.
    full_dimensions: [NonZeroUsize; DIMENSION],
}

// Constructor (private - use the builder)
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellMazeCoordinator<Buffer, DIMENSION> {
    /// Construct a new maze from a given coordinate space, scale factor, and padding.
    ///
    /// # Parameters
    ///
    /// `space`         --- the coordinate space for this maze.
    /// `scale_factors` --- the scale factor for points to cells. You can think about it
    ///                     like `cells:points`. A scale factor of 3 will yield 3 cells
    ///                     for every point (a 3:1 ratio). Each axis is scaled independently.
    ///                     The ordering of the scale factors is the same as the standard ordering
    ///                     of the coordinate axes.
    /// `padding`       --- How many extra cells to place between edge-adjacent points
    ///                     and the edge of the maze. For example, a value of 1 will yield
    ///                     a 1-cell thick "border" along that edge of the maze. The ordering
    ///                     of the outer elements is the same as the standard ordering of
    ///                     the coordinate axes. For the inner elements, the side closest
    ///                     to zero comes before the side furthest from zero.
    ///
    /// # Type Parameter
    ///
    /// `Buffer` --- the type of buffer to use. A buffer instance will be automatically
    ///              constructed from this type.
    #[must_use]
    fn new(space: BoxCoordinateSpace<DIMENSION>, scale_factors: [NonZeroUsize; DIMENSION], padding: [[usize; 2]; DIMENSION]) -> Self {
        // Arithmetic is so easy and beautiful and succinct
        let full_dimensions = space.dimensions()
            .zip_map(&scale_factors, |dim, scale_factor| {
                // NonZeroUsize::new only returns None if the scaled dimension == usize::MAX (which I don't
                // think is mathematically even possible, but in such a hypothetical case) the +1 would cause
                // the sum to overflow to zero
                (usize::from(*dim) - 1).checked_mul(usize::from(*scale_factor))
                    .and_then(|scaled_dim| NonZeroUsize::new(scaled_dim + 1))
                    .expect("The scaled dimensions do not all fit within a usize")
            })
            .zip_map(&padding, |scaled_dim, padding| {
                padding.checked_sum().and_then(|summed_padding| scaled_dim.checked_add(summed_padding)).expect("The full dimensions do not all fit within a usize")
            });

        let cells_required = full_dimensions.checked_product().expect("The full dimensions specified are too large. The number of cells in the maze does not fit within a usize.");

        Self { buffer: Buffer::new(cells_required), space, scale_factors, full_dimensions, padding }
    }
}

// Public functions
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellMazeCoordinator<Buffer, DIMENSION> {
    #[must_use]
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// The dimensions of the coordinate space, scaled by the scale factors, plus padding.
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
    pub fn scale_factors(&self) -> [NonZeroUsize; DIMENSION] {
        self.scale_factors
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
            pt[i] *= usize::from(self.scale_factors[i]);
            pt[i] += self.padding[i][0];
        }

        BlockCellLocation(pt.into())
    }

    /// Get the value of the point `pt` for mutation.
    ///
    /// # See Also
    /// 
    /// [`get()`][Self::get] for non-mutable borrowing.
    #[must_use]
    pub fn get_mut(&mut self, pt: pt!()) -> &mut <Self as MazeCoordinator>::CellVal {
        self.get_cell_value_mut(self.map_pt_to_cell_loc(pt))
    }

    /// Sugar for
    /// ```
    /// # use mazelib::implm::buffer::VecBuffer;
    /// # use mazelib::implm::cell::block::{BlockCellValue, BlockCellPrimaryValue};
    /// # use mazelib::implm::cell::inline::InlineCellValue;
    /// # use mazelib::implm::coordinate::block::BoxSpaceBlockCellMazeCoordinator;
    /// # use mazelib::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};
    /// # let mut maze = BoxSpaceBlockCellMazeCoordinator::<VecBuffer<BlockCellValue>, 1>::builder(BoxCoordinateSpace::new_checked([1])).build();
    /// # let pt: CoordinateTuplet<1> = [0].into();
    /// # let cell_type = BlockCellPrimaryValue::PASSAGE;
    /// #
    /// maze.get_mut(pt).cell_type = cell_type;
    /// ```
    ///
    /// Modifies the cell's type but keeps its mark status intact.
    ///
    /// Please see [`get_mut()`][Self::get_mut] for details.
    pub fn set_type(&mut self, pt: pt!(), cell_type: BlockCellPrimaryValue) {
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
    /// # See Also
    /// [`Self::get_cell_value()`] for non-mutable borrowing.
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
    /// # use mazelib::implm::cell::block::{BlockCellLocation, BlockCellValue, BlockCellPrimaryValue};
    /// # use mazelib::implm::cell::inline::InlineCellValue;
    /// # use mazelib::implm::coordinate::block::BoxSpaceBlockCellMazeCoordinator;
    /// # use mazelib::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};
    /// # let mut maze = BoxSpaceBlockCellMazeCoordinator::<VecBuffer<BlockCellValue>, 1>::builder(BoxCoordinateSpace::new_checked([1])).build();
    /// # let loc: BlockCellLocation<1> = [0].into();
    /// # let cell_type = BlockCellPrimaryValue::PASSAGE;
    /// #
    /// maze.get_cell_value_mut(loc).cell_type = cell_type;
    /// ```
    ///
    /// Please see [`Self::get_cell_value_mut()`] for details.
    pub fn set_cell_value_type(&mut self, loc: <Self as MazeCoordinator>::CellLoc, cell_type: BlockCellPrimaryValue) {
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

    //noinspection RsUnnecessaryQualifications
    /// Set `pt` to [`BlockCellPrimaryValue::PASSAGE`].
    fn make_passage(&mut self, pt: pt!()) {
        let cell_loc = self.map_pt_to_cell_loc(pt);

        self.set_cell_value_type(cell_loc, PASSAGE);
        self.set_unvisited_neighbours_to_wall(cell_loc);
    }

    //noinspection RsUnnecessaryQualifications
    /// Set `from` and `to` to [`BlockCellPrimaryValue::PASSAGE`]. If the scale factor along the axis
    /// of adjacency is greater than 1, then all intermediate cells will be set to passages too.
    ///
    /// All cells that are adjacent to from, or any of the intermediate cells, and are unvisited,
    /// will be set to [`BlockCellPrimaryValue::WALL`]. Note that this excludes `to`, so that
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

    //noinspection RsUnnecessaryQualifications
    /// Set `pt` to [`BlockCellPrimaryValue::WALL`].
    fn make_wall(&mut self, pt: pt!()) {
        self.set_type(pt, WALL);
    }

    //noinspection RsUnnecessaryQualifications
    /// Set `from` and `to` to [`BlockCellPrimaryValue::WALL`]. If the scale factor along the axis of
    /// adjacency is greater than 1, then all intermediate cells will be set to walls too.
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

    //noinspection RsUnnecessaryQualifications
    /// Set `pt` to [`BlockCellPrimaryValue::BOUNDARY`].
    fn make_boundary(&mut self, pt: pt!()) {
        self.set_type(pt, BOUNDARY);
    }

    //noinspection RsUnnecessaryQualifications
    /// Set `from` and `to` to [`BlockCellPrimaryValue::BOUNDARY`]. If the scale factor along the axis
    /// of adjacency is greater than 1, then all intermediate cells will be set to boundaries too.
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

// Builder
impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellMazeCoordinator<Buffer, DIMENSION> {
    /// Construct a new builder for a `BoxSpaceBlockCellMazeCoordinator`.
    pub fn builder(space: BoxCoordinateSpace<DIMENSION>) -> BoxSpaceBlockCellMazeCoordinatorBuilder<Buffer, DIMENSION> {
        BoxSpaceBlockCellMazeCoordinatorBuilder::new(space)
    }
}

/// A builder for a [`BoxSpaceBlockCellMazeCoordinator`].
#[must_use]
pub struct BoxSpaceBlockCellMazeCoordinatorBuilder<Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> {
    _buffer: PhantomData<Buffer>,  // We're not actually interested in constructing a buffer yet
    /// The maze's coordinate space.
    space: BoxCoordinateSpace<DIMENSION>,
    /// The number of cells a point corresponds to, for each axis.
    /// 
    /// The scale factors are ordered from most minor axis to most major.
    scale_factors: [NonZeroUsize; DIMENSION],
    /// The number of cells on the edge of the maze that are not mapped to any point, for each
    /// direction.
    /// 
    /// The scale factor pairs are ordered from most minor axis to most major. The number of cells
    /// on the negative edge is the first in each pair, and the number on the positive edge is
    /// second.
    padding: [[usize; 2]; DIMENSION],
}

impl <Buffer: MazeBuffer<BlockCellValue>, const DIMENSION: usize> BoxSpaceBlockCellMazeCoordinatorBuilder<Buffer, DIMENSION> {
    /// Construct a new builder for a `BoxSpaceBlockCellMazeCoordinator`.
    /// 
    /// # Parameters
    /// 
    /// `space` --- the coordinate space to use for the maze.
    fn new(space: BoxCoordinateSpace<DIMENSION>) -> Self {
        Self {
            _buffer: PhantomData,
            space,
            scale_factors: [NONZERO_USIZE_TWO; DIMENSION],
            padding: [[1, 1]; DIMENSION],
        }
    }

    /// Set the number of cells a point corresponds to, for each axis.
    /// 
    /// The scale factors are ordered from most minor axis to most major.
    /// 
    /// # See Also
    ///
    /// [`Self::scale_factors_checked()`]. If you're using integer literals, you may with to use
    /// this instead.
    pub fn scale_factors(mut self, scale_factors: [NonZeroUsize; DIMENSION]) -> Self {
        self.scale_factors = scale_factors;

        return self
    }

    /// Set the number of cells a point corresponds to, for each axis.
    /// 
    /// The scale factors are ordered from most minor axis to most major.
    ///
    /// All scale factors must be non-zero.
    ///
    /// # Panics
    ///
    /// If `count` is zero.
    /// 
    /// # See Also
    /// 
    /// [`Self::scale_factors()`], which takes `NonZeroUsize`s.
    pub fn scale_factors_checked(self, scale_factors: [usize; DIMENSION]) -> Self {
        self.scale_factors(try_usize_array_to_nonzero_usize_array(scale_factors).expect("All scale factors must be non-zero"))
    }

    /// Set the number of cells on the edge of the maze that are not mapped to any point, for each
    /// direction.
    ///
    /// The scale factor pairs are ordered from most minor axis to most major. The number of cells
    /// on the negative edge is the first in each pair, and the number on the positive edge is
    /// second.
    pub fn padding(mut self, padding: [[usize; 2]; DIMENSION]) -> Self {
        self.padding = padding;

        return self
    }

    /// Finalise the [`BoxSpaceBlockCellMazeCoordinator`].
    #[must_use]
    pub fn build(&self) -> BoxSpaceBlockCellMazeCoordinator<Buffer, DIMENSION> {
        BoxSpaceBlockCellMazeCoordinator::new(self.space, self.scale_factors, self.padding)
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
        writeln!(f, "\tscale_factors: {:?}", self.scale_factors)?;
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

        let mut text_export = Vec::<u8>::new();

        if let Err(err) = BoxSpaceBlockCellTextMazeExporter::default().export(self, &mut text_export) {
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