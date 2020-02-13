use super::{
  common::*,
  items::ItemStack,
  tiles::Tiles,
};
use scroll::{
  ctx::TryFromCtx,
  Error as ScrollError,
  Pread,
  LE,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LogicSensor {
  pub logic_check: u8,
  pub enabled: TBool,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct TileEntity {
  pub id: i64,
  pub position: Point,
  pub target_dummy: Option<i16>,
  pub item_frame: Option<ItemStack>,
  pub logic_sensor: Option<LogicSensor>,
}

impl<'a> TryFromCtx<'a, ()> for TileEntity {
  type Error = ScrollError;

  fn try_from_ctx(buf: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let tile_entity_type = buf.gread::<u8>(offset)?;
    let id = buf.gread_with::<i64>(offset, LE)?;
    let position = Point {
      x: buf.gread_with::<i16>(offset, LE)? as i32,
      y: buf.gread_with::<i16>(offset, LE)? as i32,
    };
    let mut tile_entity = TileEntity {
      id,
      position,
      target_dummy: None,
      item_frame: None,
      logic_sensor: None,
    };
    match tile_entity_type {
      0 => tile_entity.target_dummy = Some(buf.gread_with::<i16>(offset, LE)?),
      1 => {
        tile_entity.item_frame = Some(buf.gread_with::<ItemStack>(offset, LE)?)
      }
      2 => {
        tile_entity.logic_sensor = Some(LogicSensor {
          logic_check: buf.gread::<u8>(offset)?,
          enabled: buf.gread_with::<TBool>(offset, LE)?,
        })
      }
      _ => {}
    };
    Ok((tile_entity, *offset))
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, IntoIterator)]
pub struct TileEntityVec(Vec<TileEntity>);

impl<'a> TryFromCtx<'a, ()> for TileEntityVec {
  type Error = ScrollError;

  fn try_from_ctx(buf: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let tile_entity_count = buf.gread_with::<i64>(offset, LE)?;
    let mut tile_entities: Vec<TileEntity> = vec![];
    for _ in 0..tile_entity_count {
      let tile_entity = buf.gread::<TileEntity>(offset)?;
      tile_entities.push(tile_entity);
    }
    Ok((Self(tile_entities), *offset))
  }
}

impl TileEntityVec {
  #[inline]
  pub fn assign_to_tile(tile_entities: Self, tiles: &mut Tiles) {
    tile_entities.into_iter().for_each(|tile_entity| {
      let mut tile = tiles.tile_at_point(&tile_entity.position);
      tile.tile_entity = Some(tile_entity);
    });
  }
}
