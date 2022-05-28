use std::io::{Result, Write};
use crate::interface::buffer::{MazeBuffer, BufferLocation};
use crate::interface::render::MazeRendererNonSeeking;
use crate::implm::cell::block::{BoxSpaceBlockCellManager, BlockCellValue, BlockCellValueType};
use crate::implm::render::text::{TextMazeRenderer, BoxSpaceTextMazeRenderer};
use crate::implm::render::text::line_break::WriteLineBreak;

impl <Buffer: MazeBuffer<BlockCellValue>> MazeRendererNonSeeking<BoxSpaceBlockCellManager<Buffer, 2>> for BoxSpaceTextMazeRenderer {
    fn render<Output: Write>(&self, maze: &BoxSpaceBlockCellManager<Buffer, 2>, output: &mut Output) -> Result<()> {
        let [width, height] = maze.get_full_dimensions();

        for y in 0..height {
            for x in 0..width {
                let pt = BufferLocation(x + y * width);

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

impl <Buffer: MazeBuffer<BlockCellValue>> TextMazeRenderer<BoxSpaceBlockCellManager<Buffer, 2>> for BoxSpaceTextMazeRenderer {}