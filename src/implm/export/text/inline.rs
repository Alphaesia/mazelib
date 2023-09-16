use std::io::{Result, Write};
use std::num::NonZeroUsize;

use crate::implm::cell::inline::{InlineCellValue, InlineCellValueEdgeType as EdgeType};
use crate::implm::coordinate::inline::BoxSpaceInlineCellMazeCoordinator;
use crate::implm::export::text::TextMazeExporter;
use crate::interface::buffer::MazeBuffer;
use crate::interface::cell::CellID;
use crate::interface::coordinate::MazeCoordinator;
use crate::interface::export::MazeExporter;
use crate::internal::line_break::WriteLineBreak;
use crate::internal::util::{nonzero_usize_array_to_usize_array, NONZERO_USIZE_ONE, NONZERO_USIZE_THREE};

/// An exporter that renders [`BoxSpaceInlineCellMazeCoordinator`]s to text.
pub struct BoxSpaceInlineCellTextMazeExporter {
    /// The number of the characters to emit horizontally per cell, excluding walls.
    chars_per_cell_horizontally: NonZeroUsize,
    /// The number of the characters to emit vertically per cell, excluding walls.
    chars_per_cell_vertically:   NonZeroUsize,
}

impl BoxSpaceInlineCellTextMazeExporter {
    /// Construct a new builder for a `BoxSpaceInlineCellTextMazeExporter`.
    ///
    /// Optional, see [`DefaultMazeExporter`][crate::interface::export::DefaultMazeExporter].
    pub fn builder() -> BoxSpaceInlineCellTextMazeExporterBuilder {
        BoxSpaceInlineCellTextMazeExporterBuilder::new()
    }

    /// Return the number of the characters that is emitted horizontally per cell, excluding walls.
    #[must_use]
    pub fn chars_per_cell_horizontally(&self) -> NonZeroUsize {
        self.chars_per_cell_horizontally
    }

    /// Return the number of the characters that is emitted vertically per cell, excluding walls.
    #[must_use]
    pub fn chars_per_cell_vertically(&self) -> NonZeroUsize {
        self.chars_per_cell_vertically
    }
}

impl Default for BoxSpaceInlineCellTextMazeExporter {
    fn default() -> Self {
        Self::builder().build()
    }
}

/// A builder for a [`BoxSpaceInlineCellTextMazeExporter`].
#[must_use]
pub struct BoxSpaceInlineCellTextMazeExporterBuilder {
    /// The number of the characters to emit horizontally per cell.
    chars_per_cell_horizontally: NonZeroUsize,
    /// The number of the characters to emit vertically per cell.
    chars_per_cell_vertically:   NonZeroUsize,
}

impl BoxSpaceInlineCellTextMazeExporterBuilder {
    /// Create a new builder for a [`BoxSpaceInlineCellTextMazeExporter`].
    fn new() -> Self {
        Self {
            chars_per_cell_horizontally: NONZERO_USIZE_THREE,
            chars_per_cell_vertically:   NONZERO_USIZE_ONE,
        }
    }

    /// Set the number of the characters to emit horizontally per cell, excluding walls.
    ///
    /// # See Also
    /// 
    /// [`Self::chars_per_cell_horizontally_checked()`]. If you're using integer literals, you may
    /// wish to use this instead.
    pub fn chars_per_cell_horizontally(mut self, count: NonZeroUsize) -> Self {
        self.chars_per_cell_horizontally = count;

        return self
    }

    /// Set the number of the characters to emit horizontally per cell, excluding walls.
    ///
    /// `count` must be non-zero.
    ///
    /// # Panics
    ///
    /// If `count` is zero.
    ///
    /// # See Also
    ///
    /// [`Self::chars_per_cell_horizontally()`], which takes a `NonZeroUsize`
    pub fn chars_per_cell_horizontally_checked(self, count: usize) -> Self {
        self.chars_per_cell_horizontally(NonZeroUsize::new(count).expect("count was zero"))
    }

    /// Set the number of the characters to emit vertically per cell, excluding walls.
    ///
    /// # See Also
    /// 
    /// [`Self::chars_per_cell_vertically_checked()`]. If you're using integer literals, you may
    /// wish to use this instead.
    pub fn chars_per_cell_vertically(mut self, count: NonZeroUsize) -> Self {
        self.chars_per_cell_vertically = count;

        return self
    }

    /// Set the number of the characters to emit vertically per cell, excluding walls.
    ///
    /// `count` must be non-zero.
    ///
    /// # Panics
    ///
    /// If `count` is zero.
    ///
    /// # See Also
    ///
    /// [`Self::chars_per_cell_vertically()`], which takes a `NonZeroUsize`
    pub fn chars_per_cell_vertically_checked(self, count: usize) -> Self {
        self.chars_per_cell_vertically(NonZeroUsize::new(count).expect("count was zero"))
    }


    /// Finalise the [`BoxSpaceInlineCellTextMazeExporter`].
    #[must_use]
    pub fn build(self) -> BoxSpaceInlineCellTextMazeExporter {
        BoxSpaceInlineCellTextMazeExporter {
            chars_per_cell_horizontally: self.chars_per_cell_horizontally,
            chars_per_cell_vertically:   self.chars_per_cell_vertically,
        }
    }
}

impl <Buffer: MazeBuffer<InlineCellValue<2>>, Output: Write> MazeExporter<BoxSpaceInlineCellMazeCoordinator<Buffer, 2>, Output> for BoxSpaceInlineCellTextMazeExporter {
    fn export(&self, maze: &BoxSpaceInlineCellMazeCoordinator<Buffer, 2>, output: &mut Output) -> Result<()> {
        let [width, height] = nonzero_usize_array_to_usize_array(maze.coord_space().dimensions());

        // Below +1's: cause we're looking at walls not cells

        // Track where the walls are above the current row, so we know where to print horizontal walls
        let mut top_walls = vec![EdgeType::PASSAGE; height + 1];

        // Track relevant connections for wall intersections, so we know how to print them
        // [has_left_connection, has_top_connection]
        // (bottom and right can be worked out at print time)
        let mut wall_connections = vec![[EdgeType::PASSAGE; 2]; height + 1];

        for y in 0..height {
            // TODO can we get rid of these and just write to the output directly?
            // We export two lines simultaneously
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

                line_top_walls.push_str(&(match top_wall_actual {
                    EdgeType::WALL => "─",
                    EdgeType::BOUNDARY => "━",
                    EdgeType::PASSAGE => " ",
                    EdgeType::UNVISITED => "┄",
                }).repeat(self.chars_per_cell_horizontally.into()));

                line_side_walls.push(match left_wall_actual {
                    EdgeType::WALL => '│',
                    EdgeType::BOUNDARY => '┃',
                    EdgeType::PASSAGE => ' ',
                    EdgeType::UNVISITED => '┆',
                });

                line_side_walls.push_str(&" ".repeat(self.chars_per_cell_horizontally.into()));

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
            
            for _ in 0..(self.chars_per_cell_vertically.into()) {
                output.write_all(line_side_walls.as_bytes())?;
                output.write_line_break()?;
            }
        }

        // Draw the bottom side
        {
            let mut line: String = String::with_capacity(width * 2 + 1);

            for x in 0..width {
                let pt = CellID(x + (height - 1) * width);  // We're iterating over the last row

                let cell_value = maze.buffer().get(pt);

                let walls = cell_value.edges;

                line.push(Self::get_box_char(wall_connections[x][0], walls[0][0], walls[1][1], EdgeType::PASSAGE));

                line.push_str(&(match walls[1][1] {
                    EdgeType::WALL => "─",
                    EdgeType::BOUNDARY => "━",
                    EdgeType::PASSAGE => " ",
                    EdgeType::UNVISITED => "┄",
                }).repeat(self.chars_per_cell_horizontally.into()));

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

impl BoxSpaceInlineCellTextMazeExporter {
    // not recommended reading
    #[must_use]
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

impl <Buffer: MazeBuffer<InlineCellValue<2>>, Output: Write> TextMazeExporter<BoxSpaceInlineCellMazeCoordinator<Buffer, 2>, Output> for BoxSpaceInlineCellTextMazeExporter {}