use super::{
  common::*,
  items::ItemStack,
  tiles::TileMatrix,
};
use crate::enums::EntityType;
use scroll::{
  ctx::{
    TryFromCtx,
    TryIntoCtx,
  },
  Endian,
  Error as ScrollError,
  Pread,
  Pwrite,
  LE,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
pub struct LogicSensor {
  pub logic_check: u8,
  pub enabled: TBool,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct TileEntity {
  pub id: i32,
  pub position: Point,
  // TODO: these should be an enum
  pub target_dummy: Option<i16>,
  pub item_frame: Option<ItemStack>,
  pub logic_sensor: Option<LogicSensor>,
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

impl TryIntoCtx<Endian> for TileEntity {
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

#[derive(Clone, Debug, Default, PartialEq, Eq, IntoIterator, AsRef)]
pub struct TileEntityVec(Vec<TileEntity>);

impl<'a> TryFromCtx<'a, Endian> for TileEntityVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let tile_entity_count = buf.gread_with::<i32>(offset, LE)?;
    let mut tile_entities: Vec<TileEntity> = vec![];
    for _ in 0..tile_entity_count {
      let tile_entity = buf.gread::<TileEntity>(offset)?;
      tile_entities.push(tile_entity);
    }
    Ok((Self(tile_entities), *offset))
  }
}

impl TryIntoCtx<Endian> for TileEntityVec {
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

impl TileEntityVec {
  #[inline]
  pub fn assign_to_tile(tile_entities: Self, tiles: &mut TileMatrix) {
    tile_entities.into_iter().for_each(|tile_entity| {
      let mut tile = tiles.tile_at_point(&tile_entity.position);
      tile.tile_entity = Some(tile_entity);
    });
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, AsRef)]
pub struct PressurePlate(Point);

impl<'a> TryFromCtx<'a, Endian> for PressurePlate {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let point = buf.gread_with::<Point>(offset, LE)?;
    Ok((Self(point), *offset))
  }
}

impl TryIntoCtx<Endian> for PressurePlate {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let point = self.as_ref();
    buf.gwrite(point, offset)?;
    Ok(*offset)
  }
}

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
  pub fn assign_to_tile(pressure_plates: Self, tiles: &mut TileMatrix) {
    pressure_plates.into_iter().for_each(|pressure_plate| {
      let mut tile = tiles.tile_at_point(pressure_plate.as_ref());
      tile.pressure_plate = Some(pressure_plate);
    });
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
pub struct Room {
  pub entity_type: EntityType,
  pub position: Point,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, IntoIterator, AsRef)]
pub struct RoomVec(Vec<Room>);

impl<'a> TryFromCtx<'a, ()> for RoomVec {
  type Error = ScrollError;

  fn try_from_ctx(buf: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
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

impl TryIntoCtx<Endian> for RoomVec {
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
    assert_eq!(11, buf.pwrite(te1, 0).unwrap());
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
    assert_eq!(11, buf.pwrite(te2, 0).unwrap());
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
    assert_eq!(16, buf.pwrite(te3, 0).unwrap());
    assert_eq!(te3, buf.pread::<TileEntity>(0).unwrap());
  }

  #[test]
  fn test_tile_entity_vec_rw() {
    let tev = TileEntityVec(vec![
      TileEntity {
        id: 123,
        position: Point { x: 0, y: 0 },
        target_dummy: Some(1),
        logic_sensor: None,
        item_frame: None,
      },
      TileEntity {
        id: 123,
        position: Point { x: 0, y: 0 },
        target_dummy: None,
        logic_sensor: None,
        item_frame: Some(ItemStack {
          quantity: 1,
          item_type: Some(ItemType::Pwnhammer),
          modifier: Some(0),
        }),
      },
    ]);

    let mut buf = [0; 31];
    assert_eq!(31, buf.pwrite(tev.clone(), 0).unwrap());
    assert_eq!(tev, buf.pread::<TileEntityVec>(0).unwrap());
  }

  #[test]
  fn test_pressure_plate_rw() {
    let pp = PressurePlate(Point { x: 0, y: 0 });
    let mut buf = [0; 8];
    assert_eq!(8, buf.pwrite(pp.clone(), 0).unwrap());
    assert_eq!(pp, buf.pread::<PressurePlate>(0).unwrap());
  }

  #[test]
  fn test_pressure_plate_vec_rw() {
    let ppv = PressurePlateVec(vec![
      PressurePlate(Point { x: 0, y: 0 }),
      PressurePlate(Point { x: 1, y: 1 }),
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
    assert_eq!(28, buf.pwrite(rv.clone(), 0).unwrap());
    assert_eq!(rv, buf.pread::<RoomVec>(0).unwrap());
  }
}
