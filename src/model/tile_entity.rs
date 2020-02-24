use super::{
  common::*,
  items::ItemStack,
  tiles::TileMatrix,
};
use crate::{
  enums::EntityType,
  model::tiles::{
    Tile,
    TileVec,
  },
};
use scroll::{
  ctx::{
    SizeWith,
    TryFromCtx,
    TryIntoCtx,
  },
  Endian,
  Error as ScrollError,
  Pread,
  Pwrite,
  LE,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct LogicSensor {
  pub logic_check: u8,
  pub enabled: TBool,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct TileEntity {
  pub id: i32,
  pub position: Point,
  // TODO: these should be an enum
  pub target_dummy: Option<i16>,
  pub item_frame: Option<ItemStack>,
  pub logic_sensor: Option<LogicSensor>,
}

impl SizeWith<TileEntity> for TileEntity {
  fn size_with(ctx: &TileEntity) -> usize {
    i32::size_with(&LE)
      + Point::size_with(&LE)
      + ctx.target_dummy.map_or(0, |_| i16::size_with(&LE))
      + ctx
        .item_frame
        .map_or(0, |stack| ItemStack::size_with(&stack))
      + ctx.logic_sensor.map_or(0, |_| LogicSensor::size_with(&LE))
  }
}

impl<'a> TryFromCtx<'a, Endian> for TileEntity {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let tile_entity_type = buf.gread::<u8>(offset)?;
    let id = buf.gread_with::<i32>(offset, LE)?;
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
      1 => tile_entity.item_frame = Some(buf.gread::<ItemStack>(offset)?),
      2 => tile_entity.logic_sensor = Some(buf.gread::<LogicSensor>(offset)?),
      _ => {}
    };
    Ok((tile_entity, *offset))
  }
}

impl TryIntoCtx<Endian> for &TileEntity {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let TileEntity {
      id,
      position,
      target_dummy,
      item_frame,
      logic_sensor,
    } = self;
    let tile_entity_type: u8 = match item_frame {
      Some(_) => 1,
      _ => {
        match logic_sensor {
          Some(_) => 2,
          _ => 0,
        }
      }
    };
    buf.gwrite(tile_entity_type, offset)?;
    buf.gwrite_with(id, offset, LE)?;
    buf.gwrite_with(position.x as i16, offset, LE)?;
    buf.gwrite_with(position.y as i16, offset, LE)?;
    match tile_entity_type {
      0 => {
        buf.gwrite_with(target_dummy.unwrap(), offset, LE)?;
      }
      1 => {
        buf.gwrite(item_frame.unwrap(), offset)?;
      }
      2 => {
        buf.gwrite(logic_sensor.unwrap(), offset)?;
      }
      _ => {}
    }
    Ok(*offset)
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Pread)]
pub struct TileEntitiesInfo {
  pub count: i32,
}

impl SizeWith<TileMatrix> for TileEntitiesInfo {
  fn size_with(ctx: &TileMatrix) -> usize {
    let size = u32::size_with(&LE)
      + ctx
        .as_ref()
        .iter()
        .map(|tv| {
          tv.as_ref()
            .iter()
            .filter(|tile| tile.tile_entity.is_some())
            .map(|tile| {
              TileEntity::size_with(&tile.tile_entity.as_ref().unwrap())
            })
            .fold(0usize, |acc, len| acc + len)
        })
        .fold(0, |acc, len| acc + len);
    debug!("TileEntitiesInfo size: {}", size);
    size
  }
}

impl TryIntoCtx<&TileMatrix> for &TileEntitiesInfo {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: &TileMatrix,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    buf.gwrite(self.count, offset)?;
    let len = ctx.as_ref().len();
    for i in 0..len {
      let mut j = 0;
      let tv: &TileVec = &ctx[i];
      let tv_len = tv.as_ref().len();
      while j < tv_len {
        let tile: &Tile = &tv[j];
        match &tile.tile_entity {
          Some(tile_entity) => {
            buf.gwrite_with(tile_entity, offset, LE)?;
          }
          _ => {}
        };
        j += tile.run_length.length as usize;
      }
    }
    Ok(*offset)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TileEntityVec(Vec<TileEntity>, TileEntitiesInfo);

impl TileEntityVec {
  pub fn tile_entities_info(&self) -> TileEntitiesInfo {
    self.1
  }
}

impl<'a> TryFromCtx<'a, Endian> for TileEntityVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let tile_entity_info = buf.gread_with::<TileEntitiesInfo>(offset, LE)?;
    let mut tile_entities: Vec<TileEntity> = vec![];
    for _ in 0..tile_entity_info.count {
      let tile_entity = buf.gread::<TileEntity>(offset)?;
      tile_entities.push(tile_entity);
    }
    Ok((Self(tile_entities, tile_entity_info), *offset))
  }
}

impl TileEntityVec {
  pub fn move_to_tile(tile_entities: Self, tiles: &mut TileMatrix) {
    &tile_entities.0.into_iter().for_each(|tile_entity| {
      let mut tile = tiles.tile_at_point(&tile_entity.position);
      tile.tile_entity = Some(tile_entity);
    });
  }
}

// #[derive(Copy, Clone, Debug, PartialEq, Eq, AsRef)]
// pub struct PressurePlate(Point);

// impl<'a> TryFromCtx<'a, Endian> for PressurePlate {
//   type Error = ScrollError;

//   fn try_from_ctx(
//     buf: &'a [u8],
//     _: Endian,
//   ) -> Result<(Self, usize), Self::Error> {
//     let offset = &mut 0;
//     let point = buf.gread_with::<Point>(offset, LE)?;
//     Ok((Self(point), *offset))
//   }
// }

// impl TryIntoCtx<Endian> for PressurePlate {
//   type Error = ScrollError;

//   fn try_into_ctx(
//     self,
//     buf: &mut [u8],
//     _: Endian,
//   ) -> Result<usize, Self::Error> {
//     let offset = &mut 0;
//     let point = self.as_ref();
//     buf.gwrite(point, offset)?;
//     Ok(*offset)
//   }
// }

pub type PressurePlate = Point;

#[derive(Clone, Debug, Default, PartialEq, Eq, IntoIterator, AsRef)]
pub struct PressurePlateVec(Vec<PressurePlate>);

impl<'a> TryFromCtx<'a, ()> for PressurePlateVec {
  type Error = ScrollError;

  fn try_from_ctx(buf: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let pressure_plate_count = buf.gread_with::<i32>(offset, LE)?;
    let mut pressure_plates: Vec<PressurePlate> = vec![];
    for _ in 0..pressure_plate_count {
      let pressure_plate = buf.gread::<PressurePlate>(offset)?;
      pressure_plates.push(pressure_plate);
    }
    Ok((Self(pressure_plates), *offset))
  }
}

impl TryIntoCtx<Endian> for PressurePlateVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let vec = self.as_ref();
    let len = vec.len();
    buf.gwrite_with(len as i32, offset, LE)?;
    for i in 0..len {
      buf.gwrite(vec[i], offset)?;
    }
    Ok(*offset)
  }
}

impl PressurePlateVec {
  #[inline]
  pub fn move_to_tile(pressure_plates: Self, tiles: &mut TileMatrix) {
    pressure_plates.into_iter().for_each(|pressure_plate| {
      let mut tile = tiles.tile_at_point(&pressure_plate);
      tile.pressure_plate = Some(pressure_plate);
    });
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct Room {
  pub entity_type: EntityType,
  pub position: Point,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, IntoIterator, AsRef)]
pub struct RoomVec(Vec<Room>);

impl SizeWith<RoomVec> for RoomVec {
  fn size_with(ctx: &RoomVec) -> usize {
    let size =
      i32::size_with(&LE) + (ctx.as_ref().len() * Room::size_with(&LE));
    debug!("RoomVec size: {}", size);
    size
  }
}

impl<'a> TryFromCtx<'a, Endian> for RoomVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let room_count = buf.gread_with::<i32>(offset, LE)?;
    let mut rooms: Vec<Room> = vec![];
    for _ in 0..room_count {
      let room = buf.gread::<Room>(offset)?;
      rooms.push(room);
    }
    Ok((Self(rooms), *offset))
  }
}

impl TryIntoCtx<Endian> for &RoomVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let vec = self.as_ref();
    let len = vec.len();
    buf.gwrite_with(len as i32, offset, LE)?;
    for i in 0..len {
      buf.gwrite(vec[i], offset)?;
    }
    Ok(*offset)
  }
}

#[cfg(test)]
mod test_tile_entity {
  use super::*;
  use crate::enums::ItemType;

  #[test]
  fn test_tile_entity_rw() {
    let te1 = TileEntity {
      id: 123,
      position: Point { x: 0, y: 0 },
      target_dummy: Some(1),
      logic_sensor: None,
      item_frame: None,
    };
    let mut buf = [0; 11];
    assert_eq!(11, buf.pwrite(&te1, 0).unwrap());
    assert_eq!(te1, buf.pread::<TileEntity>(0).unwrap());

    let te2 = TileEntity {
      id: 123,
      position: Point { x: 0, y: 0 },
      target_dummy: None,
      logic_sensor: Some(LogicSensor {
        logic_check: 1,
        enabled: TBool::True,
      }),
      item_frame: None,
    };
    let mut buf = [0; 11];
    assert_eq!(11, buf.pwrite(&te2, 0).unwrap());
    assert_eq!(te2, buf.pread::<TileEntity>(0).unwrap());

    let te3 = TileEntity {
      id: 123,
      position: Point { x: 0, y: 0 },
      target_dummy: None,
      logic_sensor: None,
      item_frame: Some(ItemStack {
        quantity: 1,
        item_type: Some(ItemType::Pwnhammer),
        modifier: Some(0),
      }),
    };
    let mut buf = [0; 16];
    assert_eq!(16, buf.pwrite(&te3, 0).unwrap());
    assert_eq!(te3, buf.pread::<TileEntity>(0).unwrap());
  }

  #[test]
  fn test_pressure_plate_rw() {
    let pp = PressurePlate { x: 0, y: 0 };
    let mut buf = [0; 8];
    assert_eq!(8, buf.pwrite(pp, 0).unwrap());
    assert_eq!(&pp, &buf.pread::<PressurePlate>(0).unwrap());
  }

  #[test]
  fn test_pressure_plate_vec_rw() {
    let ppv = PressurePlateVec(vec![
      PressurePlate { x: 0, y: 0 },
      PressurePlate { x: 1, y: 1 },
    ]);
    let mut buf = [0; 20];
    assert_eq!(20, buf.pwrite(ppv.clone(), 0).unwrap());
    assert_eq!(ppv, buf.pread::<PressurePlateVec>(0).unwrap());
  }

  #[test]
  fn test_room_vec_rw() {
    let rv = RoomVec(vec![
      Room {
        entity_type: EntityType::Derpling,
        position: Point { x: 0, y: 0 },
      },
      Room {
        entity_type: EntityType::Herpling,
        position: Point { x: 2, y: 2 },
      },
    ]);
    let mut buf = [0; 28];
    assert_eq!(28, buf.pwrite(&rv, 0).unwrap());
    assert_eq!(rv, buf.pread::<RoomVec>(0).unwrap());
  }
}
