use crate::model::header::*;
use crate::parser::common::*;
use nom::{
  combinator::{map, map_res, value, verify},
  multi::{length_value, many0, many_m_n},
  number::complete::{le_i32, le_u16, le_u32, le_u8},
  sequence::tuple,
  IResult,
};
use std::mem::size_of;
use std::str::from_utf8;

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

pub fn offsets(buf: &[u8]) -> IResult<&[u8], Offsets> {
  map(
    length_value(
      map(le_u16, |count: u16| count * size_of::<i32>() as u16),
      many0(le_i32),
    ),
    |v| Offsets::new(v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8]),
  )(buf)
}

pub fn parse_header(buf: &[u8]) -> IResult<&[u8], Header> {
  let (buf, version) = version(buf)?;
  let (buf, _) = metadata(buf)?;
  let (buf, revision) = revision(buf)?;
  let (buf, is_favorite) = is_favorite(buf)?;
  let (buf, offsets) = offsets(buf)?;
  Ok((
    buf,
    Header::new(String::from(version), revision, is_favorite, offsets),
  ))
}

#[cfg(test)]
mod test_header {
  use super::*;
  use crate::test_helpers::{unwrap as ok, *};

  #[test]
  fn test_offsets() {
    let len = &9u16.to_le_bytes();
    let offs = &[0i32.to_le_bytes(); 9].concat();
    let mut v = vec![];
    v.extend_from_slice(len);
    v.extend(offs);

    assert_eq!(
      // unwrap(positions(&WORLD[24..66])),
      // &[127, 2802, 2860224, 2879758, 2880141, 2880453, 2880457, 2880461, 2880489, 0]
      ok(offsets(&v)),
      Offsets {
        header: 0,
        tiles: 0,
        chests: 0,
        signs: 0,
        npcs: 0,
        tile_entities: 0,
        pressure_plates: 0,
        town_manager: 0,
        footer: 0
      }
    );
  }

  #[test]
  fn test_parse_header() {
    let mut v = vec![];
    {
      let len = &9u16.to_le_bytes();
      let offs = &[0i32.to_le_bytes(); 9].concat();
      v.extend_from_slice(len);
      v.extend(offs);
    }
    assert_eq!(
      ok(parse_header(
        &[
          &194i32.to_le_bytes(),
          "relogic".as_bytes(),
          &2u8.to_le_bytes(),
          &160u32.to_le_bytes(),
          &1u8.to_le_bytes(),
          &v[..]
        ]
        .concat()
      )),
      Header::new(
        String::from("1.3.5.3"),
        160,
        true,
        Offsets {
          header: 0,
          tiles: 0,
          chests: 0,
          signs: 0,
          npcs: 0,
          tile_entities: 0,
          pressure_plates: 0,
          town_manager: 0,
          footer: 0
        }
      )
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
