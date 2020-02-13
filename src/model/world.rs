use super::header::*;
use crate::model::{
  items::*,
  npc::*,
  properties::Properties,
  status::Status,
  tile_entity::*,
  tiles::*,
};
use scroll::{
  Pread,
  Pwrite,
  LE,
};

#[derive(Clone, Debug, PartialEq)]
pub struct World {
  pub status: WorldStatus,
  pub tiles: Tiles,
  pub npcs: NPCVec,
  pub mobs: MobVec,
}

#[derive(Clone, Debug, PartialEq, Pwrite, Pread)]
pub struct WorldStatus {
  pub header: Header,
  pub properties: Properties,
  pub status: Status,
}

impl World {
  #[inline]
  pub fn read(bytes: &[u8]) -> Result<World, scroll::Error> {
    let offset = &mut 0;
    let status = bytes.gread_with::<WorldStatus>(offset, LE)?;
    let mut tiles = bytes
      .gread_with::<Tiles>(offset, status.properties.as_tiles_context())?;
    let chests = bytes.gread_with::<Chests>(offset, LE)?;
    Chests::assign_to_tile(chests, &mut tiles);
    let signs = bytes.gread_with::<Signs>(offset, LE)?;
    Signs::assign_to_tile(signs, &mut tiles);

    let npcs = bytes.gread::<NPCVec>(offset)?;
    let mobs = bytes.gread::<MobVec>(offset)?;
    let tile_entities = bytes.gread::<TileEntityVec>(offset)?;
    TileEntityVec::assign_to_tile(tile_entities, &mut tiles);
    Ok(World {
      status,
      tiles,
      npcs,
      mobs,
    })
  }
}
