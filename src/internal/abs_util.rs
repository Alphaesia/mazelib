/// Find the absolute value of the difference of two unsigned integers
#[inline(always)]
pub fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}