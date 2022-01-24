use crate::interface::render::MazeRenderer;
use crate::interface::cell::CellManager;
use crate::implm::cell::block::{BoxSpaceBlockCellManager, BlockCellValue};
use crate::interface::buffer::{MazeBuffer, BufferLocation};

pub trait TextMazeRenderer<CellSpace: CellManager> : MazeRenderer<CellSpace, Output=Vec<String>> {}

pub struct BoxSpaceBlockCellTextMazeRenderer {}

impl<Buffer: MazeBuffer<BlockCellValue>> MazeRenderer<BoxSpaceBlockCellManager<Buffer, 2>> for BoxSpaceBlockCellTextMazeRenderer {
    type Output = Vec<String>;

    fn render(maze: &BoxSpaceBlockCellManager<Buffer, 2>) -> Self::Output {
        let [width, height] = maze.get_full_dimensions();

        let mut render = Vec::with_capacity(height);

        for y in 0..height {
            let mut line: String = String::with_capacity(width);

            for x in 0..width {
                let pt = BufferLocation(x + y * width);

                line.push(match maze.buffer().get(pt) {
                    BlockCellValue::PASSAGE => ' ',
                    BlockCellValue::WALL => '█',
                    BlockCellValue::BOUNDARY => '█',
                    BlockCellValue::UNVISITED => '.'
                });
            };

            render.push(line);
        }

        return render
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>> TextMazeRenderer<BoxSpaceBlockCellManager<Buffer, 2>> for BoxSpaceBlockCellTextMazeRenderer {}

// impl <Buffer: MazeBuffer<<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace, <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType> + MazeBuffer<TwoDimensionalSpace, Basic>> TextRenderer<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> {
//     fn render_internal_unaligned(maze: &UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, marker: Option<<<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType>) -> <TextRenderer<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> as MazeRenderer<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>>>::Output {
//         let [width, height] = maze.space().dimensions();
//
//         let mut render = Vec::with_capacity(height);
//
//         for y in 0..height {
//             let mut line: String = String::with_capacity(width);
//
//             for x in 0..width {
//                 let pt = [x, y].into();
//
//                 if marker.map(|marker| pt == marker).unwrap_or(false) {
//                     line.push('@')
//                 } else {
//                     line.push(match maze.get_pt(pt) {
//                         <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::PASSAGE => ' ',
//                         <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::WALL => '█',
//                         <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::BOUNDARY => '█',
//                         <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::UNVISITED => '.'
//                     })
//                 }
//             };
//
//             render.push(line);
//         }
//
//         return render
//     }
// }