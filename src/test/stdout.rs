use mazelib::maze::Maze;
use std::{thread, io};
use std::thread::JoinHandle;
use std::sync::Weak;

pub(crate) fn start_progress_thread<CellSpace: 'static>(maze: Weak<Maze<CellSpace>>) -> io::Result<JoinHandle<!>> where
        CellSpace: mazelib::cell::space::CellSpace,
        CellSpace::CoordSpace: Sync {
    thread::Builder::new().name(String::from("maze-progress-monitor")).spawn(move || {
        loop {
            dbg!(&maze);
        }
    })
}