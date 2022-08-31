use std::io::{Result, Write};

use crate::implm::cell::inline::{BoxSpaceInlineCellManager, InlineCellValue, InlineCellValueEdgeType as EdgeType};
use crate::implm::render::text::{BoxSpaceTextMazeRenderer, TextMazeRenderer};
use crate::implm::render::text::line_break::WriteLineBreak;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::{CellID, CellManager};
use crate::interface::render::MazeRendererNonSeeking;

impl <Buffer: MazeBuffer<InlineCellValue<2>>> MazeRendererNonSeeking<BoxSpaceInlineCellManager<Buffer, 2>> for BoxSpaceTextMazeRenderer {
    fn render<Output: Write>(&self, maze: &BoxSpaceInlineCellManager<Buffer, 2>, output: &mut Output) -> Result<()> {
        let [width, height] = maze.coord_space().dimensions();

        // Below +1's: cause we're looking at walls not cells

        // Track where the walls are above the current row, so we know where to print horizontal walls
        let mut top_walls = vec![EdgeType::PASSAGE; height + 1];

        // Track relevant connections for wall intersections, so we know how to print them
        // [has_left_connection, has_top_connection]
        // (bottom and right can be worked out at print time)
        let mut wall_connections = vec![[EdgeType::PASSAGE; 2]; height + 1];

        for y in 0..height {
            // TODO can we get rid of these and just write to the output directly?
            // We render two lines simultaneously
            let mut line_top_walls: String = String::with_capacity(width * 2 + 1);
            let mut line_side_walls: String = String::with_capacity(width * 2 + 1);

            // Track if there was a wall just before this one, so we know how to print joiners
            let mut wall_previously = EdgeType::PASSAGE;

            for x in 0..width {
                let pt = CellID(x + y * width);

                let cell_value = maze.buffer().get(pt);

                let walls = cell_value.edges;

                let left_wall_actual = match (walls[0][0], wall_previously) {
                    (EdgeType::BOUNDARY, _) | (_, EdgeType::BOUNDARY) => EdgeType::BOUNDARY,
                    (EdgeType::WALL, _) | (_, EdgeType::WALL) => EdgeType::WALL,
                    (EdgeType::UNVISITED, _) | (_, EdgeType::UNVISITED) => EdgeType::UNVISITED,
                    (EdgeType::PASSAGE, EdgeType::PASSAGE) => EdgeType::PASSAGE,
                };

                let top_wall_actual = match (walls[1][0], top_walls[x]) {
                    (EdgeType::BOUNDARY, _) | (_, EdgeType::BOUNDARY) => EdgeType::BOUNDARY,
                    (EdgeType::WALL, _) | (_, EdgeType::WALL) => EdgeType::WALL,
                    (EdgeType::UNVISITED, _) | (_, EdgeType::UNVISITED) => EdgeType::UNVISITED,
                    (EdgeType::PASSAGE, EdgeType::PASSAGE) => EdgeType::PASSAGE,
                };

                line_top_walls.push(Self::get_box_char(wall_connections[x][0], wall_connections[x][1], top_wall_actual, left_wall_actual));

                line_top_walls.push_str(match top_wall_actual {
                    EdgeType::WALL => "──",
                    EdgeType::BOUNDARY => "━━",
                    EdgeType::PASSAGE => "  ",
                    EdgeType::UNVISITED => "┄┄",
                });

                line_side_walls.push(match left_wall_actual {
                    EdgeType::WALL => '│',
                    EdgeType::BOUNDARY => '┃',
                    EdgeType::PASSAGE => ' ',
                    EdgeType::UNVISITED => '┆',
                });

                line_side_walls.push_str("  ");  // Cells are 2-chars wide

                wall_previously = walls[0][1];
                top_walls[x] = walls[1][1];

                wall_connections[x][1] = left_wall_actual;
                wall_connections[x + 1][0] = top_wall_actual;
            };

            // Draw the right side

            line_top_walls.push(Self::get_box_char(wall_connections[width][0], wall_connections[width][1], EdgeType::PASSAGE, wall_previously));

            if let EdgeType::WALL = wall_previously {
                line_side_walls.push('│');
                wall_connections[width][1] = wall_previously;
            } else if let EdgeType::BOUNDARY = wall_previously {
                line_side_walls.push('┃');
                wall_connections[width][1] = wall_previously;
            } else if let EdgeType::UNVISITED = wall_previously {
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
                let pt = CellID(x + (height - 1) * width);  // We're iterating over the last row

                let cell_value = maze.buffer().get(pt);

                let walls = cell_value.edges;

                line.push(Self::get_box_char(wall_connections[x][0], walls[0][0], walls[1][1], EdgeType::PASSAGE));

                line.push_str(match walls[1][1] {
                    EdgeType::WALL => "──",
                    EdgeType::BOUNDARY => "━━",
                    EdgeType::PASSAGE => "  ",
                    EdgeType::UNVISITED => "┄┄",
                });

                wall_connections[x + 1][0] = walls[1][1];
            }

            // Bottom-right corner
            line.push(Self::get_box_char(wall_connections[width][0], wall_connections[width][1], EdgeType::PASSAGE, EdgeType::PASSAGE));

            output.write_all(line.as_bytes())?;
            output.write_line_break()?;
        }

        return Ok(())
    }
}

impl BoxSpaceTextMazeRenderer {
    // not recommended reading
    fn get_box_char(left_wall: EdgeType, top_wall: EdgeType, right_wall: EdgeType, bottom_wall: EdgeType) -> char {
        match (left_wall, top_wall, right_wall, bottom_wall) {
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED) => '┼',
            (EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED) => '┽',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED) => '╀',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED) => '┾',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY)                   => '╁',
            (EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED) => '╃',
            (EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED) => '┿',
            (EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY)                   => '╅',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED) => '╄',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY)                   => '╂',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::BOUNDARY)                   => '╆',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::BOUNDARY)                   => '╊',
            (EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::BOUNDARY)                   => '╈',
            (EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY)                   => '╉',
            (EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED) => '╇',
            (EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::BOUNDARY)                   => '╋',

            (EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED) => '├',
            (EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED) => '┞',
            (EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED) => '┝',
            (EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY)                   => '┟',
            (EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED) => '┡',
            (EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY)                   => '┠',
            (EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::BOUNDARY)                   => '┢',
            (EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::BOUNDARY)                   => '┣',

            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED) => '┬',
            (EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED) => '┭',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED) => '┮',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY)                   => '┰',
            (EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED) => '┯',
            (EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY)                   => '┱',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::BOUNDARY)                   => '┲',
            (EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::BOUNDARY)                   => '┳',

            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED) => '┤',
            (EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED) => '┥',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED) => '┦',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::BOUNDARY)                   => '┧',
            (EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED) => '┩',
            (EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::BOUNDARY)                   => '┪',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::BOUNDARY)                   => '┨',
            (EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::BOUNDARY)                   => '┫',

            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE)                    => '┴',
            (EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE)                    => '┵',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE)                    => '┸',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::PASSAGE)                    => '┶',
            (EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE)                    => '┹',
            (EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::PASSAGE)                    => '┷',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::PASSAGE)                    => '┺',
            (EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::PASSAGE)                    => '┻',

            (EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED) => '│',
            (EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED) => '╿',
            (EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::BOUNDARY)                   => '╽',
            (EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::BOUNDARY)                   => '┃',

            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE)                    => '─',
            (EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE)                    => '╾',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::PASSAGE)                    => '╼',
            (EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::PASSAGE)                    => '━',

            (EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED) => '┌',
            (EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED) => '┍',
            (EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY)                   => '┎',
            (EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::BOUNDARY)                   => '┏',

            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED) => '┐',
            (EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED) => '┑',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::BOUNDARY)                   => '┒',
            (EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::BOUNDARY)                   => '┓',

            (EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE)                    => '└',
            (EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE)                    => '┖',
            (EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::PASSAGE)                    => '┕',
            (EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::PASSAGE)                    => '┗',

            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::PASSAGE)                    => '┘',
            (EdgeType::BOUNDARY,                    EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::PASSAGE)                    => '┙',
            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::PASSAGE)                    => '┚',
            (EdgeType::BOUNDARY,                    EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::PASSAGE)                    => '┛',

            (EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::PASSAGE)                    => '╴',
            (EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::PASSAGE)                    => '╸',

            (EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE,                     EdgeType::PASSAGE)                    => '╵',
            (EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::PASSAGE,                     EdgeType::PASSAGE)                    => '╹',

            (EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED,  EdgeType::PASSAGE)                    => '╶',
            (EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::BOUNDARY,                    EdgeType::PASSAGE)                    => '╺',

            (EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::WALL | EdgeType::UNVISITED) => '╷',
            (EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::BOUNDARY)                   => '╻',

            (EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::PASSAGE,                     EdgeType::PASSAGE)                     => ' ',
        }
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<2>>> TextMazeRenderer<BoxSpaceInlineCellManager<Buffer, 2>> for BoxSpaceTextMazeRenderer {}