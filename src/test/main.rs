#![allow(unused_imports, dead_code)]

use std::io::{BufWriter, Result, Write};

#[cfg(feature = "img")] use image::ImageFormat;
use rand::thread_rng;

use mazelib::implm::buffer::VecBuffer;
use mazelib::implm::cell::block::BlockCellValue;
use mazelib::implm::cell::inline::InlineCellValue;
use mazelib::implm::coordinator::block::{BoxSpaceBlockCellMazeCoordinator, BoxSpaceBlockCellMazeCoordinatorBuilder};
use mazelib::implm::coordinator::inline::BoxSpaceInlineCellMazeCoordinatorBuilder;
#[cfg(feature = "img")] use mazelib::implm::export::img::BoxSpaceImageMazeExporter;
#[cfg(feature = "minecraft")] use mazelib::implm::export::minecraft::BoxSpaceSchematicMazeExporter;
use mazelib::implm::export::text::{BoxSpaceBlockCellTextMazeExporter, BoxSpaceBlockCellTextMazeExporterBuilder, BoxSpaceInlineCellTextMazeExporter, BoxSpaceTextMazeExporter};
use mazelib::implm::generate::{HuntAndKillGenerator, NAryTreeGenerator, RecursiveBacktrackerGenerator};
use mazelib::implm::point::boxy::BoxCoordinateSpace;
use mazelib::interface::export::{DefaultMazeExporter, MazeExporter};
use mazelib::interface::generate::DefaultMazeGenerator;
use mazelib::util::apply_solid_border;

fn main() -> Result<()> {
    generate_maze_box_text()
}

fn generate_maze_box_text() -> Result<()> {
    let space = BoxCoordinateSpace::<2>::new_checked([9, 9]);

    let mut maze = BoxSpaceBlockCellMazeCoordinatorBuilder::<VecBuffer<BlockCellValue>, 2>::new(space).build();

    apply_solid_border(&mut maze);

    HuntAndKillGenerator::generate(&mut maze);

    let mut output = BufWriter::new(std::io::stdout());

    BoxSpaceBlockCellTextMazeExporter::builder().build().export(&maze, &mut output)?;
    
    output.flush()
}

fn generate_maze_inline_text() -> Result<()> {
    let space = BoxCoordinateSpace::<2>::new_checked([9, 9]);

    let mut maze = BoxSpaceInlineCellMazeCoordinatorBuilder::<VecBuffer<InlineCellValue<2>>, 2>::new(space).build();

    apply_solid_border(&mut maze);

    HuntAndKillGenerator::generate(&mut maze);

    let mut output = BufWriter::new(std::io::stdout());

    BoxSpaceInlineCellTextMazeExporter::builder().build().export(&maze, &mut output)?;

    output.flush()
}