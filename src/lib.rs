use std::string::String;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct GeneratorInfo {
  pub seed: String,
  pub version: u64,
}

impl GeneratorInfo {
  fn new<S>(seed: S, version: u64) -> Self
  where
    S: Into<String>,
  {
    GeneratorInfo {
      seed: seed.into(),
      version,
    }
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Rect {
  pub left: i32,
  pub right: i32,
  pub top: i32,
  pub bottom: i32,
}

impl Rect {
  fn new(left: i32, right: i32, top: i32, bottom: i32) -> Self {
    Rect {
      left,
      right,
      top,
      bottom,
    }
  }
}

impl From<Vec<i32>> for Rect {
  fn from(v: Vec<i32>) -> Self {
    Rect::new(v[0], v[1], v[2], v[3])
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

pub(self) mod helpers {
  use nom::{
    combinator::map,
    multi::{length_value, many0, many1},
    number::complete::le_u8,
    IResult,
  };

  pub fn string1(buf: &[u8]) -> IResult<&[u8], String> {
    map(length_value(le_u8, many1(map(le_u8, |v| v as char))), |v| {
      v.iter().collect()
    })(buf)
  }

  pub fn string0(buf: &[u8]) -> IResult<&[u8], String> {
    map(length_value(le_u8, many0(map(le_u8, |v| v as char))), |v| {
      v.iter().collect()
    })(buf)
  }
}

#[cfg(test)]
pub(crate) mod test_helpers {
  use nom::IResult;
  pub fn unwrap<T>(res: IResult<&[u8], T>) -> T {
    res.unwrap().1
  }
}

pub(self) mod header {
  use super::{helpers::*, GeneratorInfo, Rect};
  use nom::{
    bits::bits,
    bits::complete::take as take_bits,
    combinator::map,
    multi::{count, length_value, many0},
    number::complete::{le_i32, le_u128, le_u16, le_u32, le_u64, le_u8},
    sequence::tuple,
    IResult,
  };
  use std::mem::size_of;
  use uuid::Uuid;

  pub fn name(buf: &[u8]) -> IResult<&[u8], String> {
    string1(buf)
  }

  fn version(buf: &[u8]) -> IResult<&[u8], i32> {
    le_i32(buf)
  }

  fn metadata(buf: &[u8]) -> IResult<&[u8], u64> {
    le_u64(buf)
  }

  fn revision(buf: &[u8]) -> IResult<&[u8], u32> {
    le_u32(buf)
  }

  fn favorite(buf: &[u8]) -> IResult<&[u8], bool> {
    map(le_u8, |flag| flag != 0)(buf)
  }

  fn positions(buf: &[u8]) -> IResult<&[u8], Vec<i32>> {
    length_value(
      map(le_u16, |count: u16| count * size_of::<i32>() as u16),
      many0(le_i32),
    )(buf)
  }

  fn importances(buf: &[u8]) -> IResult<&[u8], Vec<bool>> {
    map(
      length_value(
        map(le_u16, |length| (length as f32 / 8.0).ceil() as u16),
        many0(bits::<_, _, (_, _), _, _>(map(
          count(take_bits(1usize), 8),
          |v: Vec<u8>| v.into_iter().rev().map(|i| i != 0).collect(),
        ))),
      ),
      |v: Vec<Vec<bool>>| v.into_iter().flatten().collect(),
    )(buf)
  }

  fn generator_info(buf: &[u8]) -> IResult<&[u8], GeneratorInfo> {
    map(tuple((string0, le_u64)), |(seed, version)| {
      GeneratorInfo::new(seed, version)
    })(buf)
  }

  fn uuid(buf: &[u8]) -> IResult<&[u8], Uuid> {
    map(le_u128, |v| Uuid::from_u128_le(v))(buf)
  }

  fn id(buf: &[u8]) -> IResult<&[u8], u32> {
    le_u32(buf)
  }

  fn bounds(buf: &[u8]) -> IResult<&[u8], Rect> {
    map(count(le_i32, 4), |v| Rect::from(v))(buf)
  }

  #[cfg(test)]
  use lazy_static::*;
  #[cfg(test)]
  use std::fs::read;

  #[cfg(test)]
  lazy_static! {
    static ref WORLD: std::vec::Vec<u8> =
      read("./src/tests/fixtures/Foon.wld").expect("Unable to read file");
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn test_name() {
      assert_eq!(unwrap(name(&WORLD[127..133])), "Foon");
    }

    #[test]
    fn test_version() {
      assert_eq!(unwrap(version(&WORLD[0..4])), 194);
    }

    #[test]
    fn test_metadata() {
      assert_eq!(unwrap(metadata(&WORLD[4..12])), 172097103742133618);
    }

    #[test]
    fn test_revision() {
      assert_eq!(unwrap(revision(&WORLD[12..16])), 160);
    }

    #[test]
    fn test_favorite() {
      assert_eq!(unwrap(favorite(&WORLD[16..24])), false);
    }

    #[test]
    fn test_positions() {
      assert_eq!(
        unwrap(positions(&WORLD[24..66])),
        &[127, 2802, 2860224, 2879758, 2880141, 2880453, 2880457, 2880461, 2880489, 0]
      );
    }

    #[test]
    fn test_importances() {
      assert_eq!(
        unwrap(importances(&WORLD[66..127]))[..32],
        [
          false, false, false, true, true, true, false, false, false, false, true, true, true,
          true, true, true, true, true, true, true, true, true, false, false, true, false, true,
          true, true, true, false, true
        ]
      );
    }

    #[test]
    fn test_generator_info() {
      assert_eq!(
        unwrap(generator_info(&WORLD[132..151])),
        GeneratorInfo::new("1451234789", 833223655425)
      );
    }

    #[test]
    fn test_uuid() {
      assert_eq!(
        unwrap(uuid(&WORLD[151..167])),
        Uuid::parse_str("d578e106-3827-f648-a224-254c06ca78cb").unwrap()
      );
    }

    #[test]
    fn test_id() {
      assert_eq!(unwrap(id(&WORLD[167..171])), 1468463142)
    }

    #[test]
    fn test_bounds() {}
  }
}
