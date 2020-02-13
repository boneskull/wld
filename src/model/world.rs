use super::header::*;
use crate::model::{
  common::{
    TBool,
    TString,
  },
  items::*,
  npc::*,
  properties::Properties,
  status::Status,
  tile_entity::*,
  tiles::*,
};
use scroll::{
  ctx::TryFromCtx,
  Error as ScrollError,
  Pread,
  Pwrite,
  LE,
};

#[derive(Clone, Debug, PartialEq)]
pub struct World {
  pub status: WorldStatus,
  pub tiles: TileMatrix,
  pub npcs: NPCVec,
  pub mobs: MobVec,
  pub rooms: RoomVec,
  pub footer: Footer,
}

#[derive(Clone, Debug, PartialEq, Pwrite, Pread)]
pub struct WorldStatus {
  pub header: Header,
  pub properties: Properties,
  pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Footer {
  pub name: TString,
  pub id: i32,
}

impl<'a> TryFromCtx<'a, WorldCtx<'a>> for Footer {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: WorldCtx<'a>,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let is_valid = buf.gread::<TBool>(offset)?;
    if is_valid == TBool::False {
      return Err(ScrollError::Custom("invalid footer".to_string()));
    }
    let name = buf.gread::<TString>(offset)?;
    if name != *ctx.name {
      return Err(ScrollError::Custom(format!(
        "invalid footer: name mismatch ({:?} vs. {:?})",
        name, *ctx.name
      )));
    }
    let id = buf.gread_with::<i32>(offset, LE)?;
    if id != *ctx.id {
      return Err(ScrollError::Custom(
        "invalid footer: id mismatch".to_string(),
      ));
    }
    Ok((Self { id, name }, *offset))
  }
}

impl World {
  #[inline]
  pub fn read(bytes: &[u8]) -> Result<World, scroll::Error> {
    let offset = &mut 0;
    // order matters
    let status = bytes.gread::<WorldStatus>(offset)?;

    // need this context to associate various bits with other bits
    let world_ctx = status.properties.as_world_context();

    let mut tiles = bytes.gread_with::<TileMatrix>(offset, world_ctx)?;
    let chests = bytes.gread::<ChestVec>(offset)?;
    ChestVec::assign_to_tile(chests, &mut tiles);
    let signs = bytes.gread::<SignVec>(offset)?;
    SignVec::assign_to_tile(signs, &mut tiles);

    let npcs = bytes.gread::<NPCVec>(offset)?;
    let mobs = bytes.gread::<MobVec>(offset)?;
    let tile_entities = bytes.gread::<TileEntityVec>(offset)?;
    TileEntityVec::assign_to_tile(tile_entities, &mut tiles);
    let rooms = bytes.gread::<RoomVec>(offset)?;
    // footer is essentially useless, but needed on output and performs
    // assertions
    let mut offset = status.header.offsets.footer as usize;
    let footer = bytes.gread_with::<Footer>(&mut offset, world_ctx)?;
    Ok(World {
      status,
      tiles,
      npcs,
      mobs,
      rooms,
      footer,
    })
  }
}
