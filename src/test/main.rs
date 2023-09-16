#![allow(dead_code)]

use std::fs::File;
use std::io::{BufWriter, Result, Write};

use image::ImageFormat;

use mazelib::implm::buffer::VecBuffer;
use mazelib::implm::cell::block::BlockCellValue;
use mazelib::implm::cell::inline::InlineCellValue;
use mazelib::implm::coordinate::block::BoxSpaceBlockCellMazeCoordinator;
use mazelib::implm::coordinate::inline::BoxSpaceInlineCellMazeCoordinatorBuilder;
use mazelib::implm::export::img::BoxSpaceImageMazeExporter;
use mazelib::implm::export::text::{BoxSpaceBlockCellTextMazeExporter, BoxSpaceInlineCellTextMazeExporter};
use mazelib::implm::generate::HuntAndKillGenerator;
use mazelib::implm::point::boxy::BoxCoordinateSpace;
use mazelib::interface::export::MazeExporter;
use mazelib::interface::generate::DefaultMazeGenerator;
use mazelib::util::apply_solid_border;

fn main() -> Result<()> {
    generate_maze_box_img()
}

fn generate_maze_box_text() -> Result<()> {
    let space = BoxCoordinateSpace::<2>::new_checked([9, 9]);

    let mut maze = BoxSpaceBlockCellMazeCoordinator::<VecBuffer<BlockCellValue>, 2>::builder(space).build();

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

fn generate_maze_box_img() -> Result<()> {
    let space = BoxCoordinateSpace::<2>::new_checked([9, 9]);

    let mut maze = BoxSpaceBlockCellMazeCoordinator::<VecBuffer<BlockCellValue>, 2>::builder(space).scale_factors_checked([2, 3]).padding([[3, 2], [1, 0]]).build();

    apply_solid_border(&mut maze);

    HuntAndKillGenerator::generate(&mut maze);

    let mut output = BufWriter::new(File::create("test.bmp")?);

    BoxSpaceImageMazeExporter::new(ImageFormat::Bmp).export(&maze, &mut output)?;

    output.flush()
}