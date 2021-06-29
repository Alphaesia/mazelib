use crate::geometry::space::{CoordinateSpace, BoxCoordinateSpace};
use crate::cell::data::{CellData, Basic};
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::util::{get_neighbouring_unvisiteds, get_common_axis, Product};
#[cfg(debug_assertions)] use crate::util::absolute_difference;
use crate::maze::Maze;
use crate::geometry::node::CoordinateTuplet;
use std::convert::TryInto;
use crate::maze;

// Cleaner type signatures TM
macro_rules! pt_type {
    () => { <<Self as CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType };
}

/// A cell space maps an abstract coordinate space into a physical space. It determines
/// where a point is physically, and how it is physically connected to other points.
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
///
/// Passing in an out-of-bounds point as an argument will cause a panic with debug assertions
/// enabled, and is undefined with them off. However, an out-of-bounds point will never cause
/// an out-of-bounds write; memory safety is always preserved.
pub trait CellSpace<Maze: maze::Maze<Self> + ?Sized> : Send + Sync + Debug {
    type CellType: CellData;
    type CoordSpace: CoordinateSpace;

    /// The maximum number of physical cells required (e.g. including passage tiles between intersections)
    fn cells_required(space: &Self::CoordSpace) -> usize;

    /// Is this the 1st, 2nd, 3rd, etc. node? Used as a point ID. Must be contiguous,
    /// starting from 0, and the maximum value must be less than [`self::size()`].
    /// For internal storage purposes.
    fn ordinal(space: <Self as CellSpace<Maze>>::CoordSpace, pt: pt_type!()) -> usize;

    fn is_passage(maze: &Maze, pt: pt_type!()) -> bool;
    fn is_wall(maze: &Maze, pt: pt_type!()) -> bool;
    fn is_boundary(maze: &Maze, pt: pt_type!()) -> bool;
    fn is_unvisited(maze: &Maze, pt: pt_type!()) -> bool;

    // TODO allow making pts wherever? (or just of dimension n-1? (e.g. in 2D you have a line, in 3D you can make a wall)
    // TODO maybe allow having from == to?

    fn make_passage(maze: &mut Maze, pt: pt_type!());
    /// Note: from and to must be adjacent
    fn make_passage_between(maze: &mut Maze, from: pt_type!(), to: pt_type!());
    fn make_wall(maze: &mut Maze, pt: pt_type!());
    /// Note: from and to must be adjacent
    fn make_wall_between(maze: &mut Maze, from: pt_type!(), to: pt_type!());
    fn make_boundary(maze: &mut Maze, pt: pt_type!());
    /// Note: from and to must be adjacent
    fn make_boundary_between(maze: &mut Maze, from: pt_type!(), to: pt_type!());
}

/// A CellSpace that treats walls and passages as separate cells.
///
/// As an analogy, imagine a black-and-white pixel image of a maze,
/// where all white pixels are passages and all black pixels are
/// walls. That is what a BoxyCellSpace is.
///
/// If you have a better name please let me know.
pub trait BoxyCellSpace<Maze, CoordSpace, const DIMENSION: usize>: CellSpace<Maze> where
        Maze: maze::Maze<Self>,
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]> {
    fn scale() -> usize;

    // TODO need better name than "extended"
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_passage_between(maze: &mut Maze, from: pt_type!(), to: pt_type!());
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_wall_between(maze: &mut Maze, from: pt_type!(), to: pt_type!());
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_boundary_between(maze: &mut Maze, from: pt_type!(), to: pt_type!());

    // TODO might take these out / move them somewhere else
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_unaligned_extended_passage_between(maze: &mut Maze, from: pt_type!(), to: pt_type!());
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_unaligned_extended_wall_between(maze: &mut Maze, from: pt_type!(), to: pt_type!());
    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_unaligned_extended_boundary_between(maze: &mut Maze, from: pt_type!(), to: pt_type!());
}

//region UnalignedBoxyCellSpace

// I think eventually the best approach would be to re-use this code as a specialisation for
// AlignedBoxyCellSpace where scale == 1, when specialisation becomes possible

#[derive(Debug)]
pub struct UnalignedBoxyCellSpace<Maze, CoordSpace, const DIMENSION: usize> where
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]> {
    _maze: PhantomData<Maze>,
    _space: PhantomData<CoordSpace>
}

impl <Maze, CoordSpace, const DIMENSION: usize> CellSpace<Maze> for UnalignedBoxyCellSpace<Maze, CoordSpace, DIMENSION> where
        Maze: maze::Maze<Self>,
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]> {
    type CellType = Basic;
    type CoordSpace = CoordSpace;

    fn cells_required(space: &Self::CoordSpace) -> usize {
        space.logical_size()
    }

    fn ordinal(space: <Self as CellSpace<Maze>>::CoordSpace, pt: pt_type!()) -> usize {
        let tuplet: [usize; DIMENSION] = pt.into();
        let dimensions: [usize; DIMENSION] = space.dimensions();

        #[cfg(debug_assertions)]
        for pair in dimensions.zip(tuplet) {
            if pair.1 > pair.0 { // If coord.x >= space.width
                panic!("Point {:?} is out of bounds (space: {:?})", pt, dimensions)
            }
        }

        let mut ordinal = tuplet[0];

        for i in 1..DIMENSION {
            ordinal += tuplet[i] * dimensions[i - 1];
        }

        return ordinal
    }

    fn is_passage(maze: &Maze, pt: pt_type!()) -> bool {
        maze.buffer().get_pt(pt).is_passage()
    }

    fn is_wall(maze: &Maze, pt: pt_type!()) -> bool {
        maze.buffer().get_pt(pt).is_wall()
    }

    fn is_boundary(maze: &Maze, pt: pt_type!()) -> bool {
        maze.buffer().get_pt(pt).is_boundary()
    }

    fn is_unvisited(maze: &Maze, pt: pt_type!()) -> bool {
        maze.buffer().get_pt(pt).is_unvisited()
    }

    fn make_passage(maze: &mut Maze, pt: pt_type!()) {
        maze.mut_buffer().set_pt(pt, Self::CellType::PASSAGE)
    }

    /// Note: from and to must be adjacent
    fn make_passage_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        #[cfg(debug_assertions)]
        if CoordSpace::are_adjacent(from, to) == false {
            panic!("from and to must be adjacent")
        }

        #[cfg(debug_assertions)]
        if maze.buffer().get_pt(from).is_boundary() || maze.buffer().get_pt(to).is_boundary() {
            panic!("cannot make a passage that crosses a boundary")
        }

        // Passages

        Self::make_passage(maze, from);
        Self::make_passage(maze, to);

        // Walls

        {
            let mut neighbours = get_neighbouring_unvisiteds(maze, from);
            neighbours.retain(|&neighbour| neighbour != to);
            for neighbour in neighbours { Self::make_wall(maze, neighbour) }
        }

        {
            let mut neighbours = get_neighbouring_unvisiteds(maze, to);
            neighbours.retain(|&neighbour| neighbour != from);
            for neighbour in neighbours { Self::make_wall(maze, neighbour) }
        }
    }

    fn make_wall(maze: &mut Maze, pt: pt_type!()) {
        maze.mut_buffer().set_pt(pt, Self::CellType::WALL)
    }

    /// Note: from and to must be adjacent
    fn make_wall_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        #[cfg(debug_assertions)]
        if CoordSpace::are_adjacent(from, to) == false {
            panic!("from and to must be adjacent")
        }

        #[cfg(debug_assertions)]
        if maze.buffer().get_pt(from).is_boundary() || maze.buffer().get_pt(to).is_boundary() {
            panic!("cannot make a wall that crosses a boundary")
        }

        Self::make_wall(maze, from);
        Self::make_wall(maze, to);
    }

    fn make_boundary(maze: &mut Maze, pt: pt_type!()) {
        maze.mut_buffer().set_pt(pt, Self::CellType::BOUNDARY)
    }

    /// Note: from and to must be adjacent
    fn make_boundary_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
        #[cfg(debug_assertions)]
        if from == to {
            panic!("from and to must not be the same point")
        }

        #[cfg(debug_assertions)]
        if CoordSpace::are_adjacent(from, to) == false {
            panic!("from and to must be adjacent")
        }

        Self::make_boundary(maze, from);
        Self::make_boundary(maze, to);
    }
}

impl <Maze, CoordSpace: CoordinateSpace, const DIMENSION: usize> BoxyCellSpace<Maze, CoordSpace, DIMENSION> for UnalignedBoxyCellSpace<Maze, CoordSpace, DIMENSION> where
        Maze: maze::Maze<Self>,
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]> {
    fn scale() -> usize {
        1
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    #[allow(unused_variables)]
    fn make_extended_passage_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
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
        if maze.buffer().get_pt(start).is_boundary() || pts.iter().any(|pt| maze.buffer().get_pt(*pt).is_boundary()) {
            panic!("cannot make a passage that crosses a boundary")
        }

        // Passages

        Self::make_passage(maze, start);

        for pt in &pts {
            Self::make_passage(maze, *pt);
        }

        // Walls

        // Start
        {
            let mut neighbours = get_neighbouring_unvisiteds(maze, start);

            // pts[0] is the next pt, immediately adjacent to start
            neighbours.retain(|&neighbour| neighbour != pts[0]);

            for neighbour in neighbours {
                Self::make_wall(maze, neighbour)
            }
        }

        // Everything else
        {
            let mut ignoring = start;

            for pt in pts {
                let mut neighbours = get_neighbouring_unvisiteds(maze, pt);

                neighbours.retain(|&neighbour| neighbour != ignoring);

                for neighbour in neighbours {
                    Self::make_wall(maze, neighbour)
                }

                ignoring = pt;
            }
        }
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_wall_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
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
            if maze.buffer().get_pt(pt).is_boundary() {
                panic!("cannot make a wall that crosses a boundary")
            }

            Self::make_wall(maze, pt);
        }
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_boundary_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
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

            Self::make_boundary(maze, pt.into());
        }
    }

    #[inline]
    fn make_unaligned_extended_passage_between(maze: &mut Maze, from: <Self::CoordSpace as CoordinateSpace>::PtType, to: <Self::CoordSpace as CoordinateSpace>::PtType) {
        Self::make_extended_passage_between(maze, from, to);
    }

    #[inline]
    fn make_unaligned_extended_wall_between(maze: &mut Maze, from: <Self::CoordSpace as CoordinateSpace>::PtType, to: <Self::CoordSpace as CoordinateSpace>::PtType) {
        Self::make_extended_wall_between(maze, from, to);
    }

    #[inline]
    fn make_unaligned_extended_boundary_between(maze: &mut Maze, from: <Self::CoordSpace as CoordinateSpace>::PtType, to: <Self::CoordSpace as CoordinateSpace>::PtType) {
        Self::make_extended_boundary_between(maze, from, to);
    }
}

//endregion

//region AlignedBoxyCellSpace

#[derive(Debug)]
pub struct AlignedBoxyCellSpace<Maze, CoordSpace, const DIMENSION: usize> where
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]> {
    _maze: PhantomData<Maze>,
    _space: PhantomData<CoordSpace>
}

impl <Maze, CoordSpace, const DIMENSION: usize> AlignedBoxyCellSpace<Maze, CoordSpace, { DIMENSION }> where
        Maze: maze::Maze<Self>,
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]> {
    
    /// Helper function for make_passage, make_wall, etc
    #[inline]
    pub fn make(maze: &mut Maze, pt: pt_type!(), cell_type: <Self as CellSpace<Maze>>::CellType) {
        let pt: [usize; DIMENSION] = pt.into();

        let pt = pt.map(|coordinate| coordinate * Self::scale() + 1);

        Self::make_unaligned(maze, pt.into(), cell_type);
    }

    /// Helper function for make_passage, make_wall, etc
    #[inline]
    pub fn make_unaligned(maze: &mut Maze, pt: pt_type!(), cell_type: <Self as CellSpace<Maze>>::CellType) {
        maze.mut_buffer().set_pt(pt, cell_type);
    }

    pub fn make_passage_unaligned(maze: &mut Maze, pt: pt_type!()) {
        Self::make_unaligned(maze, pt, <Self as CellSpace<Maze>>::CellType::PASSAGE)
    }

    pub fn make_wall_unaligned(maze: &mut Maze, pt: pt_type!()) {
        Self::make_unaligned(maze, pt, <Self as CellSpace<Maze>>::CellType::WALL)
    }

    pub fn make_boundary_unaligned(maze: &mut Maze, pt: pt_type!()) {
        Self::make_unaligned(maze, pt, <Self as CellSpace<Maze>>::CellType::BOUNDARY)
    }

    fn get_adjacent_cells(maze: &Maze, pt: pt_type!()) -> Vec<pt_type!()> {
        let mut neighbours: Vec<pt_type!()> = Vec::with_capacity(2usize.pow(DIMENSION.try_into().expect("DIMENSION is too big")));

        let pt: [usize; DIMENSION] = pt.into();

        for i in 0..DIMENSION {
            if pt[i] > 0 {
                // Sneakily make a new point w/ a different value without knowing the dimension at write time
                let mut new_pt = pt.clone();

                new_pt[i] -= 1;

                neighbours.push(new_pt.into());
            }

            if pt[i] < maze.space().dimensions()[i] * Self::scale() {
                // Sneakily make a new point w/ a different value without knowing the dimension at write time
                let mut new_pt = pt.clone();

                new_pt[i] += 1;

                neighbours.push(new_pt.into());
            }
        }

        return neighbours;
    }

    fn get_adjacent_unvisited(maze: &Maze, pt: pt_type!()) -> Vec<pt_type!()> {
        let mut vec = Self::get_adjacent_cells(maze, pt);

        vec.retain(|cell| maze.buffer().get_pt(*cell).is_unvisited());

        return vec;
    }
}

impl <Maze, CoordSpace, const DIMENSION: usize> CellSpace<Maze> for AlignedBoxyCellSpace<Maze, CoordSpace, { DIMENSION }> where
        Maze: maze::Maze<Self>,
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]> {

    type CellType = Basic;
    type CoordSpace = CoordSpace;

    fn cells_required(space: &Self::CoordSpace) -> usize {
        space.dimensions()
            .map(|dim| dim * Self::scale() + 1)
            .product()
    }

    fn ordinal(space: Self::CoordSpace, pt: pt_type!()) -> usize {
        let tuplet: [usize; DIMENSION] = pt.into();
        let dimensions: [usize; DIMENSION] = space.dimensions().map(|dimension| dimension * Self::scale() + 1);

        #[cfg(debug_assertions)]
        for pair in dimensions.zip(tuplet) {
            if pair.1 > pair.0 { // If coord.x >= space.width
                panic!("Point {:?} is out of bounds (space: {:?})", pt, dimensions)
            }
        }

        let mut ordinal = tuplet[0];

        for i in 1..DIMENSION {
            ordinal += tuplet[i] * dimensions[i - 1];
        }

        return ordinal
    }

    fn is_passage(maze: &Maze, pt: pt_type!()) -> bool {
        // Scale point
        let pt = pt.into().map(|dim| dim * Self::scale() + 1);

        return maze.buffer().get_pt(pt.into()).is_passage();
    }

    fn is_wall(maze: &Maze, pt: pt_type!()) -> bool {
        // Scale point
        let pt = pt.into().map(|dim| dim * Self::scale() + 1);

        return maze.buffer().get_pt(pt.into()).is_wall();
    }

    fn is_boundary(maze: &Maze, pt: pt_type!()) -> bool {
        // Scale point
        let pt = pt.into().map(|dim| dim * Self::scale() + 1);

        return maze.buffer().get_pt(pt.into()).is_boundary();
    }

    fn is_unvisited(maze: &Maze, pt: pt_type!()) -> bool {
        // Scale point
        let pt = pt.into().map(|dim| dim * Self::scale() + 1);

        return maze.buffer().get_pt(pt.into()).is_unvisited();
    }

    fn make_passage(maze: &mut Maze, pt: pt_type!()) {
        Self::make(maze, pt, Self::CellType::PASSAGE)
    }

    /// Note: from and to must be adjacent
    fn make_passage_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
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
        if maze.buffer().get_pt(from).is_boundary() || maze.buffer().get_pt(connection).is_boundary() || maze.buffer().get_pt(to).is_boundary() {
            panic!("cannot make a passage that crosses a boundary")
        }

        // Passages

        Self::make_passage_unaligned(maze, from);
        Self::make_passage_unaligned(maze, connection);
        Self::make_passage_unaligned(maze, to);

        // Walls

        {
            let mut neighbours = Self::get_adjacent_unvisited(maze, from);
            neighbours.retain(|&neighbour| neighbour != connection);
            for neighbour in neighbours { Self::make_wall_unaligned(maze, neighbour) }
        }

        {
            let mut neighbours = Self::get_adjacent_unvisited(maze, connection);
            neighbours.retain(|&neighbour| neighbour != from && neighbour != to);
            for neighbour in neighbours { Self::make_wall_unaligned(maze, neighbour) }
        }

        {
            let mut neighbours = Self::get_adjacent_unvisited(maze, to);
            neighbours.retain(|&neighbour| neighbour != connection);
            for neighbour in neighbours { Self::make_wall_unaligned(maze, neighbour) }
        }
    }

    fn make_wall(maze: &mut Maze, pt: pt_type!()) {
        Self::make(maze, pt, Self::CellType::WALL)
    }

    /// Note: from and to must be adjacent
    fn make_wall_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
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
        if maze.buffer().get_pt(from).is_boundary() || maze.buffer().get_pt(connection).is_boundary() || maze.buffer().get_pt(to).is_boundary() {
            panic!("cannot make a wall that crosses a boundary")
        }

        Self::make_wall_unaligned(maze, from);
        Self::make_wall_unaligned(maze, connection);
        Self::make_wall_unaligned(maze, to);
    }

    fn make_boundary(maze: &mut Maze, pt: pt_type!()) {
        Self::make(maze, pt, Self::CellType::BOUNDARY)
    }

    /// Note: from and to must be adjacent
    fn make_boundary_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
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

        Self::make_boundary_unaligned(maze, from.into());
        Self::make_boundary_unaligned(maze, connection_tuplet.into());
        Self::make_boundary_unaligned(maze, to.into());
    }
}

impl <Maze, CoordSpace: CoordinateSpace, const DIMENSION: usize> BoxyCellSpace<Maze, CoordSpace, DIMENSION> for AlignedBoxyCellSpace<Maze, CoordSpace, DIMENSION> where
        Maze: maze::Maze<Self>,
        CoordSpace: BoxCoordinateSpace<{ DIMENSION }>,
        <CoordSpace as BoxCoordinateSpace<{ DIMENSION }>>::PtType: CoordinateTuplet<{ DIMENSION }>,
        <CoordSpace as CoordinateSpace>::PtType: Into<[usize; DIMENSION]>,
        <CoordSpace as CoordinateSpace>::PtType: From<[usize; DIMENSION]> {
    // TODO make this configurable
    #[inline]
    fn scale() -> usize {
        2
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    #[allow(unused_variables)]
    fn make_extended_passage_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
        // Just scale the points then send it off

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        let from = from.map(|coord| coord * Self::scale() + 1);
        let to = to.map(|coord| coord * Self::scale() + 1);

        Self::make_unaligned_extended_passage_between(maze, from.into(), to.into());
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_wall_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
        // Just scale the points then send it off

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        let from = from.map(|coord| coord * Self::scale() + 1);
        let to = to.map(|coord| coord * Self::scale() + 1);

        Self::make_unaligned_extended_wall_between(maze, from.into(), to.into());
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_extended_boundary_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
        // Just scale the points then send it off

        let from: [usize; DIMENSION] = from.into();
        let to: [usize; DIMENSION] = to.into();

        let from = from.map(|coord| coord * Self::scale() + 1);
        let to = to.map(|coord| coord * Self::scale() + 1);

        Self::make_unaligned_extended_boundary_between(maze, from.into(), to.into());
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    #[allow(unused_variables)]
    fn make_unaligned_extended_passage_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
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
        if maze.buffer().get_pt(start).is_boundary() || pts.iter().any(|pt| maze.buffer().get_pt(*pt).is_boundary()) {
            panic!("cannot make a passage that crosses a boundary")
        }

        // Passages

        Self::make_passage(maze, start);

        for pt in &pts {
            Self::make_passage(maze, *pt);
        }

        // Walls

        // Start
        {
            let mut neighbours = Self::get_adjacent_unvisited(maze, start);

            // pts[0] is the next pt, immediately adjacent to start
            neighbours.retain(|&neighbour| neighbour != pts[0]);

            for neighbour in neighbours {
                Self::make_wall(maze, neighbour)
            }
        }

        // Everything else
        {
            let mut ignoring = start;

            for pt in pts {
                let mut neighbours = Self::get_adjacent_unvisited(maze, pt);

                neighbours.retain(|&neighbour| neighbour != ignoring);

                for neighbour in neighbours {
                    Self::make_passage_unaligned(maze, neighbour)
                }

                ignoring = pt;
            }
        }
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_unaligned_extended_wall_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
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
            if maze.buffer().get_pt(pt).is_boundary() {
                panic!("cannot make a wall that crosses a boundary")
            }

            Self::make_wall_unaligned(maze, pt);
        }
    }

    /// Note: from and to must be colinear parallel to a coordinate axis
    fn make_unaligned_extended_boundary_between(maze: &mut Maze, from: pt_type!(), to: pt_type!()) {
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

            Self::make_boundary_unaligned(maze, pt.into());
        }
    }
}

//endregion