use std::fmt::Debug;

use sysinfo::{RefreshKind, SystemExt};

use crate::geometry::space::CoordinateSpace;
use crate::util::fsize;
use crate::buffer::MazeCreationError::AllocationFailure;
use crate::cell::data::CellData;

// #[derive(Debug)]
// pub struct VecBufferMaze<CellSpace: cell::space::CellSpace<Self>> {
//     space: CellSpace::CoordSpace,
//     buffer: VecBuffer<Self>,
//     _cell_space: PhantomData<CellSpace>
// }
//
// impl <CellSpace: cell::space::CellSpace<Self>> VecBufferMaze<CellSpace> {
//     /// Fails if creating the [crate::buffer::VecBuffer] fails.
//     pub fn new(space: CellSpace::CoordSpace) -> Result<Self, MazeCreationError> {
//         let buffer = VecBuffer::new(space)?;
//
//         Ok(Self { space, buffer, _cell_space: PhantomData })
//     }
// }
//
// impl <CellSpace: cell::space::CellSpace<Self>> Maze for VecBufferMaze<CellSpace> {
//     type CellSpace = CellSpace;
//     type Buffer = VecBuffer<Self>;
//
//     fn space(&self) -> CellSpace::CoordSpace {
//         self.space
//     }
//
//     fn buffer(&self) -> &Self::Buffer {
//         &self.buffer
//     }
//
//     fn mut_buffer(&mut self) -> &mut Self::Buffer {
//         &mut self.buffer
//     }
// }

/// Note: MazeBuffers should never expose direct constructors. Rather, they should return
/// a buffer wrapped in a Maze. This ensures that the CoordinateSpace used for buffer
/// construction is the same one that is bundled in with the maze (and consequently, the
/// same one that will be received by generators, renderers, and others). This prevents
/// out-of-bounds-read panics.
pub trait MazeBuffer<CoordSpace: CoordinateSpace, CellType: CellData> : Debug + Send + Sync {
    /// This should NOT be called from generation code. This should only be called by the CellSpace.
    /// All other users should proxy through the cell space.
    fn get_cell(&self, ordinal: usize) -> CellType;
    fn set_cell(&mut self, ordinal: usize, cell_type: CellType);

    fn space(&self) -> &CoordSpace;
}

pub enum MazeCreationError {
    AllocationFailure
}

#[derive(Debug)]
pub struct VecBuffer<CoordSpace: CoordinateSpace, CellType: CellData> {
    buf: Vec<CellType>,
    space: CoordSpace,
}

impl <CoordSpace: CoordinateSpace, CellType: CellData> VecBuffer<CoordSpace, CellType> {
    // TODO get rid of cells_required param
    /// Returns [crate::buffer:MazeCreationError::AllocationFailure] if the predicted buffer size
    /// would be over 100% of reported available memory, or if actually allocating the buffer
    /// fails.
    pub fn new(space: CoordSpace, cells_required: usize) -> Result<Self, MazeCreationError> {
        //let cells_required = CellSpace::cells_required(&space);

        let likely_to_succeed = check_memory_usage(std::mem::size_of::<CellType>() * cells_required);

        if likely_to_succeed == false {
            return Err(AllocationFailure)
        }

        let mut buffer = Vec::with_capacity(0);

        match buffer.try_reserve_exact(cells_required) {
            Ok(_) => {}
            Err(_) => return Err(AllocationFailure)
        }

        buffer.fill(CellType::default());

        let buffer = Self { buf: vec![CellType::default(); cells_required], space };

        return Ok(buffer);
    }
}

impl <CoordSpace: CoordinateSpace, CellType: CellData> MazeBuffer<CoordSpace, CellType> for VecBuffer<CoordSpace, CellType> {
    /// Intended for access from CellSpaces only. End users should use CellSpace::get_pt().
    fn get_cell(&self, ordinal: usize) -> CellType {
        self.buf[ordinal]
    }

    /// Intended for access from CellSpaces only. End users should use CellSpace::set_pt().
    fn set_cell(&mut self, ordinal: usize, cell_type: CellType) {
        self.buf[ordinal] = cell_type
    }

    fn space(&self) -> &CoordSpace {
        &self.space
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