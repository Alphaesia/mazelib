#![allow(non_camel_case_types)]

use crate::cell;
use crate::geometry::space::{CoordinateSpace, TwoDimensionalSpace};
use std::time::Duration;
use crate::cell::manager::AlignedBoxyCellSpace;
use crate::buffer::VecBuffer;
use crate::cell::data::Basic;
use crate::geometry::node::CoordinatePair;
use crate::render::{TextRenderer, MazeRendererWithMarker};

#[cfg(target_pointer_width = "64")]
pub(crate) type fsize = f64;

#[cfg(target_pointer_width = "32")]
pub(crate) type fsize = f32;

#[inline]
pub(crate) fn get_neighbouring_unvisiteds<CellSpace: cell::manager::CellManager>(maze: &CellSpace, pt: <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType) -> Vec<<<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType> {
    let mut neighbours = maze.space().neighbours_of_pt(pt).to_vec();

    neighbours.retain(|&neighbour| maze.is_unvisited(neighbour));

    return neighbours
}

#[inline]
pub(crate) fn get_neighbouring_walls<CellSpace: cell::manager::CellManager>(maze: &CellSpace, pt: <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType) -> Vec<<<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType> {
    let mut neighbours = (*maze).space().neighbours_of_pt(pt).to_vec();

    neighbours.retain(|&neighbour| maze.is_wall(neighbour));

    return neighbours
}

/// Find the absolute value of the difference of two unsigned integers
#[inline(always)]
pub(crate) fn absolute_difference(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

/// If pt1 and pt2 are colinear parallel to a coordinate axis and not identical,
/// return the axis of their colinearity.
///
/// The return value is the index that represents the coordinate axis they are parallel to.
///
/// As an example, if they are colinear along x, 0 is returned. If they are colinear along
/// y returns 1. get_common_axis([0, 0, 0], [0, 2, 0]) returns 1.
#[inline]
pub(crate) fn get_common_axis<const DIMENSION: usize>(pt1: [usize; DIMENSION], pt2: [usize; DIMENSION]) -> Option<usize> {
    let mut common_axis: Option<usize> = None;

    for i in 0..DIMENSION {
        if pt1[i] != pt2[i] {
            if common_axis != None {
                return None
            } else {
                common_axis = Some(i)
            }
        }
    }

    return common_axis
}

pub(crate) trait Product<T> {
    fn product(&self) -> T;
}

impl <const LENGTH: usize> Product<usize> for [usize; LENGTH] {
    fn product(&self) -> usize {
        let mut product = 1;

        for x in self {
            product *= x;
        }

        return product;
    }
}

/// coerce!(Input type, Output type, input)
///
/// Coerces any reference to any other reference.
/// For when the type system refuses to cooperate.
///
/// Obviously, only use in cases where you have to and can prove correctness manually.
///
/// Works on anything (I think).
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! coerce {
    ($In: ty, $Out: ty, $input: expr) => {
        {
            let ptr: *const $In = $input;
            let coerced_ptr = std::mem::transmute::<*const $In, *const $Out>(ptr);
            coerced_ptr.as_ref().unwrap()
        }
    };
}

/// Print a half-generated maze to stdout
/// Note: maze must be two-dimensional
#[cfg(debug_assertions)]
#[allow(dead_code)]
pub(crate) fn debug_maze<CellSpace: cell::manager::CellManager>(maze: &CellSpace, cursor: <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType) {
    // Coerce maze and point into something we can print
    let (coerced_maze, coerced_pt) = unsafe {
        (coerce!(CellSpace, AlignedBoxyCellSpace<VecBuffer<TwoDimensionalSpace, Basic>, TwoDimensionalSpace, 2>, maze),
         *coerce!(<<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType, CoordinatePair, &cursor))
    };

    let coerced_pt = [coerced_pt.x * 2 + 1, coerced_pt.y * 2 + 1].into();

    let result = TextRenderer::render_with_marker(coerced_maze, coerced_pt);

    for line in result {
        println!("{}", line);
    }

    std::thread::sleep(Duration::from_millis(500));
}