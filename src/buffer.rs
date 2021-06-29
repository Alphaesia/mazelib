use std::fmt::Debug;

use sysinfo::{RefreshKind, SystemExt};

use crate::geometry::space::CoordinateSpace;
use crate::util::fsize;
use crate::{cell, maze};
use crate::maze::Maze;
use crate::buffer::MazeCreationError::AllocationFailure;

/// Note: MazeBuffers should never expose direct constructors. Rather, they should return
/// a buffer wrapped in a Maze. This ensures that the CoordinateSpace used for buffer
/// construction is the same one that is bundled in with the maze (and consequently, the
/// same one that will be received by generators, renderers, and others). This prevents
/// out-of-bounds-read panics.
pub trait MazeBuffer<Maze: maze::Maze<CellSpace>, CellSpace: cell::space::CellSpace<Maze>> : Debug + Send + Sync {
    /// This should NOT be called from generation code. This should only be called by the CellSpace.
    /// All other users should proxy through the cell space.
    fn get_pt(&self, pt: <<CellSpace as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType) -> CellSpace::CellType;
    fn set_pt(&mut self, pt: <<CellSpace as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType, cell_type: CellSpace::CellType);
}

pub enum MazeCreationError {
    AllocationFailure
}

#[derive(Debug)]
pub struct VecBuffer<Maze: maze::Maze<CellSpace>, CellSpace: cell::space::CellSpace<Maze>> {
    buf: Vec<CellSpace::CellType>,
    space: CellSpace::CoordSpace
}

impl <Maze: maze::Maze<CellSpace>, CellSpace: 'static + cell::space::CellSpace<Maze>> VecBuffer<Maze, CellSpace> {
    /// See [crate::buffer::MazeBuffer] for why this returns a Maze instead of a VecBuffer
    /// Returns [crate::buffer:MazeCreationError::AllocationFailure] if the predicted buffer size
    /// would be over 100% of reported available memory, or if actually allocating the buffer
    /// fails.
    pub fn new(space: CellSpace::CoordSpace) -> Result<Maze, MazeCreationError> {
        let cells_required = CellSpace::cells_required(&space);

        let likely_to_succeed = check_memory_usage(std::mem::size_of::<CellSpace::CellType>() * cells_required);

        if likely_to_succeed == false {
            return Err(AllocationFailure)
        }

        let mut buffer = Vec::with_capacity(0);

        match buffer.try_reserve_exact(cells_required) {
            Ok(_) => {}
            Err(_) => return Err(AllocationFailure)
        }

        buffer.fill(CellSpace::CellType::default());

        let buffer = Self { buf: vec![CellSpace::CellType::default(); cells_required], space };

        return Ok(Maze::new(space, Box::from(buffer)));
    }
}

impl <Maze: maze::Maze<CellSpace>, CellSpace: cell::space::CellSpace<Maze>> MazeBuffer<Maze, CellSpace> for VecBuffer<Maze, CellSpace> {
    fn get_pt(&self, pt: <<CellSpace as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType) -> CellSpace::CellType {
        self.buf[CellSpace::ordinal(self.space, pt)]
    }

    fn set_pt(&mut self, pt: <<CellSpace as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType, cell_type: CellSpace::CellType) {
        self.buf[CellSpace::ordinal(self.space, pt)] = cell_type
    }
}

/// predicted_usage is in BYTES
/// Returns whether allocation should go ahead
fn check_memory_usage(predicted_usage: usize) -> bool {
    // MEGABYTES
    let available_memory = (sysinfo::System::new_with_specifics(RefreshKind::new().with_memory()).get_available_memory() / 1024) as usize;
    let predicted_memory_mb = predicted_usage / 1024 / 1024;
    let memory_usage = predicted_memory_mb as fsize / available_memory as fsize;

    // Let's prevent people from rendering their computer unusable
    if predicted_memory_mb >= available_memory {
        return false;
    }

    // Warn for dangerously high memory consumption
    if predicted_memory_mb as f64 / available_memory as f64 > 0.7 {
        eprintln!("Allocating a maze buffer of {} MB (predicted), which is {:0>4.1}% of the {} MB of reported available memory", predicted_memory_mb, memory_usage * 100.0, available_memory)
    }

    return true;
}