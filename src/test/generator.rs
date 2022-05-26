use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use crate::implm::buffer::VecBuffer;
use crate::implm::cell::block::{BlockCellValue, BoxSpaceBlockCellManager, BoxSpaceBlockCellManagerBuilder};
use crate::implm::generate::{HuntAndKillGenerator, NAryTreeGenerator, RecursiveBacktrackerGenerator};
use crate::implm::point::boxy::BoxCoordinateSpace;
use crate::implm::render::text::BoxSpaceTextMazeRenderer;
use crate::interface::generate::MazeGenerator;
use crate::interface::render::MazeRendererNonSeeking;
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

    let actual = render_block_cell_maze(&maze);

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

    let actual = render_block_cell_maze(&maze);

    assert_eq!(expected, actual)
}

#[test]
fn test_recursive_backtracker_block_cell_generation() {
    let expected = "\
.█...████..██████..
█ █ █     █       .
█ █ █ █ ███ █ ████.
█ █ █ █     █     █
█ █ █ ███████████ █
█ █ █ █   █     █ █
█ █ █ █ ███ ███ █ █
█ █   █ █   █ █   █
█ █ ███ █ ███ ███ █
█ █ █   █ █ █     █
█ █ █ █ █ █ █ ████.
█ █   █ █ █ █   █ .
█ ████.██ █ ███ █ █
█     █   █   █ █ █
.████ █ ███ ███ █ █
█   █   █ █   █ █ █
█ ███████ █ █ █ █ █
█           █     █
.███████████.█████.
".replace('\n', get_line_sep());

    type Generator = RecursiveBacktrackerGenerator;

    let mut rng = get_test_rng();
    let mut maze = get_new_block_cell_maze();

    Generator::new().generate_with_rng(&mut maze, &mut rng);

    let actual = render_block_cell_maze(&maze);

    assert_eq!(expected, actual)
}

fn get_test_rng() -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(0)
}

fn get_new_block_cell_maze() -> BoxSpaceBlockCellManager<VecBuffer<BlockCellValue>, 2> {
    BoxSpaceBlockCellManagerBuilder::<VecBuffer<BlockCellValue>, 2>::new(BoxCoordinateSpace::<2>::new([9, 9])).build()
}

fn render_block_cell_maze(maze: &BoxSpaceBlockCellManager<VecBuffer<BlockCellValue>, 2>) -> String {
    let mut str_buffer = Vec::<u8>::new();

    BoxSpaceTextMazeRenderer::new().render(maze, &mut str_buffer).unwrap();

    let as_string = String::from_utf8(str_buffer).unwrap();

    return as_string;
}