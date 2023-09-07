#![cfg(any(feature = "minecraft", doc))]

use crate::interface::coordinator::MazeCoordinator;
use crate::interface::render::MazeRenderer;

mod block;
mod schem;

/// Render a two-dimensional maze (i.e. `BoxCoordinateSpace<2>`) into a Minecraft schematic
/// and return it.
///
/// Specifically, it renders the maze into a
/// [Sponge Schematic (v3)](https://github.com/SpongePowered/Schematic-Specification/blob/master/versions/schematic-3.md),
/// and returns it as a gzipped serialised NBT byte array. Note that schematics cannot be used
/// directly with the vanilla game, and must be imported with mods or 3rd-party tools (such as
/// [WorldEdit](https://enginehub.org/worldedit/)).
///
/// The blocks used cannot be changed, however most programs that let you import schematics
/// also let you replace blocks of one type with another. The vanilla game also supports this
/// on small areas with [`/fill <from> <to> replace <block>`](https://minecraft.fandom.com/wiki/Commands/fill).
pub struct BoxSpaceSchematicMazeRenderer {
    _private: ()
}

impl BoxSpaceSchematicMazeRenderer {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

pub trait SchematicMazeRenderer<M: MazeCoordinator> : MazeRenderer<M> {}