#![allow(unused, unused_imports, unused_mut)]
#![feature(generic_arg_infer)]

use std::fs::File;
use std::io;
use std::io::BufWriter;

#[cfg(feature = "img")] use image::ImageFormat;
use rand::{SeedableRng, thread_rng};
use rand::rngs::{StdRng, ThreadRng};

use mazelib::implm::buffer::VecBuffer;
use mazelib::implm::cell::block::BlockCellValue;
use mazelib::implm::cell::inline::{InlineCellValue, InlineCellValueWallType};
use mazelib::implm::generate::{HuntAndKillGenerator, NAryTreeGenerator, RecursiveBacktrackerGenerator};
use mazelib::implm::maze::block::BoxSpaceBlockCellMazeBuilder;
use mazelib::implm::point::boxy::BoxCoordinateSpace;
#[cfg(feature = "img")] use mazelib::implm::render::img::BoxSpaceImageMazeRenderer;
#[cfg(feature = "minecraft")] use mazelib::implm::render::minecraft::BoxSpaceSchematicMazeRenderer;
use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
use mazelib::interface::buffer::MazeBuffer;
use mazelib::interface::cell::CellID;
use mazelib::interface::generate::DefaultMazeGenerator;
use mazelib::interface::render::DefaultMazeRendererNonSeeking;
use mazelib::util::solid_border;

fn main() {
    const DIMENSION: usize = 2;

    type Space = BoxCoordinateSpace<DIMENSION>;
    type CellType = BlockCellValue;
    type BufferType = VecBuffer<CellType>;
    type Maze = BoxSpaceBlockCellMazeBuilder<BufferType, DIMENSION>;
    type Generator = HuntAndKillGenerator;
    type Renderer = BoxSpaceTextMazeRenderer;

    let mut rng = thread_rng();

    let space = Space::new([9, 9]);

    let mut maze = Maze::new(space).build();

    solid_border(&mut maze);

    Generator::generate_with_rng(&mut maze, &mut rng);

    let output = std::io::stdout();

    Renderer::render(&maze, &mut BufWriter::new(output)).unwrap();
}