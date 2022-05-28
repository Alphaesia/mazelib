use std::io::{Write, Result};
use crate::interface::render::MazeRendererNonSeeking;
use crate::interface::cell::CellManager;
use crate::implm::cell::inline::{BoxSpaceInlineCellManager, InlineCellValue, InlineCellValueWallType as WallType};
use crate::interface::buffer::{MazeBuffer, BufferLocation};
use crate::implm::render::text::{TextMazeRenderer, BoxSpaceTextMazeRenderer};
use crate::implm::render::text::line_break::WriteLineBreak;

impl <Buffer: MazeBuffer<InlineCellValue<2>>> MazeRendererNonSeeking<BoxSpaceInlineCellManager<Buffer, 2>> for BoxSpaceTextMazeRenderer {
    fn render<Output: Write>(&self, maze: &BoxSpaceInlineCellManager<Buffer, 2>, output: &mut Output) -> Result<()> {
        let [width, height] = maze.coord_space().dimensions();

        // Below +1's: cause we're looking at walls not cells

        // Track where the walls are above the current row, so we know where to print horizontal walls
        let mut top_walls = vec![WallType::PASSAGE; height + 1];

        // Track relevant connections for wall intersections, so we know how to print them
        // [has_left_connection, has_top_connection]
        // (bottom and right can be worked out at print time)
        let mut wall_connections = vec![[WallType::PASSAGE; 2]; height + 1];

        for y in 0..height {
            // TODO can we get rid of these and just write to the output directly?
            // We render two lines simultaneously
            let mut line_top_walls: String = String::with_capacity(width * 2 + 1);
            let mut line_side_walls: String = String::with_capacity(width * 2 + 1);

            // Track if there was a wall just before this one, so we know how to print joiners
            let mut wall_previously = WallType::PASSAGE;

            for x in 0..width {
                let pt = BufferLocation(x + y * width);

                let cell_value = maze.buffer().get(pt);

                let walls = cell_value.walls;

                let left_wall_actual = match (walls[0][0], wall_previously) {
                    (WallType::BOUNDARY, _) | (_, WallType::BOUNDARY) => WallType::BOUNDARY,
                    (WallType::WALL, _) | (_, WallType::WALL) => WallType::WALL,
                    (WallType::UNVISITED, _) | (_, WallType::UNVISITED) => WallType::UNVISITED,
                    (WallType::PASSAGE, WallType::PASSAGE) => WallType::PASSAGE,
                };

                let top_wall_actual = match (walls[1][0], top_walls[x]) {
                    (WallType::BOUNDARY, _) | (_, WallType::BOUNDARY) => WallType::BOUNDARY,
                    (WallType::WALL, _) | (_, WallType::WALL) => WallType::WALL,
                    (WallType::UNVISITED, _) | (_, WallType::UNVISITED) => WallType::UNVISITED,
                    (WallType::PASSAGE, WallType::PASSAGE) => WallType::PASSAGE,
                };

                line_top_walls.push(Self::get_box_char(wall_connections[x][0], wall_connections[x][1], top_wall_actual, left_wall_actual));

                line_top_walls.push_str(match top_wall_actual {
                    WallType::WALL => "──",
                    WallType::BOUNDARY => "━━",
                    WallType::PASSAGE => "  ",
                    WallType::UNVISITED => "┄┄",
                });

                line_side_walls.push(match left_wall_actual {
                    WallType::WALL => '│',
                    WallType::BOUNDARY => '┃',
                    WallType::PASSAGE => ' ',
                    WallType::UNVISITED => '┆',
                });

                line_side_walls.push_str("  ");  // Cells are 2-chars wide

                wall_previously = walls[0][1];
                top_walls[x] = walls[1][1];

                wall_connections[x][1] = left_wall_actual;
                wall_connections[x + 1][0] = top_wall_actual;
            };

            // Draw the right side

            line_top_walls.push(Self::get_box_char(wall_connections[width][0], wall_connections[width][1], WallType::PASSAGE, wall_previously));

            if let WallType::WALL = wall_previously {
                line_side_walls.push('│');
                wall_connections[width][1] = wall_previously;
            } else if let WallType::BOUNDARY = wall_previously {
                line_side_walls.push('┃');
                wall_connections[width][1] = wall_previously;
            } else if let WallType::UNVISITED = wall_previously {
                line_side_walls.push('┆');
                wall_connections[width][1] = wall_previously;
            }

            output.write_all(line_top_walls.as_bytes())?;
            output.write_line_break()?;
            output.write_all(line_side_walls.as_bytes())?;
            output.write_line_break()?;
        }

        // Draw the bottom side
        {
            let mut line: String = String::with_capacity(width * 2 + 1);

            for x in 0..width {
                let pt = BufferLocation(x + (height - 1) * width);  // We're iterating over the last row

                let cell_value = maze.buffer().get(pt);

                let walls = cell_value.walls;

                line.push(Self::get_box_char(wall_connections[x][0], walls[0][0], walls[1][1], WallType::PASSAGE));

                line.push_str(match walls[1][1] {
                    WallType::WALL => "──",
                    WallType::BOUNDARY => "━━",
                    WallType::PASSAGE => "  ",
                    WallType::UNVISITED => "┄┄",
                });

                wall_connections[x + 1][0] = walls[1][1];
            }

            // Bottom-right corner
            line.push(Self::get_box_char(wall_connections[width][0], wall_connections[width][1], WallType::PASSAGE, WallType::PASSAGE));

            output.write_all(line.as_bytes())?;
            output.write_line_break()?;
        }

        return Ok(())
    }
}

impl BoxSpaceTextMazeRenderer {
    // not recommended reading
    fn get_box_char(left_wall: WallType, top_wall: WallType, right_wall: WallType, bottom_wall: WallType) -> char {
        match (left_wall, top_wall, right_wall, bottom_wall) {
            (WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED) => '┼',
            (WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED) => '┽',
            (WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED) => '╀',
            (WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED) => '┾',
            (WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY)                   => '╁',
            (WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED) => '╃',
            (WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED) => '┿',
            (WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY)                   => '╅',
            (WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED) => '╄',
            (WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY)                   => '╂',
            (WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::BOUNDARY)                   => '╆',
            (WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::BOUNDARY)                   => '╊',
            (WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::BOUNDARY)                   => '╈',
            (WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY)                   => '╉',
            (WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED) => '╇',
            (WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::BOUNDARY)                   => '╋',

            (WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED) => '├',
            (WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED) => '┞',
            (WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED) => '┝',
            (WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY)                   => '┟',
            (WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED) => '┡',
            (WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY)                   => '┠',
            (WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::BOUNDARY)                   => '┢',
            (WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::BOUNDARY)                   => '┣',

            (WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED) => '┬',
            (WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED) => '┭',
            (WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED) => '┮',
            (WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY)                   => '┰',
            (WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED) => '┯',
            (WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY)                   => '┱',
            (WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::BOUNDARY)                   => '┲',
            (WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::BOUNDARY)                   => '┳',

            (WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED) => '┤',
            (WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED) => '┥',
            (WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED) => '┦',
            (WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::BOUNDARY)                   => '┧',
            (WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED) => '┩',
            (WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::BOUNDARY)                   => '┪',
            (WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::BOUNDARY)                   => '┨',
            (WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::BOUNDARY)                   => '┫',

            (WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE)                    => '┴',
            (WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE)                    => '┵',
            (WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE)                    => '┸',
            (WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::PASSAGE)                    => '┶',
            (WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE)                    => '┹',
            (WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::PASSAGE)                    => '┷',
            (WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::PASSAGE)                    => '┺',
            (WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::PASSAGE)                    => '┻',

            (WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED) => '│',
            (WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED) => '╿',
            (WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::BOUNDARY)                   => '╽',
            (WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::BOUNDARY)                   => '┃',

            (WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE)                    => '─',
            (WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE)                    => '╾',
            (WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::PASSAGE)                    => '╼',
            (WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::PASSAGE)                    => '━',

            (WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED) => '┌',
            (WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED) => '┍',
            (WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY)                   => '┎',
            (WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::BOUNDARY)                   => '┏',

            (WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED) => '┐',
            (WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED) => '┑',
            (WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::BOUNDARY)                   => '┒',
            (WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::BOUNDARY)                   => '┓',

            (WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE)                    => '└',
            (WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE)                    => '┖',
            (WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::PASSAGE)                    => '┕',
            (WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::PASSAGE)                    => '┗',

            (WallType::WALL | WallType::UNVISITED,  WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::PASSAGE)                    => '┘',
            (WallType::BOUNDARY,                    WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::PASSAGE)                    => '┙',
            (WallType::WALL | WallType::UNVISITED,  WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::PASSAGE)                    => '┚',
            (WallType::BOUNDARY,                    WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::PASSAGE)                    => '┛',

            (WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::PASSAGE)                    => '╴',
            (WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::PASSAGE)                    => '╸',

            (WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE,                     WallType::PASSAGE)                    => '╵',
            (WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::PASSAGE,                     WallType::PASSAGE)                    => '╹',

            (WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED,  WallType::PASSAGE)                    => '╶',
            (WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::BOUNDARY,                    WallType::PASSAGE)                    => '╺',

            (WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::WALL | WallType::UNVISITED) => '╷',
            (WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::BOUNDARY)                   => '╻',

            (WallType::PASSAGE,                     WallType::PASSAGE,                     WallType::PASSAGE,                    WallType::PASSAGE)                     => ' ',
        }
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<2>>> TextMazeRenderer<BoxSpaceInlineCellManager<Buffer, 2>> for BoxSpaceTextMazeRenderer {}