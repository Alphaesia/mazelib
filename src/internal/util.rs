use std::num::{NonZeroUsize, TryFromIntError};

#[cfg(windows)]
#[must_use]
pub fn get_line_sep() -> &'static str {
    "\r\n"
}

#[cfg(not(windows))]
#[must_use]
pub fn get_line_sep() -> &'static str {
    "\n"
}

// SAFETY: trivial
pub const NONZERO_USIZE_ONE:   NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1) };
pub const NONZERO_USIZE_TWO:   NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(2) };
pub const NONZERO_USIZE_THREE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(3) };

#[must_use]
pub fn nonzero_usize_array_to_usize_array<const LENGTH: usize>(array: [NonZeroUsize; LENGTH]) -> [usize; LENGTH] {
    array.map(|x| usize::from(x))
}

#[must_use]
pub fn try_usize_array_to_nonzero_usize_array<const LENGTH: usize>(array: [usize; LENGTH]) -> Result<[NonZeroUsize; LENGTH], TryFromIntError> {
    array.try_map(|x| NonZeroUsize::try_from(x))
}

#[inline]
#[must_use]
pub fn offset_usize(mut value: usize, offset: isize) -> usize {
    if offset >= 0 {
        value += TryInto::<usize>::try_into(offset).unwrap();
    } else {
        value -= TryInto::<usize>::try_into(offset.abs()).unwrap();
    }

    return value
}
