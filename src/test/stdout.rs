// use std::{thread, io};
// use std::thread::JoinHandle;
// use std::sync::Weak;
// use mazelib::{maze, cell};
//
// pub(crate) fn start_progress_thread<Maze: maze::Maze>(maze: Weak<Maze>) -> io::Result<JoinHandle<!>> where
//         <Maze::CellSpace as cell::space::CellSpace<Maze>>::CoordSpace: Sync {
//     thread::Builder::new().name(String::from("maze-progress-monitor")).spawn(move || {
//         loop {
//             dbg!(&maze);
//         }
//     })
// }