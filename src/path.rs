use std::cmp::max;
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
        let mut first_seen_indices_of_pts = HashMap::<T, usize>::new();

        let mut ranges_to_remove: Vec<RangeInclusive<usize>> = Vec::new();

        'outer: for (i, pt) in self.vec.iter().enumerate() {
            match first_seen_indices_of_pts.entry(*pt) {
                Occupied(entry) => {
                    // Indices are inclusive

                    let start_index = *entry.get();
                    let mut end_index = i;

                    // Hooray for long variable names

                    let mut removals_to_condense_start_index = 0usize;

                    // We're looking for the first range that intersects our current range
                    loop {
                        if removals_to_condense_start_index >= ranges_to_remove.len() {
                            break;
                        }

                        let existing_removal_range = &ranges_to_remove[removals_to_condense_start_index];

                        let (existing_removal_range_start_index, existing_removal_range_end_index) = (*existing_removal_range.start(), *existing_removal_range.end());

                        // If another range has a start >= ours, we know that our range will fully encompass it
                        if existing_removal_range_start_index >= start_index {
                            break;  // we're done here
                        } else {
                            if existing_removal_range_end_index >= start_index {
                                if end_index > existing_removal_range_end_index {
                                    for j in (removals_to_condense_start_index..ranges_to_remove.len()).rev() {
                                        let potential_overlap = &ranges_to_remove[j].clone();

                                        if potential_overlap.start() <= &end_index {
                                            end_index = *potential_overlap.end();

                                            ranges_to_remove.drain((removals_to_condense_start_index+1)..=j);
                                        }
                                    }

                                    ranges_to_remove[removals_to_condense_start_index] = existing_removal_range_start_index..=end_index;
                                }

                                continue 'outer;
                            } else {
                                removals_to_condense_start_index += 1;
                            }
                        }
                    }

                    let mut removals_to_condense_end_index = removals_to_condense_start_index;

                    // <= because want to amalgamate a range that starts immediately where we end
                    while removals_to_condense_end_index < ranges_to_remove.len() && ranges_to_remove[removals_to_condense_end_index].start() <= &end_index {
                        removals_to_condense_end_index += 1;
                    }

                    // If we're combining any removal ranges, use the greater of the end indices
                    end_index = if removals_to_condense_end_index > removals_to_condense_start_index {
                        max(end_index, *ranges_to_remove[removals_to_condense_end_index - 1].end())
                    } else {
                        end_index
                    };

                    dbg!(&ranges_to_remove);

                    dbg!((start_index+1)..=end_index);

                    ranges_to_remove.drain(removals_to_condense_start_index..removals_to_condense_end_index);

                    ranges_to_remove.insert(removals_to_condense_start_index, (start_index+1)..=end_index);

                    dbg!(&ranges_to_remove);

                    eprintln!("================");
                },
                Vacant(entry) => {
                    entry.insert(i);
                },
            }
        }

        dbg!(&ranges_to_remove);

        for range_to_remove in ranges_to_remove.into_iter().rev() {
            self.vec.drain(range_to_remove);
        }

        dbg!(&self, self.len());

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