use crate::geometry::node::{Point, CoordinateTuplet};

pub trait CellSpace {
    type CellPtType: Point;

    fn size(&self) -> usize;
    fn ordinal_of(&self, cell: Self::CellPtType)
    fn neighbours_of(&self, cell: Self::CellPtType) -> Vec<Self::CellPtType>;
}

pub struct BoxyCellSpace<CellPtType: CoordinateTuplet<DIMENSION>, const DIMENSION: usize> {
    dimensions: [usize; DIMENSION]
}

impl <CellPtType: CoordinateTuplet<DIMENSION>, const DIMENSION: usize> CellSpace for BoxyCellSpace<CellPtType, DIMENSION> {
    type CellPtType = CellPtType;

    fn size(&self) -> usize {
        self.dimensions.iter().product()
    }

    fn ordinal_of(&self, cell: Self::CellPtType) {
        let mut total = 0usize;

        for i in DIMENSION..0
    }

    fn neighbours_of(&self, cell: CellPtType) -> Vec<CellPtType> {
        let mut neighbours = Vec::with_capacity(2 ** DIMENSION);

        for i in 0..DIMENSION {
            let cell_part = cell.into()[i];
            let max_value = self.dimensions[i];

            // Check the cell is in-bounds
            #[cfg(debug_assertions)]
            if cell_part >= max_value {
                panic!("Cell {:?} is out of bounds (cell space: {:?})", cell, self.dimensions)
            }

            if cell_part > 0 {
                neighbours.push(cell.offset(i, -1));
            }

            if cell_part < max_value {
                neighbours.push(cell.offset(i, 1));
            }
        }

        return neighbours
    }
}