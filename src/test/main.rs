#![allow(unused_imports, unused_mut)]
#![feature(generic_arg_infer)]

use mazelib::implm::point::boxy::BoxCoordinateSpace;
use mazelib::implm::buffer::VecBuffer;
use mazelib::implm::cell::block::{BoxSpaceBlockCellManager, BlockCellValue};
use mazelib::implm::render::text::BoxSpaceBlockCellTextMazeRenderer;
use mazelib::interface::render::MazeRenderer;
use mazelib::interface::cell::CellManager;
use mazelib::implm::template::boxy::SolidBorderTemplate;
use mazelib::interface::template::Template;
use mazelib::implm::generate::BinaryTreeGenerator;
use mazelib::interface::generate::MazeGenerator;
use rand::thread_rng;

fn main() {
    type CellType = BlockCellValue;
    type BufferType = VecBuffer<CellType>;

    let space = BoxCoordinateSpace::new([7, 7]);

    let mut cell_manager = BoxSpaceBlockCellManager::<BufferType, _>::new(space, [2, 2], [[1, 1], [1, 1]]);

    SolidBorderTemplate::apply(&mut cell_manager);

    BinaryTreeGenerator {}.generate(&mut cell_manager, &mut thread_rng());

    let render = BoxSpaceBlockCellTextMazeRenderer::render(&cell_manager);

    for line in render {
        println!("{}", line)
    }
}