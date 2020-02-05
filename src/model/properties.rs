use crate::model::common::Point;
use derive_new::new;

pub type MoonStyle = u8;
pub type UndergroundIceStyle = i32;
pub type UndergroundJungleStyle = i32;
pub type HellStyle = i32;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new)]
pub struct QuadrantStyle {
  pub x1: i32,
  pub x2: i32,
  pub x3: i32,
  pub far_left: i32,
  pub near_left: i32,
  pub near_right: i32,
  pub far_right: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
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
  pub seed: String,
  pub version: u64,
}

impl GeneratorInfo {
  pub fn new<S>(seed: S, version: u64) -> Self
  where
    S: Into<String>,
  {
    GeneratorInfo {
      seed: seed.into(),
      version,
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
pub enum EvilType {
  Crimson,
  Corruption,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
pub struct Properties {
  pub dungeon_point: Point,
  pub evil_type: EvilType,
}

#[cfg(test)]
mod test_generator_info {
  use super::GeneratorInfo;

  #[test]
  fn test_new() {
    assert_eq!(
      GeneratorInfo::new("foo", 1),
      GeneratorInfo {
        seed: String::from("foo"),
        version: 1
      }
    )
  }
}
