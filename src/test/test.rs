#![allow(unused_imports, non_camel_case_types, unused_variables, dead_code, unused_must_use)]

use mazelib::geometry::space::TwoDimensionalSpace;
use mazelib::buffer::{VecBuffer, MazeBuffer, MazeCreationError};
use mazelib::generate::{HuntAndKillGenerator, MazeGenerator, BinaryTreeGenerator};
use mazelib::render::{TextRenderer, MazeRenderer, BitmapRenderer};
use mazelib::cell::space::{UnalignedBoxyCellSpace, AlignedBoxyCellSpace};
use mazelib::geometry::node::CoordinatePair;
use mazelib::template::{SolidBorderTemplate, Template};
use std::path::Path;
use nfd::Response::{Okay, OkayMultiple, Cancel};
use crate::stdout::start_progress_thread;
use std::sync::Arc;

pub fn test() {
    test_bmp()
}

pub fn test_space() {
    let space = TwoDimensionalSpace::new(9, 9);

    for pt in space {
        println!("{:?}", pt);
    }
}

pub fn test_maze() {
    let space = TwoDimensionalSpace::new(9, 9);

    let mut maze = match VecBuffer::<AlignedBoxyCellSpace<TwoDimensionalSpace, 2>>::new(space) {
        Ok(maze) => maze,
        Err(err) => match err {
            MazeCreationError::AllocationFailure => panic!("Allocation failure - maze is too big")
        }
    };

    SolidBorderTemplate::apply(&mut maze);

    //type generator = HuntAndKillGenerator<UnalignedBoxyCellSpace<TwoDimensionalSpace, 2>>;
    type generator = BinaryTreeGenerator;
    type renderer = TextRenderer;

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
    let space = TwoDimensionalSpace::new(999, 999);

    let mut maze = match VecBuffer::<AlignedBoxyCellSpace<TwoDimensionalSpace, 2>>::new(space) {
        Ok(maze) => maze,
        Err(err) => match err {
            MazeCreationError::AllocationFailure => panic!("Allocation failure - maze is too big")
        }
    };

    SolidBorderTemplate::apply(&mut maze);

    type generator = HuntAndKillGenerator<AlignedBoxyCellSpace<TwoDimensionalSpace, 2>>;
    // type generator = BinaryTreeGenerator;
    type renderer = BitmapRenderer;

    let mut maze = Arc::from(maze);
    let weak = Arc::downgrade(&maze);

    let monitor = start_progress_thread(weak);

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