use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::BufRead;
use std::string::String;
use std::vec::Vec;

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

#[derive(Clone, Default, Debug)]
pub struct Mount {
  pub device: String,
  pub mount_point: String,
  pub file_system_type: String,
  pub options: Vec<String>,
}

/// Implements `Display` for `Mount` to simulate behavior of Unix mount command.
///
/// # Examples
/// ```
/// # use wld::Mount;
/// # use std::string::String;
/// let mount = Mount {
/// 	device: String::from("/dev/sda1"),
/// 	mount_point: String::from("/mnt/disk"),
/// 	file_system_type: String::from("ext4"),
/// 	options: vec![String::from("ro"), String::from("nosuid")]
/// };
/// assert!(mount.to_string() == "/dev/sda1 on /mnt/disk type ext4 (ro,nosuid)");
/// ```
impl std::fmt::Display for Mount {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{} on {} type {} ({})",
      self.device,
      self.mount_point,
      self.file_system_type,
      self.options.join(",")
    )
  }
}

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

pub(self) mod parsers {
  use super::Mount;
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
    let (i, file_system_type) = not_whitespace(i)?; // file_system_type
    let (i, _) = space1(i)?;
    let (i, options) = mount_opts(i)?; // options
    let (i, _) = all_consuming(tuple((space1, char('0'), space1, char('0'), space0)))(i)?;
    Ok((
      i,
      Mount {
        device: device,
        mount_point: mount_point,
        file_system_type: file_system_type.to_string(),
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
      let mount1 = Mount {
        device: "device".to_string(),
        mount_point: "mount_point".to_string(),
        file_system_type: "file_system_type".to_string(),
        options: vec![
          "options".to_string(),
          "a".to_string(),
          "b=c".to_string(),
          "d e".to_string(),
        ],
      };
      let (_, mount2) =
        parse_line("device mount_point file_system_type options,a,b=c,d\\040e 0 0").unwrap();
      assert_eq!(mount1.device, mount2.device);
      assert_eq!(mount1.mount_point, mount2.mount_point);
      assert_eq!(mount1.file_system_type, mount2.file_system_type);
      assert_eq!(mount1.options, mount2.options);
    }

    #[test]
    fn test_parse_line_wsl2() {
      let mount3 = Mount {
        device: "C:\\".to_string(),
        mount_point: "/mnt/c".to_string(),
        file_system_type: "9p".to_string(),
        options: vec![
          "rw".to_string(),
          "dirsync".to_string(),
          "noatime".to_string(),
          "aname=drvfs;path=C:\\;uid=1000;gid=1000;symlinkroot=/mnt/".to_string(),
          "mmap".to_string(),
          "access=client".to_string(),
          "msize=65536".to_string(),
          "trans=fd".to_string(),
          "rfd=8".to_string(),
          "wfd=8".to_string(),
        ],
      };
      let (_, mount4) =
        parse_line("C:\\134 /mnt/c 9p rw,dirsync,noatime,aname=drvfs;path=C:\\;uid=1000;gid=1000;symlinkroot=/mnt/,mmap,access=client,msize=65536,trans=fd,rfd=8,wfd=8 0 0").unwrap();
      assert_eq!(mount3.device, mount4.device);
      assert_eq!(mount3.mount_point, mount4.mount_point);
      assert_eq!(mount3.file_system_type, mount4.file_system_type);
      assert_eq!(mount3.options, mount4.options);
    }
  }
}
