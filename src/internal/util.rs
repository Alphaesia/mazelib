#[cfg(windows)]
pub fn get_line_sep() -> &'static str {
    "\r\n"
}

#[cfg(not(windows))]
pub fn get_line_sep() -> &'static str {
    "\n"
}