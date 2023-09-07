use std::io::{Result, Write};

use crate::implm::cell::block::{BlockCellValue, BlockCellValueType};
use crate::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinator;
use crate::implm::export::text::{BoxSpaceTextMazeExporter, TextMazeExporter};
use crate::implm::export::text::line_break::WriteLineBreak;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::CellID;
use crate::interface::export::MazeExporter;
use crate::internal::util::nonzero_usize_array_to_usize_array;

impl <Buffer: MazeBuffer<BlockCellValue>, Output: Write> MazeExporter<BoxSpaceBlockCellMazeCoordinator<Buffer, 2>, Output> for BoxSpaceTextMazeExporter {
    fn export(&self, maze: &BoxSpaceBlockCellMazeCoordinator<Buffer, 2>, output: &mut Output) -> Result<()> {
        let [width, height] = nonzero_usize_array_to_usize_array(maze.get_full_dimensions());

        for y in 0..height {
            for x in 0..width {
                let pt = CellID(x + y * width);

                let char = match maze.buffer().get(pt).cell_type {
                    BlockCellValueType::PASSAGE   => ' ',
                    BlockCellValueType::WALL      => '█',
                    BlockCellValueType::BOUNDARY  => '█',
                    BlockCellValueType::UNVISITED => '.'
                };

                output.write_all(char.to_string().as_bytes())?;
            };

            output.write_line_break()?;
        }

        return Ok(())
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>, Output: Write> TextMazeExporter<BoxSpaceBlockCellMazeCoordinator<Buffer, 2>, Output> for BoxSpaceTextMazeExporter {}