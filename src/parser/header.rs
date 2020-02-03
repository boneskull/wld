use super::common::*;
use nom::combinator::{value, verify};
use nom::multi::many_m_n;
use nom::number::complete::le_u8;
use nom::sequence::tuple;
use std::str::from_utf8;

use nom::{
  combinator::map_res,
  number::complete::{le_i32, le_u32, le_u64},
  IResult,
};

pub fn version(buf: &[u8]) -> IResult<&[u8], &str> {
  map_res(le_i32, |v| match v {
    71 => Ok("1.2.0.3.1"),
    77 => Ok("1.2.2"),
    104 => Ok("1.2.3"),
    140 => Ok("1.3.0.1"),
    151 => Ok("1.3.0.4"),
    153 => Ok("1.3.0.5"),
    154 => Ok("1.3.0.6"),
    155 => Ok("1.3.0.7"),
    156 => Ok("1.3.0.8"),
    170 => Ok("1.3.2"),
    174 => Ok("1.3.3"),
    178 => Ok("1.3.4"),
    194 => Ok("1.3.5.3"),
    _ => Err(()),
  })(buf)
}

pub fn metadata(buf: &[u8]) -> IResult<&[u8], (bool, u8)> {
  tuple((
    value(
      true,
      verify(many_m_n(0, 7, le_u8), |v| {
        // TODO this should fail the parser
        from_utf8(&v).unwrap() == "relogic"
      }),
    ),
    le_u8,
  ))(buf)
}

pub fn revision(buf: &[u8]) -> IResult<&[u8], u32> {
  le_u32(buf)
}

pub fn favorite(buf: &[u8]) -> IResult<&[u8], bool> {
  bool(buf)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::test_helpers::*;

  #[test]
  fn test_version() {
    assert_eq!(unwrap(version(&WORLD[0..4])), "1.3.5.3");
  }

  #[test]
  fn test_metadata() {
    assert_eq!(unwrap(metadata(&WORLD[4..12])), (true, 2));
    assert_eq!(
      metadata(&["rel0gic".as_bytes(), &[2u8]].concat()),
      Err(nom::Err::Error((
        &[114u8, 101u8, 108u8, 48u8, 103u8, 105u8, 99u8, 2u8][..],
        nom::error::ErrorKind::Verify
      )))
    );
  }

  #[test]
  fn test_revision() {
    assert_eq!(unwrap(revision(&WORLD[12..16])), 160);
  }

  #[test]
  fn test_favorite() {
    assert_eq!(unwrap(favorite(&WORLD[16..24])), false);
  }
}
