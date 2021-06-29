use crate::geometry::node::{Point, CoordinatePair, CoordinateTriplet, CoordinateTuplet};
use std::iter::FusedIterator;
use std::fmt::Debug;
use crate::util::absolute_difference;

pub trait CoordinateSpace : Copy + Clone + Debug + Sized + Send + Sync + IntoIterator<Item = Self::PtType> {
    type PtType: Point;
    type IntoIter: Iterator<Item = Self::PtType>;

    fn origin() -> Self::PtType;

    /// The maximum number of points/nodes in this coordinate space.
    fn logical_size(&self) -> usize;

    fn neighbours_of_pt(&self, pt: Self::PtType) -> Vec<Self::PtType>;

    fn iter_from(&self, pt: Self::PtType) -> <Self as CoordinateSpace>::IntoIter;

    fn are_adjacent(pt1: Self::PtType, pt2: Self::PtType) -> bool;
}

/// An n-dimensional coordinate space shaped like a box.
/// Basically just think of your regular 2D and 3D cartesian planes/spaces.
///
/// All dimensions of the space must be at least 1.
pub trait BoxCoordinateSpace<const DIMENSION: usize> : CoordinateSpace {
    type PtType: CoordinateTuplet<DIMENSION>;

    /// Get the dimensions of the coordinate space. These are, for lack
    /// of a better word, "one-indexed". This means that for a width of
    /// X, the valid range of X values is 0 <= x < X.
    ///
    /// The most minor coordinate comes first. The ordering is also consistent
    /// with the ordering of co-ordinates for the associated CoordinateTuplets.
    /// (e.g. (x, y, z), (width, height, depth))
    fn dimensions(&self) -> [usize; DIMENSION];

    /// Whether the point is on the outer edge of the space (e.g. on the "surface")
    fn is_adjacent_to_edge(&self, pt: <Self as CoordinateSpace>::PtType) -> bool;

    // DIRTY TEMP - how do I do narrowing on iterators?
    fn iter(&self) -> Box<dyn Iterator<Item=<Self as BoxCoordinateSpace<DIMENSION>>::PtType>>;
}

#[derive(Copy, Clone, Debug)]
pub struct TwoDimensionalSpace {
    pub width: usize,
    pub height: usize
}

impl TwoDimensionalSpace {
    pub fn new(width: usize, height: usize) -> Self {
        debug_assert!(width >= 1 && height >= 1, "No dimension can be be zero");

        Self { width, height }
    }
}

impl CoordinateSpace for TwoDimensionalSpace {
    type PtType = CoordinatePair;
    type IntoIter = TwoDimensionalSpaceIterator;

    fn origin() -> CoordinatePair {
        CoordinatePair::from((0, 0))
    }

    fn logical_size(&self) -> usize {
        self.width * self.height
    }

    fn neighbours_of_pt(&self, pt: CoordinatePair) -> Vec<CoordinatePair> {
        debug_assert!(pt.x < self.width && pt.y < self.height, "Point {:?} is out of bounds (space: {}x{})", pt, self.width, self.height);

        let mut neighbours = Vec::with_capacity(4);

        if pt.x < self.width - 1 { neighbours.push(pt + (1, 0)) }
        if pt.y < self.height - 1 { neighbours.push(pt + (0, 1)) }
        if pt.x > 0 { neighbours.push(pt + (-1, 0)) }
        if pt.y > 0 { neighbours.push(pt + (0, -1)) }

        return neighbours
    }

    fn iter_from(&self, pt: CoordinatePair) -> TwoDimensionalSpaceIterator {
        debug_assert!(pt.x < self.width && pt.y < self.height, "Point {:?} is out of bounds (space: {}x{})", pt, self.width, self.height);

        TwoDimensionalSpaceIterator { space: self.clone(), pos: Some(pt) }
    }

    fn are_adjacent(pt1: Self::PtType, pt2: Self::PtType) -> bool {
        absolute_difference(pt1.x, pt2.x) == 1 || absolute_difference(pt1.y, pt2.y) == 1
    }
}

impl IntoIterator for TwoDimensionalSpace {
    type Item = CoordinatePair;
    type IntoIter = TwoDimensionalSpaceIterator;

    fn into_iter(self) -> Self::IntoIter {
        TwoDimensionalSpaceIterator { space: self, pos: None }
    }
}

pub struct TwoDimensionalSpaceIterator {
    space: TwoDimensionalSpace,
    pos: Option<CoordinatePair>
}

impl Iterator for TwoDimensionalSpaceIterator {
    type Item = CoordinatePair;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment iterator position then return it
        match self.pos {
            None => self.pos = Some(TwoDimensionalSpace::origin()),
            Some(pos) => {
                if pos.x + 1 < self.space.width {
                    self.pos = Some(pos + (1, 0))
                } else if pos.y + 1 < self.space.height {
                    self.pos = Some(CoordinatePair::from((0, pos.y + 1)))
                } else {
                    return None
                }
            }
        }

        return self.pos
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let max = self.space.width * self.space.height;

        let remaining = match self.pos {
            Some(pos) => max - pos.y * self.space.width - pos.x - 1,  // -1 to account for 0-indexing
            None => max
        };

        (remaining, Some(remaining))
    }
}

impl FusedIterator for TwoDimensionalSpaceIterator {}

impl BoxCoordinateSpace<2> for TwoDimensionalSpace {
    type PtType = <Self as CoordinateSpace>::PtType;

    fn dimensions(&self) -> [usize; 2] {
        [self.width, self.height]
    }

    fn is_adjacent_to_edge(&self, pt: <Self as CoordinateSpace>::PtType) -> bool {
        let tuple = Into::<[usize; 2]>::into(pt);

        tuple.contains(&0) || tuple[0] == self.width - 1 || tuple[1] == self.height - 1
    }

    fn iter(&self) -> Box<dyn Iterator<Item=<Self as BoxCoordinateSpace<2>>::PtType>> {
        todo!()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ThreeDimensionalSpace {
    pub width: usize,
    pub height: usize,
    pub depth: usize
}

impl ThreeDimensionalSpace {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        debug_assert!(width >= 1 && height >= 1 && depth >= 1, "No dimension can be be zero");

        Self { width, height, depth }
    }
}

impl CoordinateSpace for ThreeDimensionalSpace {
    type PtType = CoordinateTriplet;
    type IntoIter = ThreeDimensionalSpaceIterator;

    fn origin() -> CoordinateTriplet {
        CoordinateTriplet::from((0, 0, 0))
    }

    fn logical_size(&self) -> usize {
        self.width * self.height * self.depth
    }

    fn neighbours_of_pt(&self, pt: CoordinateTriplet) -> Vec<CoordinateTriplet> {
        debug_assert!(pt.x < self.width && pt.y < self.height, "Point {:?} is out of bounds (space: {}x{}x{})", pt, self.width, self.height, self.depth);

        let mut neighbours = Vec::with_capacity(6);

        if pt.x < self.width - 1 { neighbours.push(pt + (1, 0, 0)) }
        if pt.y < self.height - 1 { neighbours.push(pt + (0, 1, 0)) }
        if pt.z < self.depth - 1 { neighbours.push(pt + (0, 0, 1)) }
        if pt.x > 0 { neighbours.push(pt + (-1, 0, 0)) }
        if pt.y > 0 { neighbours.push(pt + (0, -1, 0)) }
        if pt.z > 0 { neighbours.push(pt + (0, 0, -1)) }

        return neighbours
    }

    fn iter_from(&self, pt: CoordinateTriplet) -> ThreeDimensionalSpaceIterator {
        debug_assert!(pt.x < self.width && pt.y < self.height, "Point {:?} is out of bounds (space: {}x{}x{})", pt, self.width, self.height, self.depth);

        ThreeDimensionalSpaceIterator { space: self.clone(), pos: Some(pt) }
    }

    fn are_adjacent(pt1: Self::PtType, pt2: Self::PtType) -> bool {
        absolute_difference(pt1.x, pt2.x) == 1 || absolute_difference(pt1.y, pt2.y) == 1 || absolute_difference(pt1.z, pt2.z) == 1
    }
}

impl IntoIterator for ThreeDimensionalSpace {
    type Item = CoordinateTriplet;
    type IntoIter = ThreeDimensionalSpaceIterator;

    fn into_iter(self) -> Self::IntoIter {
        ThreeDimensionalSpaceIterator { space: self, pos: None }
    }
}

pub struct ThreeDimensionalSpaceIterator {
    space: ThreeDimensionalSpace,
    pos: Option<CoordinateTriplet>
}

impl Iterator for ThreeDimensionalSpaceIterator {
    type Item = CoordinateTriplet;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment iterator position then return it
        match self.pos {
            None => self.pos = Some(ThreeDimensionalSpace::origin()),
            Some(pos) => {
                if pos.x + 1 < self.space.width {
                    self.pos = Some(pos + (1, 0, 0))
                } else if pos.y + 1 < self.space.height {
                    self.pos = Some(CoordinateTriplet::from((0, pos.y + 1, pos.z)))
                } else if pos.z + 1 < self.space.depth {
                    self.pos = Some(CoordinateTriplet::from((0, 0, pos.z + 1)))
                } else {
                    return None
                }
            }
        }

        return self.pos
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let max = self.space.width * self.space.height;

        let remaining = match self.pos {
            // -1 to account for 0-indexing
            Some(pos) => max - pos.z * self.space.height * pos.y * self.space.width - pos.x - 1,
            None => max
        };

        (remaining, Some(remaining))
    }
}

impl FusedIterator for ThreeDimensionalSpaceIterator {}

impl BoxCoordinateSpace<3> for ThreeDimensionalSpace {
    type PtType = <Self as CoordinateSpace>::PtType;

    fn dimensions(&self) -> [usize; 3] {
        [self.width, self.height, self.depth]
    }

    fn is_adjacent_to_edge(&self, pt: <Self as CoordinateSpace>::PtType) -> bool {
        let tuple = Into::<[usize; 3]>::into(pt);

        tuple.contains(&0) || tuple[0] == self.width - 1 || tuple[1] == self.height - 1 || tuple[2] == self.depth - 1
    }

    fn iter(&self) -> Box<dyn Iterator<Item=<Self as BoxCoordinateSpace<3>>::PtType>> {
        todo!()
    }
}