//! Minecraft-related export formats.
#![cfg(any(feature = "minecraft", doc))]

use std::io::Write;

use crate::interface::coordinate::MazeCoordinator;
use crate::interface::export::MazeExporter;

mod block;
mod schem;

/// Export a 2D maze into a Minecraft schematic.
///
/// Specifically, it exports the maze to a
/// [Sponge Schematic (v3)](https://github.com/SpongePowered/Schematic-Specification/blob/master/versions/schematic-3.md),
/// and returns it as a gzipped serialised NBT byte array. Note that schematics cannot be used
/// directly with the vanilla game, and must be imported with mods or 3rd-party tools (such as
/// [WorldEdit](https://enginehub.org/worldedit/)).
///
/// The blocks used cannot be changed, however most programs that let you import schematics
/// also let you replace blocks of one type with another. The vanilla game also supports this
/// on small areas with [`/fill <from> <to> replace <block>`](https://minecraft.fandom.com/wiki/Commands/fill).
pub trait SchematicMazeExporter<M: MazeCoordinator, O: Write> : MazeExporter<M, O> {}

/// A [`SchematicMazeExporter`] for mazes that
/// use [`BoxCoordinateSpace`][crate::implm::point::boxy::BoxCoordinateSpace]s.
pub struct BoxSpaceSchematicMazeExporter {
    _private: ()
}

impl BoxSpaceSchematicMazeExporter {
    /// Construct a new instance.
    ///
    /// Optional, see [`DefaultMazeExporter`][crate::interface::export::DefaultMazeExporter].
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }
}