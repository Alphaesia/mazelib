use fluent_asserter::prelude::*;
use crate::implm::buffer::VecBuffer;
use crate::implm::cell::block::{BlockCellValue, BlockCellValueType};
use crate::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinatorBuilder as MazeBuilder;
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::CellID;
use crate::interface::coordinator::MazeCoordinator;

// We test both at a coordinator level and a buffer level
// (i.e. ignoring and taking into account the resolution)

// TODO add tests for mazes with padding

#[test]
#[allow(unused_variables)]
fn test_construction() {
    // Everything is scoped so we can release memory sooner

    {
        let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([9, 9]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([1, 1]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([7, 2]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([62, 18]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([999, 999]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([4, 4]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([2, 2]).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([3, 3]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([2, 2]).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([2, 2]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([2, 2]).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([1, 1]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([2, 2]).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([4, 4]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([7, 9]).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([17, 61]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([12, 83]).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([99, 99]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([11, 11]).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([7, 7, 7]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 3>::new(coord_space).scale_factor_checked([1, 1, 1]).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([4, 8, 15, 16, 23]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 5>::new(coord_space).scale_factor_checked([2, 2, 2, 2, 2]).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([2, 2, 2, 2, 2, 2]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 6>::new(coord_space).scale_factor_checked([3, 1, 4, 1, 5, 9]).build();
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
        assert_that_code!(|| MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([0, 0]).build()).panics().with_having_message("All scalars must be non-zero")
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
        assert_that_code!(|| MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([1, 0]).build()).panics().with_having_message("All scalars must be non-zero")
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
        assert_that_code!(|| MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([0, 1]).build()).panics().with_having_message("All scalars must be non-zero")
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
        assert_that_code!(|| MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([1, usize::MAX]).build()).panics().with_having_message("The scaled dimensions do not all fit within a usize")
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
        assert_that_code!(|| MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([usize::MAX, 1]).build()).panics().with_having_message("The scaled dimensions do not all fit within a usize")
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
        assert_that_code!(|| MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([usize::MAX, usize::MAX]).build()).panics().with_having_message("The scaled dimensions do not all fit within a usize")
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
        assert_that_code!(|| MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([1, 1]).padding([[usize::MAX, usize::MAX], [usize::MAX, usize::MAX]]).build()).panics().with_having_message("The full dimensions do not all fit within a usize")
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
        assert_that_code!(|| MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([usize::MAX / 16, usize::MAX / 16]).build()).panics().with_having_message("The full dimensions specified are too large. The number of cells in the maze does not fit within a usize.")
    }
}

#[test]
fn test_initialisation() {
    // Everything is scoped so we can release memory sooner

    {
        let coord_space = BoxCoordinateSpace::new_checked([3, 3]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).build();

        for y in 0usize..3 {
            for x in 0usize..3 {
                assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
                assert_eq!(maze.buffer().get(CellID(x + y * 3)).cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).build();

        for y in 0usize..5 {
            for x in 0usize..5 {
                assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
                assert_eq!(maze.buffer().get(CellID(x + y * 5)).cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    {
        let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
        let maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([2, 2]).build();

        for y in 0usize..5 {
            for x in 0usize..5 {
                assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
            }
        }

        for y in 0usize..9 {
            for x in 0usize..9 {
                assert_eq!(maze.buffer().get(CellID(x + y * 9)).cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_boundary_single_without_scaling_or_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([1, 1]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
            assert_eq!(maze.buffer().get(CellID(x + y * 5)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First boundary

    maze.make_boundary([1, 2].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second boundary

    maze.make_boundary([4, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third boundary

    maze.make_boundary([1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_boundary_single_with_scaling_without_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([2, 2]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            assert_eq!(maze.buffer().get(CellID(x + y * 9)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First boundary

    maze.make_boundary([1, 2].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second boundary

    maze.make_boundary([4, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 8 && y == 8 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third boundary

    maze.make_boundary([1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 8 && y == 8 || x == 2 && y == 6 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_boundary_between_without_scaling_or_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([1, 1]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
            assert_eq!(maze.buffer().get(CellID(x + y * 5)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First boundary

    maze.make_boundary_between([1, 2].into(), [1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second boundary

    maze.make_boundary_between([4, 4].into(), [3, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third boundary

    maze.make_boundary_between([1, 3].into(), [2, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4
                    || x == 2 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_boundary_between_with_scaling_without_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([2, 2]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            assert_eq!(maze.buffer().get(CellID(x + y * 9)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First boundary

    maze.make_boundary_between([1, 2].into(), [1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 2 && y == 5 || x == 2 && y == 6 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second boundary

    maze.make_boundary_between([4, 4].into(), [3, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 2 && y == 5 || x == 2 && y == 6
                    || x == 8 && y == 8 || x == 7 && y == 8 || x == 6 && y == 8 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third boundary

    maze.make_boundary_between([1, 3].into(), [2, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4
                    || x == 2 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 2 && y == 5 || x == 2 && y == 6
                    || x == 8 && y == 8 || x == 7 && y == 8 || x == 6 && y == 8
                    || x == 3 && y == 6 || x == 4 && y == 6 {
                assert_eq!(cell.cell_type, BlockCellValueType::BOUNDARY);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_wall_single_without_scaling_or_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([1, 1]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
            assert_eq!(maze.buffer().get(CellID(x + y * 5)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First wall

    maze.make_wall([1, 2].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second wall

    maze.make_wall([4, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third wall

    maze.make_wall([1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_wall_single_with_scaling_without_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([2, 2]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            assert_eq!(maze.buffer().get(CellID(x + y * 9)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First wall

    maze.make_wall([1, 2].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second wall

    maze.make_wall([4, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 8 && y == 8 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third wall

    maze.make_wall([1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 8 && y == 8 || x == 2 && y == 6 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_wall_between_without_scaling_or_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([1, 1]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
            assert_eq!(maze.buffer().get(CellID(x + y * 5)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First wall

    maze.make_wall_between([1, 2].into(), [1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second wall

    maze.make_wall_between([4, 4].into(), [3, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third wall

    maze.make_wall_between([1, 3].into(), [2, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4
                    || x == 2 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_wall_between_with_scaling_without_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([2, 2]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            assert_eq!(maze.buffer().get(CellID(x + y * 9)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First wall

    maze.make_wall_between([1, 2].into(), [1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 2 && y == 5 || x == 2 && y == 6 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second wall

    maze.make_wall_between([4, 4].into(), [3, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 2 && y == 5 || x == 2 && y == 6
                    || x == 8 && y == 8 || x == 7 && y == 8 || x == 6 && y == 8 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third wall

    maze.make_wall_between([1, 3].into(), [2, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4
                    || x == 2 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 2 && y == 5 || x == 2 && y == 6
                    || x == 8 && y == 8 || x == 7 && y == 8 || x == 6 && y == 8
                    || x == 3 && y == 6 || x == 4 && y == 6 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_passage_single_without_scaling_or_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([1, 1]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
            assert_eq!(maze.buffer().get(CellID(x + y * 5)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First passage

    maze.make_passage([1, 2].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 0 || x == 2) && y == 2 || x == 1 && (y == 1 || y == 3) {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second passage

    maze.make_passage([4, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 0 || x == 2) && y == 2 || x == 1 && (y == 1 || y == 3)
                    || x == 3 && y == 4 || x == 4 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third passage

    maze.make_passage([1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 0 || x == 2) && y == 2 || x == 1 && y == 1
                    || x == 3 && y == 4 || x == 4 && y == 3
                    || (x == 0 || x == 2) && y == 3 || x == 1 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_passage_single_with_scaling_without_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([2, 2]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            assert_eq!(maze.buffer().get(CellID(x + y * 9)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First passage

    maze.make_passage([1, 2].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 1 || x == 3) && y == 4 || x == 2 && (y == 3 || y == 5) {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second passage

    maze.make_passage([4, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 8 && y == 8 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 1 || x == 3) && y == 4 || x == 2 && (y == 3 || y == 5)
                    || x == 7 && y == 8 || x == 8 && y == 7 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third passage

    maze.make_passage([1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 4 && y == 4 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 8 && y == 8 || x == 2 && y == 6 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 1 || x == 3) && y == 4 || x == 2 && (y == 3 || y == 5)
                    || x == 7 && y == 8 || x == 8 && y == 7
                    || (x == 1 || x == 3) && y == 6 || x == 2 && (y == 5 || y == 7) {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_passage_between_without_scaling_or_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([1, 1]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
            assert_eq!(maze.buffer().get(CellID(x + y * 5)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First wall

    maze.make_passage_between([1, 2].into(), [1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 0 || x == 2) && y == 2 || x == 1 && y == 1 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second wall

    maze.make_passage_between([4, 4].into(), [3, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 0 || x == 2) && y == 2 || x == 1 && y == 1
                    || x == 4 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third wall

    maze.make_passage_between([1, 3].into(), [2, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4
                    || x == 2 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 0 || x == 2) && y == 2 || x == 1 && y == 1
                    || x == 4 && y == 3
                    || x == 0 && y == 3 || x == 1 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}

#[test]
fn test_passage_between_with_scaling_without_padding() {
    let coord_space = BoxCoordinateSpace::new_checked([5, 5]);
    let mut maze = MazeBuilder::<VecBuffer<BlockCellValue>, 2>::new(coord_space).scale_factor_checked([2, 2]).padding([[0, 0], [0, 0]]).build();

    // Empty

    for y in 0usize..5 {
        for x in 0usize..5 {
            assert_eq!(maze.get([x, y].into()).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            assert_eq!(maze.buffer().get(CellID(x + y * 9)).cell_type, BlockCellValueType::UNVISITED);
        }
    }

    // First wall

    maze.make_passage_between([1, 2].into(), [1, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 2 && y == 5 || x == 2 && y == 6 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 1 || x == 3) && (y == 4 || y == 5) || x == 2 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Second wall

    maze.make_passage_between([4, 4].into(), [3, 4].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 2 && y == 5 || x == 2 && y == 6
                    || x == 8 && y == 8 || x == 7 && y == 8 || x == 6 && y == 8 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 1 || x == 3) && (y == 4 || y == 5) || x == 2 && y == 3
                    || (x == 7 || x == 8) && y == 7 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    // Third wall

    maze.make_passage_between([1, 3].into(), [2, 3].into());

    for y in 0usize..5 {
        for x in 0usize..5 {
            let cell = maze.get([x, y].into());

            if x == 1 && y == 2 || x == 1 && y == 3
                    || x == 4 && y == 4 || x == 3 && y == 4
                    || x == 2 && y == 3 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }

    for y in 0usize..9 {
        for x in 0usize..9 {
            let cell = maze.buffer().get(CellID(x + y * 9));

            if x == 2 && y == 4 || x == 2 && y == 5 || x == 2 && y == 6
                    || x == 8 && y == 8 || x == 7 && y == 8 || x == 6 && y == 8
                    || x == 3 && y == 6 || x == 4 && y == 6 {
                assert_eq!(cell.cell_type, BlockCellValueType::PASSAGE);
            } else if (x == 1 || x == 3) && (y == 4 || y == 5) || x == 2 && y == 3
                    || (x == 7 || x == 8) && y == 7
                    || x == 1 && y == 6 || x == 2 && y == 7 || x == 3 && y == 7 {
                assert_eq!(cell.cell_type, BlockCellValueType::WALL);
            } else {
                assert_eq!(cell.cell_type, BlockCellValueType::UNVISITED);
            }
        }
    }
}