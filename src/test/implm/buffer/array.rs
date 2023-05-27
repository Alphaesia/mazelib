use std::num::NonZeroUsize;
use fluent_asserter::prelude::*;
use crate::implm::buffer::ArrayBuffer;
use crate::implm::cell::block::{BlockCellValue, BlockCellValueType};
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::CellID;

#[test]
fn test_construction() {
    assert_that_code!(|| ArrayBuffer::<BlockCellValue, 5>::new(5.try_into().unwrap())).does_not_panic();

    assert_that_code!(|| ArrayBuffer::<BlockCellValue, 5>::new(4.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| ArrayBuffer::<BlockCellValue, 5>::new(3.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| ArrayBuffer::<BlockCellValue, 5>::new(2.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| ArrayBuffer::<BlockCellValue, 5>::new(1.try_into().unwrap())).does_not_panic();

    assert_that_code!(|| ArrayBuffer::<BlockCellValue, 5>::new(6.try_into().unwrap())).panics().with_message("cell_count is greater than the buffer's capacity");
    assert_that_code!(|| ArrayBuffer::<BlockCellValue, 5>::new(7.try_into().unwrap())).panics().with_message("cell_count is greater than the buffer's capacity");
    assert_that_code!(|| ArrayBuffer::<BlockCellValue, 5>::new(8.try_into().unwrap())).panics().with_message("cell_count is greater than the buffer's capacity");
    assert_that_code!(|| ArrayBuffer::<BlockCellValue, 5>::new(9.try_into().unwrap())).panics().with_message("cell_count is greater than the buffer's capacity");

    assert_that_code!(|| ArrayBuffer::<BlockCellValue, 5>::new(NonZeroUsize::MAX)).panics().with_message("cell_count is greater than the buffer's capacity");
}

#[test]
fn test_get_set() {
    let mut buffer = ArrayBuffer::<BlockCellValue, 4>::new(4.try_into().unwrap());

    assert_eq!(BlockCellValue::default(), buffer.get(CellID(0)));
    assert_eq!(BlockCellValue::default(), buffer.get(CellID(1)));
    assert_eq!(BlockCellValue::default(), buffer.get(CellID(2)));
    assert_eq!(BlockCellValue::default(), buffer.get(CellID(3)));

    buffer.set(CellID(1), BlockCellValue { cell_type: BlockCellValueType::BOUNDARY, marked: false });

    assert_eq!(BlockCellValue::default(), buffer.get(CellID(0)));
    assert_eq!(BlockCellValue { cell_type: BlockCellValueType::BOUNDARY, marked: false }, buffer.get(CellID(1)));
    assert_eq!(BlockCellValue::default(), buffer.get(CellID(2)));
    assert_eq!(BlockCellValue::default(), buffer.get(CellID(3)));

    let cell_ref = buffer.get_mut(CellID(2));

    cell_ref.marked = true;

    assert_eq!(BlockCellValue::default(), buffer.get(CellID(0)));
    assert_eq!(BlockCellValue { cell_type: BlockCellValueType::BOUNDARY, marked: false }, buffer.get(CellID(1)));
    assert_eq!(BlockCellValue { cell_type: BlockCellValueType::default(), marked: true }, buffer.get(CellID(2)));
    assert_eq!(BlockCellValue::default(), buffer.get(CellID(3)));

    buffer.set(CellID(1), BlockCellValue { cell_type: BlockCellValueType::PASSAGE, marked: true });

    assert_eq!(BlockCellValue::default(), buffer.get(CellID(0)));
    assert_eq!(BlockCellValue { cell_type: BlockCellValueType::PASSAGE, marked: true }, buffer.get(CellID(1)));
    assert_eq!(BlockCellValue { cell_type: BlockCellValueType::default(), marked: true }, buffer.get(CellID(2)));
    assert_eq!(BlockCellValue::default(), buffer.get(CellID(3)));

    buffer.set(CellID(2), BlockCellValue { cell_type: BlockCellValueType::WALL, marked: true });

    assert_eq!(BlockCellValue::default(), buffer.get(CellID(0)));
    assert_eq!(BlockCellValue { cell_type: BlockCellValueType::PASSAGE, marked: true }, buffer.get(CellID(1)));
    assert_eq!(BlockCellValue { cell_type: BlockCellValueType::WALL, marked: true }, buffer.get(CellID(2)));
    assert_eq!(BlockCellValue::default(), buffer.get(CellID(3)));

    buffer.set(CellID(2), BlockCellValue::default());

    assert_eq!(BlockCellValue::default(), buffer.get(CellID(0)));
    assert_eq!(BlockCellValue { cell_type: BlockCellValueType::PASSAGE, marked: true }, buffer.get(CellID(1)));
    assert_eq!(BlockCellValue::default(), buffer.get(CellID(2)));
    assert_eq!(BlockCellValue::default(), buffer.get(CellID(3)));
}

#[test]
#[allow(unused_must_use)]
fn test_out_of_bounds_handling() {
    // Buffer size == capacity

    assert_that_code!(|| {
        let buffer = ArrayBuffer::<BlockCellValue, 4>::new(4.try_into().unwrap());

        buffer.get(CellID(5));
    }).panics();

    assert_that_code!(|| {
        let mut buffer = ArrayBuffer::<BlockCellValue, 4>::new(4.try_into().unwrap());

        buffer.get_mut(CellID(5));
    }).panics();

    assert_that_code!(|| {
        let mut buffer = ArrayBuffer::<BlockCellValue, 4>::new(4.try_into().unwrap());

        buffer.set(CellID(5), BlockCellValue::default());
    }).panics();

    // Buffer size < capacity

    assert_that_code!(|| {
        let buffer = ArrayBuffer::<BlockCellValue, 8>::new(4.try_into().unwrap());

        buffer.get(CellID(5));
    }).panics();

    assert_that_code!(|| {
        let mut buffer = ArrayBuffer::<BlockCellValue, 8>::new(4.try_into().unwrap());

        buffer.get_mut(CellID(5));
    }).panics();

    assert_that_code!(|| {
        let mut buffer = ArrayBuffer::<BlockCellValue, 8>::new(4.try_into().unwrap());

        buffer.set(CellID(5), BlockCellValue::default());
    }).panics();
}