#[macro_use]
mod mount;

#[cfg(test)]
use lazy_static::*;
pub use mount::Mount;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
#[cfg(test)]
use std::fs::read;
use std::io::BufRead;

#[cfg(test)]
lazy_static! {
  static ref WORLD: std::vec::Vec<u8> =
    read("./src/tests/fixtures/Foon.wld").expect("Unable to read file");
}

#[derive(Default)]
pub struct ParseError;

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "A parsing error occurred.")
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    <ParseError as Display>::fmt(self, f)
  }
}

impl Error for ParseError {}

pub fn mounts() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
  let file = std::fs::File::open("/proc/mounts")?;
  let buf_reader = std::io::BufReader::new(file);
  for line in buf_reader.lines() {
    match parsers::parse_line(&line?[..]) {
      Ok((_, m)) => {
        println!("{}", m);
      }
      Err(_) => return Err(ParseError::default().into()),
    }
  }
  Ok(())
}

pub(self) mod props {
  use nom::character::complete::char;
  use nom::combinator::map;
  use nom::combinator::map_parser;
  use nom::combinator::rest;
  use nom::multi::{length_value, many0};
  use nom::number::complete::le_u16;
  use nom::IResult;
  use std::mem::size_of;

  fn name(buf: &[u8]) -> IResult<&[u8], &[u8]> {
    length_value(map(le_u16, |count| count * size_of::<u8>() as u16), rest)(buf)
  }

  #[cfg(test)]
  mod tests {
    use super::super::WORLD;
    use super::*;

    #[test]
    fn test_name() {
      assert_eq!(name(&WORLD[538..570]).unwrap().1, "Butts");
    }
  }
}

pub(self) mod header {
  use nom::combinator::map;
  use nom::multi::{fold_many0, length_value, many0};
  use nom::number::complete::{le_i32, le_u16, le_u32, le_u64, le_u8};
  use nom::IResult;
  use std::mem::size_of;

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
    map(le_u64, |flag: u64| flag > 0)(buf)
  }

  fn positions(buf: &[u8]) -> IResult<&[u8], Vec<i32>> {
    length_value(
      map(le_u16, |count: u16| count * size_of::<i32>() as u16),
      many0(le_i32),
    )(buf)
  }

  // this should return whatever a "Set" is
  fn importances(buf: &[u8]) -> IResult<&[u8], Vec<bool>> {
    length_value(
      map(le_u16, |count| count * size_of::<u8>() as u16),
      fold_many0(le_u8, Vec::new(), |mut acc, item| {
        let mut mask: u8 = 1;
        acc.push(item & mask == mask);
        while mask < 128 {
          mask = mask << 1;
          acc.push(item & mask == mask);
        }
        acc
      }),
    )(buf)
  }

  #[cfg(test)]
  mod tests {
    use super::super::WORLD;
    use super::*;

    #[test]
    fn test_version() {
      assert_eq!(version(&WORLD[0..4]).unwrap().1, 194);
    }

    #[test]
    fn test_metadata() {
      assert_eq!(metadata(&WORLD[4..12]).unwrap().1, 172097103742133618);
    }

    #[test]
    fn test_revision() {
      assert_eq!(revision(&WORLD[12..16]).unwrap().1, 160);
    }

    #[test]
    fn test_favorite() {
      assert_eq!(favorite(&WORLD[16..24]).unwrap().1, false);
    }

    #[test]
    fn test_positions() {
      assert_eq!(
        positions(&WORLD[24..66]).unwrap().1,
        &[127, 2802, 2860224, 2879758, 2880141, 2880453, 2880457, 2880461, 2880489, 0]
      );
    }

    #[test]
    fn test_importances() {
      assert_eq!(
        importances(&WORLD[66..538]).unwrap().1[..32],
        [
          false, false, false, true, true, true, false, false, false, false, true, true, true,
          true, true, true, true, true, true, true, true, true, false, false, true, false, true,
          true, true, true, false, true
        ]
      );
    }
  }
}

pub(self) mod parsers {
  use super::*;
  use nom::bytes::complete::{escaped_transform, is_not, tag};
  use nom::character::complete::{char, space0, space1};
  use nom::combinator::{all_consuming, map_parser, recognize, value};
  use nom::multi::separated_list;
  use nom::sequence::tuple;

  fn not_whitespace(i: &str) -> nom::IResult<&str, &str> {
    is_not(" \t")(i)
  }

  fn escaped_space(i: &str) -> nom::IResult<&str, &str> {
    value(" ", tag("040"))(i)
  }

  fn windows_backslash(i: &str) -> nom::IResult<&str, &str> {
    value("\\", tag("134"))(i)
  }

  fn windows_options_backslash(i: &str) -> nom::IResult<&str, &str> {
    value("\\;", tag(";"))(i)
  }

  fn escaped_backslash(i: &str) -> nom::IResult<&str, &str> {
    recognize(char('\\'))(i)
  }

  fn transform_escaped(i: &str) -> nom::IResult<&str, std::string::String> {
    escaped_transform(
      is_not("\\"),
      '\\',
      nom::branch::alt((
        escaped_backslash,
        windows_backslash,
        escaped_space,
        windows_options_backslash,
      )),
    )(i)
  }

  fn mount_opts(i: &str) -> nom::IResult<&str, std::vec::Vec<std::string::String>> {
    separated_list(char(','), map_parser(is_not(", \t"), transform_escaped))(i)
  }

  pub fn parse_line(i: &str) -> nom::IResult<&str, Mount> {
    let (i, device) = map_parser(not_whitespace, transform_escaped)(i)?; // device
    let (i, _) = space1(i)?;
    let (i, mount_point) = map_parser(not_whitespace, transform_escaped)(i)?; // mount_point
    let (i, _) = space1(i)?;
    let (i, file_system_type) = map_parser(not_whitespace, transform_escaped)(i)?; // file_system_type
    let (i, _) = space1(i)?;
    let (i, options) = mount_opts(i)?; // options
    let (i, _) = all_consuming(tuple((space1, char('0'), space1, char('0'), space0)))(i)?;
    Ok((
      i,
      Mount {
        device: device,
        mount_point: mount_point,
        file_system_type: file_system_type,
        options: options,
      },
    ))
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_not_whitespace() {
      assert_eq!(not_whitespace("abcd efg"), Ok((" efg", "abcd")));
      assert_eq!(not_whitespace("abcd\tefg"), Ok(("\tefg", "abcd")));
      assert_eq!(
        not_whitespace(" abcdefg"),
        Err(nom::Err::Error((" abcdefg", nom::error::ErrorKind::IsNot)))
      );
    }

    #[test]
    fn test_escaped_space() {
      assert_eq!(escaped_space("040"), Ok(("", " ")));
      assert_eq!(
        escaped_space(" "),
        Err(nom::Err::Error((" ", nom::error::ErrorKind::Tag)))
      );
    }

    #[test]
    fn test_escaped_backslash() {
      assert_eq!(escaped_backslash("\\"), Ok(("", "\\")));
      assert_eq!(
        escaped_backslash("not a backslash"),
        Err(nom::Err::Error((
          "not a backslash",
          nom::error::ErrorKind::Char
        )))
      );
    }

    #[test]
    fn test_windows_backslash() {
      assert_eq!(windows_backslash("134"), Ok(("", "\\")));
      assert_eq!(
        windows_backslash("not a backslash"),
        Err(nom::Err::Error((
          "not a backslash",
          nom::error::ErrorKind::Tag
        )))
      );
    }

    #[test]
    fn test_transform_escaped() {
      assert_eq!(
        transform_escaped("abc\\040def\\\\g\\040h"),
        Ok(("", std::string::String::from("abc def\\g h")))
      );
      assert_eq!(
        transform_escaped("\\bad"),
        Err(nom::Err::Error(("bad", nom::error::ErrorKind::Tag)))
      );
    }

    #[test]
    fn test_mount_opts() {
      assert_eq!(
        mount_opts("a,bc,d\\040e"),
        Ok((
          "",
          vec!["a".to_string(), "bc".to_string(), "d e".to_string()]
        ))
      );
    }

    #[test]
    fn test_parse_line() {
      let mount1 = mount!(
        "device",
        "mount_point",
        "file_system_type",
        "options",
        "a",
        "b=c",
        "d e"
      );
      let (_, mount2) =
        parse_line("device mount_point file_system_type options,a,b=c,d\\040e 0 0").unwrap();
      assert_eq!(mount1.device, mount2.device);
      assert_eq!(mount1.mount_point, mount2.mount_point);
      assert_eq!(mount1.file_system_type, mount2.file_system_type);
      assert_eq!(mount1.options, mount2.options);
    }

    #[test]
    fn test_parse_line_wsl2() {
      let mount3 = mount!(
        "C:\\",
        "/mnt/c",
        "9p",
        "rw",
        "dirsync",
        "noatime",
        "aname=drvfs;path=C:\\;uid=1000;gid=1000;symlinkroot=/mnt/",
        "mmap",
        "access=client",
        "msize=65536",
        "trans=fd",
        "rfd=8",
        "wfd=8"
      );
      let (_, mount4) =
        parse_line("C:\\134 /mnt/c 9p rw,dirsync,noatime,aname=drvfs;path=C:\\;uid=1000;gid=1000;symlinkroot=/mnt/,mmap,access=client,msize=65536,trans=fd,rfd=8,wfd=8 0 0").unwrap();
      assert_eq!(mount3.device, mount4.device);
      assert_eq!(mount3.mount_point, mount4.mount_point);
      assert_eq!(mount3.file_system_type, mount4.file_system_type);
      assert_eq!(mount3.options, mount4.options);
    }
  }
}
