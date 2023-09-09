use std::cmp::max;
use std::io::{Result, Write};
use std::num::NonZeroUsize;

use crate::implm::cell::block::{BlockCellValue, BlockCellValueType};
use crate::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinator;
use crate::implm::export::text::TextMazeExporter;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::CellID;
use crate::interface::export::MazeExporter;
use crate::internal::line_break::WriteLineBreak;
use crate::internal::util::{nonzero_usize_array_to_usize_array, NONZERO_USIZE_ONE, NONZERO_USIZE_TWO};

/// An exporter that renders [`BoxSpaceBlockCellMazeCoordinator`]s to text.
pub struct BoxSpaceBlockCellTextMazeExporter {
    /// The character to emit for passage cells.
    passage_char:   char,
    /// The character to emit for wall cells.
    wall_char:      char,
    /// The character to emit for boundary cells.
    boundary_char:  char,
    /// The character to emit for unvisited cells.
    unvisited_char: char,
    
    /// The number of the characters to emit horizontally per cell.
    chars_per_cell_horizontally: NonZeroUsize,
    /// The number of the characters to emit vertically per cell.
    chars_per_cell_vertically:   NonZeroUsize,
}

impl BoxSpaceBlockCellTextMazeExporter {
    /// Construct a new builder for a `BoxSpaceBlockCellTextMazeExporter`.
    ///
    /// Optional, see [`DefaultMazeExporter`][crate::interface::export::DefaultMazeExporter].
    pub fn builder() -> BoxSpaceBlockCellTextMazeExporterBuilder {        
        BoxSpaceBlockCellTextMazeExporterBuilder::new()
    }

    /// Return the character that is emitted for passage cells.
    #[must_use]
    pub fn passage_char(&self) -> char {
        self.passage_char
    }

    /// Return the character that is emitted for wall cells.
    #[must_use]
    pub fn wall_char(&self) -> char {
        self.wall_char
    }

    /// Return the character that is emitted for boundary cells.
    #[must_use]
    pub fn boundary_char(&self) -> char {
        self.boundary_char
    }

    /// Return the character that is emitted for unvisited cells.
    #[must_use]
    pub fn unvisited_char(&self) -> char {
        self.unvisited_char
    }

    /// Return the number of the characters that is emitted horizontally per cell.
    #[must_use]
    pub fn chars_per_cell_horizontally(&self) -> NonZeroUsize {
        self.chars_per_cell_horizontally
    }

    /// Return the number of the characters that is emitted vertically per cell.
    #[must_use]
    pub fn chars_per_cell_vertically(&self) -> NonZeroUsize {
        self.chars_per_cell_vertically
    }
}

impl Default for BoxSpaceBlockCellTextMazeExporter {
    fn default() -> Self {
        Self::builder().build()
    }
}

/// A builder for a [`BoxSpaceBlockCellTextMazeExporter`].
#[must_use]
pub struct BoxSpaceBlockCellTextMazeExporterBuilder {
    /// The character to emit for passage cells.
    passage_char:   char,
    /// The character to emit for wall cells.
    wall_char:      char,
    /// The character to emit for boundary cells.
    boundary_char:  char,
    /// The character to emit for unvisited cells.
    unvisited_char: char,

    /// The number of the characters to emit horizontally per cell.
    chars_per_cell_horizontally: NonZeroUsize,
    /// The number of the characters to emit vertically per cell.
    chars_per_cell_vertically:   NonZeroUsize,
}

impl BoxSpaceBlockCellTextMazeExporterBuilder {
    /// Create a new builder for a [`BoxSpaceBlockCellTextMazeExporter`].
    fn new() -> Self {
        Self {
            passage_char:   ' ',
            wall_char:      '█',
            boundary_char:  '█',
            unvisited_char: '.',
            
            chars_per_cell_horizontally: NONZERO_USIZE_TWO,
            chars_per_cell_vertically:   NONZERO_USIZE_ONE,
        }
    }

    /// Set the character to emit for passage cells.
    pub fn passage_char(mut self, char: char) -> Self {
        self.passage_char = char;

        return self
    }

    /// Set the character to emit for wall cells.
    pub fn wall_char(mut self, char: char) -> Self {
        self.wall_char = char;

        return self
    }

    /// Set the character to emit for boundary cells.
    pub fn boundary_char(mut self, char: char) -> Self {
        self.boundary_char = char;

        return self
    }

    /// Set the character to emit for unvisited cells.
    pub fn unvisited_char(mut self, char: char) -> Self {
        self.unvisited_char = char;

        return self
    }

    /// Set the number of the characters to emit horizontally per cell.
    pub fn chars_per_cell_horizontally(mut self, count: NonZeroUsize) -> Self {
        self.chars_per_cell_horizontally = count;

        return self
    }

    /// Set the number of the characters to emit horizontally per cell.
    /// 
    /// `count` must be non-zero.
    pub fn chars_per_cell_horizontally_checked(self, count: usize) -> Self {
        self.chars_per_cell_horizontally(NonZeroUsize::new(count).expect("count was zero"))
    }

    /// Set the number of the characters to emit vertically per cell.
    pub fn chars_per_cell_vertically(mut self, count: NonZeroUsize) -> Self {
        self.chars_per_cell_vertically = count;

        return self
    }

    /// Set the number of the characters to emit vertically per cell.
    ///
    /// `count` must be non-zero.
    pub fn chars_per_cell_vertically_checked(self, count: usize) -> Self {
        self.chars_per_cell_vertically(NonZeroUsize::new(count).expect("count was zero"))
    }


    /// Finalise the [`BoxSpaceBlockCellTextMazeExporter`].
    #[must_use]
    pub fn build(self) -> BoxSpaceBlockCellTextMazeExporter {
        BoxSpaceBlockCellTextMazeExporter {
            passage_char:   self.passage_char,
            wall_char:      self.wall_char,
            boundary_char:  self.boundary_char,
            unvisited_char: self.unvisited_char,

            chars_per_cell_horizontally: self.chars_per_cell_horizontally,
            chars_per_cell_vertically:   self.chars_per_cell_vertically,
        }
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>, Output: Write> MazeExporter<BoxSpaceBlockCellMazeCoordinator<Buffer, 2>, Output> for BoxSpaceBlockCellTextMazeExporter {
    fn export(&self, maze: &BoxSpaceBlockCellMazeCoordinator<Buffer, 2>, output: &mut Output) -> Result<()> {
        let [width, height] = nonzero_usize_array_to_usize_array(maze.get_full_dimensions());

        let mut passage_char_utf8_buffer = [0u8; 4];
        let mut wall_char_utf8_buffer = [0u8; 4];
        let mut boundary_char_utf8_buffer = [0u8; 4];
        let mut unvisited_char_utf8_buffer = [0u8; 4];

        // Cache this
        let passage_char = self.passage_char.encode_utf8(&mut passage_char_utf8_buffer).as_bytes();
        let wall_char = self.wall_char.encode_utf8(&mut wall_char_utf8_buffer).as_bytes();
        let boundary_char = self.boundary_char.encode_utf8(&mut boundary_char_utf8_buffer).as_bytes();
        let unvisited_char = self.unvisited_char.encode_utf8(&mut unvisited_char_utf8_buffer).as_bytes();

        if self.chars_per_cell_vertically == NONZERO_USIZE_ONE {
            // Optimised for n = 1
            // Avoid storing the line buffer and write to the output directly
            // Makes memory usage O(1)
            // If they're not using a BufWriter and we end up making a bunch of
            // syscalls this'll probably perform worse but that's on them.
            
            for y in 0..height {
                for x in 0..width {
                    let pt = CellID(x + y * width);

                    let char = match maze.buffer().get(pt).cell_type {
                        BlockCellValueType::PASSAGE   => passage_char,
                        BlockCellValueType::WALL      => wall_char,
                        BlockCellValueType::BOUNDARY  => boundary_char,
                        BlockCellValueType::UNVISITED => unvisited_char
                    };

                    for _ in 0..usize::from(self.chars_per_cell_horizontally) {
                        output.write_all(char)?;
                    }
                };

                output.write_line_break()?;
            }
        } else {
            // The maze *probably* has a thick solid border of wall or border cells
            // and the passage cell's byte-encoded length is *probably* less than or
            // equal to the wall/border cell's lengths. If either of those assumptions
            // are false it will self-correct by increasing the capacity size as necessary.
            let mut line_buffer = Vec::with_capacity(width * max(wall_char.len(), boundary_char.len()));

            for y in 0..height {
                for x in 0..width {
                    let pt = CellID(x + y * width);

                    let char = match maze.buffer().get(pt).cell_type {
                        BlockCellValueType::PASSAGE   => passage_char,
                        BlockCellValueType::WALL      => wall_char,
                        BlockCellValueType::BOUNDARY  => boundary_char,
                        BlockCellValueType::UNVISITED => unvisited_char
                    };

                    for _ in 0..usize::from(self.chars_per_cell_horizontally) {
                        line_buffer.extend_from_slice(char);
                    }
                };

                for _ in 0..usize::from(self.chars_per_cell_vertically) {
                    output.write_all(&line_buffer)?;
                    output.write_line_break()?;
                }
                
                // Line buffer is hoisted outside the loop to avoid reallocating it needlessly
                line_buffer.clear();
            }
        }

        return Ok(())
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>, Output: Write> TextMazeExporter<BoxSpaceBlockCellMazeCoordinator<Buffer, 2>, Output> for BoxSpaceBlockCellTextMazeExporter {}