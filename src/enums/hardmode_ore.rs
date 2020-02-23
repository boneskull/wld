use num_traits::FromPrimitive;
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum HardmodeOre {
  UnknownOre = -1,
  CobaltOre = 107,
  MythrilOre = 108,
  AdamantiteOre = 111,
  PalladiumOre = 221,
  OrichalcumOre = 222,
  TitaniumOre = 223,
}

impl SizeWith<Endian> for HardmodeOre {
  fn size_with(_: &Endian) -> usize {
    i32::size_with(&LE)
  }
}

impl<'a> TryFromCtx<'a, Endian> for HardmodeOre {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread_with::<i32>(offset, LE)?;
    let ore = match Self::from_i32(value) {
      Some(o) => o,
      _ => Self::UnknownOre,
    };
    Ok((ore, *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a HardmodeOre {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    buf.gwrite_with(*self as i32, offset, LE)?;
    Ok(*offset)
  }
}
