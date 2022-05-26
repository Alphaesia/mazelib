use std::fmt::Debug;
use std::hash::Hash;
use rand::Rng;

pub trait CoordinateSpace : Sized + Clone + Copy + Send + Sync + IntoIterator<Item = Self::PtType> + Debug {
    type PtType: Point;
    type IntoIter: Iterator<Item = Self::PtType>;

    /// The maximum number of points/nodes in this coordinate space.
    fn logical_size(&self) -> usize;

    fn neighbours_of_pt(&self, pt: Self::PtType) -> Vec<Self::PtType>;

    /// Note, a point is NOT considered to be adjacent to itself
    fn are_adjacent(pt1: Self::PtType, pt2: Self::PtType) -> bool;

    fn iter_from(&self, pt: Self::PtType) -> <Self as CoordinateSpace>::IntoIter;

    /// Return a random point in this coordinate space.
    fn choose<RNG: Rng + ?Sized>(&self, rng: &mut RNG) -> Self::PtType;
}

pub trait Point: Sized + Clone + Copy + PartialEq + Eq + Hash + Debug {}