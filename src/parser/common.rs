use crate::model::common::*;
use nom::{
  combinator::map,
  multi::{count, length_value, many0, many1},
  number::complete::{le_i32, le_u8},
  IResult,
};
use std::string::String;

fn to_string(v: Vec<u8>) -> String {
  String::from_utf8_lossy(&v).to_string()
}

pub fn string1(buf: &[u8]) -> IResult<&[u8], String> {
  map(length_value(le_u8, many1(le_u8)), to_string)(buf)
}

pub fn string0(buf: &[u8]) -> IResult<&[u8], String> {
  map(length_value(le_u8, many0(le_u8)), to_string)(buf)
}

pub fn point(buf: &[u8]) -> IResult<&[u8], Point> {
  // y, x
  map(count(le_i32, 2), |v| Point::new(v[1], v[0]))(buf)
}

pub fn bool(buf: &[u8]) -> IResult<&[u8], bool> {
  map(le_u8, |flag| flag != 0)(buf)
}

#[cfg(test)]
mod test_common {
  // this is helpful:
  // https://cryptii.com/pipes/integer-converter

  use super::*;
  use crate::test_helpers::{*, unwrap as ok};

  #[test]
  fn test_bool() {
    assert_eq!(ok(bool(&[1])), true);
    assert_eq!(ok(bool(&[0])), false);
    assert_eq!(bool(&[]), Err(NomError((&[][..], ErrorKind::Eof))))
  }

  #[test]
  fn test_point() {
    assert_eq!(
      ok(point(&[666i32.to_le_bytes(), 999i32.to_le_bytes()].concat())),
      Point { x: 999, y: 666 }
    );
    assert_eq!(
      point(&666i32.to_le_bytes()),
      Err(NomError((&[][..], ErrorKind::Eof)))
    );
  }

  #[test]
  fn test_string0() {
    assert_eq!(
      ok(string0(&p_string("boneskull"))),
      String::from("boneskull")
    );
    assert_eq!(ok(string0(&p_string(""))), String::from(""));
    assert_eq!(
      string0(&[]),
      Err(NomError((&[][..], ErrorKind::Eof)))
    );
  }

  #[test]
  fn test_string1() {
    assert_eq!(
      ok(string0(&p_string("boneskull"))),
      String::from("boneskull")
    );
    assert_eq!(
      string1(&[0]),
      Err(NomError((&[][..], ErrorKind::Eof)))
    );
    assert_eq!(
      string0(&[]),
      Err(NomError((&[][..], ErrorKind::Eof)))
    );
  }
}
