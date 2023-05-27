use fluent_asserter::prelude::*;
use crate::implm::buffer::VecBuffer;
use crate::implm::cell::block::{BlockCellValue, BlockCellValueType};
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::CellID;

#[test]
fn test_construction() {
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(1.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(2.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(3.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(4.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(5.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(6.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(7.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(8.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(9.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(10.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(11.try_into().unwrap())).does_not_panic();
    assert_that_code!(|| VecBuffer::<BlockCellValue>::new(12.try_into().unwrap())).does_not_panic();
}

#[test]
fn test_get_set() {
    let mut buffer = VecBuffer::<BlockCellValue>::new(4.try_into().unwrap());

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
    assert_that_code!(|| {
        let buffer = VecBuffer::<BlockCellValue>::new(4.try_into().unwrap());

        buffer.get(CellID(5));
    }).panics();

    assert_that_code!(|| {
        let mut buffer = VecBuffer::<BlockCellValue>::new(4.try_into().unwrap());

        buffer.get_mut(CellID(5));
    }).panics();

    assert_that_code!(|| {
        let mut buffer = VecBuffer::<BlockCellValue>::new(4.try_into().unwrap());

        buffer.set(CellID(5), BlockCellValue::default());
    }).panics();
}