use image::{Rgba, RgbaImage};
use crate::interface::buffer::MazeBuffer;
use crate::interface::render::MazeRenderer;
use crate::implm::cell::block::{BoxSpaceBlockCellManager, BlockCellValue};
use crate::implm::render::img::{BoxSpaceImageMazeRenderer, ImageMazeRenderer};

impl <Buffer: MazeBuffer<BlockCellValue>> MazeRenderer<BoxSpaceBlockCellManager<Buffer, 2>> for BoxSpaceImageMazeRenderer {
    type Output = RgbaImage;

    fn render(maze: &BoxSpaceBlockCellManager<Buffer, 2>) -> Self::Output {
        let [width, height] = maze.get_full_dimensions().map(|dim| TryInto::<u32>::try_into(dim).expect("Cannot render mazes with dimensions larger than u32"));

        let mut img = RgbaImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel = match maze.get_cell([x, y].into()) {
                    BlockCellValue::PASSAGE => Rgba::from([255, 255, 255, 255]),
                    BlockCellValue::WALL | BlockCellValue::BOUNDARY => Rgba::from([0, 0, 0, 255]),
                    BlockCellValue::UNVISITED => Rgba::from([0, 0, 0, 0]),
                };

                img.put_pixel(x as u32, y as u32, pixel);
            }
        }

        return img;
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>> ImageMazeRenderer<BoxSpaceBlockCellManager<Buffer, 2>> for BoxSpaceImageMazeRenderer {}