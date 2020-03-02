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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct HardmodeOre {
  pub raw_value: i32,
  pub ore_type: HardmodeOreType,
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
    let raw_value = buf.gread_with::<i32>(offset, LE)?;
    let ore_type = match HardmodeOreType::from_i32(raw_value) {
      Some(o) => o,
      _ => HardmodeOreType::UnknownOre,
    };
    Ok((
      Self {
        ore_type,
        raw_value,
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
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    buf.gwrite_with(&self.raw_value, offset, LE)?;
    assert!(
      *offset == HardmodeOre::size_with(&LE),
      "HardmodeOre size mismatch"
    );
    Ok(*offset)
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum HardmodeOreType {
  UnknownOre,
  Cobalt = 107,
  Mythril = 108,
  Adamantite = 111,
  Palladium = 221,
  Orichalcum = 222,
  Titanium = 223,
}
