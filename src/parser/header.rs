use crate::model::{generator_info::GeneratorInfo, point::Point, rect::Rect};
use nom::{
  bits::bits,
  bits::complete::take as take_bits,
  combinator::map,
  multi::{count, length_value, many0},
  number::complete::{le_i32, le_i64, le_u128, le_u16, le_u32, le_u64},
  sequence::tuple,
  IResult,
};
use std::mem::size_of;
use uuid::Uuid;
use super::common::*;

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
  bool(buf)
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
  map(
    count(le_i32, 4), // we are guaranteed four items
    |v| Rect::new(v[0], v[1], v[2], v[3]),
  )(buf)
}

fn world_size(buf: &[u8]) -> IResult<&[u8], Point> {
  point(buf)
}

fn is_expert(buf: &[u8]) -> IResult<&[u8], bool> {
  bool(buf)
}

fn created_on(buf: &[u8]) -> IResult<&[u8], i64> {
  // see https://docs.microsoft.com/en-us/dotnet/api/system.datetime.frombinary?view=netframework-4.8#System_DateTime_FromBinary_System_Int64_
  // first 2 bits: Kind (UTC, local, unknown)
  // 62 bits: Ticks
  // "a single tick represents 100 nanoseconds"
  // convert this into a timestamp then create a DateTime with crate chrono
  le_i64(buf)
}

#[cfg(test)]
mod header_tests {
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
        false, false, false, true, true, true, false, false, false, false, true, true, true, true,
        true, true, true, true, true, true, true, true, false, false, true, false, true, true,
        true, true, false, true
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
  fn test_bounds() {
    assert_eq!(
      unwrap(bounds(&WORLD[171..188])),
      Rect {
        left: 0,
        top: 0,
        right: 67200,
        bottom: 19200
      }
    )
  }

  #[test]
  fn test_world_size() {
    assert_eq!(
      unwrap(world_size(&WORLD[187..195])),
      Point { x: 4200, y: 1200 }
    )
  }

  #[test]
  fn test_is_expert() {
    assert_eq!(unwrap(is_expert(&WORLD[195..196])), false)
  }}
