use crate::model::point::Point;
use nom::{
  combinator::map,
  multi::{count, length_value, many0, many1},
  number::complete::{le_i32, le_u8},
  IResult,
};
use std::string::String;

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

pub fn point(buf: &[u8]) -> IResult<&[u8], Point> {
  // y, x
  map(count(le_i32, 2), |v| Point::new(v[1], v[0]))(buf)
}

pub fn bool(buf: &[u8]) -> IResult<&[u8], bool> {
  map(le_u8, |flag| flag != 0)(buf)
}
