use crate::geometry::space::{TwoDimensionalSpace, CoordinateSpace, BoxCoordinateSpace};
use crate::{cell, maze};
use crate::cell::space::{UnalignedBoxyCellSpace, AlignedBoxyCellSpace, BoxyCellSpace};
use crate::maze::Maze;
use bmp::Pixel;
use crate::cell::data::CellData;
use std::path::Path;
use std::marker::PhantomData;

pub trait MazeRenderer<Maze: maze::Maze<CellSpace>, CellSpace: cell::space::CellSpace<Maze>> {
    type Output;

    fn render(maze: &Maze) -> Self::Output;
}

pub trait MazeRendererWithMarker<Maze: maze::Maze<CellSpace>, CellSpace: cell::space::CellSpace<Maze>> : MazeRenderer<Maze, CellSpace> {
    fn render_with_marker(maze: &Maze, marker: <<CellSpace as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType) -> Self::Output;
}

pub struct TextRenderer<Maze: maze::Maze<CellSpace>, CellSpace: cell::space::CellSpace<Maze>> {
    _maze: PhantomData<Maze>,
    _space: PhantomData<CellSpace>
}

impl <Maze: maze::Maze<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>> TextRenderer<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> {
    fn render_internal_unaligned(maze: &Maze, marker: Option<<<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType>) -> <TextRenderer<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> as MazeRenderer<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>>::Output {
        let [width, height] = maze.space().dimensions();

        let mut render = Vec::with_capacity(height);

        for y in 0..height {
            let mut line: String = String::with_capacity(width);

            for x in 0..width {
                let pt = [x, y].into();

                if marker.map(|marker| pt == marker).unwrap_or(false) {
                    line.push('@')
                } else {
                    line.push(match maze.buffer().get_pt(pt) {
                        <UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CellType::PASSAGE => ' ',
                        <UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CellType::WALL => '█',
                        <UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CellType::BOUNDARY => '█',
                        <UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CellType::UNVISITED => '.'
                    })
                }
            };

            render.push(line);
        }

        return render
    }
}

impl <Maze: maze::Maze<AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>> TextRenderer<Maze, AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> {
    fn render_internal_aligned(maze: &Maze, marker: Option<<<AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType>) -> <TextRenderer<Maze, AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> as MazeRenderer<Maze, AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>>::Output {
        let [width, height] = maze.space().dimensions().map(|dim| dim * <AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>::scale() + 1);

        let mut render = Vec::with_capacity(height);

        for y in 0..height {
            let mut line: String = String::with_capacity(width);

            for x in 0..width {
                let pt = [x, y].into();

                if marker.map(|marker| pt == marker).unwrap_or(false) {
                    line.push('@')
                } else {
                    line.push(match maze.buffer().get_pt(pt) {
                        <AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CellType::PASSAGE => ' ',
                        <AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CellType::WALL => '█',
                        <AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CellType::BOUNDARY => '█',
                        <AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CellType::UNVISITED => '.'
                    })
                }
            };

            render.push(line);
        }

        return render
    }
}

impl <Maze: maze::Maze<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>> MazeRenderer<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> for TextRenderer<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> {
    type Output = Vec<String>;

    fn render(maze: &Maze) -> Self::Output {
        Self::render_internal_unaligned(maze, None)
    }
}

impl <Maze: maze::Maze<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>> MazeRendererWithMarker<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> for TextRenderer<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> {
    fn render_with_marker(maze: &Maze, marker: <<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType) -> Self::Output {
        Self::render_internal_unaligned(maze, Some(marker))
    }
}

impl <Maze: maze::Maze<AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>> MazeRenderer<Maze, AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> for TextRenderer<Maze, AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> {
    type Output = Vec<String>;

    fn render(maze: &Maze) -> Self::Output {
        Self::render_internal_aligned(maze, None)
    }
}

impl <Maze: maze::Maze<AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>> MazeRendererWithMarker<Maze, AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> for TextRenderer<Maze, AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> {
    fn render_with_marker(maze: &Maze, marker: <<AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType) -> Self::Output {
        Self::render_internal_aligned(maze, Some(marker))
    }
}

pub struct BitmapRenderer<Maze: maze::Maze<CellSpace>, CellSpace: cell::space::CellSpace<Maze>> {
    _maze: PhantomData<Maze>,
    _space: PhantomData<CellSpace>
}

const MARKER: Pixel = Pixel { r: 255, g: 0, b: 0 };
const WALL: Pixel = Pixel { r: 0, g: 0, b: 0 };
const PASSAGE: Pixel = Pixel { r: 255, g: 255, b: 255 };
const UNVISITED: Pixel = Pixel { r: 50, g: 50, b: 50 };

impl <Maze: maze::Maze<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>> BitmapRenderer<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> {
    fn render_internal(maze: &Maze, marker: Option<<<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType>) -> bmp::Image {
        // TODO no clone
        let space = *(&maze.space() as &TwoDimensionalSpace);

        let mut img = bmp::Image::new(space.width as u32, space.height as u32);

        for pt in space {
            if marker.map(|marker| marker == pt).unwrap_or(false) {
                img.set_pixel(pt.x as u32, pt.y as u32, MARKER)
            } else if maze.buffer().get_pt(pt).is_passage() {
                img.set_pixel(pt.x as u32, pt.y as u32, PASSAGE)
            } else if maze.buffer().get_pt(pt).is_wall() {
                img.set_pixel(pt.x as u32, pt.y as u32, WALL)
            } else {
                img.set_pixel(pt.x as u32, pt.y as u32, UNVISITED)
            }
        };

        return img;
    }
    fn render_internal_aligned(maze: &Maze, marker: Option<<<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType>) -> bmp::Image {
        let [width, height] = maze.space().dimensions().map(|dim| dim * <AlignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>::scale() + 1);

        let mut img = bmp::Image::new(width as u32, height as u32);

        for y in 0..height {
            for x in 0..width {
                let pt = [x, y].into();

                if marker.map(|marker| marker == pt).unwrap_or(false) {
                    img.set_pixel(pt.x as u32, pt.y as u32, MARKER)
                } else if maze.buffer().get_pt(pt).is_passage() {
                    img.set_pixel(pt.x as u32, pt.y as u32, PASSAGE)
                } else if maze.buffer().get_pt(pt).is_wall() {
                    img.set_pixel(pt.x as u32, pt.y as u32, WALL)
                } else {
                    img.set_pixel(pt.x as u32, pt.y as u32, UNVISITED)
                }
            };
        }

        return img;
    }

    fn render_and_save_internal(maze: &Maze, path: &Path, marker: Option<<<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType>) -> std::io::Result<()> {
        let img = Self::render_internal(maze, marker);

        return img.save(Path::new(&path));

        //return std::io::Result::Ok(());
    }

    pub fn render_and_save(maze: &Maze, path: &Path) -> std::io::Result<()> {
        Self::render_and_save_internal(maze, path, None)
    }

    pub fn render_and_save_with_marker(maze: &Maze, path: &Path, marker: <<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType) -> std::io::Result<()> {
        Self::render_and_save_internal(maze, path, Some(marker))
    }

    fn render_and_save_internal_aligned(maze: &Maze, path: &Path, marker: Option<<<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType>) -> std::io::Result<()> {
        let img = Self::render_internal_aligned(maze, marker);

        return img.save(&path);
    }

    pub fn render_and_save_aligned(maze: &Maze, path: &Path) -> std::io::Result<()> {
        Self::render_and_save_internal_aligned(maze, path, None)
    }

    pub fn render_and_save_with_marker_aligned(maze: &Maze, path: &Path, marker: <<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType) -> std::io::Result<()> {
        Self::render_and_save_internal_aligned(maze, path, Some(marker))
    }
}

impl <Maze: maze::Maze<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>> MazeRenderer<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> for BitmapRenderer<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> {
    type Output = bmp::Image;

    fn render(maze: &Maze) -> Self::Output {
        Self::render_internal(maze, None)
    }
}

impl <Maze: maze::Maze<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>>> MazeRendererWithMarker<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> for BitmapRenderer<Maze, UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2>> {
    fn render_with_marker(maze: &Maze, marker: <<UnalignedBoxyCellSpace<Maze, TwoDimensionalSpace, 2> as cell::space::CellSpace<Maze>>::CoordSpace as CoordinateSpace>::PtType) -> Self::Output {
        Self::render_internal(maze, Some(marker))
    }
}