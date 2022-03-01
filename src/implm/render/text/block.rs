use crate::interface::buffer::{MazeBuffer, BufferLocation};
use crate::interface::render::MazeRenderer;
use crate::implm::cell::block::{BoxSpaceBlockCellManager, BlockCellValue};
use crate::implm::render::text::{TextMazeRenderer, BoxSpaceTextMazeRenderer};

impl <Buffer: MazeBuffer<BlockCellValue>> MazeRenderer<BoxSpaceBlockCellManager<Buffer, 2>> for BoxSpaceTextMazeRenderer {
    type Output = Vec<String>;

    fn render(maze: &BoxSpaceBlockCellManager<Buffer, 2>) -> Self::Output {
        let [width, height] = maze.get_full_dimensions();

        let mut render = Vec::with_capacity(height);

        for y in 0..height {
            let mut line: String = String::with_capacity(width);

            for x in 0..width {
                let pt = BufferLocation(x + y * width);

                line.push(match maze.buffer().get(pt) {
                    BlockCellValue::PASSAGE => ' ',
                    BlockCellValue::WALL => '█',
                    BlockCellValue::BOUNDARY => '█',
                    BlockCellValue::UNVISITED => '.'
                });
            };

            render.push(line);
        }

        return render
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>> TextMazeRenderer<BoxSpaceBlockCellManager<Buffer, 2>> for BoxSpaceTextMazeRenderer {}