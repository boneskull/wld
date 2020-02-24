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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum EvilType {
  Corruption,
  Crimson,
}

impl SizeWith<Endian> for EvilType {
  fn size_with(_: &Endian) -> usize {
    u8::size_with(&LE)
  }
}

impl<'a> TryFromCtx<'a, Endian> for EvilType {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let raw_value = buf.gread::<u8>(offset)?;
    Ok((
      if raw_value != 0 {
        Self::Crimson
      } else {
        Self::Corruption
      },
      *offset,
    ))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a EvilType {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let mut size = 0;
    let value = *self as u8;
    size += value.try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}
