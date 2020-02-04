use crate::model::world::Header;
use crate::parser::common::*;
use std::str::from_utf8;

use nom::{
  combinator::{map_res, value, verify},
  multi::many_m_n,
  number::complete::{le_i32, le_u32, le_u8},
  sequence::tuple,
  IResult,
};

fn version(buf: &[u8]) -> IResult<&[u8], &str> {
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

fn metadata(buf: &[u8]) -> IResult<&[u8], (bool, u8)> {
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

fn revision(buf: &[u8]) -> IResult<&[u8], u32> {
  le_u32(buf)
}

fn is_favorite(buf: &[u8]) -> IResult<&[u8], bool> {
  bool(buf)
}

pub fn parse_header(buf: &[u8]) -> IResult<&[u8], Header> {
  let (buf, version) = version(buf)?;
  let (buf, _) = metadata(buf)?;
  let (buf, revision) = revision(buf)?;
  let (buf, is_favorite) = is_favorite(buf)?;
  Ok((
    buf,
    Header::new(String::from(version), revision, is_favorite),
  ))
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::test_helpers::{*, unwrap as ok};

  #[test]
  fn test_parse_header() {
    assert_eq!(
      ok(parse_header(
        &[
          &194i32.to_le_bytes(),
          "relogic".as_bytes(),
          &2u8.to_le_bytes(),
          &160u32.to_le_bytes(),
          &1u8.to_le_bytes()
        ]
        .concat()
      )),
      Header::new(String::from("1.3.5.3"), 160, true)
    );
  }

  #[test]
  fn test_version() {
    assert_eq!(ok(version(&178i32.to_le_bytes())), "1.3.4");
    assert_eq!(
      version(&123i32.to_le_bytes()),
      Err(NomError((&123i32.to_le_bytes()[..], ErrorKind::MapRes)))
    );
  }

  #[test]
  fn test_metadata() {
    assert_eq!(
      ok(metadata(&["relogic".as_bytes(), &[0x02]].concat())),
      (true, 2)
    );
    assert_eq!(
      metadata(&["rel0gic".as_bytes(), &[0x02]].concat()),
      Err(nom::Err::Error((
        &["rel0gic".as_bytes(), &[0x02]].concat()[..],
        ErrorKind::Verify
      )))
    );
  }

  #[test]
  fn test_revision() {
    assert_eq!(ok(revision(&160u32.to_le_bytes())), 160);
  }

  #[test]
  fn test_is_favorite() {
    assert_eq!(ok(is_favorite(&1u8.to_le_bytes())), true);
    assert_eq!(ok(is_favorite(&0u8.to_le_bytes())), false);
  }
}
