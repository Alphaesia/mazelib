use std::collections::HashMap;
use std::fs::File;
use std::time::SystemTime;
use crate::interface::buffer::MazeBuffer;
use crate::interface::render::MazeRenderer;
use crate::implm::cell::block::{BoxSpaceBlockCellManager, BlockCellValue};
use crate::implm::render::minecraft::{BoxSpaceSchematicMazeRenderer, SchematicMazeRenderer};
use crate::implm::render::minecraft::schem::{SpongeSchematicV3, SpongeSchematicV3SchematicObject, SpongeSchematicV3MetadataObject, SpongeSchematicV3BlockContainer};

impl <Buffer: MazeBuffer<BlockCellValue>> MazeRenderer<BoxSpaceBlockCellManager<Buffer, 2>> for BoxSpaceSchematicMazeRenderer {
    type Output = ();

    fn render(maze: &BoxSpaceBlockCellManager<Buffer, 2>) -> Self::Output {
        // Spec: https://github.com/SpongePowered/Schematic-Specification/blob/master/versions/schematic-3.md

        let [width, length] = maze.get_full_dimensions().map(|dim| TryInto::<u16>::try_into(dim).expect("Cannot render mazes with dimensions larger than u16"));

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
                match maze.get_cell([x, z].into()) {
                    BlockCellValue::PASSAGE => {
                        // y = 0
                        data[x + z * width_usize] = FLOOR_BLOCK;
                    },
                    BlockCellValue::WALL | BlockCellValue::BOUNDARY => {
                        for y in 0..4 {
                            data[x + z * width_usize + y * width_usize * length_usize] = WALL_BLOCK;
                        }
                    },
                    BlockCellValue::UNVISITED => {},
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

        // TODO make all renderers write to io::Writers
        match nbt::to_gzip_writer(&mut File::create("test.schem").unwrap(), &schem, None) {
            Ok(_) => (),
            Err(err) => { dbg!(err); panic!() }
        }
    }
}

impl <Buffer: MazeBuffer<BlockCellValue>> SchematicMazeRenderer<BoxSpaceBlockCellManager<Buffer, 2>> for BoxSpaceSchematicMazeRenderer {}