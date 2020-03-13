use crate::{
  enums::{
    EvilType,
    TBool,
  },
  models::{
    Position,
    TString,
    VariableTBitVec,
    WorldCtx,
  },
};
use scroll::{
  ctx::{
    SizeWith,
    TryFromCtx,
    TryIntoCtx,
  },
  Endian,
  Pread,
  Pwrite,
  LE,
};
use std::fmt::Debug;
pub use uuid::Uuid;

/// Represents the size of a Terraria map, as a rectangle.
///
/// See [Terraria Wiki: Map Size] for more information.
///
/// [Terraria Wiki: Map Size]: https://terraria.gamepedia.com/Map_size
///
/// # Notes
///
/// - For our purposes, the _top left corner_ is the origin.
/// - While the fields are of type [`i32`], I don't believe they can be negative
///   integers.
/// - [`Bounds::left`] and [`Bounds::top`] are always `0`, as far as I can tell.
#[derive(
  Copy, Clone, Debug, Default, PartialEq, Eq, Pread, Pwrite, SizeWith,
)]
#[repr(C)]
pub struct Bounds {
  /// The minimum x-coordinate of the map.
  pub left: i32,
  /// The maximum x-coordinate of the map.
  pub right: i32,
  /// The minimum y-coordinate of the map.
  pub top: i32,
  /// The maximum y-coordinate of the map.
  pub bottom: i32,
}

/// Represents the appearance of the moon.
///
/// # Notes
///
/// - This is _not_ the same as the [moon phase](crate::models::Properties::moon_phase).
pub type MoonStyle = u8;

/// Represents the appearance of the underground snow biome's background.
pub type UndergroundSnowStyle = i32;

/// Represents the appearance of the underground jungle biome's background.
pub type UndergroundJungleStyle = i32;

/// Represents the appearance of the hell biome's background.
pub type HellStyle = i32;

/// Represents styles which differ by "quandrant" of the map (by X-axis).
///
/// # Notes
///
/// - A "quadrant" is not necessarily 1/4th of the map, but there will be four.
#[derive(
  Copy,
  Clone,
  Debug,
  Default,
  PartialEq,
  Eq,
  Constructor,
  Pread,
  Pwrite,
  SizeWith,
)]
#[repr(C)]
pub struct QuadrantStyle {
  /// The X-coordinate separating the first quadrant from the second.
  pub x1: i32,

  /// The X-coordinate separating the second quadrant from the third.
  pub x2: i32,

  /// The X-coordinate separating the third quadrant from the fourth.
  pub x3: i32,

  /// The value of the first quadrant's style, counting from the left.
  pub first: i32,

  /// The value of the second quadrant's style, counting from the left.
  pub second: i32,

  /// The value of the third quadrant's style, counting from the left.
  pub third: i32,

  /// The value of the fourth quadrant's style, counting from the left.
  pub fourth: i32,
}

/// A bucket in which some style information lives.
///
/// # Notes
///
/// - Probably unnecessary.
#[derive(
  Copy, Clone, Debug, Default, PartialEq, Eq, Pread, Pwrite, SizeWith,
)]
#[repr(C)]
pub struct WorldStyle {
  /// Represents the appearance of the moon.
  pub moon: MoonStyle,

  /// Represents the appearance of the surface (?) over four map "quadrants".
  pub trees: QuadrantStyle,

  /// Represents the appearance of the moss (?) over four map "quadrants".
  pub moss: QuadrantStyle,

  /// Represents the appearance of the underground snow biome.
  ///
  /// See [Terraria Wiki: Ice Biome](https://terraria.gamepedia.com/Ice_biome) for more information.
  pub underground_snow: UndergroundSnowStyle,

  /// Represents the appearance of the underground jungle biome.
  ///
  /// See [Terraria Wiki: Underground Jungle](https://terraria.gamepedia.com/Underground_Jungle) for more information.
  pub underground_jungle: UndergroundJungleStyle,

  /// Represents the appearance of the hell biome.
  ///
  /// See [Terraria Wiki: The Underworld](https://terraria.gamepedia.com/The_Underworld) for more information.
  pub hell: HellStyle,
}

/// Information about the world seed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GeneratorInfo {
  /// The seed value which was used to generate the world.
  pub seed: TString,

  /// The version of the generator, I think.
  pub version: u64,
}

impl SizeWith<GeneratorInfo> for GeneratorInfo {
  fn size_with(ctx: &Self) -> usize {
    TString::size_with(&ctx.seed) + u64::size_with(&LE)
  }
}

impl GeneratorInfo {
  /// Convenience method to create a [`GeneratorInfo`] instance.
  pub fn new<S>(seed: S, version: u64) -> Self
  where
    S: Into<TString>,
  {
    Self {
      seed: seed.into(),
      version,
    }
  }
}

impl<'a> TryFromCtx<'a, Endian> for GeneratorInfo {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let seed = buf.gread::<TString>(offset)?;
    let version = buf.gread_with::<u64>(offset, LE)?;
    Ok((Self { seed, version }, *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a GeneratorInfo {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let GeneratorInfo { seed, version } = self;
    let offset = &mut 0;
    buf.gwrite(seed, offset)?;
    buf.gwrite_with(version, offset, LE)?;
    assert!(
      *offset == GeneratorInfo::size_with(self),
      "GeneratorInfo size mismatch"
    );

    Ok(*offset)
  }
}

/// A UUID.
///
/// Fancy wrapper around [`Uuid`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, AsRef)]
pub struct TUuid(Uuid);

impl SizeWith<Endian> for TUuid {
  fn size_with(_: &Endian) -> usize {
    u128::size_with(&LE)
  }
}

impl<'a> TryFromCtx<'a, Endian> for TUuid {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let raw_uuid = buf.gread_with::<u128>(offset, LE)?;
    Ok((Self(Uuid::from_u128_le(raw_uuid)), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TUuid {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let uuid = self.as_ref();
    let offset = &mut 0;
    buf.gwrite_with(uuid.to_u128_le(), offset, LE)?;
    let expected_size = TUuid::size_with(&LE);
    assert!(
      expected_size == *offset,
      "TUuid offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );

    Ok(*offset)
  }
}

impl From<Uuid> for TUuid {
  fn from(uuid: Uuid) -> Self {
    Self(uuid)
  }
}

#[derive(Clone, Debug, PartialEq, Pread, Pwrite)]
#[repr(C)]
pub struct Properties {
  pub tile_frame_importances: VariableTBitVec,
  pub name: TString,
  pub generator: GeneratorInfo,
  pub uuid: TUuid,
  pub id: i32,
  pub bounds: Bounds,
  pub height: i32,
  pub width: i32,
  pub is_expert: TBool,
  pub created_on: u64, // TODO
  pub style: WorldStyle,
  pub spawn_point: Position,
  pub underground_level: f64,
  pub cavern_level: f64,
  pub current_time: f64,
  pub is_daytime: TBool,
  pub moon_phase: u32,
  pub is_blood_moon: TBool,
  pub is_eclipse: TBool,
  pub dungeon_point: Position,
  pub evil_type: EvilType,
}

impl Properties {
  #[must_use]
  pub const fn as_world_context(&self) -> WorldCtx<'_> {
    WorldCtx {
      world_height: &self.height,
      world_width: &self.width,
      tile_frame_importances: &self.tile_frame_importances,
      id: &self.id,
      name: &self.name,
    }
  }
}

impl SizeWith<Properties> for Properties {
  fn size_with(ctx: &Self) -> usize {
    let size = VariableTBitVec::size_with(&ctx.tile_frame_importances)
      + TString::size_with(&ctx.name)
      + GeneratorInfo::size_with(&ctx.generator)
      + TUuid::size_with(&LE)
      + (i32::size_with(&LE) * 3)
      + Bounds::size_with(&LE)
      + (TBool::size_with(&LE) * 4)
      + u64::size_with(&LE)
      + WorldStyle::size_with(&LE)
      + (Position::size_with(&LE) * 2)
      + (f64::size_with(&LE) * 3)
      + u32::size_with(&LE)
      + EvilType::size_with(&LE);
    debug!("Properties size: {}", size);
    size
  }
}

#[cfg(test)]
mod test_properties {
  use super::{
    Bounds,
    EvilType,
    GeneratorInfo,
    Position,
    Properties,
    Pwrite,
    QuadrantStyle,
    TString,
    TUuid,
    TryFromCtx,
    Uuid,
    VariableTBitVec,
    WorldStyle,
  };
  use crate::enums::TBool::{
    False,
    True,
  };
  use scroll::LE;

  #[test]
  fn test_properties_rw() {
    let bitflags = vec![
      false, false, false, true, true, true, false, false, false, false, true,
      true, true, true, true, true, true, true, true, true, true, true, false,
      false, true, false, true, true, true, true, false, true, false, true,
      true, true, true, false, false, false, false, false, true, false, false,
      false, false, false, false, false, true, false, false, false, false,
      true, false, false, false, false, false, true, false, false, false,
      false, false, false, false, false, false, true, true, true, true, false,
      false, true, true, true, false, true, true, true, true, true, true, true,
      true, true, true, true, true, true, true, true, true, true, true, true,
      true, true, true, true, true, true, true, false, false, false, true,
      false, false, true, true, false, false, false, false, false, false,
      false, false, false, false, true, true, false, true, true, false, false,
      true, true, true, true, true, true, true, true, false, true, true, true,
      true, false, false, false, false, true, false, false, false, false,
      false, false, false, false, false, false, false, false, false, false,
      false, true, false, false, false, false, false, true, true, true, true,
      false, false, false, true, false, false, false, false, false, true, true,
      true, true, false, false, false, false, false, false, false, false,
      false, false, false, false, false, true, false, false, false, false,
      false, true, false, true, true, false, true, false, false, true, true,
      true, true, true, true, false, false, false, false, false, false, true,
      true, false, false, true, false, true, false, true, true, true, true,
      true, true, true, true, true, true, true, true, true, false, false,
      false, false, false, false, true, false, false, false, false, false,
      false, false, false, false, false, false, false, false, false, true,
      true, true, false, false, false, true, true, true, true, true, true,
      true, true, true, false, true, true, true, true, true, true, true, true,
      true, true, true, true, true, true, true, true, true, true, true, true,
      true, true, true, true, true, true, false, false, false, true, false,
      true, true, true, true, true, false, false, true, true, false, false,
      false, false, false, false, false, false, false, true, true, false, true,
      true, true, false, false, false, false, false, false, false, false,
      false, true, false, false, false, false, true, true, true, false, true,
      true, true, true, true, true, true, false, false, false, false, false,
      false, false, true, true, true, true, true, true, true, false, true,
      false, false, false, false, false, true, true, true, true, true, true,
      true, true, true, true, false, false, false, false, false, false, false,
      false, false, true, true, false, false, false, true, true, true, true,
      true, false, false, false, false, true, true, false, false, true, true,
      true, false, true, true, true, false, false, false, false, false, true,
      true, true, true, true, true, true, true, true, true, true, false, false,
      false, false, false, false, true, true, true, true, true, true, false,
      false, false, true, true, true, true, true, true, true, true, true,
      false, false,
    ];
    let props = &Properties {
      tile_frame_importances: VariableTBitVec::from(bitflags),
      name: TString::from("Foon"),
      generator: GeneratorInfo {
        seed: TString::from("1451234789"),
        version: 9_860_045_932_737_703_464,
      },
      uuid: TUuid(
        Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(),
      ),
      id: 1_468_463_142,
      bounds: Bounds {
        left: 0,
        right: 67200,
        top: 0,
        bottom: 19200,
      },
      width: 4200,
      height: 1200,
      is_expert: False,
      created_on: 8_518_612_034_984_415,
      style: WorldStyle {
        moon: 1,
        trees: QuadrantStyle::new(4, 5, 0, 0, 3072, 4200, 4200),
        moss: QuadrantStyle::new(1, 0, 3, 3, 1210, 4200, 4200),
        underground_snow: 3,
        underground_jungle: 0,
        hell: 0,
      },
      spawn_point: Position::new(2098, 229),
      underground_level: 300.0,
      cavern_level: 528.0,
      current_time: 0.0,
      is_daytime: True,
      moon_phase: 0_u32,
      is_blood_moon: False,
      is_eclipse: True,
      dungeon_point: Position::new(3426, 211),
      evil_type: EvilType::Corruption,
    };
    let mut bytes = [0; 255];
    let _res = bytes.pwrite_with::<&Properties>(props, 0, LE).unwrap();
    let (parsed, size) = &Properties::try_from_ctx(&bytes[..], LE).unwrap();
    assert_eq!(parsed, props);
    assert_eq!(*size, 255);
  }
}
