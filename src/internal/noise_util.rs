#[macro_export]
macro_rules! pt {
    () => {
        <<Self as CellManager>::CoordSpace as CoordinateSpace>::PtType
    };
}

pub(crate) use pt;