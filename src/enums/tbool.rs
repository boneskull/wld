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
pub enum TBool {
  False,
  True,
}

impl SizeWith<Endian> for TBool {
  fn size_with(_: &Endian) -> usize {
    u8::size_with(&LE)
  }
}

impl<'a> TryFromCtx<'a, Endian> for TBool {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread::<u8>(offset)?;
    Ok((if value == 0 { Self::False } else { Self::True }, *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TBool {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    buf.gwrite(if *self == TBool::True { 1_u8 } else { 0_u8 }, offset)?;
    let expected_size = TBool::size_with(&LE);
    assert!(
      expected_size == *offset,
      "TBool offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );

    Ok(*offset)
  }
}

#[cfg(test)]
mod test_tbool {
  use super::{
    Pread,
    Pwrite,
    TBool,
  };
  #[test]
  fn test_tbool_true_rw() {
    let t = &TBool::True;
    let mut bytes = [0; 1];
    let _res = bytes.pwrite::<&TBool>(t, 0).unwrap();
    assert_eq!(bytes.pread::<TBool>(0).unwrap(), TBool::True);
  }

  #[test]
  fn test_tbool_false_rw() {
    let t = &TBool::False;
    let mut bytes = [0; 1];
    let _res = bytes.pwrite::<&TBool>(t, 0).unwrap();
    assert_eq!(bytes.pread::<TBool>(0).unwrap(), TBool::False);
  }
}
