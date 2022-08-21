use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Index, IndexMut, RangeInclusive};
use std::slice::SliceIndex;
use crate::interface::cell::CellValue;
use crate::interface::point::CoordinateSpace;

#[derive(Debug)]
pub struct Path<T: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug> {
    vec: Vec<T>,
}

pub type PointPath<CoordSpace: CoordinateSpace> = Path<CoordSpace::PtType>;
pub type CellPath<CellType: CellValue> = Path<CellType>;

impl <T: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug> Path<T> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        Self { vec }
    }

    pub fn starting_at(pt: T) -> Self {
        Self { vec: vec![pt] }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn contains(&self, pt: T) -> bool {
        self.vec.contains(&pt)
    }

    /// Contains a movement from one position to another.
    ///
    /// Basically checks if `from` exists in the path and
    /// is immediately followed by `to`.
    pub fn contains_movement(&self, from: T, to: T) -> bool {
        for i in 0..(self.len() - 1) {
            if &self.vec[i] == &from && &self.vec[i + 1] == &to {
                return true;
            }
        }

        return false;
    }

    pub fn push(&mut self, pt: T) {
        self.vec.push(pt)
    }

    /// Return whether this path is simple.
    ///
    /// A simple path has no loops (no repeated points).
    pub fn is_simple(&self) -> bool {
        let mut seen_pts = HashSet::new();

        for pt in &self.vec {
            // Minor abuse of ::replace as a replacement for insert_if_missing
            // If already in the set, then this path must not be simple
            match seen_pts.replace(pt) {
                Some(_) => return false,
                None => {}
            }
        }

        return true;
    }

    /// Make this path into a simple path.
    ///
    /// A simple path is a path that has no loops (no repeated points).
    ///
    /// If this path is already simple, this method has no effect.
    pub fn make_simple(&mut self) {
        // Track the first time we've seen each point (not in a removed range),
        // so we know where the start indices for ranges are
        let mut first_seen_indices_of_pts = HashMap::<T, usize>::new();

        let mut ranges_to_remove: Vec<RangeInclusive<usize>> = Vec::new();

        for (i, pt) in self.vec.iter().enumerate() {
            match first_seen_indices_of_pts.entry(*pt) {
                Occupied(entry) => {
                    // Indices are inclusive

                    // + 1 because we don't want to remove both the start and the end (they're duplicates, so we only want to remove one)
                    let new_removal_range = (*entry.get() + 1)..=i;

                    /* We're looking for the first range that intersects our current range */

                    // Note: as our new range's end index is i, it will necessarily always be
                    // greater than every other range's start and end indices.

                    let mut ranges_to_condense_start = 0usize;

                    loop {
                        if ranges_to_condense_start >= ranges_to_remove.len() {
                            break;
                        }

                        let other_range = &ranges_to_remove[ranges_to_condense_start];

                        // If another range has a start >= ours, since our end always exceeds their end, we know that our range will fully encompass it
                        // Note that we can never have a partial overlap, because creating a range deregisters all points that were inside it
                        if other_range.start() >= new_removal_range.start() {
                            break;
                        } else {
                            ranges_to_condense_start += 1;
                        }
                    }

                    // We know we will intersect every range after because we know our new end index is always the greatest of all them

                    /* Now we have our correct range (and which previous ranges to remove), we put this information to use */

                    // Any point that is removed shouldn't be eligible for a new removal range (as we've already
                    // removed it, and any new range would start later than the existing range)
                    first_seen_indices_of_pts.retain(|_pt, first_seen_index| new_removal_range.contains(first_seen_index) == false);

                    // Remove all the ranges that we're condensing...
                    ranges_to_remove.drain(ranges_to_condense_start..ranges_to_remove.len());

                    // ...then add in our new amalgamate range
                    ranges_to_remove.insert(ranges_to_condense_start, new_removal_range);
                },
                Vacant(entry) => {
                    // First time we've seen this point so we register ti
                    entry.insert(i);
                },
            }
        }

        // Actually apply the removal ranges (i.e. remove the points that we've determined need to be removed
        for range_to_remove in ranges_to_remove.into_iter().rev() {
            self.vec.drain(range_to_remove);
        }

        debug_assert!(self.is_simple());  // Sanity check
    }

    /// Return where this path is a cycle.
    ///
    /// A cycle is a path that starts and ends at the same point.
    /// A zero-length path is not considered a cycle.
    pub fn is_cycle(&self) -> bool {
        self.vec.len() >= 1 && self.vec[0] == self.vec[self.vec.len() - 1]
    }

    /// Return whether this path is a simple cycle.
    ///
    /// A simple cycle is a simple path that is also a cycle.
    ///
    /// # See Also
    /// * [`Self::is_simple`]
    /// * [`Self::is_cycle`]
    pub fn is_simple_cycle(&self) -> bool {
        // Cycle check is cheaper so lets do it first
        self.is_cycle() && self.is_simple()
    }
}

impl <T: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug, I: SliceIndex<[T]>> Index<I> for Path<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.vec.index(index)
    }
}

impl <T: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug, I: SliceIndex<[T]>> IndexMut<I> for Path<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.vec.index_mut(index)
    }
}

impl <T: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug> IntoIterator for Path<T> {
    type Item = <Vec<T> as IntoIterator>::Item;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}