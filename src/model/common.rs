use bitvec::prelude::*;
use nano_leb128::ULEB128 as NanoUleb128;
use scroll::{
  ctx::{
    StrCtx,
    TryFromCtx,
    TryIntoCtx,
  },
  Endian,
  Error as ScrollError,
  Pread,
  Pwrite,
  Uleb128,
  LE,
};
use std::convert::{
  TryFrom,
  TryInto,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Pread, Pwrite)]
pub struct Rect {
  pub left: i32,
  pub right: i32,
  pub top: i32,
  pub bottom: i32,
}

#[derive(
  Copy, Clone, Debug, Default, PartialEq, Eq, Pread, Pwrite, Constructor,
)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, AsRef)]
pub struct TString(String);

impl<'a> TryFromCtx<'a, Endian> for TString {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let str_len = Uleb128::read(buf, offset)?;
    let value =
      buf.gread_with::<&str>(offset, StrCtx::Length(str_len as usize))?;
    Ok((Self(value.to_string()), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TString {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let value = self.as_ref();
    let mut size = 0;
    let str_len = match u64::try_from(value.len()) {
      Ok(l) => l,
      Err(e) => return Err(ScrollError::Custom(format!("{:?}", e))),
    };
    size += match NanoUleb128::from(str_len).write_into(buf) {
      Ok(s) => s,
      Err(e) => return Err(ScrollError::Custom(format!("{:?}", e))),
    };
    size += value.as_bytes().try_into_ctx(&mut buf[size..], ())?;
    Ok(size)
  }
}

impl From<&str> for TString {
  fn from(s: &str) -> Self {
    Self(s.to_string())
  }
}

impl From<String> for TString {
  fn from(s: String) -> Self {
    Self(s)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TBool {
  False,
  True,
}

impl<'a> TryFromCtx<'a, Endian> for TBool {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread::<u8>(offset)?;
    Ok((
      if value == 0 {
        TBool::False
      } else {
        TBool::True
      },
      *offset,
    ))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TBool {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let mut size = 0;
    buf.gwrite(if *self == TBool::True { 1u8 } else { 0u8 }, &mut size)?;
    Ok(size)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, From, Index, AsRef)]
pub struct VariableTBitVec(BitVec<Lsb0, u8>);

impl From<Vec<bool>> for VariableTBitVec {
  fn from(v: Vec<bool>) -> Self {
    Self(BitVec::<Lsb0, u8>::from(&v[..]))
  }
}

impl<'a> TryFromCtx<'a, Endian> for VariableTBitVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let len = buf.gread_with::<i16>(offset, LE)?;
    let byte_len = (len as f32 / 8.0).ceil() as usize;
    let bits =
      BitVec::<Lsb0, u8>::from_slice(&buf[*offset..*offset + byte_len]);
    *offset += byte_len;
    Ok((Self(bits), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a VariableTBitVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let bits = self.as_ref();
    let mut size = 0;
    let tfi_size: i16 = bits.len().try_into().unwrap();
    size += tfi_size.try_into_ctx(&mut buf[size..], ctx)?;
    size += bits.as_slice().try_into_ctx(&mut buf[size..], ())?;
    Ok(size)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, From, Index, AsRef, AsMut)]
pub struct TBitVec(BitVec<Lsb0, u8>);

impl From<Vec<bool>> for TBitVec {
  fn from(v: Vec<bool>) -> Self {
    Self(BitVec::<Lsb0, u8>::from(&v[..]))
  }
}

impl<'a> TryFromCtx<'a, Endian> for TBitVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let bits = BitVec::<Lsb0, u8>::from_slice(&buf[*offset..*offset + 1]);
    *offset += 1;
    Ok((Self(bits), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TBitVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let bits = self.as_ref();
    let mut size = 0;
    size += bits.as_slice().try_into_ctx(&mut buf[size..], ())?;
    Ok(size)
  }
}

#[cfg(test)]
mod test_common {
  use super::*;
  use scroll::LE;
  #[test]
  fn test_tbool_rw() {
    let t = &TBool::True;
    let mut bytes = [0; 1];
    let _res = bytes.pwrite_with::<&TBool>(t, 0, LE).unwrap();
    assert_eq!(
      TBool::try_from_ctx(&bytes[..], LE).unwrap(),
      (TBool::True, 1)
    );

    let t = &TBool::False;
    let mut bytes = [0; 1];
    let _res = bytes.pwrite_with::<&TBool>(t, 0, LE).unwrap();
    assert_eq!(
      TBool::try_from_ctx(&bytes[..], LE).unwrap(),
      (TBool::False, 1)
    );
  }

  #[test]
  fn test_tstring_rw() {
    let t = &TString("foo".to_string());
    let mut bytes = [0; 4];
    let _res = bytes.pwrite_with::<&TString>(t, 0, LE).unwrap();
    assert_eq!(
      TString::try_from_ctx(&bytes[..], LE).unwrap(),
      (TString("foo".to_string()), 4)
    );
  }
}
