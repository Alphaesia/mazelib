#![allow(unused, unused_imports, unused_mut)]
#![feature(generic_arg_infer)]

use std::fs::File;
use std::io;
use std::io::BufWriter;
use rand::{SeedableRng, thread_rng};
use rand::rngs::{StdRng, ThreadRng};
//use image::ImageFormat;
use mazelib::implm::point::boxy::BoxCoordinateSpace;
use mazelib::implm::buffer::VecBuffer;
use mazelib::implm::cell::block::{BoxSpaceBlockCellManager, BlockCellValue, BoxSpaceBlockCellManagerBuilder};
use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
use mazelib::interface::render::DefaultMazeRendererNonSeeking;
use mazelib::interface::cell::CellManager;
use mazelib::implm::template::boxy::SolidBorderTemplate;
use mazelib::interface::template::Template;
use mazelib::implm::generate::{NAryTreeGenerator, HuntAndKillGenerator, RecursiveBacktrackerGenerator};
use mazelib::interface::generate::DefaultMazeGenerator;
use mazelib::implm::cell::inline::{BoxSpaceInlineCellManager, InlineCellValue, InlineCellValueWallType};
//use mazelib::implm::render::img::BoxSpaceImageMazeRenderer;
//use mazelib::implm::render::minecraft::BoxSpaceSchematicMazeRenderer;
use mazelib::interface::buffer::{BufferLocation, MazeBuffer};

fn main() {
    const DIMENSION: usize = 2;

    type Space = BoxCoordinateSpace<DIMENSION>;
    type CellType = BlockCellValue;
    type BufferType = VecBuffer<CellType>;
    type CellManager = BoxSpaceBlockCellManagerBuilder<BufferType, DIMENSION>;
    type Template = SolidBorderTemplate;
    type Generator = HuntAndKillGenerator;
    type Renderer = BoxSpaceTextMazeRenderer;

    let mut rng = thread_rng();

    let space = Space::new([9, 9]);

    let mut cell_manager = CellManager::new(space).build();

    //Template::apply(&mut cell_manager);

    Generator::generate_with_rng(&mut cell_manager, &mut rng);

    let output = std::io::stdout();

    Renderer::render(&cell_manager, &mut BufWriter::new(output)).unwrap();
}