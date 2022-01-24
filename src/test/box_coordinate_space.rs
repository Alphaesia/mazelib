use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::interface::point::CoordinateSpace;

#[test]
fn test_size() {
    // Zero-dimensional
    assert_eq!(BoxCoordinateSpace::<0>::new([]).logical_size(), 0);

    // One-dimensional
    assert_eq!(BoxCoordinateSpace::<1>::new([0]).logical_size(), 0);
    assert_eq!(BoxCoordinateSpace::<1>::new([1]).logical_size(), 1);
    assert_eq!(BoxCoordinateSpace::<1>::new([2]).logical_size(), 2);
    assert_eq!(BoxCoordinateSpace::<1>::new([3]).logical_size(), 3);
    assert_eq!(BoxCoordinateSpace::<1>::new([9]).logical_size(), 9);
    assert_eq!(BoxCoordinateSpace::<1>::new([10]).logical_size(), 10);

    assert_eq!(BoxCoordinateSpace::<2>::new([0, 0]).logical_size(), 0);
    assert_eq!(BoxCoordinateSpace::<2>::new([1, 0]).logical_size(), 0);
    assert_eq!(BoxCoordinateSpace::<2>::new([0, 1]).logical_size(), 0);
    assert_eq!(BoxCoordinateSpace::<2>::new([1, 1]).logical_size(), 1);
    assert_eq!(BoxCoordinateSpace::<2>::new([2, 1]).logical_size(), 2);
    assert_eq!(BoxCoordinateSpace::<2>::new([1, 2]).logical_size(), 2);
    assert_eq!(BoxCoordinateSpace::<2>::new([2, 2]).logical_size(), 4);
    assert_eq!(BoxCoordinateSpace::<2>::new([9, 9]).logical_size(), 81);
    assert_eq!(BoxCoordinateSpace::<2>::new([61, 73]).logical_size(), 4453);

    assert_eq!(BoxCoordinateSpace::<3>::new([0, 0, 0]).logical_size(), 0);
    assert_eq!(BoxCoordinateSpace::<3>::new([1, 0, 0]).logical_size(), 0);
    assert_eq!(BoxCoordinateSpace::<3>::new([0, 1, 0]).logical_size(), 0);
    assert_eq!(BoxCoordinateSpace::<3>::new([0, 0, 1]).logical_size(), 0);
    assert_eq!(BoxCoordinateSpace::<3>::new([1, 1, 0]).logical_size(), 0);
    assert_eq!(BoxCoordinateSpace::<3>::new([0, 1, 1]).logical_size(), 0);
    assert_eq!(BoxCoordinateSpace::<3>::new([1, 0, 1]).logical_size(), 0);
    assert_eq!(BoxCoordinateSpace::<3>::new([1, 1, 1]).logical_size(), 1);
    assert_eq!(BoxCoordinateSpace::<3>::new([2, 1, 1]).logical_size(), 2);
    assert_eq!(BoxCoordinateSpace::<3>::new([1, 2, 1]).logical_size(), 2);
    assert_eq!(BoxCoordinateSpace::<3>::new([2, 2, 1]).logical_size(), 4);
    assert_eq!(BoxCoordinateSpace::<3>::new([2, 2, 2]).logical_size(), 8);
    assert_eq!(BoxCoordinateSpace::<3>::new([9, 9, 9]).logical_size(), 729);
    assert_eq!(BoxCoordinateSpace::<3>::new([61, 73, 12]).logical_size(), 53436);
}