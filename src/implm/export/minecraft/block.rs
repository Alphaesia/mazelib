use std::collections::HashMap;
use std::io::{Result, Write};
use std::time::SystemTime;

use crate::implm::cell::block::{BlockCellValue, BlockCellPrimaryValue};
use crate::implm::coordinate::block::BoxSpaceBlockCellMazeCoordinator;
use crate::implm::export::minecraft::{BoxSpaceSchematicMazeExporter, SchematicMazeExporter};
use crate::implm::export::minecraft::schem::{SpongeSchematicV3, SpongeSchematicV3BlockContainer, SpongeSchematicV3MetadataObject, SpongeSchematicV3SchematicObject};
use crate::interface::buffer::MazeBuffer;
use crate::interface::export::MazeExporter;
use crate::internal::util::nonzero_usize_array_to_usize_array;

impl <Buffer: MazeBuffer<BlockCellValue>, Output: Write> MazeExporter<BoxSpaceBlockCellMazeCoordinator<Buffer, 2>, Output> for BoxSpaceSchematicMazeExporter {
    fn export(&self, maze: &BoxSpaceBlockCellMazeCoordinator<Buffer, 2>, output: &mut Output) -> Result<()> {
        // Spec: https://github.com/SpongePowered/Schematic-Specification/blob/master/versions/schematic-3.md

        let [width, length] = nonzero_usize_array_to_usize_array(maze.get_full_dimensions()).map(|dim| TryInto::<u16>::try_into(dim).expect("Cannot export mazes with dimensions larger than u16"));

        let mut palette: HashMap<String, i32> = HashMap::new();

        const AIR_BLOCK: i8 = 0;
        const WALL_BLOCK: i8 = 1;
        const FLOOR_BLOCK: i8 = 2;

        // Any tool that lets you place schematics usually also lets you change the blocks
        // used, so our choices here don't matter
        palette.insert("minecraft:air".to_string(), AIR_BLOCK as i32);
        palette.insert("minecraft:deepslate_bricks".to_string(), WALL_BLOCK as i32);
        palette.insert("minecraft:snow_block".to_string(), FLOOR_BLOCK as i32);

        // Technically this is supposed to be an array of varints, but we only three blocks
        // so we can just pretend it's an array of bytes
        let mut data = vec![AIR_BLOCK; (width as usize) * (length as usize) * 4usize];

        let width_usize = width as usize;
        let length_usize = length as usize;

        for z in 0..length_usize {
            for x in 0..width_usize {
                match maze.get_cell_value([x, z].into()).cell_type {
                    BlockCellPrimaryValue::PASSAGE => {
                        // y = 0
                        data[x + z * width_usize] = FLOOR_BLOCK;
                    },
                    BlockCellPrimaryValue::WALL | BlockCellPrimaryValue::BOUNDARY => {
                        for y in 0..4 {
                            data[x + z * width_usize + y * width_usize * length_usize] = WALL_BLOCK;
                        }
                    },
                    BlockCellPrimaryValue::UNVISITED => {},
                }
            }
        }

        let schem = SpongeSchematicV3 {
            Schematic: SpongeSchematicV3SchematicObject {
                Version: 3,
                DataVersion: 2975, // 1.18.2
                Metadata: Some(SpongeSchematicV3MetadataObject {
                    Name: None,
                    Author: None,
                    Date: Some(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i64),
                    RequiredMods: None,
                }),
                Width: width as i16,
                Height: 4,
                Length: length as i16,
                Offset: None,
                Blocks: Some(SpongeSchematicV3BlockContainer {
                    Palette: palette,
                    Data: data,
                    BlockEntities: None,
                }),
                Biomes: None,
                Entities: None,
            }
        };

        return match nbt::to_gzip_writer(output, &schem, None) {
            Ok(_) => Ok(()),
            Err(err) => match err {
                nbt::Error::IoError(err) => Err(err),
                err => panic!("[Bug] Failed to serialise schematic: {}", err),
            }
        }
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>, Output: Write> SchematicMazeExporter<BoxSpaceBlockCellMazeCoordinator<Buffer, 2>, Output> for BoxSpaceSchematicMazeExporter {}