use std::io::{Result, Write};
use crate::interface::buffer::MazeBuffer;
use crate::interface::render::MazeRendererNonSeeking;
use crate::implm::cell::block::{BlockCellValue, BlockCellValueType};
use crate::implm::maze::block::BoxSpaceBlockCellMaze;
use crate::implm::render::text::{BoxSpaceTextMazeRenderer, TextMazeRenderer};
use crate::implm::render::text::line_break::WriteLineBreak;
use crate::interface::cell::CellID;

impl <Buffer: MazeBuffer<BlockCellValue>> MazeRendererNonSeeking<BoxSpaceBlockCellMaze<Buffer, 2>> for BoxSpaceTextMazeRenderer {
    fn render<Output: Write>(&self, maze: &BoxSpaceBlockCellMaze<Buffer, 2>, output: &mut Output) -> Result<()> {
        let [width, height] = maze.get_full_dimensions();

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

impl <Buffer: MazeBuffer<BlockCellValue>> TextMazeRenderer<BoxSpaceBlockCellMaze<Buffer, 2>> for BoxSpaceTextMazeRenderer {}