use std::io::{Result, Write};

use crate::implm::cell::block::{BlockCell, BlockCellValue, BlockCellValueType};
use crate::implm::maze::block::BoxSpaceBlockCellMaze;
use crate::implm::render::text::{BoxSpaceTextMazeRenderer, TextMazeRenderer};
use crate::implm::render::text::line_break::WriteLineBreak;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::CellManager;
use crate::interface::render::MazeRendererNonSeeking;

impl <Buffer: MazeBuffer<BlockCellValue>> MazeRendererNonSeeking<BoxSpaceBlockCellMaze<Buffer, 2>> for BoxSpaceTextMazeRenderer<BoxSpaceBlockCellMaze<Buffer, 2>> {
    fn render<Output: Write>(&self, maze: &BoxSpaceBlockCellMaze<Buffer, 2>, output: &mut Output) -> Result<()> {
        let [width, height] = maze.get_full_dimensions();

        let path = self.path.as_ref().map(|path| maze.map_pt_path_to_cell_path(path));

        for y in 0..height {
            for x in 0..width {
                let cell_loc: BlockCell<2> = [x, y].into();

                let is_on_path = path.as_ref().map_or(false, |path| path.contains(cell_loc));

                let char = match maze.get_cell_value(cell_loc).cell_type {
                    BlockCellValueType::PASSAGE   => if is_on_path { '*' } else { ' ' },
                    BlockCellValueType::WALL      => if is_on_path { 'X' } else { '█' },
                    BlockCellValueType::BOUNDARY  => if is_on_path { '#' } else { '█' },
                    BlockCellValueType::UNVISITED => if is_on_path { '%' } else { '.' }
                };

                output.write_all(char.to_string().as_bytes())?;
            };

            output.write_line_break()?;
        }

        return Ok(())
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>> TextMazeRenderer<BoxSpaceBlockCellMaze<Buffer, 2>> for BoxSpaceTextMazeRenderer<BoxSpaceBlockCellMaze<Buffer, 2>> {}