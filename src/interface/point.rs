use std::fmt::Debug;

pub trait CoordinateSpace : Copy + Clone + Send + Sync + Sized + IntoIterator<Item = Self::PtType> + Debug {
    type PtType: Point;
    type IntoIter: Iterator<Item = Self::PtType>;

    /// May return `None` if the coordinate space is empty.
    fn origin(&self) -> Option<Self::PtType>;

    /// The maximum number of points/nodes in this coordinate space.
    fn logical_size(&self) -> usize;

    fn neighbours_of_pt(&self, pt: Self::PtType) -> Vec<Self::PtType>;

    fn iter_from(&self, pt: Self::PtType) -> <Self as CoordinateSpace>::IntoIter;

    /// Note, a point is NOT considered to be adjacent to itself
    fn are_adjacent(pt1: Self::PtType, pt2: Self::PtType) -> bool;
}

pub trait Point: Copy + Clone + Sized + PartialEq + Eq + Debug {}