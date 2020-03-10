use crate::{
  enums::{
    TBool,
    TileEntityType,
  },
  models::Position,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct TileEntity {
  pub id: i32,
  pub position_x: i16,
  pub position_y: i16,
  pub tile_entity_type: TileEntityType,
}

impl SizeWith<TileEntity> for TileEntity {
  fn size_with(ctx: &Self) -> usize {
    u8::size_with(&LE) + // raw_type
    i32::size_with(&LE)
      + (i16::size_with(&LE) * 2)// position is i16, not standard i32
      + TileEntityType::size_with(&ctx.tile_entity_type)
  }
}

impl<'a> TryFromCtx<'a, Endian> for TileEntity {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let raw_type = buf.gread::<u8>(offset)?;
    Ok((
      Self {
        id: buf.gread_with::<i32>(offset, LE)?,
        position_x: buf.gread_with::<i16>(offset, LE)?,
        position_y: buf.gread_with::<i16>(offset, LE)?,
        tile_entity_type: buf.gread_with::<TileEntityType>(offset, raw_type)?,
      },
      *offset,
    ))
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
      position_x,
      position_y,
      tile_entity_type,
    } = self;
    let raw_type = tile_entity_type.raw_type();
    buf.gwrite(raw_type, offset)?;
    buf.gwrite_with(id, offset, LE)?;
    buf.gwrite_with(position_x, offset, LE)?;
    buf.gwrite_with(position_y, offset, LE)?;
    buf.gwrite(tile_entity_type, offset)?;

    let expected_size = TileEntity::size_with(self);
    assert!(
      *offset == expected_size,
      "TileEntity size mismatch; expected {:?}, got {:?}",
      expected_size,
      offset
    );

    Ok(*offset)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct TileEntities {
  pub count: i32,
  pub tile_entities: Vec<TileEntity>,
}

impl TileEntities {
  #[must_use]
  pub fn find_tile_entity_at_position(
    &self,
    position: Position,
  ) -> Option<&TileEntity> {
    let s = &self.tile_entities;
    s.iter().find(|tile_entity| {
      i32::from(tile_entity.position_x) == position.x
        && i32::from(tile_entity.position_y) == position.y
    })
  }
}

impl<'a> TryFromCtx<'a, Endian> for TileEntities {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let count = buf.gread_with::<i32>(offset, LE)?;
    Ok((
      Self {
        count,
        tile_entities: (0..count)
          .map(|_| buf.gread::<TileEntity>(offset))
          .collect::<Result<Vec<_>, Self::Error>>()?,
      },
      *offset,
    ))
  }
}

impl SizeWith<TileEntities> for TileEntities {
  fn size_with(ctx: &Self) -> usize {
    i32::size_with(&LE)
      + ctx
        .tile_entities
        .iter()
        .map(|tile_entity| TileEntity::size_with(tile_entity))
        .sum::<usize>()
  }
}

impl TryIntoCtx<Endian> for &TileEntities {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let TileEntities {
      count,
      tile_entities,
    } = self;
    buf.gwrite_with(count, offset, LE)?;
    tile_entities
      .iter()
      .map(|tile_entity| buf.gwrite(tile_entity, offset))
      .collect::<Result<Vec<_>, Self::Error>>()?;
    let expected_size = TileEntities::size_with(self);
    assert!(
      expected_size == *offset,
      "TileEntities offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );
    Ok(*offset)
  }
}

#[cfg(test)]
mod test_tile_entity {
  use super::{
    LogicSensor,
    Pread,
    Pwrite,
    TBool,
    TileEntity,
    TileEntityType,
  };
  use crate::{
    enums::ItemType,
    models::ItemStack,
  };

  #[test]
  fn test_tile_entity_with_target_dummy_rw() {
    let te1 = TileEntity {
      id: 123,
      position_x: 0,
      position_y: 0,
      tile_entity_type: TileEntityType::TargetDummy(1),
    };
    let mut buf = [0; 11];
    assert_eq!(11, buf.pwrite(&te1, 0).unwrap());
    assert_eq!(te1, buf.pread::<TileEntity>(0).unwrap());
  }

  #[test]
  fn test_tile_entity_with_logic_sensor_rw() {
    let te2 = TileEntity {
      id: 123,
      position_x: 0,
      position_y: 0,
      tile_entity_type: TileEntityType::LogicSensor(LogicSensor {
        logic_check: 1,
        enabled: TBool::True,
      }),
    };
    let mut buf = [0; 11];
    assert_eq!(11, buf.pwrite(&te2, 0).unwrap());
    assert_eq!(te2, buf.pread::<TileEntity>(0).unwrap());
  }

  #[test]
  fn test_tile_entity_with_item_frame_rw() {
    let te3 = TileEntity {
      id: 123,
      position_x: 0,
      position_y: 0,
      tile_entity_type: TileEntityType::ItemFrame(ItemStack {
        quantity: 1,
        item_type: Some(ItemType::Pwnhammer),
        modifier: Some(0),
      }),
    };
    let mut buf = [0; 16];
    assert_eq!(16, buf.pwrite(&te3, 0).unwrap());
    assert_eq!(te3, buf.pread::<TileEntity>(0).unwrap());
  }
}
