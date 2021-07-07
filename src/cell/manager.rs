use crate::geometry::space::{CoordinateSpace, BoxCoordinateSpace};
use crate::cell::data::{CellData, Basic};
use std::fmt::Debug;
use crate::util::{get_neighbouring_unvisiteds, get_common_axis, Product};
#[cfg(debug_assertions)] use crate::util::absolute_difference;
use crate::geometry::node::CoordinateTuplet;
use std::convert::TryInto;
use crate::buffer::MazeBuffer;
use std::marker::PhantomData;

// Cleaner type signatures TM
macro_rules! pt_type {
    () => { <<Self as CellSpace>::CoordSpace as CoordinateSpace>::PtType };
}

/// A cell space determines how cells physically connect.
///
/// It operates on a conceptual level between the raw storage (the buffer) and the abstract
/// graph representation of the maze - the cell space arbitrates how individual cells connect.
///
/// A CellSpace handles updating the maze buffer. When you or a generator want to modify a
/// cell, you must talk to the associated cell space.
///
/// For instance, if the generator wants to connect two cells together as passages, the
/// cell space is responsible for updating the buffer with the relevant cell changes
/// (e.g. removing the adjacent wall from the origin cell, and marking the destination cell
/// as visited).
///
/// A maze is accompanied by a cell space at all times, and it cannot be changed after creation.
/// It is a logical error to use a cell space with a different maze object, or a maze object
/// with a different cell space.
///
/// Passing in an out-of-bounds point as an argument will cause a panic with debug assertions
/// enabled, and is undefined with them off. However, an out-of-bounds point will never cause
/// an out-of-bounds write; memory safety is always preserved.
pub trait CellManager: Send + Sync + Debug {
    type CoordSpace: CoordinateSpace;
    type CellType: CellData;

    // fn space(&self) -> &Self::CoordSpace;

    // /// Is this the 1st, 2nd, 3rd, etc. node? Used as a point ID. Must be contiguous,
    // /// starting from 0, and the maximum value must be less than [`self::size()`].
    // /// For internal storage purposes.
    // fn ordinal_of_pt(&self, pt: pt_type!()) -> usize;
    // 
    // /// Like [`self::ordinal_of_pt()`], but for individual buffer cells.
    // /// For internal storage purposes.
    // fn ordinal_of_cell(&self, pt: pt_type!()) -> usize;

    fn get_cell(&self, cell: usize) -> Self::CellType;

    // TODO allow making pts wherever? (or just of dimension n-1? (e.g. in 2D you have a line, in 3D you can make a wall)
    // TODO maybe allow having from == to?

    fn make_passage(&mut self, cell: usize);
    /// Note: from and to must be adjacent
    fn make_passage_between(&mut self, from: usize, to: usize);
    fn make_wall(&mut self, cell: usize);
    /// Note: from and to must be adjacent
    fn make_wall_between(&mut self, from: usize, to: usize);
    fn make_boundary(&mut self, cell: usize);
    /// Note: from and to must be adjacent
    fn make_boundary_between(&mut self, from: usize, to: usize);
}

#[derive(Debug)]
pub struct BasicCellSpace<CoordSpace: CoordinateSpace, Buffer: MazeBuffer<CoordSpace, Basic>> {
    buffer: Buffer
}

impl <CoordSpace: CoordinateSpace, Buffer: MazeBuffer<CoordSpace, Basic>> BasicCellSpace<CoordSpace, Buffer> {
    fn ordinal_of_cell(pt: pt_type!()) -> usize {
        let tuplet: [usize; DIMENSION] = pt.into();
        let dimensions: [usize; DIMENSION] = self.buffer.space().dimensions();

        #[cfg(debug_assertions)]
        for pair in dimensions.zip(tuplet) {
            if pair.1 > pair.0 { // If coord.x >= space.width
                panic!("Cell {:?} is out of bounds (space: {:?})", pt, dimensions)
            }
        }

        let mut ordinal = tuplet[0];

        for i in 1..DIMENSION {
            ordinal += tuplet[i] * dimensions[i - 1];
        }

        return ordinal
    }
}

impl <CoordSpace: CoordinateSpace, Buffer: MazeBuffer<CoordSpace, Basic>> CellManager for BasicCellSpace<CoordSpace, Buffer> {
    type CoordSpace = CoordSpace;
    type CellType = Basic;

    fn get_cell(&self, cell: usize) -> Self::CellType {
        todo!()
    }

    fn make_passage(&mut self, cell: usize) {
        todo!()
    }

    fn make_passage_between(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn make_wall(&mut self, cell: usize) {
        todo!()
    }

    fn make_wall_between(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn make_boundary(&mut self, cell: usize) {
        todo!()
    }

    fn make_boundary_between(&mut self, from: usize, to: usize) {
        todo!()
    }
}

//region UnalignedBoxyCellSpace

// I think eventually the best approach would be to re-use this code as a specialisation for
// AlignedBoxyCellSpace where scale == 1, when specialisation becomes possible

#[derive(Debug)]
pub struct UnalignedBoxyCellSpace<Buffer, CoordSpace, const DIMENSION: usize> where
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]>,
        Buffer: MazeBuffer<CoordSpace, Basic> {
    buffer: Buffer,
    _space: PhantomData<CoordSpace>
}

impl <Buffer, CoordSpace, const DIMENSION: usize> UnalignedBoxyCellSpace<Buffer, CoordSpace, DIMENSION> where
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]>,
        Buffer: MazeBuffer<CoordSpace, Basic> {
    pub fn new(buffer: Buffer) -> Self {
        Self { buffer, _space: PhantomData }
    }
}

impl <Buffer, CoordSpace, const DIMENSION: usize> CellManager for UnalignedBoxyCellSpace<Buffer, CoordSpace, DIMENSION> where
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]>,
        Buffer: MazeBuffer<CoordSpace, Basic> {
    type CellType = Basic;
    type CoordSpace = CoordSpace;

    fn cells_required(space: &Self::CoordSpace) -> usize {
        space.logical_size()
    }

    fn space(&self) -> &Self::CoordSpace {
        self.buffer.space()
    }

    fn ordinal_of_pt(&self, pt: pt_type!()) -> usize {
        let tuplet: [usize; DIMENSION] = pt.into();
        let dimensions: [usize; DIMENSION] = self.buffer.space().dimensions();

        #[cfg(debug_assertions)]
        for pair in dimensions.zip(tuplet) {
            if pair.1 > pair.0 { // If coord.x >= space.width
                panic!("Point {:?} is out of bounds (space: {:?})", pt, dimensions)
            }
        }

        self.ordinal_of_cell(pt)
    }

    fn ordinal_of_cell(&self, pt: pt_type!()) -> usize {
        let tuplet: [usize; DIMENSION] = pt.into();
        let dimensions: [usize; DIMENSION] = self.buffer.space().dimensions();

        #[cfg(debug_assertions)]
        for pair in dimensions.zip(tuplet) {
            if pair.1 > pair.0 { // If coord.x >= space.width
                panic!("Cell {:?} is out of bounds (space: {:?})", pt, dimensions)
            }
        }

        let mut ordinal = tuplet[0];

        for i in 1..DIMENSION {
            ordinal += tuplet[i] * dimensions[i - 1];
        }

        return ordinal
    }

    fn get_pt(&self, pt: pt_type!()) -> Self::CellType {
        self.buffer.get_cell(self.ordinal_of_pt(pt))
    }

    fn get_cell(&self, pt: pt_type!()) -> Self::CellType {
        self.buffer.get_cell(self.ordinal_of_cell(pt))
    }

    fn is_passage(&self, pt: <Self::CoordSpace as CoordinateSpace>::PtType) -> bool {
        self.get_pt(pt).is_passage()
    }

    fn is_wall(&self, pt: pt_type!()) -> bool {
        self.get_pt(pt).is_wall()
    }

    fn is_boundary(&self, pt: pt_type!()) -> bool {
        self.get_pt(pt).is_boundary()
    }

    fn is_unvisited(&self, pt: pt_type!()) -> bool {
        self.get_pt(pt).is_unvisited()
    }

    fn make_passage(&mut self, pt: pt_type!()) {
        self.buffer.set_cell(self.ordinal_of_pt(pt), Self::CellType::PASSAGE)
    }

    /// Note: from and to must be adjacent
    fn make_passage_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        #[cfg(debug_assertions)]
        if CoordSpace::are_adjacent(from, to) == false {
            panic!("from and to must be adjacent")
        }

        #[cfg(debug_assertions)]
        if self.buffer.get_cell(self.ordinal_of_pt(from)).is_boundary() || self.buffer.get_cell(self.ordinal_of_pt(to)).is_boundary() {
            panic!("cannot make a passage that crosses a boundary")
        }

        // Passages

        self.make_passage(from);
        self.make_passage(to);

        // Walls

        {
            let mut neighbours = get_neighbouring_unvisiteds(self, from);
            neighbours.retain(|&neighbour| neighbour != to);
            for neighbour in neighbours { self.make_wall(neighbour) }
        }

        {
            let mut neighbours = get_neighbouring_unvisiteds(self, to);
            neighbours.retain(|&neighbour| neighbour != from);
            for neighbour in neighbours { Self::make_wall(self, neighbour) }
        }
    }

    fn make_wall(&mut self, pt: pt_type!()) {
        self.buffer.set_cell(self.ordinal_of_pt(pt), Self::CellType::WALL)
    }

    /// Note: from and to must be adjacent
    fn make_wall_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        #[cfg(debug_assertions)]
        if CoordSpace::are_adjacent(from, to) == false {
            panic!("from and to must be adjacent")
        }

        #[cfg(debug_assertions)]
        if self.get_pt(from).is_boundary() || self.get_pt(to).is_boundary() {
            panic!("cannot make a wall that crosses a boundary")
        }

        self.make_wall(from);
        self.make_wall(to);
    }

    fn make_boundary(&mut self, pt: pt_type!()) {
        self.buffer.set_cell(self.ordinal_of_pt(pt), Self::CellType::BOUNDARY)
    }

    /// Note: from and to must be adjacent
    fn make_boundary_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        #[cfg(debug_assertions)]
        if CoordSpace::are_adjacent(from, to) == false {
            panic!("from and to must be adjacent")
        }

        self.make_boundary(from);
        self.make_boundary(to);
    }
}

impl <Buffer, CoordSpace: CoordinateSpace, const DIMENSION: usize> BoxyCellSpace<CoordSpace, DIMENSION> for UnalignedBoxyCellSpace<Buffer, CoordSpace, DIMENSION> where
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]>,
        Buffer: MazeBuffer<CoordSpace, Basic> {
    fn scale() -> usize {
        1
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    #[allow(unused_variables)]
    fn make_extended_passage_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        let common_axis = get_common_axis(from, to).expect("from and to must be colinear parallel to a coordinate axis");

        let smallest;
        let largest;

        if from[common_axis] < to[common_axis] {
            smallest = from;
            largest = to;
        } else {
            smallest = to;
            largest = from;
        }

        // Let's get rid of from and to so we don't accidentally re-use them
        let from = ();
        let to = ();

        // We keep start separate because for the first tile we don't have to worry about overwriting a previous passage
        let start: pt_type!() = smallest.into();

        // All the points to be changed, except the start;
        let mut pts: Vec<pt_type!()> = Vec::with_capacity(largest[common_axis] - smallest[common_axis]);

        for i in 1..=largest[common_axis] {
            let mut additional_pt = smallest.clone();

            additional_pt[common_axis] += i;

            pts.push(additional_pt.into());
        }

        // More unbinding
        let smallest = ();
        let largest = ();

        #[cfg(debug_assertions)]
        if self.get_pt(start).is_boundary() || pts.iter().any(|pt| self.get_pt(*pt).is_boundary()) {
            panic!("cannot make a passage that crosses a boundary")
        }

        // Passages

        self.make_passage(start);

        for pt in &pts {
            self.make_passage(*pt);
        }

        // Walls

        // Start
        {
            let mut neighbours = get_neighbouring_unvisiteds(self, start);

            // pts[0] is the next pt, immediately adjacent to start
            neighbours.retain(|&neighbour| neighbour != pts[0]);

            for neighbour in neighbours {
                self.make_wall(neighbour)
            }
        }

        // Everything else
        {
            let mut ignoring = start;

            for pt in pts {
                let mut neighbours = get_neighbouring_unvisiteds(self, pt);

                neighbours.retain(|&neighbour| neighbour != ignoring);

                for neighbour in neighbours {
                    self.make_wall(neighbour)
                }

                ignoring = pt;
            }
        }
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_wall_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        let common_axis = get_common_axis(from, to).expect("from and to must be colinear parallel to a coordinate axis");

        let smallest;
        let largest;

        if from[common_axis] < to[common_axis] {
            smallest = from;
            largest = to;
        } else {
            smallest = to;
            largest = from;
        }

        // Go through all the points between them and set them
        for i in 1..=largest[common_axis] {
            let mut pt = smallest.clone();

            pt[common_axis] += i;

            let pt: pt_type!() = pt.into();

            #[cfg(debug_assertions)]
            if self.get_pt(pt).is_boundary() {
                panic!("cannot make a wall that crosses a boundary")
            }

            self.make_wall(pt);
        }
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_boundary_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        let common_axis = get_common_axis(from, to).expect("from and to must be colinear parallel to a coordinate axis");

        let smallest;
        let largest;

        if from[common_axis] < to[common_axis] {
            smallest = from;
            largest = to;
        } else {
            smallest = to;
            largest = from;
        }

        // Go through all the points between them and set them
        for i in 1..=largest[common_axis] {
            let mut pt = smallest.clone();

            pt[common_axis] += i;

            self.make_boundary(pt.into());
        }
    }

    #[inline]
    fn make_unaligned_extended_passage_between(&mut self, from: <Self::CoordSpace as CoordinateSpace>::PtType, to: <Self::CoordSpace as CoordinateSpace>::PtType) {
        Self::make_extended_passage_between(self, from, to);
    }

    #[inline]
    fn make_unaligned_extended_wall_between(&mut self, from: <Self::CoordSpace as CoordinateSpace>::PtType, to: <Self::CoordSpace as CoordinateSpace>::PtType) {
        Self::make_extended_wall_between(self, from, to);
    }

    #[inline]
    fn make_unaligned_extended_boundary_between(&mut self, from: <Self::CoordSpace as CoordinateSpace>::PtType, to: <Self::CoordSpace as CoordinateSpace>::PtType) {
        Self::make_extended_boundary_between(self, from, to);
    }
}

//endregion

//region AlignedBoxyCellSpace

#[derive(Debug)]
pub struct AlignedBoxyCellSpace<Buffer, CoordSpace, const DIMENSION: usize> where
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]>,
        Buffer: MazeBuffer<CoordSpace, Basic> {
    buffer: Buffer,
    _space: PhantomData<CoordSpace>,
}

impl <Buffer, CoordSpace, const DIMENSION: usize> AlignedBoxyCellSpace<Buffer, CoordSpace, { DIMENSION }> where
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]>,
        Buffer: MazeBuffer<CoordSpace, Basic> {

    pub fn new(buffer: Buffer) -> Self {
        Self { buffer, _space: PhantomData }
    }
    
    /// Helper function for make_passage, make_wall, etc
    #[inline]
    pub fn make(&mut self, pt: pt_type!(), cell_type: <Self as CellManager>::CellType) {
        let pt: [usize; DIMENSION] = pt.into();

        let pt = pt.map(|coordinate| coordinate * Self::scale() + 1);

        self.make_unaligned(pt.into(), cell_type);
    }

    /// Helper function for make_passage, make_wall, etc
    #[inline]
    pub fn make_unaligned(&mut self, pt: pt_type!(), cell_type: <Self as CellManager>::CellType) {
        self.buffer.set_cell(self.ordinal_of_pt(pt), cell_type);
    }

    pub fn make_passage_unaligned(&mut self, pt: pt_type!()) {
        self.make_unaligned(pt, <Self as CellManager>::CellType::PASSAGE)
    }

    pub fn make_wall_unaligned(&mut self, pt: pt_type!()) {
        self.make_unaligned(pt, <Self as CellManager>::CellType::WALL)
    }

    pub fn make_boundary_unaligned(&mut self, pt: pt_type!()) {
        self.make_unaligned(pt, <Self as CellManager>::CellType::BOUNDARY)
    }

    fn get_adjacent_cells(&self, pt: pt_type!()) -> Vec<pt_type!()> {
        let mut neighbours: Vec<pt_type!()> = Vec::with_capacity(2usize.pow(DIMENSION.try_into().expect("DIMENSION is too big")));

        let pt: [usize; DIMENSION] = pt.into();

        for i in 0..DIMENSION {
            if pt[i] > 0 {
                // Sneakily make a new point w/ a different value without knowing the dimension at write time
                let mut new_pt = pt.clone();

                new_pt[i] -= 1;

                neighbours.push(new_pt.into());
            }

            if pt[i] < self.buffer.space().dimensions()[i] * Self::scale() {
                // Sneakily make a new point w/ a different value without knowing the dimension at write time
                let mut new_pt = pt.clone();

                new_pt[i] += 1;

                neighbours.push(new_pt.into());
            }
        }

        return neighbours;
    }

    fn get_adjacent_unvisited(&self, pt: pt_type!()) -> Vec<pt_type!()> {
        let mut vec = self.get_adjacent_cells(pt);

        vec.retain(|cell| self.is_unvisited(*cell));

        return vec;
    }
}

impl <Buffer, CoordSpace, const DIMENSION: usize> CellManager for AlignedBoxyCellSpace<Buffer, CoordSpace, { DIMENSION }> where
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]>,
        Buffer: MazeBuffer<CoordSpace, Basic> {

    type CellType = Basic;
    type CoordSpace = CoordSpace;

    fn cells_required(space: &Self::CoordSpace) -> usize {
        space.dimensions()
            .map(|dim| dim * Self::scale() + 1)
            .product()
    }

    fn space(&self) -> &Self::CoordSpace {
        self.buffer.space()
    }

    fn ordinal_of_pt(&self, pt: pt_type!()) -> usize {
        let tuplet: [usize; DIMENSION] = pt.into();
        let dimensions: [usize; DIMENSION] = self.buffer.space().dimensions().map(|dimension| dimension * Self::scale() + 1);

        #[cfg(debug_assertions)]
        for pair in dimensions.zip(tuplet) {
            if pair.1 > pair.0 { // If coord.x >= space.width
                panic!("Point {:?} is out of bounds (space: {:?})", pt, dimensions)
            }
        }

        let tuplet = tuplet.map(|element| element * 2);

        return self.ordinal_of_cell(tuplet.into())
    }

    fn ordinal_of_cell(&self, pt: pt_type!()) -> usize {
        let tuplet: [usize; DIMENSION] = pt.into();
        let dimensions: [usize; DIMENSION] = self.buffer.space().dimensions().map(|dimension| dimension * Self::scale() + 1);

        #[cfg(debug_assertions)]
        for pair in dimensions.zip(tuplet) {
            if pair.1 > pair.0 { // If coord.x >= space.width
                panic!("Cell {:?} is out of bounds (space: {:?})", pt, dimensions)
            }
        }

        let mut ordinal = tuplet[0];

        for i in 1..DIMENSION {
            ordinal += tuplet[i] * dimensions[i - 1];
        }

        return ordinal
    }

    fn get_pt(&self, pt: pt_type!()) -> Self::CellType {
        // Scale point
        let pt = pt.into().map(|dim| dim * Self::scale() + 1);

        self.buffer.get_cell(self.ordinal_of_pt(pt.into()))
    }

    fn get_cell(&self, pt: pt_type!()) -> Self::CellType {
        // No scaling
        self.buffer.get_cell(self.ordinal_of_cell(pt))
    }

    fn is_passage(&self, pt: pt_type!()) -> bool {
        self.get_pt(pt).is_passage()
    }

    fn is_wall(&self, pt: pt_type!()) -> bool {
        self.get_pt(pt).is_wall()
    }

    fn is_boundary(&self, pt: pt_type!()) -> bool {
        self.get_pt(pt).is_boundary()
    }

    fn is_unvisited(&self, pt: pt_type!()) -> bool {
        self.get_pt(pt).is_unvisited()
    }

    fn make_passage(&mut self, pt: pt_type!()) {
        self.make(pt, Self::CellType::PASSAGE)
    }

    /// Note: from and to must be adjacent
    fn make_passage_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        #[cfg(debug_assertions)]
        if CoordSpace::are_adjacent(from, to) == false {
            panic!("from and to must be adjacent")
        }

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        // from and to are colinear along the common axis (and are identical otherwise)
        let common_axis = get_common_axis(from, to).expect("from and to must be directly adjacent");

        #[cfg(debug_assertions)]
        if absolute_difference(from[common_axis], to[common_axis]) != 1 {
            panic!("inconsistency - from and to are reported to be adjacent, but the absolute difference between them on the common axis is not 1 but {} (i.e. they are not touching", absolute_difference(from[common_axis], to[common_axis]))
        }

        // Scale points
        let from = from.map(|coord| coord * Self::scale() + 1);
        let to = to.map(|coord| coord * Self::scale() + 1);

        // Get the cell in-between them

        let mut connection_tuplet = from;

        if from[common_axis] < to[common_axis] {
            connection_tuplet[common_axis] += 1
        } else {
            connection_tuplet[common_axis] -= 1
        }

        // We redefine from and to so the unscaled versions are not accidentally used
        let from: pt_type!() = from.into();
        let to: pt_type!() = to.into();
        let connection: pt_type!() = connection_tuplet.into();

        #[cfg(debug_assertions)]
        if self.is_boundary(from) || self.is_boundary(connection) || self.is_boundary(to) {
            panic!("cannot make a passage that crosses a boundary")
        }

        // Passages

        self.make_passage_unaligned(from);
        self.make_passage_unaligned(connection);
        self.make_passage_unaligned(to);

        // Walls

        {
            let mut neighbours = self.get_adjacent_unvisited(from);
            neighbours.retain(|&neighbour| neighbour != connection);
            for neighbour in neighbours { self.make_wall_unaligned(neighbour) }
        }

        {
            let mut neighbours = self.get_adjacent_unvisited(connection);
            neighbours.retain(|&neighbour| neighbour != from && neighbour != to);
            for neighbour in neighbours { self.make_wall_unaligned(neighbour) }
        }

        {
            let mut neighbours = self.get_adjacent_unvisited(to);
            neighbours.retain(|&neighbour| neighbour != connection);
            for neighbour in neighbours { self.make_wall_unaligned(neighbour) }
        }
    }

    fn make_wall(&mut self, pt: pt_type!()) {
        self.make(pt, Self::CellType::WALL)
    }

    /// Note: from and to must be adjacent
    fn make_wall_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        #[cfg(debug_assertions)]
        if CoordSpace::are_adjacent(from, to) == false {
            panic!("from and to must be adjacent")
        }

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        // from and to are colinear along the common axis (and are identical otherwise)
        let common_axis = get_common_axis(from, to).expect("from and to must be directly adjacent");

        #[cfg(debug_assertions)]
        if absolute_difference(from[common_axis], to[common_axis]) != 1 {
            panic!("inconsistency - from and to are reported to be adjacent, but the absolute difference between them on the common axis is not 1 but {} (i.e. they are not touching", absolute_difference(from[common_axis], to[common_axis]))
        }

        // Scale the points
        let from = from.map(|coord| coord * Self::scale() + 1);
        let to = to.map(|coord| coord * Self::scale() + 1);

        // Get the cell in-between them

        let mut connection_tuplet = from;

        if from[common_axis] < to[common_axis] {
            connection_tuplet[common_axis] += 1
        } else {
            connection_tuplet[common_axis] -= 1
        }

        // We redefine from and to so the unscaled versions are not accidentally used
        let from: pt_type!() = from.into();
        let to: pt_type!() = to.into();
        let connection: pt_type!() = connection_tuplet.into();

        #[cfg(debug_assertions)]
        if self.is_boundary(from) || self.is_boundary(connection) || self.is_boundary(to) {
            panic!("cannot make a wall that crosses a boundary")
        }

        self.make_wall_unaligned(from);
        self.make_wall_unaligned(connection);
        self.make_wall_unaligned(to);
    }

    fn make_boundary(&mut self, pt: pt_type!()) {
        self.make(pt, Self::CellType::BOUNDARY)
    }

    /// Note: from and to must be adjacent
    fn make_boundary_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        #[cfg(debug_assertions)]
        if CoordSpace::are_adjacent(from, to) == false {
            panic!("from and to must be adjacent")
        }

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        // from and to are colinear along the common axis (and are identical otherwise)
        let common_axis = get_common_axis(from, to).expect("from and to must be directly adjacent");

        #[cfg(debug_assertions)]
        if absolute_difference(from[common_axis], to[common_axis]) != 1 {
            panic!("inconsistency - from and to are reported to be adjacent, but the absolute difference between them on the common axis is not 1 but {} (i.e. they are not touching", absolute_difference(from[common_axis], to[common_axis]))
        }

        // Scale the points
        let from = from.map(|coord| coord * Self::scale() + 1);
        let to = to.map(|coord| coord * 2+ 1);

        // Get the cell in-between them

        let mut connection_tuplet = from;

        if from[common_axis] < to[common_axis] {
            connection_tuplet[common_axis] += 1
        } else {
            connection_tuplet[common_axis] -= 1
        }

        self.make_boundary_unaligned(from.into());
        self.make_boundary_unaligned(connection_tuplet.into());
        self.make_boundary_unaligned(to.into());
    }
}

impl <Buffer, CoordSpace: CoordinateSpace, const DIMENSION: usize> BoxyCellSpace<CoordSpace, DIMENSION> for AlignedBoxyCellSpace<Buffer, CoordSpace, DIMENSION> where
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]>,
        Buffer: MazeBuffer<CoordSpace, Basic> {
    // TODO make this configurable
    #[inline]
    fn scale() -> usize {
        2
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    #[allow(unused_variables)]
    fn make_extended_passage_between(&mut self, from: pt_type!(), to: pt_type!()) {
        // Just scale the points then send it off

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        let from = from.map(|coord| coord * Self::scale() + 1);
        let to = to.map(|coord| coord * Self::scale() + 1);

        self.make_unaligned_extended_passage_between(from.into(), to.into());
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_wall_between(&mut self, from: pt_type!(), to: pt_type!()) {
        // Just scale the points then send it off

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        let from = from.map(|coord| coord * Self::scale() + 1);
        let to = to.map(|coord| coord * Self::scale() + 1);

        self.make_unaligned_extended_passage_between(from.into(), to.into());
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_boundary_between(&mut self, from: pt_type!(), to: pt_type!()) {
        // Just scale the points then send it off

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        let from = from.map(|coord| coord * Self::scale() + 1);
        let to = to.map(|coord| coord * Self::scale() + 1);

        self.make_unaligned_extended_passage_between(from.into(), to.into());
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    #[allow(unused_variables)]
    fn make_unaligned_extended_passage_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        // from and to are colinear along the common axis (and are identical otherwise)
        let common_axis = get_common_axis(from, to).expect("from and to must be colinear parallel to a coordinate axis");

        let smallest;
        let largest;

        if from[common_axis] < to[common_axis] {
            smallest = from;
            largest = to;
        } else {
            smallest = to;
            largest = from;
        }

        // Let's get rid of from and to so we don't accidentally re-use them
        let from = ();
        let to = ();

        // We keep start separate because for the first tile we don't have to worry about overwriting a previous passage
        let start: pt_type!() = smallest.into();

        // All the points to be changed, except the start;
        let mut pts: Vec<pt_type!()> = Vec::with_capacity(largest[common_axis] - smallest[common_axis]);

        for i in 1..=largest[common_axis] {
            let mut additional_pt = smallest.clone();

            additional_pt[common_axis] += i;

            pts.push(additional_pt.into());
        }

        // More unbinding
        let smallest = ();
        let largest = ();

        #[cfg(debug_assertions)]
        if self.is_boundary(start) || pts.iter().any(|pt| self.is_boundary(*pt)) {
            panic!("cannot make a passage that crosses a boundary")
        }

        // Passages

        self.make_passage(start);

        for pt in &pts {
            self.make_passage(*pt);
        }

        // Walls

        // Start
        {
            let mut neighbours = self.get_adjacent_unvisited(start);

            // pts[0] is the next pt, immediately adjacent to start
            neighbours.retain(|&neighbour| neighbour != pts[0]);

            for neighbour in neighbours {
                self.make_wall(neighbour)
            }
        }

        // Everything else
        {
            let mut ignoring = start;

            for pt in pts {
                let mut neighbours = self.get_adjacent_unvisited(pt);

                neighbours.retain(|&neighbour| neighbour != ignoring);

                for neighbour in neighbours {
                    self.make_passage_unaligned(neighbour)
                }

                ignoring = pt;
            }
        }
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_unaligned_extended_wall_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        // from and to are colinear along the common axis (and are identical otherwise)
        let common_axis = get_common_axis(from, to).expect("from and to must be colinear parallel to a coordinate axis");

        let smallest;
        let largest;

        if from[common_axis] < to[common_axis] {
            smallest = from;
            largest = to;
        } else {
            smallest = to;
            largest = from;
        }

        // Go through all the points between them and set them
        for i in 0..=largest[common_axis] {
            let mut pt = smallest.clone();

            pt[common_axis] += i;

            let pt: pt_type!() = pt.into();

            #[cfg(debug_assertions)]
            if self.get_pt(pt).is_boundary() {
                panic!("cannot make a wall that crosses a boundary")
            }

            self.make_wall_unaligned(pt);
        }
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_unaligned_extended_boundary_between(&mut self, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        // from and to are colinear along the common axis (and are identical otherwise)
        let common_axis = get_common_axis(from, to).expect("from and to must be colinear parallel to a coordinate axis");

        let smallest;
        let largest;

        if from[common_axis] < to[common_axis] {
            smallest = from;
            largest = to;
        } else {
            smallest = to;
            largest = from;
        }

        // Go through all the points between them and set them
        for i in 0..=largest[common_axis] {
            let mut pt = smallest.clone();

            pt[common_axis] += i;

            self.make_boundary_unaligned(pt.into());
        }
    }
}

//endregion