use std::io::{Result, Write};

pub(super) trait WriteLineBreak: Write {
    fn write_line_break(&mut self) -> Result<()>;
}

impl <T: Write> WriteLineBreak for T {
    fn write_line_break(&mut self) -> Result<()> {
        #[cfg(windows)]
        self.write_all('\r'.to_string().as_bytes())?;

        self.write_all('\n'.to_string().as_bytes())?;

        return Ok(())
    }
}