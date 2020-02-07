use crate::model::common::*;
use bit_vec::BitVec;
use derive_new::new;
use scroll::{
  ctx::{TryFromCtx, TryIntoCtx},
  Endian, Pread, Pwrite,
};
use std::convert::TryInto;
use std::fmt::Debug;
pub use uuid::Uuid;

pub type MoonStyle = u8;
pub type UndergroundIceStyle = i32;
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
  pub underground_ice: UndergroundIceStyle,
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

  fn try_from_ctx(buf: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let seed = buf.gread_with::<TString>(offset, ctx)?;
    let version = buf.gread_with::<u64>(offset, ctx)?;
    Ok((Self::new(seed, version), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a GeneratorInfo {
  type Error = scroll::Error;

  fn try_into_ctx(self, buf: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
    let GeneratorInfo { seed, version } = self;
    let mut size = 0;
    size += seed.try_into_ctx(&mut buf[size..], ctx)?;
    size += version.try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TBitVec(BitVec);

impl From<Vec<bool>> for TBitVec {
  fn from(v: Vec<bool>) -> Self {
    Self(BitVec::from_fn(v.len(), |i| v[i]))
  }
}

impl<'a> TryFromCtx<'a, Endian> for TBitVec {
  type Error = scroll::Error;

  fn try_from_ctx(buf: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let len = buf.gread_with::<i16>(offset, ctx)?;
    let len = (len as f32 / 8.0).ceil() as usize;
    let bits = BitVec::from_bytes(&buf[*offset..(*offset + len)]);
    *offset += len;
    Ok((Self(bits), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TBitVec {
  type Error = scroll::Error;

  fn try_into_ctx(self, buf: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
    let bits = &self.0;
    let mut size = 0;
    let tfi_size: i16 = bits.len().try_into().unwrap();
    size += tfi_size.try_into_ctx(&mut buf[size..], ctx)?;
    size += bits.to_bytes().try_into_ctx(&mut buf[size..], ())?;
    Ok(size)
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new)]
pub struct TUuid(Uuid);

impl<'a> TryFromCtx<'a, Endian> for TUuid {
  type Error = scroll::Error;

  fn try_from_ctx(buf: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let raw_uuid = buf.gread_with::<u128>(offset, ctx)?;

    Ok((Self(Uuid::from_u128_le(raw_uuid)), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TUuid {
  type Error = scroll::Error;

  fn try_into_ctx(self, buf: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
    let uuid = self.0;
    let mut size = 0;
    size += uuid.to_u128_le().try_into_ctx(&mut buf[size..], ctx)?;
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
  pub size: Point,
  pub is_expert: TBool,
  pub created_on: u128, // TODO
  pub style: WorldStyle,
  pub spawn_point: Point,
  pub underground_level: f64,
  pub cavern_level: f64,
}

#[cfg(test)]
mod test_properties {
  use super::*;
  use scroll::LE;

  #[test]
  fn test_properties_rw() {
    let props = &Properties {
      tile_frame_importances: TBitVec::from(vec![
        true, false, true, false, true, false, true, false,
      ]),
      name: TString::from("boneskullandia"),
      generator: GeneratorInfo {
        seed: TString::from("herp"),
        version: 123456789,
      },
      uuid: TUuid(Uuid::NAMESPACE_DNS), // why not
      id: 12345678,
      bounds: Rect {
        left: 0,
        right: 67200,
        top: 0,
        bottom: 19200,
      },
      size: Point::new(4200, 1200),
      is_expert: TBool::from(false),
      created_on: 444444,
      style: WorldStyle {
        moon: 1,
        trees: QuadrantStyle::new(1, 2, 3, 4, 5, 6, 7),
        moss: QuadrantStyle::new(1, 2, 3, 4, 5, 6, 7),
        underground_ice: 1,
        underground_jungle: 0,
        hell: 1,
      },
      spawn_point: Point::new(2098, 229),
      underground_level: 300.0,
      cavern_level: 528.0,
    };
    let mut bytes = [0; 200];
    let _res = bytes.pwrite_with::<&Properties>(props, 0, LE).unwrap();
    let parsed = &Properties::try_from_ctx(&bytes[..], LE).unwrap().0;
    assert_eq!(parsed, props);
  }
}
