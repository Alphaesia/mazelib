#![allow(unused_imports)]

use std::io::BufWriter;

#[cfg(feature = "img")] use image::ImageFormat;
use rand::thread_rng;

use mazelib::implm::buffer::VecBuffer;
use mazelib::implm::cell::block::BlockCellValue;
use mazelib::implm::cell::inline::InlineCellValue;
use mazelib::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinatorBuilder;
use mazelib::implm::coordinator::inline::BoxSpaceInlineCellMazeCoordinatorBuilder;
use mazelib::implm::generate::{HuntAndKillGenerator, NAryTreeGenerator, RecursiveBacktrackerGenerator};
use mazelib::implm::point::boxy::BoxCoordinateSpace;
#[cfg(feature = "img")] use mazelib::implm::export::img::BoxSpaceImageMazeExporter;
#[cfg(feature = "minecraft")] use mazelib::implm::export::minecraft::BoxSpaceSchematicMazeExporter;
use mazelib::implm::export::text::BoxSpaceTextMazeExporter;
use mazelib::interface::generate::DefaultMazeGenerator;
use mazelib::interface::export::DefaultMazeExporter;
use mazelib::util::apply_solid_border;

fn main() {
    const DIMENSION: usize = 2;

    type Space = BoxCoordinateSpace<DIMENSION>;
    type CellType = InlineCellValue<DIMENSION>;
    type BufferType = VecBuffer<CellType>;
    type Maze = BoxSpaceInlineCellMazeCoordinatorBuilder<BufferType, DIMENSION>;
    type Generator = HuntAndKillGenerator;
    type Exporter = BoxSpaceTextMazeExporter;

    let mut rng = thread_rng();

    let space = Space::new_checked([9, 9]);

    let mut maze = Maze::new(space).build();

    apply_solid_border(&mut maze);

    Generator::generate_with_rng(&mut maze, &mut rng);

    let output = std::io::stdout();

    Exporter::export(&maze, &mut BufWriter::new(output)).unwrap();
}