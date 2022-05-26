use std::io::{Result, Write};
use crate::internal::util::get_line_sep;

pub(super) trait WriteLineBreak: Write {
    fn write_line_break(&mut self) -> Result<()>;
}

impl <T: Write> WriteLineBreak for T {
    fn write_line_break(&mut self) -> Result<()> {
        self.write_all(get_line_sep().as_bytes())
    }
}