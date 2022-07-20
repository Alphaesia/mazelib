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
use mazelib::implm::maze::block::{BoxSpaceBlockCellMaze, BoxSpaceBlockCellMazeBuilder};
use mazelib::implm::point::boxy::BoxCoordinateSpace;
#[cfg(feature = "img")] use mazelib::implm::render::img::BoxSpaceImageMazeRenderer;
#[cfg(feature = "minecraft")] use mazelib::implm::render::minecraft::BoxSpaceSchematicMazeRenderer;
use mazelib::implm::render::text::BoxSpaceTextMazeRenderer;
use mazelib::implm::solve::RandomMouse;
use mazelib::interface::buffer::MazeBuffer;
use mazelib::interface::cell::CellID;
use mazelib::interface::generate::DefaultMazeGenerator;
use mazelib::interface::render::MazeRendererNonSeeking;
use mazelib::interface::solve::MazeSolver;
use mazelib::PointPath;
use mazelib::util::solid_border;

fn main() {
    const DIMENSION: usize = 2;

    type CoordSpace = BoxCoordinateSpace<DIMENSION>;
    type CellType = BlockCellValue;
    type BufferType = VecBuffer<CellType>;
    type Maze = BoxSpaceBlockCellMazeBuilder<BufferType, DIMENSION>;
    type Generator = HuntAndKillGenerator;
    type Renderer = BoxSpaceTextMazeRenderer<BoxSpaceBlockCellMaze<BufferType, DIMENSION>>;
    let output = std::io::stdout();

    let mut rng = thread_rng();

    let [width, height] = [11, 11];

    let space = CoordSpace::new([width, height]);

    let mut maze = Maze::new(space).build();

    solid_border(&mut maze);

    Generator::generate_with_rng(&mut maze, &mut rng);

    Renderer::new().render(&maze, &mut BufWriter::new(std::io::stdout())).unwrap();

    let mut solution = RandomMouse::new().pathfind(&mut maze, [0, 0].into(), [width - 1, height - 1].into()).unwrap();

    dbg!(&solution, solution.len());

    println!("Was Simple: {}", if solution.is_simple() { "Yes" } else { "NO" });

    solution.make_simple();

    Renderer::with_path(solution).render(&maze, &mut BufWriter::new(output)).unwrap();
}