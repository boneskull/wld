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
  LE,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum InvasionType {
  NoInvasion = 0,
  GoblinInvasion = 1,
  FrostLegion = 2,
  PirateInvasion = 3,
  MartianMadness = 4,
}

impl SizeWith<Endian> for InvasionType {
  fn size_with(_: &Endian) -> usize {
    i32::size_with(&LE)
  }
}

impl<'a> TryFromCtx<'a, Endian> for InvasionType {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread_with::<i32>(offset, LE)?;
    let invasion_type = match Self::from_i32(value) {
      Some(it) => it,
      _ => Self::NoInvasion,
    };
    Ok((invasion_type, *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a InvasionType {
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
