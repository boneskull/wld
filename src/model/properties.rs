use crate::model::common::*;
use bitvec::prelude::*;
use derive_new::new;
use scroll::{
  ctx::{TryFromCtx, TryIntoCtx},
  Endian, Pread, Pwrite,
};
use std::convert::TryInto;
use std::fmt::Debug;
pub use uuid::Uuid;

pub type MoonStyle = u8;
pub type UndergroundSnowStyle = i32;
pub type UndergroundJungleStyle = i32;
pub type HellStyle = i32;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new, Pread, Pwrite)]
pub struct QuadrantStyle {
  pub x1: i32,
  pub x2: i32,
  pub x3: i32,
  pub far_left: i32,
  pub near_left: i32,
  pub near_right: i32,
  pub far_right: i32,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new, Pread, Pwrite)]
pub struct WorldStyle {
  pub moon: MoonStyle,
  pub trees: QuadrantStyle,
  pub moss: QuadrantStyle,
  pub underground_snow: UndergroundSnowStyle,
  pub underground_jungle: UndergroundJungleStyle,
  pub hell: HellStyle,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GeneratorInfo {
  pub seed: TString,
  pub version: u64,
}

impl GeneratorInfo {
  pub fn new<S>(seed: S, version: u64) -> Self
  where
    S: Into<TString>,
  {
    GeneratorInfo {
      seed: seed.into(),
      version,
    }
  }
}

impl<'a> TryFromCtx<'a, Endian> for GeneratorInfo {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let seed = buf.gread_with::<TString>(offset, ctx)?;
    let version = buf.gread_with::<u64>(offset, ctx)?;
    Ok((Self::new(seed, version), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a GeneratorInfo {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let GeneratorInfo { seed, version } = self;
    let mut size = 0;
    size += seed.try_into_ctx(&mut buf[size..], ctx)?;
    size += version.try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TBitVec(BitVec<Lsb0, u8>);

impl From<Vec<bool>> for TBitVec {
  fn from(v: Vec<bool>) -> Self {
    Self(BitVec::<Lsb0, u8>::from(&v[..]))
  }
}

impl<'a> TryFromCtx<'a, Endian> for TBitVec {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let len = buf.gread_with::<i16>(offset, ctx)?;
    let byte_len = (len as f32 / 8.0).ceil() as usize;
    let bits =
      BitVec::<Lsb0, u8>::from_slice(&buf[*offset..*offset + byte_len]);
    *offset += byte_len;
    Ok((Self(bits), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TBitVec {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let bits = &self.0;
    let mut size = 0;
    let tfi_size: i16 = bits.len().try_into().unwrap();
    size += tfi_size.try_into_ctx(&mut buf[size..], ctx)?;
    size += bits.as_slice().try_into_ctx(&mut buf[size..], ())?;
    Ok(size)
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new)]
pub struct TUuid(Uuid);

impl<'a> TryFromCtx<'a, Endian> for TUuid {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let raw_uuid = buf.gread_with::<u128>(offset, ctx)?;

    Ok((Self(Uuid::from_u128_le(raw_uuid)), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TUuid {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let uuid = self.0;
    let mut size = 0;
    size += uuid.to_u128_le().try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}

impl From<Uuid> for TUuid {
  fn from(uuid: Uuid) -> Self {
    Self(uuid)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EvilType {
  Corruption,
  Crimson,
}

impl<'a> TryFromCtx<'a, Endian> for EvilType {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    _ctx: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let raw_value = buf.gread::<u8>(offset)?;
    let evil_type = if raw_value != 0 {
      Self::Crimson
    } else {
      Self::Corruption
    };
    Ok((evil_type, *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a EvilType {
  type Error = scroll::Error;

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

#[derive(Clone, Debug, PartialEq, new, Pread, Pwrite)]
pub struct Properties {
  pub tile_frame_importances: TBitVec,
  pub name: TString,
  pub generator: GeneratorInfo,
  pub uuid: TUuid,
  pub id: u32,
  pub bounds: Rect,
  pub height: i32,
  pub width: i32,
  pub is_expert: TBool,
  pub created_on: u64, // TODO
  pub style: WorldStyle,
  pub spawn_point: Point,
  pub underground_level: f64,
  pub cavern_level: f64,
  pub time: f64,
  pub is_daytime: TBool,
  pub moon_phase: u32,
  pub is_blood_moon: TBool,
  pub is_eclipse: TBool,
  pub dungeon_point: Point,
  pub evil_type: EvilType,
}

#[cfg(test)]
mod test_properties {
  use super::*;
  use crate::model::common::TBool::*;
  use scroll::LE;

  #[test]
  fn test_properties_rw() {
    let props = &Properties {
      tile_frame_importances: TBitVec::from(vec![
        false, false, false, true, true, true, false, false, false, false,
        true, true, true, true, true, true, true, true, true, true, true, true,
        false, false, true, false, true, true, true, true, false, true, false,
        true, true, true, true, false, false, false, false, false, true, false,
        false, false, false, false, false, false, true, false, false, false,
        false, true, false, false, false, false, false, true, false, false,
        false, false, false, false, false, false, false, true, true, true,
        true, false, false, true, true, true, false, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, false,
        false, false, true, false, false, true, true, false, false, false,
        false, false, false, false, false, false, false, true, true, false,
        true, true, false, false, true, true, true, true, true, true, true,
        true, false, true, true, true, true, false, false, false, false, true,
        false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, true, false, false, false, false,
        false, true, true, true, true, false, false, false, true, false, false,
        false, false, false, true, true, true, true, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
        true, false, false, false, false, false, true, false, true, true,
        false, true, false, false, true, true, true, true, true, true, false,
        false, false, false, false, false, true, true, false, false, true,
        false, true, false, true, true, true, true, true, true, true, true,
        true, true, true, true, true, false, false, false, false, false, false,
        true, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, true, true, true, false, false,
        false, true, true, true, true, true, true, true, true, true, false,
        true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, false, false, false, true, false, true, true, true, true,
        true, false, false, true, true, false, false, false, false, false,
        false, false, false, false, true, true, false, true, true, true, false,
        false, false, false, false, false, false, false, false, true, false,
        false, false, false, true, true, true, false, true, true, true, true,
        true, true, true, false, false, false, false, false, false, false,
        true, true, true, true, true, true, true, false, true, false, false,
        false, false, false, true, true, true, true, true, true, true, true,
        true, true, false, false, false, false, false, false, false, false,
        false, true, true, false, false, false, true, true, true, true, true,
        false, false, false, false, true, true, false, false, true, true, true,
        false, true, true, true, false, false, false, false, false, true, true,
        true, true, true, true, true, true, true, true, true, false, false,
        false, false, false, false, true, true, true, true, true, true, false,
        false, false, true, true, true, true, true, true, true, true, true,
        false, false,
      ]),
      name: TString::from("Foon"),
      generator: GeneratorInfo {
        seed: TString::from("1451234789"),
        version: 9860045932737703464,
      },
      uuid: TUuid(
        Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(),
      ),
      id: 1468463142,
      bounds: Rect {
        left: 0,
        right: 67200,
        top: 0,
        bottom: 19200,
      },
      width: 4200,
      height: 1200,
      is_expert: False,
      created_on: 8518612034984415,
      style: WorldStyle {
        moon: 1,
        trees: QuadrantStyle::new(4, 5, 0, 0, 3072, 4200, 4200),
        moss: QuadrantStyle::new(1, 0, 3, 3, 1210, 4200, 4200),
        underground_snow: 3,
        underground_jungle: 0,
        hell: 0,
      },
      spawn_point: Point::new(2098, 229),
      underground_level: 300.0,
      cavern_level: 528.0,
      time: 0.0,
      is_daytime: True,
      moon_phase: 0u32,
      is_blood_moon: False,
      is_eclipse: True,
      dungeon_point: Point::new(3426, 211),
      evil_type: EvilType::Corruption,
    };
    let mut bytes = [0; 255];
    let _res = bytes.pwrite_with::<&Properties>(props, 0, LE).unwrap();
    let (parsed, size) = &Properties::try_from_ctx(&bytes[..], LE).unwrap();
    assert_eq!(parsed, props);
    assert_eq!(*size, 255);
  }
}
