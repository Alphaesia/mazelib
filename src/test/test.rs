#![allow(unused_imports, non_camel_case_types, unused_variables, dead_code, unused_must_use)]

use mazelib::geometry::space::TwoDimensionalSpace;
use mazelib::buffer::{VecBuffer, MazeBuffer, MazeCreationError};
use mazelib::generate::{HuntAndKillGenerator, MazeGenerator, BinaryTreeGenerator};
use mazelib::render::{TextRenderer, MazeRenderer, BitmapRenderer};
use mazelib::cell::manager::{UnalignedBoxyCellSpace, AlignedBoxyCellSpace, CellManager};
use mazelib::geometry::node::CoordinatePair;
use mazelib::template::{SolidBorderTemplate, Template};
use std::path::Path;
use nfd::Response::{Okay, OkayMultiple, Cancel};
use std::sync::Arc;
use mazelib::cell::data::Basic;

pub fn test() {
    test_maze()
}

pub fn test_space() {
    let space = TwoDimensionalSpace::new(9, 9);

    for pt in space {
        println!("{:?}", pt);
    }
}

pub fn test_maze() {
    type cell_space = UnalignedBoxyCellSpace<VecBuffer<TwoDimensionalSpace, Basic>, TwoDimensionalSpace, 2>;

    let space = TwoDimensionalSpace::new(9, 9);

    let buffer = match VecBuffer::new(space, cell_space::cells_required(&space)) {
        Ok(maze) => maze,
        Err(err) => match err {
            MazeCreationError::AllocationFailure => panic!("Allocation failure - maze is too big")
        }
    };

    let mut maze = cell_space::new(buffer);

    SolidBorderTemplate::apply(&mut maze);

    //type generator = HuntAndKillGenerator<cell_space>;
    type generator = BinaryTreeGenerator;
    type renderer = TextRenderer<cell_space>;

    let gen_start = std::time::Instant::now();

    generator::generate_s(&mut maze, &mut rand::thread_rng());

    let gen_end = std::time::Instant::now();

    println!("Generated maze in {}s", (gen_end - gen_start).as_secs_f32());

    let render_start = std::time::Instant::now();

    let result: Vec<String> = renderer::render(&mut maze);

    let render_end = std::time::Instant::now();

    println!("Rendered maze in {}s", (render_end - render_start).as_secs_f32());

    for line in result { println!("{}", line); }
}

pub fn test_bmp() {
    type cell_space = AlignedBoxyCellSpace<VecBuffer<TwoDimensionalSpace, Basic>, TwoDimensionalSpace, 2>;

    let space = TwoDimensionalSpace::new(9, 9);

    let buffer = match VecBuffer::new(space, cell_space::cells_required(&space)) {
        Ok(maze) => maze,
        Err(err) => match err {
            MazeCreationError::AllocationFailure => panic!("Allocation failure - maze is too big")
        }
    };

    let mut maze = cell_space::new(buffer);

    SolidBorderTemplate::apply(&mut maze);

    //type generator = HuntAndKillGenerator;
    type generator = BinaryTreeGenerator;
    type renderer = BitmapRenderer<cell_space>;

    //let mut maze = Arc::from(maze);
    //let weak = Arc::downgrade(&maze);

    //let monitor = start_progress_thread(weak);

    let gen_start = std::time::Instant::now();

    generator::generate_s(&mut maze, &mut rand::thread_rng());

    let gen_end = std::time::Instant::now();

    println!("Generated maze in {}s", (gen_end - gen_start).as_secs_f32());

    // Get where to save the file (file save dialog)

    let response = match nfd::open_save_dialog(Some("bmp;"), None) {
        Ok(response) => response,
        Err(err) => return
    };

    let path = match response {
        Okay(path) => path,
        OkayMultiple(_) => panic!("Impossible to select multiple files in a save dialogue"),
        Cancel => return
    };

    let render_start = std::time::Instant::now();

    renderer::render_and_save_aligned(&mut maze, Path::new(&path));

    let render_end = std::time::Instant::now();

    println!("Rendered maze in {}s", (render_end - render_start).as_secs_f32());
}