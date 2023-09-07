#[doc(hidden)]
#[macro_export]
macro_rules! pt {
    () => {
        <<Self as MazeCoordinator>::CoordSpace as CoordinateSpace>::PtType
    };
}

pub(crate) use pt;