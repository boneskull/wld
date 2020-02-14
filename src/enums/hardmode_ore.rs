use num_traits::FromPrimitive;
use scroll::{
  ctx::{
    TryFromCtx,
    TryIntoCtx,
  },
  Endian,
  Error as ScrollError,
  Pread,
  LE,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum HardmodeOre {
  UnknownOre = -1,
  CobaltOre = 107,
  MythrilOre = 108,
  AdamantiteOre = 111,
  PalladiumOre = 221,
  OrichalcumOre = 222,
  TitaniumOre = 223,
}

impl<'a> TryFromCtx<'a, Endian> for HardmodeOre {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread_with::<i32>(offset, LE)?;
    let ore_opt = Self::from_i32(value);
    Ok((
      if ore_opt.is_none() {
        Self::UnknownOre
      } else {
        ore_opt.unwrap()
      },
      *offset,
    ))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a HardmodeOre {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let mut size = 0;
    let value = *self as i32;
    size += value.try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}
