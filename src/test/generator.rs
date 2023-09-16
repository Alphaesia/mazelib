use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::implm::buffer::VecBuffer;
use crate::implm::cell::block::BlockCellValue;
use crate::implm::cell::inline::InlineCellValue;
use crate::implm::coordinate::block::BoxSpaceBlockCellMazeCoordinator;
use crate::implm::coordinate::inline::{BoxSpaceInlineCellMazeCoordinator, BoxSpaceInlineCellMazeCoordinatorBuilder};
use crate::implm::export::text::{BoxSpaceBlockCellTextMazeExporter, BoxSpaceInlineCellTextMazeExporter};
use crate::implm::generate::{HuntAndKillGenerator, NAryTreeGenerator, RecursiveBacktrackerGenerator};
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::interface::export::MazeExporter;
use crate::interface::generate::MazeGenerator;
use crate::internal::util::get_line_sep;

#[test]
fn test_hunt_and_kill_block_cell_generation() {
    let expected = "\
.███.███.███..████.
█   █   █   █     █
█ █ █ █ █ █ ███ █ █
█ █   █   █     █ █
█ ███ ███████████ █
█ █ █   █       █ .
█ █ ███ ███ ███ ██.
█ █   █ █   █ █   █
█ ███ █ █ █ █ ███ █
█ █   █ █ █ █     █
█ █ ███ █ █ ██████.
█ █     █ █       █
█ ███████ █ █████ █
█     █   █     █ █
.████ █ ███ █████ █
█   █   █   █   █ █
█ █████████ █ █ █ █
█           █ █   █
.███████████...███.
".replace('\n', get_line_sep());

    type Generator = HuntAndKillGenerator;

    let mut rng = get_test_rng();
    let mut maze = get_new_block_cell_maze();

    Generator::new().generate_with_rng(&mut maze, &mut rng);

    let actual = export_block_cell_maze(&maze);

    assert_eq!(expected, actual)
}

#[test]
fn test_hunt_and_kill_inline_cell_generation() {
    let expected = "\
┌─────┬─────┬─────┬────────┐
│     │     │     │        │
│  ╷  ╵  ╷  ╵  ╷  └──╴  ╷  │
│  │     │     │        │  │
│  ├──┐  └──┬──┴────────┤  │
│  │  │     │           │  │
│  │  └──┐  ├──╴  ┌──┐  └──┤
│  │     │  │     │  │     │
│  ├──╴  │  │  ╷  │  └──╴  │
│  │     │  │  │  │        │
│  │  ╶──┘  │  │  └────────┤
│  │        │  │           │
│  └─────┬──┘  │  ╶─────┐  │
│        │     │        │  │
├─────┐  ╵  ┌──┘  ┌─────┤  │
│     │     │     │     │  │
│  ╶──┴─────┴──╴  │  ╷  ╵  │
│                 │  │     │
└─────────────────┴──┴─────┘
".replace('\n', get_line_sep());

    type Generator = HuntAndKillGenerator;

    let mut rng = get_test_rng();
    let mut maze = get_new_inline_cell_maze();

    Generator::new().generate_with_rng(&mut maze, &mut rng);

    let actual = export_inline_cell_maze(&maze);

    assert_eq!(expected, actual)
}

#[test]
fn test_nary_tree_block_cell_generation() {
    let expected = "\
.███.█.███.█.█████.
█   █ █   █ █     █
.██ █ ███ █ █████ █
█ █ █           █ █
█ █ ███████████ █ █
█ █     █   █   █ █
█ █████ ███ ███ █ █
█ █           █ █ █
█ ███████████ █ █ █
█         █ █   █ █
.████████ █ ███ █ █
█   █ █   █   █   █
.██ █ ███ ███ ███ █
█       █ █     █ █
.██████ █ █████ █ █
█     █     █   █ █
.████ █████ ███ █ █
█                 █
.█████████████████.
".replace('\n', get_line_sep());

    type Generator = NAryTreeGenerator;

    let mut rng = get_test_rng();
    let mut maze = get_new_block_cell_maze();

    Generator::new().generate_with_rng(&mut maze, &mut rng);

    let actual = export_block_cell_maze(&maze);

    assert_eq!(expected, actual)
}

#[test]
fn test_nary_tree_inline_cell_generation() {
    let expected = "\
┌─────┬──┬─────┬──┬────────┐
│     │  │     │  │        │
├──┐  │  └──╴  ╵  └─────┐  │
│  │  │                 │  │
│  │  └─────┬─────┬──╴  │  │
│  │        │     │     │  │
│  ├─────╴  └──╴  └──┐  │  │
│  │                 │  │  │
│  └───────────┬──┐  ╵  │  │
│              │  │     │  │
├─────┬──┬──╴  │  └──┐  ╵  │
│     │  │     │     │     │
├──╴  ╵  └──┐  ├──╴  └──┐  │
│           │  │        │  │
├────────┐  ╵  └──┬──╴  │  │
│        │        │     │  │
├─────╴  └─────╴  └──╴  ╵  │
│                          │
└──────────────────────────┘
".replace('\n', get_line_sep());

    type Generator = NAryTreeGenerator;

    let mut rng = get_test_rng();
    let mut maze = get_new_inline_cell_maze();

    Generator::new().generate_with_rng(&mut maze, &mut rng);

    let actual = export_inline_cell_maze(&maze);

    assert_eq!(expected, actual)
}

#[test]
fn test_recursive_backtracker_block_cell_generation() {
    let expected = "\
.██████████..█████.
█           █     █
█ █████ █████ ███ █
█ █   █ █       █ █
█ █ ███ █ ███████ █
█ █ █   █   █     █
█ █ █ █ ███ █ ████.
█ █ █ █ █   █ █   .
█ █ █ █ █ ███ █ ██.
█ █ █ █ █   █ █   █
█ █ █ █████ █████ █
█ █ █     █     █ █
█ █ ███ ███████ █ █
█     █     █ █   █
.████ ███ █ █ ███ █
█   █ █   █     █ █
█ ███ █ █████████ █
█     █           █
.█████.███████████.
".replace('\n', get_line_sep());

    type Generator = RecursiveBacktrackerGenerator;

    let mut rng = get_test_rng();
    let mut maze = get_new_block_cell_maze();

    Generator::new().generate_with_rng(&mut maze, &mut rng);

    let actual = export_block_cell_maze(&maze);

    assert_eq!(expected, actual)
}

#[test]
fn test_recursive_backtracker_inline_cell_generation() {
    let expected = "\
┌─────────────────┬────────┐
│                 │        │
│  ┌─────┐  ┌─────┘  ╶──┐  │
│  │     │  │           │  │
│  │  ┌──┘  │  ╶──┬─────┘  │
│  │  │     │     │        │
│  │  │  ╷  ├──╴  │  ┌─────┤
│  │  │  │  │     │  │     │
│  │  │  │  │  ╶──┤  │  ╶──┤
│  │  │  │  │     │  │     │
│  │  │  └──┴──┐  └──┴──┐  │
│  │  │        │        │  │
│  ╵  └──┐  ╶──┴──┬──┐  ╵  │
│        │        │  │     │
├─────┐  ├──╴  ╷  ╵  └──┐  │
│     │  │     │        │  │
│  ╶──┘  │  ╶──┴────────┘  │
│        │                 │
└────────┴─────────────────┘
".replace('\n', get_line_sep());

    type Generator = RecursiveBacktrackerGenerator;

    let mut rng = get_test_rng();
    let mut maze = get_new_inline_cell_maze();

    Generator::new().generate_with_rng(&mut maze, &mut rng);

    let actual = export_inline_cell_maze(&maze);

    assert_eq!(expected, actual)
}

fn get_test_rng() -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(0)
}

fn get_new_block_cell_maze() -> BoxSpaceBlockCellMazeCoordinator<VecBuffer<BlockCellValue>, 2> {
    BoxSpaceBlockCellMazeCoordinator::<VecBuffer<BlockCellValue>, 2>::builder(BoxCoordinateSpace::<2>::new_checked([9, 9])).build()
}

fn get_new_inline_cell_maze() -> BoxSpaceInlineCellMazeCoordinator::<VecBuffer<InlineCellValue<2>>, 2> {
    BoxSpaceInlineCellMazeCoordinatorBuilder::<VecBuffer<InlineCellValue<2>>, 2>::new(BoxCoordinateSpace::<2>::new_checked([9, 9])).build()
}

fn export_block_cell_maze(maze: &BoxSpaceBlockCellMazeCoordinator<VecBuffer<BlockCellValue>, 2>) -> String {
    let mut str_buffer = Vec::<u8>::new();

    BoxSpaceBlockCellTextMazeExporter::builder()
        .chars_per_cell_horizontally_checked(1)
        .chars_per_cell_vertically_checked(1)
        .build()
        .export(maze, &mut str_buffer).unwrap();

    let as_string = String::from_utf8(str_buffer).unwrap();

    return as_string;
}

fn export_inline_cell_maze(maze: &BoxSpaceInlineCellMazeCoordinator::<VecBuffer<InlineCellValue<2>>, 2>) -> String {
    let mut str_buffer = Vec::<u8>::new();

    BoxSpaceInlineCellTextMazeExporter::builder()
        .chars_per_cell_horizontally_checked(2)
        .chars_per_cell_vertically_checked(1)
        .build()
        .export(maze, &mut str_buffer).unwrap();

    let as_string = String::from_utf8(str_buffer).unwrap();

    return as_string;
}