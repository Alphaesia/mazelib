extern crate serde;
use std::collections::HashMap;
use serde::Serialize;

#[allow(non_snake_case)] // using the spec's names
#[derive(Serialize)]
pub(super) struct SpongeSchematicV3 {
    pub(super) Schematic: SpongeSchematicV3SchematicObject
}

#[allow(non_snake_case)] // using the spec's names
#[derive(Serialize)]
pub(super) struct SpongeSchematicV3SchematicObject {
    pub(super) Version: i32,
    pub(super) DataVersion: i32,
    pub(super) Metadata: Option<SpongeSchematicV3MetadataObject>,
    pub(super) Width: i16,
    pub(super) Height: i16,
    pub(super) Length: i16,
    #[serde(serialize_with="opt_i32_array")]
    pub(super) Offset: Option<[i32; 3]>,
    pub(super) Blocks: Option<SpongeSchematicV3BlockContainer>,
    pub(super) Biomes: Option<SpongeSchematicV3BiomeContainer>,
    pub(super) Entities: Option<Vec<SpongeSchematicV3EntityObject>>,
}

#[allow(non_snake_case)] // using the spec's names
#[derive(Serialize)]
pub(super) struct SpongeSchematicV3MetadataObject {
    pub(super) Name: Option<String>,
    pub(super) Author: Option<String>,
    pub(super) Date: Option<i64>,
    pub(super) RequiredMods: Option<Vec<String>>,
}

#[allow(non_snake_case)] // using the spec's names
#[derive(Serialize)]
pub(super) struct SpongeSchematicV3BlockContainer {
    pub(super) Palette: HashMap<String, i32>,
    #[serde(serialize_with="nbt::i8_array")]
    pub(super) Data: Vec<i8>,
    pub(super) BlockEntities: Option<Vec<SpongeSchematicV3BlockEntityObject>>,
}

#[allow(non_snake_case)] // using the spec's names
#[derive(Serialize)]
pub(super) struct SpongeSchematicV3BiomeContainer {
    pub(super) Palette: HashMap<String, i32>,
    #[serde(serialize_with="nbt::i8_array")]
    pub(super) Data: Vec<i8>,
}

#[allow(non_snake_case)] // using the spec's names
#[derive(Serialize)]
pub(super) struct SpongeSchematicV3BlockEntityObject {
    #[serde(serialize_with="nbt::i32_array")]
    pub(super) Pos: [i32; 3],
    pub(super) Id: String,
    pub(super) Data: Option<nbt::Map<String, nbt::Value>>
}

#[allow(non_snake_case)] // using the spec's names
#[derive(Serialize)]
pub(super) struct SpongeSchematicV3EntityObject {
    pub(super) Pos: [f64; 3],
    pub(super) Id: String,
    pub(super) Data: Option<nbt::Map<String, nbt::Value>>
}

fn opt_i32_array<T, S>(array: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: IntoIterator + Clone,
        <T as IntoIterator>::Item: std::borrow::Borrow<i32>,
        S: serde::ser::Serializer,
{
    return if let Some(array) = array {
        nbt::i32_array((*array).clone(), serializer)
    } else {
        // TODO figure out a proper way to return a dummy value
        serializer.collect_map(std::iter::empty::<((), ())>())
    }
}