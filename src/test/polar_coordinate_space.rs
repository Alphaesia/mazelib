use fluent_asserter::prelude::*;

use crate::implm::point::polar::{PolarCoordinate, PolarCoordinateSpace};
use crate::interface::point::CoordinateSpace;

#[test]
fn test_size() {
    assert_eq!(usize::from(PolarCoordinateSpace::new_checked(1, 1).logical_size()), 1);
    assert_eq!(usize::from(PolarCoordinateSpace::new_checked(2, 1).logical_size()), 2);
    assert_eq!(usize::from(PolarCoordinateSpace::new_checked(1, 2).logical_size()), 2);
    assert_eq!(usize::from(PolarCoordinateSpace::new_checked(2, 2).logical_size()), 4);
    assert_eq!(usize::from(PolarCoordinateSpace::new_checked(9, 9).logical_size()), 81);
    assert_eq!(usize::from(PolarCoordinateSpace::new_checked(61, 73).logical_size()), 4453);

    assert_that_code!(|| PolarCoordinateSpace::new_checked(0, 1)).panics().with_having_message("rings must be non-zero");

    assert_that_code!(|| PolarCoordinateSpace::new_checked(1, 0)).panics().with_having_message("sectors must be non-zero");

    assert_that_code!(|| PolarCoordinateSpace::new_checked(usize::MAX, usize::MAX)).panics().with_having_message("The dimensions specified are too large. The number of points in the space does not fit within a usize.");
}

#[test]
fn test_adjacency() {
    const RINGS: usize = 4;
    const SECTORS: usize = 5;

    let space = PolarCoordinateSpace::new_checked(RINGS, SECTORS);

    for ring in 0..RINGS {
        for sector in 0..SECTORS {
            let pt = PolarCoordinate { ring, sector };

            let prev_sector = if sector == 0 { SECTORS - 1 } else { sector - 1 };
            let next_sector = if sector == SECTORS - 1 { 0 } else { sector + 1 };

            if ring != 0 {
                let prev_ring = ring - 1;

                assert!(space.are_adjacent(pt, PolarCoordinate { ring: prev_ring, sector }));

                assert!(!space.are_adjacent(pt, PolarCoordinate { ring: prev_ring, sector: prev_sector }));
                assert!(!space.are_adjacent(pt, PolarCoordinate { ring: prev_ring, sector: next_sector }));
            }

            assert!(!space.are_adjacent(pt, PolarCoordinate { ring, sector }));

            assert!(space.are_adjacent(pt, PolarCoordinate { ring, sector: prev_sector }));
            assert!(space.are_adjacent(pt, PolarCoordinate { ring, sector: next_sector }));

            if ring != RINGS - 1 {
                let next_ring = ring + 1;

                assert!(space.are_adjacent(pt, PolarCoordinate { ring: next_ring, sector }));

                assert!(!space.are_adjacent(pt, PolarCoordinate { ring: next_ring, sector: prev_sector }));
                assert!(!space.are_adjacent(pt, PolarCoordinate { ring: next_ring, sector: next_sector }));
            }
        }
    }
}