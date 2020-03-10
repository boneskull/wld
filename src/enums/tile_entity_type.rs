use crate::models::{
  ItemStack,
  LogicSensor,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum TileEntityType {
  TargetDummy(i16),
  ItemFrame(ItemStack),
  LogicSensor(LogicSensor),
}

impl TileEntityType {
  #[must_use]
  pub fn raw_type(&self) -> u8 {
    match self {
      Self::TargetDummy(_) => 0,
      Self::ItemFrame(_) => 1,
      Self::LogicSensor(_) => 2,
    }
  }
}

impl SizeWith<TileEntityType> for TileEntityType {
  fn size_with(ctx: &Self) -> usize {
    match ctx {
      Self::TargetDummy(_) => i16::size_with(&LE),
      Self::ItemFrame(stack) => ItemStack::size_with(stack),
      Self::LogicSensor(_) => LogicSensor::size_with(&LE),
    }
  }
}

impl<'a> TryFromCtx<'a, u8> for TileEntityType {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    raw_type: u8,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = match raw_type {
      0 => Self::TargetDummy(buf.gread_with::<i16>(offset, LE)?),
      1 => Self::ItemFrame(buf.gread::<ItemStack>(offset)?),
      2 => Self::LogicSensor(buf.gread::<LogicSensor>(offset)?),
      _ => {
        return Err(Self::Error::Custom(
          "unrecognized tile entity type!".to_owned(),
        ))
      }
    };
    Ok((value, *offset))
  }
}

impl TryIntoCtx<Endian> for &TileEntityType {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    match self {
      TileEntityType::TargetDummy(value) => {
        buf.gwrite_with(value, offset, LE)?
      }
      TileEntityType::ItemFrame(frame) => buf.gwrite(frame, offset)?,
      TileEntityType::LogicSensor(sensor) => buf.gwrite(sensor, offset)?,
    };
    let expected_size = TileEntityType::size_with(self);
    assert!(
      expected_size == *offset,
      "TileEntityType offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );

    Ok(*offset)
  }
}
