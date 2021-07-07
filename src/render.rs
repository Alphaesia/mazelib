use crate::geometry::space::{TwoDimensionalSpace, CoordinateSpace, BoxCoordinateSpace};
use crate::cell;
use crate::cell::manager::{UnalignedBoxyCellSpace, AlignedBoxyCellSpace, BoxyCellSpace, CellManager};
use bmp::Pixel;
use crate::cell::data::Basic;
use std::path::Path;
use std::marker::PhantomData;
use crate::buffer::MazeBuffer;

pub trait MazeRenderer<CellSpace: cell::manager::CellManager> {
    type Output;

    fn render(maze: &CellSpace) -> Self::Output;
}

pub trait MazeRendererWithMarker<CellSpace: cell::manager::CellManager> : MazeRenderer<CellSpace> {
    fn render_with_marker(maze: &CellSpace, marker: <<CellSpace as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType) -> Self::Output;
}

pub struct TextRenderer<CellSpace: cell::manager::CellManager> {
    _space: PhantomData<CellSpace>,
}

impl <Buffer: MazeBuffer<<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace, <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType> + MazeBuffer<TwoDimensionalSpace, Basic>> TextRenderer<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> {
    fn render_internal_unaligned(maze: &UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, marker: Option<<<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType>) -> <TextRenderer<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> as MazeRenderer<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>>>::Output {
        let [width, height] = maze.space().dimensions();

        let mut render = Vec::with_capacity(height);

        for y in 0..height {
            let mut line: String = String::with_capacity(width);

            for x in 0..width {
                let pt = [x, y].into();

                if marker.map(|marker| pt == marker).unwrap_or(false) {
                    line.push('@')
                } else {
                    line.push(match maze.get_pt(pt) {
                        <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::PASSAGE => ' ',
                        <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::WALL => '█',
                        <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::BOUNDARY => '█',
                        <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::UNVISITED => '.'
                    })
                }
            };

            render.push(line);
        }

        return render
    }
}

impl <Buffer: MazeBuffer<<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace, <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType> + MazeBuffer<TwoDimensionalSpace, Basic>> TextRenderer<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> {
    fn render_internal_aligned(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, marker: Option<<<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType>) -> <TextRenderer<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> as MazeRenderer<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>>>::Output {
        let [width, height] = maze.space().dimensions().map(|dim| dim * <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>>::scale() + 1);

        let mut render = Vec::with_capacity(height);

        for y in 0..height {
            let mut line: String = String::with_capacity(width);

            for x in 0..width {
                let pt = [x, y].into();

                if marker.map(|marker| pt == marker).unwrap_or(false) {
                    line.push('@')
                } else {
                    line.push(match maze.get_cell(pt) {
                        <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::PASSAGE => ' ',
                        <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::WALL => '█',
                        <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::BOUNDARY => '█',
                        <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType::UNVISITED => '.'
                    })
                }
            };

            render.push(line);
        }

        return render
    }
}

impl <Buffer: MazeBuffer<<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace, <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType> + MazeBuffer<TwoDimensionalSpace, Basic>> MazeRenderer<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> for TextRenderer<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> {
    type Output = Vec<String>;

    fn render(maze: &UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>) -> Self::Output {
        Self::render_internal_unaligned(maze, None)
    }
}

impl <Buffer: MazeBuffer<<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace, <UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType> + MazeBuffer<TwoDimensionalSpace, Basic>> MazeRendererWithMarker<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> for TextRenderer<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> {
    fn render_with_marker(maze: &UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, marker: <<UnalignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType) -> Self::Output {
        Self::render_internal_unaligned(maze, Some(marker))
    }
}

impl <Buffer: MazeBuffer<<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace, <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType> + MazeBuffer<TwoDimensionalSpace, Basic>> MazeRenderer<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> for TextRenderer<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> {
    type Output = Vec<String>;

    fn render(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>) -> Self::Output {
        Self::render_internal_aligned(maze, None)
    }
}

impl <Buffer: MazeBuffer<<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace, <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType> + MazeBuffer<TwoDimensionalSpace, Basic>> MazeRendererWithMarker<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> for TextRenderer<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> {
    fn render_with_marker(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, marker: <<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType) -> Self::Output {
        Self::render_internal_aligned(maze, Some(marker))
    }
}

pub struct BitmapRenderer<CellSpace: cell::manager::CellManager> {
    _space: PhantomData<CellSpace>,
}

const MARKER: Pixel = Pixel { r: 255, g: 0, b: 0 };
const WALL: Pixel = Pixel { r: 0, g: 0, b: 0 };
const PASSAGE: Pixel = Pixel { r: 255, g: 255, b: 255 };
const UNVISITED: Pixel = Pixel { r: 50, g: 50, b: 50 };

impl <Buffer: MazeBuffer<<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace, <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType> + MazeBuffer<TwoDimensionalSpace, Basic>> BitmapRenderer<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> {
    fn render_internal(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, marker: Option<<<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType>) -> bmp::Image {
        // TODO no clone
        let space = *(&maze.space() as &TwoDimensionalSpace);

        let mut img = bmp::Image::new(space.width as u32, space.height as u32);

        for pt in space {
            if marker.map(|marker| marker == pt).unwrap_or(false) {
                img.set_pixel(pt.x as u32, pt.y as u32, MARKER)
            } else if maze.is_passage(pt) {
                img.set_pixel(pt.x as u32, pt.y as u32, PASSAGE)
            } else if maze.is_wall(pt) {
                img.set_pixel(pt.x as u32, pt.y as u32, WALL)
            } else {
                img.set_pixel(pt.x as u32, pt.y as u32, UNVISITED)
            }
        };

        return img;
    }
    fn render_internal_aligned(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, marker: Option<<<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType>) -> bmp::Image {
        let [width, height] = maze.space().dimensions().map(|dim| dim * <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>>::scale() + 1);

        let mut img = bmp::Image::new(width as u32, height as u32);

        for y in 0..height {
            for x in 0..width {
                let pt = [x, y].into();

                if marker.map(|marker| marker == pt).unwrap_or(false) {
                    img.set_pixel(pt.x as u32, pt.y as u32, MARKER)
                } else if maze.is_passage(pt) {
                    img.set_pixel(pt.x as u32, pt.y as u32, PASSAGE)
                } else if maze.is_wall(pt) {
                    img.set_pixel(pt.x as u32, pt.y as u32, WALL)
                } else {
                    img.set_pixel(pt.x as u32, pt.y as u32, UNVISITED)
                }
            };
        }

        return img;
    }

    fn render_and_save_internal(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, path: &Path, marker: Option<<<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType>) -> std::io::Result<()> {
        let img = Self::render_internal(maze, marker);

        return img.save(Path::new(&path));

        //return std::io::Result::Ok(());
    }

    pub fn render_and_save(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, path: &Path) -> std::io::Result<()> {
        Self::render_and_save_internal(maze, path, None)
    }

    pub fn render_and_save_with_marker(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, path: &Path, marker: <<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType) -> std::io::Result<()> {
        Self::render_and_save_internal(maze, path, Some(marker))
    }

    fn render_and_save_internal_aligned(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, path: &Path, marker: Option<<<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType>) -> std::io::Result<()> {
        let img = Self::render_internal_aligned(maze, marker);

        return img.save(&path);
    }

    pub fn render_and_save_aligned(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, path: &Path) -> std::io::Result<()> {
        Self::render_and_save_internal_aligned(maze, path, None)
    }

    pub fn render_and_save_with_marker_aligned(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, path: &Path, marker: <<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType) -> std::io::Result<()> {
        Self::render_and_save_internal_aligned(maze, path, Some(marker))
    }
}

impl <Buffer: MazeBuffer<<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace, <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType> + MazeBuffer<TwoDimensionalSpace, Basic>> MazeRenderer<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> for BitmapRenderer<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> {
    type Output = bmp::Image;

    fn render(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>) -> Self::Output {
        Self::render_internal(maze, None)
    }
}

impl <Buffer: MazeBuffer<<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace, <AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CellType> + MazeBuffer<TwoDimensionalSpace, Basic>> MazeRendererWithMarker<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> for BitmapRenderer<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>> {
    fn render_with_marker(maze: &AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2>, marker: <<AlignedBoxyCellSpace<Buffer, TwoDimensionalSpace, 2> as cell::manager::CellManager>::CoordSpace as CoordinateSpace>::PtType) -> Self::Output {
        Self::render_internal(maze, Some(marker))
    }
}