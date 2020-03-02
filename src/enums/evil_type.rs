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
      if raw_value == 0 {
        Self::Corruption
      } else {
        Self::Crimson
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
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let value = *self as u8;
    buf.gwrite_with(value, offset, LE)?;
    assert!(
      *offset == EvilType::size_with(&LE),
      "EvilType size mismatch"
    );
    Ok(*offset)
  }
}
