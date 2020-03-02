use crate::model::{
  tiles::{
    ExtendedTileAttributes,
    TileAttributes,
  },
  TileHeader,
};
use bitvec::prelude::*;
use nano_leb128::ULEB128 as NanoUleb128;
use scroll::{
  ctx::{
    SizeWith,
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
use std::{
  convert::TryFrom,
  ops::Index,
};

#[derive(
  Copy, Clone, Debug, Default, PartialEq, Eq, Pread, Pwrite, SizeWith,
)]
#[repr(C)]
pub struct Rect {
  pub left: i32,
  pub right: i32,
  pub top: i32,
  pub bottom: i32,
}

#[derive(
  Copy,
  Clone,
  Debug,
  Default,
  PartialEq,
  Eq,
  Pread,
  Pwrite,
  Constructor,
  SizeWith,
)]
#[repr(C)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct TString(String, usize);

impl SizeWith<TString> for TString {
  fn size_with(ctx: &Self) -> usize {
    (u8::size_with(&LE) * ctx.1) + (ctx.0.len() * u8::size_with(&LE))
  }
}

impl<'a> TryFromCtx<'a, Endian> for TString {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let str_len = buf.gread::<Uleb128>(offset)?;
    let count = str_len.size();
    let str_len = *str_len.as_ref() as usize;
    let value = buf.gread_with::<&str>(offset, StrCtx::Length(str_len))?;
    Ok((Self(value.to_string(), count), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TString {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let value = &self.0;

    let str_len = match u64::try_from(value.len()) {
      Ok(l) => l,
      Err(e) => return Err(ScrollError::Custom(format!("{:?}", e))),
    };
    // Uleb128 does not implement Pwrite!!!
    *offset += match NanoUleb128::from(str_len).write_into(buf) {
      Ok(s) => s,
      Err(e) => return Err(ScrollError::Custom(format!("{:?}", e))),
    };
    buf.gwrite(value.as_bytes(), offset)?;
    assert!(*offset == TString::size_with(self), "TString size mismatch");
    Ok(*offset)
  }
}

impl From<&str> for TString {
  fn from(s: &str) -> Self {
    Self(
      s.to_string(),
      match s.len() {
        0..=127 => 1,
        128..=255 => 2,
        256..=511 => 3,
        _ => 4,
      },
    )
  }
}

impl From<String> for TString {
  fn from(s: String) -> Self {
    let len = &s.len();
    Self(
      s,
      match len {
        0..=128 => 1,
        128..=256 => 2,
        256..=512 => 3,
        _ => 4,
      },
    )
  }
}

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

#[derive(Clone, Debug, Default, PartialEq, Eq, From)]
pub struct VariableTBitVec(BitVec<Lsb0, u8>, i16);

impl VariableTBitVec {
  #[must_use]
  pub const fn bitvec(&self) -> &BitVec<Lsb0, u8> {
    &self.0
  }

  #[must_use]
  pub const fn size(&self) -> i16 {
    self.1
  }
}

impl Index<usize> for VariableTBitVec {
  type Output = bool;

  fn index(&self, index: usize) -> &Self::Output {
    &self.bitvec()[index]
  }
}

impl From<Vec<bool>> for VariableTBitVec {
  fn from(v: Vec<bool>) -> Self {
    Self(BitVec::<Lsb0, u8>::from(&v[..]), v.len() as i16)
  }
}

impl SizeWith<VariableTBitVec> for VariableTBitVec {
  fn size_with(ctx: &Self) -> usize {
    i16::size_with(&LE) + ctx.bitvec().as_slice().len()
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
    let byte_len = (f32::from(len) / 8.0).ceil() as usize;
    let bits =
      BitVec::<Lsb0, u8>::from_slice(&buf[*offset..*offset + byte_len]);
    *offset += byte_len;
    Ok((Self(bits, len), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a VariableTBitVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let bits = self.bitvec();
    let size = self.size();
    buf.gwrite(size, offset)?;
    buf.gwrite(bits.as_slice(), offset)?;
    let expected_size = VariableTBitVec::size_with(self);
    assert!(
      expected_size == *offset,
      "VariableTBitVec offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );

    Ok(*offset)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, From, Index, AsRef, AsMut)]
#[repr(C)]
pub struct TBitVec(BitVec<Lsb0, u8>);

impl TBitVec {
  pub fn set(&mut self, idx: usize, value: bool) {
    self.as_mut().set(idx, value)
  }
}

impl SizeWith<TBitVec> for TBitVec {
  fn size_with(ctx: &Self) -> usize {
    ctx.as_ref().as_slice().len()
  }
}

impl From<Vec<bool>> for TBitVec {
  fn from(v: Vec<bool>) -> Self {
    Self(BitVec::from(&v[..]))
  }
}

impl<'a> From<&'a [u8]> for TBitVec {
  fn from(v: &'a [u8]) -> Self {
    Self(BitVec::from_slice(v))
  }
}

impl From<&TileHeader> for TBitVec {
  fn from(th: &TileHeader) -> Self {
    let mut bits = Self::from(vec![
      th.has_attributes,
      th.has_block,
      th.has_wall,
      false,
      false,
      th.has_extended_block_id,
      false,
      false,
    ]);
    th.liquid_type.assign_bits(&mut bits);
    th.rle_type.assign_bits(&mut bits);
    bits
  }
}

impl From<&TileAttributes> for TBitVec {
  fn from(ta: &TileAttributes) -> Self {
    let mut bits = Self::from(vec![
      ta.has_extended_attributes,
      false,
      false,
      false,
      false,
      false,
      false,
      false,
    ]);
    ta.shape.assign_bits(&mut bits);
    ta.wiring.assign_bits(&mut bits);
    bits
  }
}

impl From<&ExtendedTileAttributes> for TBitVec {
  fn from(ext_attrs: &ExtendedTileAttributes) -> Self {
    let mut bits = Self::from(vec![
      false,
      false,
      ext_attrs.is_block_inactive,
      ext_attrs.is_block_painted,
      ext_attrs.is_wall_painted,
      false,
      false,
      false,
    ]);
    ext_attrs.wiring.assign_extended_bits(&mut bits);
    bits
  }
}

impl<'a> TryFromCtx<'a, Endian> for TBitVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let bits = BitVec::<Lsb0, u8>::from_slice(&buf[*offset..=*offset]);
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
    let offset = &mut 0;
    buf.gwrite(self.as_ref().as_slice(), offset)?;
    let expected_size = TBitVec::size_with(self);
    assert!(
      expected_size == *offset,
      "TBitVec offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );
    Ok(*offset)
  }
}

#[cfg(test)]
mod test_common {
  use super::{
    Pwrite,
    TBitVec,
    TBool,
    TString,
    TileAttributes,
    TryFromCtx,
  };
  use crate::{
    enums::BlockShape,
    model::Wiring,
  };
  use scroll::LE;
  #[test]
  fn test_tbool_true_rw() {
    let t = &TBool::True;
    let mut bytes = [0; 1];
    let _res = bytes.pwrite_with::<&TBool>(t, 0, LE).unwrap();
    assert_eq!(
      TBool::try_from_ctx(&bytes[..], LE).unwrap(),
      (TBool::True, 1)
    );
  }

  #[test]
  fn test_tbool_false_rw() {
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
    let t = &TString::from("foo");
    let mut bytes = [0; 4];
    let _res = bytes.pwrite_with::<&TString>(t, 0, LE).unwrap();
    assert_eq!(
      TString::try_from_ctx(&bytes[..], LE).unwrap(),
      (TString::from("foo"), 4)
    );
  }

  #[test]
  fn test_tbitvec_from_tile_attributes() {
    let ta = TileAttributes {
      has_extended_attributes: true,
      shape: BlockShape::HalfTile,
      wiring: Wiring::default(),
    };
    assert_eq!(
      TBitVec::from(
        vec![true, false, false, false, true, false, false, false,]
      ),
      TBitVec::from(&ta)
    );

    let ta = TileAttributes {
      has_extended_attributes: false,
      shape: BlockShape::TopRightSlope,
      wiring: Wiring {
        red: true,
        blue: false,
        green: false,
        yellow: false,
        actuator: false,
      },
    };
    assert_eq!(
      TBitVec::from(
        vec![false, true, false, false, false, true, false, false,]
      ),
      TBitVec::from(&ta)
    );
  }
}
