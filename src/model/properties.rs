use derive_new::new;

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
pub enum MoonStyle {
  White,
  Orange,
  Green,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new)]
pub struct SplitStyle {
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
  pub trees: SplitStyle,
  pub moss: SplitStyle,
  pub underground_ice: i32,
  pub underground_jungle: i32,
  pub hell: i32,
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
  Corruption
}

#[cfg(test)]
mod test {
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
