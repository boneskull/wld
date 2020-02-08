use derive_new::new;
use nano_leb128::ULEB128 as NanoUleb128;
use scroll::{
  ctx::{StrCtx, TryFromCtx, TryIntoCtx},
  Endian, Pread, Pwrite, Uleb128,
};
use std::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new, Pread, Pwrite)]
pub struct Rect {
  pub left: i32,
  pub right: i32,
  pub top: i32,
  pub bottom: i32,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new, Pread, Pwrite)]
pub struct Point {
  pub y: i32,
  pub x: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, new)]
pub struct TString(String);

impl<'a> TryFromCtx<'a, Endian> for TString {
  type Error = scroll::Error;

  fn try_from_ctx(buf: &'a [u8], _ctx: Endian) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let str_len = Uleb128::read(buf, offset)?;
    let value = buf.gread_with::<&str>(offset, StrCtx::Length(str_len.try_into().unwrap()))?;
    Ok((Self(value.to_string()), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TString {
  type Error = scroll::Error;

  fn try_into_ctx(self, buf: &mut [u8], _ctx: Endian) -> Result<usize, Self::Error> {
    let value = &self.0;
    let mut size = 0;
    let str_len = match u64::try_from(value.len()) {
      Ok(l) => l,
      Err(e) => return Err(scroll::Error::Custom(format!("{:?}", e))),
    };
    size += match NanoUleb128::from(str_len).write_into(buf) {
      Ok(s) => s,
      Err(e) => return Err(scroll::Error::Custom(format!("{:?}", e))),
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct TBool(bool);

impl From<bool> for TBool {
  fn from(v: bool) -> Self {
    Self(v)
  }
}

impl<'a> TryFromCtx<'a, Endian> for TBool {
  type Error = scroll::Error;

  fn try_from_ctx(buf: &'a [u8], _ctx: Endian) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread::<u8>(offset)? != 0;
    Ok((Self(value), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TBool {
  type Error = scroll::Error;

  fn try_into_ctx(self, buf: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
    let mut size = 0;
    let value: bool = self.0;
    size += if value { 1u8 } else { 0u8 }.try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}

#[cfg(test)]
mod test_common {
  use super::*;
  use scroll::LE;

  #[test]
  fn test_tbool_rw() {
    let t = &TBool(true);
    let mut bytes = [0; 1];
    let _res = bytes.pwrite_with::<&TBool>(t, 0, LE).unwrap();
    assert_eq!(
      TBool::try_from_ctx(&bytes[..], LE).unwrap(),
      (TBool(true), 1)
    );

    let t = &TBool(false);
    let mut bytes = [0; 1];
    let _res = bytes.pwrite_with::<&TBool>(t, 0, LE).unwrap();
    assert_eq!(
      TBool::try_from_ctx(&bytes[..], LE).unwrap(),
      (TBool(false), 1)
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
