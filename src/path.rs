//! Paths through mazes.
//! 
//! # Recommended Reading
//! 
//! * [`Path`] --- the main path struct.

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Index, IndexMut, RangeInclusive};
use std::slice::SliceIndex;

use crate::interface::cell::CellValue;
use crate::interface::point::CoordinateSpace;

/// A series of movements from location to location.
/// 
/// It is a *directed path*, so locations can only be traversed in order.
/// 
/// # Examples
/// 
/// ```
/// # use mazelib::path::Path;
/// #
/// // Create a path starting at the "origin"
/// let mut path = Path::starting_at(0);
/// 
/// // Wander about
/// path.push(2);
/// path.push(3);
/// path.push(4);
/// path.push(2);
/// path.push(4);
/// path.push(3);
/// 
/// // Remove loops
/// path.make_simple();
/// 
/// // Retread the path
/// for loc in path {
///     // ...
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct Path<T: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug> {
    locs: Vec<T>,
}

// TODO make these type aliases once again once lazy_type_alias is stabilised

/// A path of points.
///
/// See [`Path`].
pub struct PointPath<CoordSpace: CoordinateSpace>(pub Path<CoordSpace::PtType>);

/// A path of cell locations.
///
/// See [`Path`].
pub struct CellPath<CellType: CellValue>(pub Path<CellType>);

impl <T: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug> Path<T> {
    /// Create a path from the given locations.
    /// 
    /// # Parameters
    /// `locs` --- The locations to create the path from. The resulting path will have the same
    ///            locations in the same order as in `locs`. It must contain at least one location.
    /// 
    /// # Panics
    /// 
    /// If `locs` is empty.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use mazelib::path::Path;
    /// #
    /// let path = Path::from_vec(vec![3, 2, 1]);
    /// 
    /// let mut iter = path.into_iter();
    /// assert_eq!(Some(3), iter.next());
    /// assert_eq!(Some(2), iter.next());
    /// assert_eq!(Some(1), iter.next());
    /// assert_eq!(None, iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn from_vec(locs: Vec<T>) -> Self {
        if locs.is_empty() { panic!("locs must be non-empty") }
        
        Self { locs }
    }

    /// Create a path where the first location is `loc`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::path::Path;
    /// #
    /// let path = Path::starting_at(8);
    ///
    /// let mut iter = path.into_iter();
    /// assert_eq!(Some(8), iter.next());
    /// assert_eq!(None, iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn starting_at(loc: T) -> Self {
        Self { locs: vec![loc] }
    }

    /// Return the number of movements in this path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::path::Path;
    /// #
    /// let path = Path::from_vec(vec![3, 2, 1]);
    ///
    /// assert_eq!(2, path.len());
    /// ```
    /// 
    /// Cycles don't shorten the length:
    /// 
    /// ```
    /// # use mazelib::path::Path;
    /// #
    /// let path = Path::from_vec(vec![3, 2, 1, 2, 3]);
    ///
    /// assert_eq!(4, path.len());
    /// ```
    pub fn len(&self) -> usize {
        self.locs.len() - 1
    }

    /// Test whether this path contains a given location.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::path::Path;
    /// #
    /// let path = Path::from_vec(vec![3, 2, 1]);
    ///
    /// assert!(path.contains(1));
    /// assert!(path.contains(2));
    /// assert!(path.contains(3));
    /// assert!(path.contains(4) == false);
    /// ```
    pub fn contains(&self, loc: T) -> bool {
        self.locs.contains(&loc)
    }

    /// Contains a movement from one location to another.
    ///
    /// Basically checks if `from` exists in the path and
    /// is immediately followed by `to`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::path::Path;
    /// #
    /// let path = Path::from_vec(vec![3, 2, 1]);
    ///
    /// assert!(path.contains_movement(3, 2));
    /// assert!(path.contains_movement(2, 1));
    /// 
    /// // Can't go backwards
    /// assert!(path.contains_movement(1, 2) == false);
    /// assert!(path.contains_movement(2, 3) == false);
    ///
    /// // Checks for non-existent points just return false
    /// assert!(path.contains_movement(1, 4) == false);
    /// assert!(path.contains_movement(8, 9) == false);
    /// ```
    pub fn contains_movement(&self, from: T, to: T) -> bool {
        for i in 0..self.len() {
            if &self.locs[i] == &from && &self.locs[i + 1] == &to {
                return true;
            }
        }

        return false;
    }

    /// Append a new location to the path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::path::Path;
    /// #
    /// let mut path = Path::from_vec(vec![3, 2, 1]);
    /// 
    /// path.push(6);
    ///
    /// assert_eq!(3, path[0]);
    /// assert_eq!(2, path[1]);
    /// assert_eq!(1, path[2]);
    /// assert_eq!(6, path[3]);
    /// ```
    pub fn push(&mut self, loc: T) {
        self.locs.push(loc)
    }

    /// Return whether this path is simple.
    ///
    /// A simple path has no loops (no location appears multiple times).
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::path::Path;
    /// #
    /// assert!(Path::from_vec(vec![3, 2, 1]).is_simple());
    /// assert!(Path::from_vec(vec![3, 2, 1, 2]).is_simple() == false);
    /// ```
    pub fn is_simple(&self) -> bool {
        Self::is_subpath_simple(&self.locs)
    }

    fn is_subpath_simple(subpath: &[T]) -> bool {
        let mut seen_locs = HashSet::new();

        for loc in subpath {
            // Minor abuse of ::replace as a replacement for insert_if_missing
            // If already in the set, then this path must not be simple
            match seen_locs.replace(loc) {
                Some(_) => return false,
                None => {}
            }
        }

        return true;
    }

    /// Make this path into a simple path.
    ///
    /// A simple path is a path that has no loops (no repeated locations). This method removes
    /// loops by removing all locations between two repeated locations (and then de-duplicating
    /// those locations). For example, `[4, 1, 5, 1]` becomes `[4, 1`].
    ///
    /// If this path is already simple, this method has no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::path::Path;
    /// #
    /// let mut path = Path::from_vec(vec![3, 2, 1, 2, 3]);
    /// 
    /// path.make_simple();
    /// assert_eq!(Path::from_vec(vec![3]), path);
    /// 
    /// path.make_simple();  // Idempotent
    /// assert_eq!(Path::from_vec(vec![3]), path);
    /// ```
    pub fn make_simple(&mut self) {
        // Track the first time we've seen each location (not in a removed range),
        // so we know where the start indices for ranges are
        let mut first_seen_indices_of_locs = HashMap::<T, usize>::new();

        let mut ranges_to_remove: Vec<RangeInclusive<usize>> = Vec::new();

        for (i, loc) in self.locs.iter().enumerate() {
            match first_seen_indices_of_locs.entry(*loc) {
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
                        // Note that we can never have a partial overlap, because creating a range deregisters all locations that were inside it
                        if other_range.start() >= new_removal_range.start() {
                            break;
                        } else {
                            ranges_to_condense_start += 1;
                        }
                    }

                    // We know we will intersect every range after because we know our new end index is always the greatest of all them

                    /* Now we have our correct range (and which previous ranges to remove), we put this information to use */

                    // Any location that is removed shouldn't be eligible for a new removal range (as we've already
                    // removed it, and any new range would start later than the existing range)
                    first_seen_indices_of_locs.retain(|_loc, first_seen_index| new_removal_range.contains(first_seen_index) == false);

                    // Remove all the ranges that we're condensing...
                    ranges_to_remove.drain(ranges_to_condense_start..ranges_to_remove.len());

                    // ...then add in our new amalgamate range
                    ranges_to_remove.insert(ranges_to_condense_start, new_removal_range);
                },
                Vacant(entry) => {
                    // First time we've seen this location so we register ti
                    entry.insert(i);
                },
            }
        }

        // Actually apply the removal ranges (i.e. remove the locations that we've determined need to be removed
        for range_to_remove in ranges_to_remove.into_iter().rev() {
            self.locs.drain(range_to_remove);
        }

        debug_assert!(self.is_simple());  // Sanity check
    }

    /// Return where this path is a cycle.
    ///
    /// A cycle is a path that starts and ends at the same location.
    /// A zero-length path is not considered a cycle.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use mazelib::path::Path;
    /// #
    /// assert!(Path::from_vec(vec![1, 2, 1   ]).is_cycle());
    /// assert!(Path::from_vec(vec![1, 2, 2, 1]).is_cycle());
    /// 
    /// assert!(Path::from_vec(vec![1, 2      ]).is_cycle() == false);
    /// assert!(Path::from_vec(vec![1, 2, 2   ]).is_cycle() == false);
    /// assert!(Path::from_vec(vec![1, 2, 1, 2]).is_cycle() == false);
    /// assert!(Path::from_vec(vec![1         ]).is_cycle() == false);
    /// ```
    pub fn is_cycle(&self) -> bool {
        self.locs.len() > 1 && self.locs[0] == self.locs[self.locs.len() - 1]
    }

    /// Return whether this path is a simple cycle.
    ///
    /// A simple cycle is a path that is a cycle and has no repeated locations, except for the
    /// first and last locations.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::path::Path;
    /// #
    /// assert!(Path::from_vec(vec![1, 2, 1   ]).is_simple_cycle());
    /// 
    /// assert!(Path::from_vec(vec![1, 2, 2, 1]).is_simple_cycle() == false);
    /// ```
    ///
    /// # See Also
    /// * [`Self::is_simple`]
    /// * [`Self::is_cycle`]
    pub fn is_simple_cycle(&self) -> bool {
        // Cycle check is cheaper so lets do it first
        self.is_cycle() && Self::is_subpath_simple(&self.locs[0..(self.locs.len() - 1)])
    }
}

impl <T: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug, I: SliceIndex<[T]>> Index<I> for Path<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.locs.index(index)
    }
}

impl <T: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug, I: SliceIndex<[T]>> IndexMut<I> for Path<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.locs.index_mut(index)
    }
}

impl <T: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug> IntoIterator for Path<T> {
    type Item = <Vec<T> as IntoIterator>::Item;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.locs.into_iter()
    }
}