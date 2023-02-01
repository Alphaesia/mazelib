use std::num::{NonZeroUsize, TryFromIntError};

#[cfg(windows)]
pub fn get_line_sep() -> &'static str {
    "\r\n"
}

#[cfg(not(windows))]
pub fn get_line_sep() -> &'static str {
    "\n"
}

// SAFETY: trivial
pub const NONZERO_USIZE_ONE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1) };
pub const NONZERO_USIZE_TWO: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(2) };

pub fn nonzero_usize_array_to_usize_array<const LENGTH: usize>(array: [NonZeroUsize; LENGTH]) -> [usize; LENGTH] {
    array.map(|x| usize::from(x))
}

pub fn try_usize_array_to_nonzero_usize_array<const LENGTH: usize>(array: [usize; LENGTH]) -> Result<[NonZeroUsize; LENGTH], TryFromIntError> {
    array.try_map(|x| NonZeroUsize::try_from(x))
}