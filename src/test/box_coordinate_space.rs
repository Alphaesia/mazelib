use fluent_asserter::prelude::*;

use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::interface::point::CoordinateSpace;

#[test]
fn test_size() {
    // One-dimensional
    assert_eq!(usize::from(BoxCoordinateSpace::<1>::new_checked([1]).logical_size()), 1);
    assert_eq!(usize::from(BoxCoordinateSpace::<1>::new_checked([2]).logical_size()), 2);
    assert_eq!(usize::from(BoxCoordinateSpace::<1>::new_checked([3]).logical_size()), 3);
    assert_eq!(usize::from(BoxCoordinateSpace::<1>::new_checked([9]).logical_size()), 9);
    assert_eq!(usize::from(BoxCoordinateSpace::<1>::new_checked([10]).logical_size()), 10);

    assert_eq!(usize::from(BoxCoordinateSpace::<2>::new_checked([1, 1]).logical_size()), 1);
    assert_eq!(usize::from(BoxCoordinateSpace::<2>::new_checked([2, 1]).logical_size()), 2);
    assert_eq!(usize::from(BoxCoordinateSpace::<2>::new_checked([1, 2]).logical_size()), 2);
    assert_eq!(usize::from(BoxCoordinateSpace::<2>::new_checked([2, 2]).logical_size()), 4);
    assert_eq!(usize::from(BoxCoordinateSpace::<2>::new_checked([9, 9]).logical_size()), 81);
    assert_eq!(usize::from(BoxCoordinateSpace::<2>::new_checked([61, 73]).logical_size()), 4453);

    assert_eq!(usize::from(BoxCoordinateSpace::<3>::new_checked([1, 1, 1]).logical_size()), 1);
    assert_eq!(usize::from(BoxCoordinateSpace::<3>::new_checked([2, 1, 1]).logical_size()), 2);
    assert_eq!(usize::from(BoxCoordinateSpace::<3>::new_checked([1, 2, 1]).logical_size()), 2);
    assert_eq!(usize::from(BoxCoordinateSpace::<3>::new_checked([2, 2, 1]).logical_size()), 4);
    assert_eq!(usize::from(BoxCoordinateSpace::<3>::new_checked([2, 2, 2]).logical_size()), 8);
    assert_eq!(usize::from(BoxCoordinateSpace::<3>::new_checked([9, 9, 9]).logical_size()), 729);
    assert_eq!(usize::from(BoxCoordinateSpace::<3>::new_checked([61, 73, 12]).logical_size()), 53436);

    assert_that_code!(|| BoxCoordinateSpace::new_checked([0])).panics().with_having_message("All dimensions must be non-zero");
    assert_that_code!(|| BoxCoordinateSpace::new_checked([0, 1])).panics().with_having_message("All dimensions must be non-zero");
    assert_that_code!(|| BoxCoordinateSpace::new_checked([1, 0])).panics().with_having_message("All dimensions must be non-zero");
    assert_that_code!(|| BoxCoordinateSpace::new_checked([1, 0, 0])).panics().with_having_message("All dimensions must be non-zero");
    assert_that_code!(|| BoxCoordinateSpace::new_checked([1, 1, 0])).panics().with_having_message("All dimensions must be non-zero");
    assert_that_code!(|| BoxCoordinateSpace::new_checked([0, 1, 1])).panics().with_having_message("All dimensions must be non-zero");

    assert_that_code!(|| BoxCoordinateSpace::new_checked([usize::MAX, usize::MAX])).panics().with_message("The dimensions specified are too large. The number of points in the space does not fit within a usize.");
}

#[test]
fn test_adjacency() {
    let space = BoxCoordinateSpace::new_checked([9, 9, 9]);

    // Check point is not adjacent to itself
    assert!(!space.are_adjacent([1, 1, 1].into(), [1, 1, 1].into()));

    assert!(space.are_adjacent([1, 1, 1].into(), [2, 1, 1].into()));
    assert!(space.are_adjacent([1, 1, 1].into(), [1, 2, 1].into()));
    assert!(space.are_adjacent([1, 1, 1].into(), [1, 1, 2].into()));

    assert!(!space.are_adjacent([1, 1, 1].into(), [2, 2, 1].into()));
    assert!(!space.are_adjacent([1, 1, 1].into(), [2, 1, 2].into()));
    assert!(!space.are_adjacent([1, 1, 1].into(), [1, 2, 2].into()));
    assert!(!space.are_adjacent([1, 1, 1].into(), [2, 2, 2].into()));

    // Check no wrapping
    assert!(!space.are_adjacent([0, 0, 0].into(), [8, 0, 0].into()));
    assert!(!space.are_adjacent([0, 0, 0].into(), [0, 8, 0].into()));
    assert!(!space.are_adjacent([0, 0, 0].into(), [0, 0, 8].into()));
}