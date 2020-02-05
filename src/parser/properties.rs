use super::common::{bool, point, string0, string1};
use crate::model::{common::*, properties::*};
use nom::{
  bits::{bits, complete::take as take_bits},
  combinator::map,
  multi::{count, length_value, many0},
  number::complete::{le_i32, le_i64, le_u128, le_u16, le_u32, le_u64, le_u8},
  sequence::tuple,
  IResult,
};
use uuid::Uuid;

fn split_style(buf: &[u8]) -> IResult<&[u8], QuadrantStyle> {
  map(count(le_i32, 7), |v| {
    QuadrantStyle::new(v[0], v[1], v[2], v[3], v[4], v[5], v[6])
  })(buf)
}

pub fn name(buf: &[u8]) -> IResult<&[u8], String> {
  string1(buf)
}

pub fn importances(buf: &[u8]) -> IResult<&[u8], Vec<bool>> {
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

pub fn generator_info(buf: &[u8]) -> IResult<&[u8], GeneratorInfo> {
  map(tuple((string0, le_u64)), |(seed, version)| {
    GeneratorInfo::new(seed, version)
  })(buf)
}

pub fn uuid(buf: &[u8]) -> IResult<&[u8], Uuid> {
  map(le_u128, |v| Uuid::from_u128_le(v))(buf)
}

pub fn id(buf: &[u8]) -> IResult<&[u8], u32> {
  le_u32(buf)
}

pub fn bounds(buf: &[u8]) -> IResult<&[u8], Rect> {
  map(
    count(le_i32, 4), // we are guaranteed four items
    |v| Rect::new(v[0], v[1], v[2], v[3]),
  )(buf)
}

pub fn world_size(buf: &[u8]) -> IResult<&[u8], Point> {
  point(buf)
}

pub fn is_expert(buf: &[u8]) -> IResult<&[u8], bool> {
  bool(buf)
}

pub fn created_on(buf: &[u8]) -> IResult<&[u8], i64> {
  // see https://docs.microsoft.com/en-us/dotnet/api/system.datetime.frombinary?view=netframework-4.8#System_DateTime_FromBinary_System_Int64_
  // first 2 bits: Kind (UTC, local, unknown)
  // 62 bits: Ticks
  // "a single tick represents 100 nanoseconds"
  // convert this into a timestamp then create a DateTime with crate chrono
  le_i64(buf)
}

pub fn world_style(buf: &[u8]) -> IResult<&[u8], WorldStyle> {
  map(
    tuple((le_u8, split_style, split_style, le_i32, le_i32, le_i32)),
    |(moon, trees, moss, underground_ice, underground_jungle, hell)| {
      WorldStyle::new(moon, trees, moss, underground_ice, underground_jungle, hell)
    },
  )(buf)
}

pub fn spawn_point(buf: &[u8]) -> IResult<&[u8], Point> {
  point(buf)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::test_helpers::*;

  #[test]
  fn test_name() {
    assert_eq!(unwrap(name(&p_string("Foon"))), "Foon");
  }

  #[test]
  fn test_generator_info() {
    assert_eq!(
      // unwrap(generator_info(&WORLD[132..151])),
      // GeneratorInfo::new("1451234789", 833223655425)
      unwrap(generator_info(
        &[&p_string("McMonkey")[..], &12345u64.to_le_bytes()[..]].concat()
      )),
      GeneratorInfo::new("McMonkey", 12345)
    );
  }

  #[test]
  fn test_uuid() {
    let u = Uuid::parse_str("d578e106-3827-f648-a224-254c06ca78cb").unwrap();

    assert_eq!(unwrap(uuid(&u.to_u128_le().to_le_bytes())), u);
  }

  #[test]
  fn test_id() {
    // assert_eq!(unwrap(id(&WORLD[167..171])), 1468463142)
    assert_eq!(unwrap(id(&2020u32.to_le_bytes())), 2020);
  }

  #[test]
  fn test_bounds() {
    // assert_eq!(
    //   unwrap(bounds(&WORLD[171..188])),
    //   Rect {
    //     left: 0,
    //     right: 67200,
    //     top: 0,
    //     bottom: 19200
    //   }
    // )
    assert_eq!(
      unwrap(bounds(
        &[
          1i32.to_le_bytes(),
          2i32.to_le_bytes(),
          3i32.to_le_bytes(),
          4i32.to_le_bytes()
        ]
        .concat()
      )),
      Rect {
        left: 1,
        right: 2,
        top: 3,
        bottom: 4
      }
    )
  }

  #[test]
  fn test_world_size() {
    assert_eq!(
      // unwrap(world_size(&WORLD[187..195])),
      // Point { x: 4200, y: 1200 }
      unwrap(world_size(
        &[1200i32.to_le_bytes(), 4200i32.to_le_bytes()].concat()
      )),
      Point { x: 4200, y: 1200 }
    )
  }

  #[test]
  fn test_is_expert() {
    // assert_eq!(unwrap(is_expert(&WORLD[195..196])), false)
    assert_eq!(unwrap(is_expert(&[0u8])), false);
    assert_eq!(unwrap(is_expert(&[1u8])), true);
  }

  #[test]
  fn test_created_on() {
    assert_eq!(unwrap(created_on(&0i64.to_le_bytes())), 0);
    // assert_eq!(unwrap(created_on(&WORLD[196..204])), -8586698140971848152)
  }

  #[test]
  fn test_world_style() {
    assert_eq!(
      unwrap(world_style(&WORLD[204..273])),
      WorldStyle {
        moon: 1,
        trees: QuadrantStyle {
          x1: 3072,
          x2: 4200,
          x3: 4200,
          far_left: 4,
          near_left: 5,
          near_right: 0,
          far_right: 0
        },
        moss: QuadrantStyle {
          x1: 1210,
          x2: 4200,
          x3: 4200,
          far_left: 1,
          near_left: 0,
          near_right: 3,
          far_right: 0
        },
        underground_ice: 3,
        underground_jungle: 0,
        hell: 0
      }
    )
  }

  #[test]
  fn test_spawn_point() {
    assert_eq!(
      unwrap(spawn_point(
        &[1200i32.to_le_bytes(), 4200i32.to_le_bytes()].concat()
      )),
      Point { x: 4200, y: 1200 }
    )
  }
}
