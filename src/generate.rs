use rand::{Rng, RngCore};
use rand::seq::SliceRandom;

use crate::{cell, util};
use crate::geometry::space::{CoordinateSpace, BoxCoordinateSpace};
use crate::geometry::node::CoordinateTuplet;
use crate::util::debug_maze;

pub trait MazeGenerator<CellSpace: cell::manager::CellManager> {
    fn generate_s(maze: &mut CellSpace, rng: &mut dyn RngCore);
    fn generate(&mut self, maze: &mut CellSpace, rng: &mut dyn RngCore);
}

//region Hunt and Kill

pub struct HuntAndKillGenerator<CellSpace: cell::manager::CellManager> {
    start: <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType,
    last_hunt_pos: <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType
}

impl <CellSpace: cell::manager::CellManager> HuntAndKillGenerator<CellSpace> {
    pub fn new() -> Self {
        Self { start: <CellSpace as cell::manager::CellManager>::CoordSpace::origin(), last_hunt_pos: <CellSpace as cell::manager::CellManager>::CoordSpace::origin() }
    }

    pub fn starting_at(start: <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType) -> Self {
        Self { start, last_hunt_pos: <CellSpace as cell::manager::CellManager>::CoordSpace::origin() }
    }
}

impl <CellSpace: cell::manager::CellManager> MazeGenerator<CellSpace> for HuntAndKillGenerator<CellSpace> {
    fn generate_s(maze: &mut CellSpace, rng: &mut dyn RngCore) {
        Self::new().generate(maze, rng)
    }

    fn generate(&mut self, maze: &mut CellSpace, rng: &mut dyn RngCore) {
        let space = *maze.space();
        
        let mut pos = self.start;

        // TODO calling stuff marked to not be called from generation code
        'driver: loop {
            // Hunt phase
            'hunt: loop {
                for candidate in space.iter_from(self.last_hunt_pos) {
                    if maze.is_unvisited(candidate) {
                        // Cache hunt pos
                        self.last_hunt_pos = candidate;

                        // Pick a random adjacent wall to become the new passage...

                        let adj_walls = util::get_neighbouring_walls(maze, candidate);

                        if adj_walls.len() > 0 {
                            let selection = rng.gen_range(0..adj_walls.len());
                            let selected_pt = adj_walls[selection];

                            maze.make_passage_between(candidate, selected_pt);
                        }

                        break 'hunt
                    }
                }

                break 'driver
            }

            // Kill phase
            'kill: loop {
                let candidate_neighbours = util::get_neighbouring_unvisiteds(maze, pos);

                // End of kill phase - if we're trapped with nowhere to go
                if candidate_neighbours.is_empty() {
                    pos = self.last_hunt_pos;  // We start hunting from the last hunt pos
                    break 'kill
                }

                let selection = rng.gen_range(0..candidate_neighbours.len());
                let selected_pt = candidate_neighbours[selection];

                // Make the selected neighbour a passage and move into it

                CellSpace::make_passage_between(maze, pos, selected_pt);
                pos = selected_pt;
            }
        }
    }
}

//endregion

//region Binary Tree

pub struct BinaryTreeGenerator {}

impl BinaryTreeGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl <CellSpace: cell::manager::CellManager> MazeGenerator<CellSpace> for BinaryTreeGenerator where <CellSpace as cell::manager::CellManager>::CoordSpace: BoxCoordinateSpace<2>, <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType: CoordinateTuplet<2> {
    fn generate_s(maze: &mut CellSpace, rng: &mut dyn RngCore) {
        Self::new().generate(maze, rng)
    }

    fn generate(&mut self, maze: &mut CellSpace, rng: &mut dyn RngCore) {
        let space = maze.space();

        for pt in *space {
            debug_maze(maze, pt);

            if CellSpace::is_wall(maze, pt) || CellSpace::is_boundary(maze, pt) { continue }

            let [x, y]: [usize; 2] = pt.into();

            let above = if y > 0 { Some([x, y - 1]) } else { None };
            let left = if x > 0 { Some([x - 1, y]) } else { None };

            if above == None && left == None {
                CellSpace::make_passage(maze, pt);
            } else if above == None {
                CellSpace::make_passage_between(maze, left.unwrap().into(), pt);
            } else if left == None {
                CellSpace::make_passage_between(maze, above.unwrap().into(), pt);
            } else {
                // Nested due to life time issues
                CellSpace::make_passage_between(maze, [left, above].choose(rng).unwrap().unwrap().into(), pt);
            }
        }
    }
}

//endregion