use std::fmt::{Debug, Formatter};
use std::num::NonZeroUsize;
use std::ops::Index;

use rand::Rng;

use crate::implm::point::boxy::CoordinateTuplet;
use crate::implm::point::boxy::BoxCoordinateSpaceIterator;
use crate::interface::point::CoordinateSpace;
use crate::internal::array_util::CheckedProduct;
use crate::internal::util::try_usize_array_to_nonzero_usize_array;

/// An `n`-dimensional coordinate space shaped like a box with box-like points.
///
/// It is a generalisation of the common two-dimensional and three-dimensional
/// Cartesian plane/space to `n` dimensions.
///
/// All (size) dimensions of the space must be at least 1. The (mathematical)
/// dimension (`DIMENSION`) must be also be at least 1.
///
/// # Coordinate Axes
///
/// As a `BoxCoordinateSpace` has `n` dimensions, there are `n` coordinate axes.
/// These are labelled from `0` to `n - 1`. The most minor axis is `0`, and the
/// most major axis is `n - 1`. These correspond directly with the array or tuple
/// indices of a [coordinate tuplet][CoordinateTuplet].
///
/// # Adjacency
///
/// ## Direct Adjacency
///
/// Two points are considered directly adjacent if:
/// * On one single coordinate axis, their respective coordinates differ by exactly 1, and,
/// * On every other axis, their respective coordinates are the same.
///
/// For example, `(1, 1, 1)` and `(1, 2, 1)` are considered directly adjacent.
/// `(1, 1, 1)` and (`1, 2, 2)` are not directly adjacent. `(1, 1, 1)` and `(1, 1, 1)` are
/// also not considered directly adjacent.
///
/// ## Edge Adjacency
///
/// A point is considered to be adjacent to the edge of the coordinate space if there is
/// some axis where the point's respective coordinate is `0` or the `length of axis - 1`.
#[derive(Copy, Clone)]
pub struct BoxCoordinateSpace<const DIMENSION: usize> {
    /// The (size) dimensions of the coordinate space.
    ///
    /// Each dimension represents the number of possible values on the
    /// respective axis. For example, imagine a dimension with value X.
    /// The valid range of values on that axis would be 0 <= x < X.
    ///
    /// The most minor coordinate comes first. The ordering is also consistent
    /// with the ordering of co-ordinates for the associated CoordinateTuplets.
    /// (e.g. (x, y, z), (width, height, depth))
    dimensions: [NonZeroUsize; DIMENSION],

    /// The total number of possible points or positions in this coordinate space.
    ///
    /// This just serves as a cache of the result of
    /// [`self.dimensions.checked_product()`][CheckedProduct::checked_product].
    size: NonZeroUsize,
}

impl <const DIMENSION: usize> BoxCoordinateSpace<DIMENSION> {
    /// Construct a new `BoxCoordinateSpace` from the given (size) dimensions.
    ///
    /// # Parameters
    ///
    /// `dimensions` --- the width, height, depth, etc. of the coordinate space.
    ///                  As the (mathematical) dimension must be non-zero, this
    ///                  array cannot be empty. Additionally, the product of all
    ///                  dimensions must fit within a `usize`. (Mazes this large
    ///                  won't fit in memory anyway).
    ///
    /// # Examples
    ///
    /// ```
    /// # unsafe {
    /// # use std::num::NonZeroUsize;
    /// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
    /// #
    /// // If you're using hard-coded constants like in this example,
    /// // you may prefer new_checked() for the ergonomics
    /// let coord_space = BoxCoordinateSpace::new([NonZeroUsize::new_unchecked(5), NonZeroUsize::new_unchecked(7)]);
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// [`new_checked()`][Self::new_checked]
    #[must_use]
    pub fn new(dimensions: [NonZeroUsize; DIMENSION]) -> Self {
        if DIMENSION == 0 {
            panic!("DIMENSION must be >= 1")
        }

        let size = dimensions.checked_product().expect("The dimensions specified are too large. The number of points in the space does not fit within a usize.");

        Self { dimensions, size }
    }

    /// Construct a new `BoxCoordinateSpace` from the given (size) dimensions.
    ///
    /// # Parameters
    ///
    /// `dimensions` --- the width, height, depth, etc. of the coordinate space.
    ///                  As the (mathematical) dimension must be non-zero, this
    ///                  array cannot be empty. All dimensions must be non-zero
    ///                  as wel. Additionally, the product of all dimensions must
    ///                  fit within a `usize`. (Mazes this large won't fit in
    ///                  memory anyway).
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
    /// #
    /// let coord_space = BoxCoordinateSpace::new_checked([5, 7, 3]);
    /// ```
    #[must_use]
    pub fn new_checked(dimensions: [usize; DIMENSION]) -> Self {
        Self::new(try_usize_array_to_nonzero_usize_array(dimensions).expect("All dimensions must be non-zero"))
    }

    /// Return the (size) dimensions of this coordinate space.
    #[must_use]
    pub fn dimensions(&self) -> [NonZeroUsize; DIMENSION] {
        self.dimensions
    }

    /// Return whether the point is [adjacent to the edge of this coordinate space](#edge-adjacency).
    #[must_use]
    pub fn is_adjacent_to_edge(&self, pt: <Self as CoordinateSpace>::PtType) -> bool {
        for i in 0..DIMENSION {
            if pt[i] == 0 || pt[i] == usize::from(self[i]) - 1 {
                return true
            }
        }

        return false
    }
}

impl <const DIMENSION: usize> CoordinateSpace for BoxCoordinateSpace<DIMENSION> {
    type PtType = CoordinateTuplet<DIMENSION>;
    type Iter = BoxCoordinateSpaceIterator<DIMENSION>;

    fn logical_size(&self) -> NonZeroUsize {
        self.size
    }

    fn neighbours_of_pt(&self, pt: Self::PtType) -> Vec<Self::PtType> {
        let mut neighbours = Vec::with_capacity(DIMENSION ^ 2);

        for dim in 0..DIMENSION {
            if pt[dim] > 0 {
                neighbours.push(pt.offset(dim, -1))
            }

            if pt[dim] + 1 < usize::from(self.dimensions[dim]) {
                neighbours.push(pt.offset(dim, 1))
            }
        }

        return neighbours
    }

    fn are_adjacent(&self, pt1: Self::PtType, pt2: Self::PtType) -> bool {
        let mut found_axis_of_adjacency = false;

        for dim in 0..DIMENSION {
            match pt1[dim].abs_diff(pt2[dim]) {
                0 => {},
                1 => {
                    if found_axis_of_adjacency {
                        return false;
                    } else {
                        found_axis_of_adjacency = true;
                    }
                },
                _ => return false,
            }
        }

        return found_axis_of_adjacency;
    }

    fn iter(&self) -> Self::Iter {
        BoxCoordinateSpaceIterator::new(*self, None)
    }

    fn iter_from(&self, pt: Self::PtType) -> Self::Iter {
        BoxCoordinateSpaceIterator::new(*self, Some(pt))
    }

    fn choose(&self, rng: &mut (impl Rng + ?Sized)) -> Self::PtType {
        self.dimensions.map(|dim| rng.gen_range(0..usize::from(dim))).into()
    }
}

impl <const DIMENSION: usize> Index<usize> for BoxCoordinateSpace<DIMENSION> {
    type Output = NonZeroUsize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.dimensions[index]
    }
}

impl <const DIMENSION: usize> Debug for BoxCoordinateSpace<DIMENSION> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BoxCoordinateSpace(dimensions = ")?;

        for i in 0..(DIMENSION - 1) {
            write!(f, "{}x", self[i])?;
        }

        write!(f, "{}", self[DIMENSION - 1])?;

        write!(f, ")")
    }
}