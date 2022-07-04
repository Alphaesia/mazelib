use std::io::{Result, Seek, Write};

use image::{ImageError, Rgba, RgbaImage};

use crate::implm::cell::block::{BlockCellValue, BlockCellValueType};
use crate::implm::maze::block::BoxSpaceBlockCellMaze;
use crate::implm::render::img::{BoxSpaceImageMazeRenderer, ImageMazeRenderer};
use crate::interface::buffer::MazeBuffer;
use crate::interface::render::MazeRenderer;

impl <Buffer: MazeBuffer<BlockCellValue>> MazeRenderer<BoxSpaceBlockCellMaze<Buffer, 2>> for BoxSpaceImageMazeRenderer {
    fn render<Output: Write + Seek>(&self, maze: &BoxSpaceBlockCellMaze<Buffer, 2>, output: &mut Output) -> Result<()> {
        let [width, height] = maze.get_full_dimensions().map(|dim| TryInto::<u32>::try_into(dim).expect("Cannot render mazes with dimensions larger than u32"));

        let mut img = RgbaImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel = match maze.get_cell_value([x, y].into()).cell_type {
                    BlockCellValueType::PASSAGE => Rgba::from([255, 255, 255, 255]),
                    BlockCellValueType::WALL | BlockCellValueType::BOUNDARY => Rgba::from([0, 0, 0, 255]),
                    BlockCellValueType::UNVISITED => Rgba::from([0, 0, 0, 0]),
                };

                img.put_pixel(x as u32, y as u32, pixel);
            }
        }

        return match img.write_to(output, self.format) {
            Ok(_) => Ok(()),
            Err(err) => match err {
                ImageError::Decoding(_) => unreachable!("Decoding error encountered during encoding???"),
                ImageError::Encoding(err) => panic!("[Bug] Failed to write image: {}", err),
                ImageError::Parameter(err) => panic!("[Bug] Failed to write image: {}", err),
                ImageError::Limits(err) => panic!("{}", err),
                ImageError::Unsupported(err) => panic!("{}", err),
                ImageError::IoError(err) => Err(err),
            }
        }
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>> ImageMazeRenderer<BoxSpaceBlockCellMaze<Buffer, 2>> for BoxSpaceImageMazeRenderer {}