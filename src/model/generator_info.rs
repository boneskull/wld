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
