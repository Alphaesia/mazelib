use crate::interface::cell::{CellValue, CellManager, CellValueType};
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::interface::buffer::{MazeBuffer, BufferLocation};
use crate::internal::noise_util::pt;
use crate::interface::point::CoordinateSpace;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum InlineCellValue<const DIMENSION: usize> {
    UNVISITED,
    BOUNDARY,
    /// For every adjacent cell, true if there is a passage
    /// that leads there - false otherwise.
    /// The first element is the cell in the positive direction,
    /// the 2nd is the cell in the negative direction.
    PASSAGE([[bool; 2]; DIMENSION/*.pow(2)*/])
}

impl <const DIMENSION: usize> CellValue for InlineCellValue<DIMENSION> {
    fn get_type(&self) -> CellValueType {
        match self {
            Self::PASSAGE(_) => CellValueType::PASSAGE,
            Self::BOUNDARY => CellValueType::BOUNDARY,
            Self::UNVISITED => CellValueType::UNVISITED,
        }
    }
}

impl <const DIMENSION: usize> Default for InlineCellValue<DIMENSION> {
    fn default() -> Self { Self::UNVISITED }
}

#[derive(Debug)]
pub struct BoxSpaceInlineCellManager<Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> {
    space: BoxCoordinateSpace<DIMENSION>,
    buffer: Buffer,
}

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> BoxSpaceInlineCellManager<Buffer, DIMENSION> {
    fn pt_to_buffer_loc(&self, pt: pt!()) -> BufferLocation {
        let mut offset = pt[0];

        for i in 1..DIMENSION {
            offset += pt[i] * self.space[i - 1];
        }

        BufferLocation(offset)
    }

    fn set(&mut self, pt: pt!(), value: InlineCellValue<DIMENSION>) {
        self.buffer.set(self.pt_to_buffer_loc(pt), value)
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<DIMENSION>>, const DIMENSION: usize> CellManager for BoxSpaceInlineCellManager<Buffer, DIMENSION> {
    type CoordSpace = BoxCoordinateSpace<DIMENSION>;
    type CellVal = InlineCellValue<DIMENSION>;

    fn coord_space(&self) -> &Self::CoordSpace {
        &self.space
    }

    fn get(&self, pt: pt!()) -> Self::CellVal {
        self.buffer.get(self.pt_to_buffer_loc(pt))
    }

    fn make_passage(&mut self, pt: pt!()) {
        match self.get(pt) {
            InlineCellValue::PASSAGE(_) => {},
            _ => self.set(pt, InlineCellValue::PASSAGE([[true, true]; DIMENSION]))
        }
    }

    fn make_passage_between(&mut self, from: pt!(), to: pt!()) {
        let adj_dim = 0usize;

        let mut existing_walls_from = match self.get(from) {
            InlineCellValue::PASSAGE(walls) => walls,
            _ => [[false, false]; DIMENSION]
        };

        existing_walls_from[adj_dim][0] = true;

        self.make_passage(from);
        self.make_passage(to);
    }

    fn make_wall(&mut self, pt: pt!()) {
        self.set(pt, InlineCellValue::PASSAGE([[true, true]; DIMENSION]))
    }

    fn make_wall_between(&mut self, from: pt!(), to: pt!()) {
        self.make_wall(from);
        self.make_wall(to);
    }

    fn make_boundary(&mut self, pt: pt!()) {
        self.set(pt, InlineCellValue::BOUNDARY)
    }

    fn make_boundary_between(&mut self, from: pt!(), to: pt!()) {
        self.make_boundary(from);
        self.make_boundary(to);
    }
}