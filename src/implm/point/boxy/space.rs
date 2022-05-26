use crate::interface::point::CoordinateSpace;
use crate::implm::point::boxy::CoordinateTuplet;
use crate::implm::point::boxy::implm::BoxCoordinateSpaceIterator;
use crate::internal::array_util::Product;
use std::ops::Index;
use rand::Rng;

/// An n-dimensional coordinate space shaped like a box.
///
/// Basically just think of your regular 2D and 3D cartesian planes/spaces,
/// but generalised to n-dimensions.
///
/// All dimensions of the space must be at least 1.
#[derive(Copy, Clone)]
pub struct BoxCoordinateSpace<const DIMENSION: usize> {
    /// The dimensions of the coordinate space. These are, for lack
    /// of a better word, "one-indexed". This means that for a width of
    /// X, the valid range of X values is 0 <= x < X.
    ///
    /// The most minor coordinate comes first. The ordering is also consistent
    /// with the ordering of co-ordinates for the associated CoordinateTuplets.
    /// (e.g. (x, y, z), (width, height, depth))
    dimensions: [usize; DIMENSION],
    size: usize,
}

impl <const DIMENSION: usize> BoxCoordinateSpace<DIMENSION> {
    pub fn new(dimensions: [usize; DIMENSION]) -> Self {
        if DIMENSION == 0 {
            panic!("DIMENSION must be >= 1")
        }

        Self { dimensions, size: dimensions.product() }
    }

    pub fn dimensions(&self) -> [usize; DIMENSION] {
        self.dimensions
    }

    /// Whether the point is on the outer edge of the space (e.g. on the "surface")
    pub fn is_adjacent_to_edge(&self, pt: <Self as CoordinateSpace>::PtType) -> bool {
        for i in 0..DIMENSION {
            if pt[i] == 0 || pt[i] == self[i] - 1 {
                return true
            }
        }

        return false
    }

    // TODO specialisation - DIRTY TEMP - how do I do narrowing on iterators?
    pub fn iter(&self) -> BoxCoordinateSpaceIterator<DIMENSION> {
        BoxCoordinateSpaceIterator::new(*self, None)
    }
}

impl <const DIMENSION: usize> CoordinateSpace for BoxCoordinateSpace<DIMENSION> {
    type PtType = CoordinateTuplet<DIMENSION>;
    type IntoIter = BoxCoordinateSpaceIterator<DIMENSION>;

    fn logical_size(&self) -> usize {
        self.size
    }

    fn neighbours_of_pt(&self, pt: Self::PtType) -> Vec<Self::PtType> {
        let mut neighbours = Vec::with_capacity(DIMENSION ^ 2);

        for dim in 0..DIMENSION {
            if pt[dim] > 0 {
                neighbours.push(pt.offset(dim, -1))
            }

            if pt[dim] + 1 < self.dimensions[dim] {
                neighbours.push(pt.offset(dim, 1))
            }
        }

        return neighbours
    }

    fn are_adjacent(pt1: Self::PtType, pt2: Self::PtType) -> bool {
        for dim in 0..DIMENSION {
            if pt1[dim].abs_diff(pt2[dim]) == 1 {
                return true
            }
        }

        return false
    }

    fn iter_from(&self, pt: Self::PtType) -> <Self as CoordinateSpace>::IntoIter {
        BoxCoordinateSpaceIterator::new(*self, Some(pt))
    }

    fn choose<RNG: Rng + ?Sized>(&self, rng: &mut RNG) -> Self::PtType {
        self.dimensions.map(|dim| rng.gen_range(0..dim)).into()
    }
}

impl <const DIMENSION: usize> Index<usize> for BoxCoordinateSpace<DIMENSION> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.dimensions[index]
    }
}