#![allow(unused, unused_imports, unused_mut)]
#![feature(generic_arg_infer)]

use mazelib::implm::point::boxy::BoxCoordinateSpace;
use mazelib::implm::buffer::VecBuffer;
use mazelib::implm::cell::block::{BoxSpaceBlockCellManager, BlockCellValue};
use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
use mazelib::interface::render::MazeRenderer;
use mazelib::interface::cell::CellManager;
use mazelib::implm::template::boxy::SolidBorderTemplate;
use mazelib::interface::template::Template;
use mazelib::implm::generate::{NAryTreeGenerator, HuntAndKillGenerator, RecursiveBacktrackerGenerator};
use mazelib::interface::generate::MazeGenerator;
use rand::thread_rng;
use mazelib::implm::cell::inline::{BoxSpaceInlineCellManager, InlineCellValue, InlineCellValueWallType};
use mazelib::interface::buffer::{BufferLocation, MazeBuffer};
use mazelib::util::convert_unvisited_to_walls;

fn main() {
    const DIMENSION: usize = 2;

    type Space = BoxCoordinateSpace<DIMENSION>;
    type CellType = InlineCellValue<DIMENSION>;
    type BufferType = VecBuffer<CellType>;
    type CellManager = BoxSpaceInlineCellManager<BufferType, DIMENSION>;
    type Template = SolidBorderTemplate;
    type Generator = RecursiveBacktrackerGenerator;
    type Renderer = BoxSpaceTextMazeRenderer;

    let space = Space::new([9, 9]);

    let mut cell_manager = CellManager::new(space);

    Template::apply(&mut cell_manager);

    Generator::generate(&mut cell_manager, &mut thread_rng());

    // convert_unvisited_to_walls(&mut cell_manager);

    let render = Renderer::render(&cell_manager);

    for line in render {
        println!("{}", line)
    }
}