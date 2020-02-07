use derive_new::new;
use scroll::{
  ctx::{StrCtx, TryFromCtx, TryIntoCtx},
  Endian, Pread, Pwrite, LE,
};

static RELOGIC: &str = "relogic";

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new, Pwrite, Pread)]
pub struct Offsets {
  pub header: i32,
  pub tiles: i32,
  pub chests: i32,
  pub signs: i32,
  pub npcs: i32,
  pub tile_entities: i32,
  pub pressure_plates: i32,
  pub town_manager: i32,
  pub footer: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Header {
  pub version: String,
  pub revision: u32,
  pub is_favorite: bool,
  pub offsets: Offsets,
}

impl Header {
  pub fn new<S>(version: S, revision: u32, is_favorite: bool, offsets: Offsets) -> Self
  where
    S: Into<String>,
  {
    Self {
      version: version.into(),
      revision,
      is_favorite,
      offsets,
    }
  }
}

impl<'a> TryFromCtx<'a, Endian> for Header {
  type Error = scroll::Error;

  fn try_from_ctx(buf: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let raw_version = buf.gread_with::<i32>(offset, LE)?;
    let _raw_signature = buf.gread_with::<&str>(offset, StrCtx::Length(7))?;
    let _ = buf.gread_with::<u8>(offset, LE)?;
    let revision = buf.gread_with::<u32>(offset, LE)?;
    let raw_is_favorite = buf.gread_with::<u64>(offset, LE)?;
    let is_favorite = raw_is_favorite != 0;
    // the following might be needed.
    let _raw_offset_lengths = buf.gread_with::<u16>(offset, LE)?;
    let (offsets, size) = Offsets::try_from_ctx(&buf[*offset..], ctx)?;
    *offset += size;

    let version = match raw_version {
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
      _ => Err(scroll::Error::Custom("unrecognized version".to_string())),
    }?;

    Ok((
      Header::new(version, revision, is_favorite, offsets),
      *offset,
    ))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a Header {
  type Error = scroll::Error;

  fn try_into_ctx(self, buf: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
    let Header {
      version,
      revision,
      is_favorite,
      offsets,
    } = self;
    let mut size = 0;

    let v: i32 = match version.as_str() {
      "1.2.0.3.1" => 71,
      "1.2.2" => 77,
      "1.2.3" => 104,
      "1.3.0.1" => 140,
      "1.3.0.4" => 151,
      "1.3.0.5" => 153,
      "1.3.0.6" => 154,
      "1.3.0.7" => 155,
      "1.3.0.8" => 156,
      "1.3.2" => 170,
      "1.3.3" => 174,
      "1.3.4" => 178,
      "1.3.5.3" => 194,
      _ => 0, // TODO error
    };

    size += v.try_into_ctx(&mut buf[size..], ctx)?;
    size += RELOGIC.as_bytes().try_into_ctx(&mut buf[size..], ())?;
    size += 2u8.try_into_ctx(&mut buf[size..], ctx)?;
    size += revision.try_into_ctx(&mut buf[size..], ctx)?;
    size += if *is_favorite { 1u64 } else { 0u64 }.try_into_ctx(&mut buf[size..], ctx)?;
    // TODO: there may be extra cruft at the end of offsets.
    size += 9u16.try_into_ctx(&mut buf[size..], ctx)?;
    size += offsets.try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}

#[cfg(test)]
mod test_header {
  use super::*;

  #[test]
  fn test_header_rw() {
    let header = &Header::new(
      "1.3.5.3",
      160,
      true,
      Offsets {
        header: 0,
        tiles: 2,
        chests: 4,
        signs: 6,
        npcs: 8,
        tile_entities: 10,
        pressure_plates: 12,
        town_manager: 14,
        footer: 16,
      },
    );
    let mut bytes = [0; 70];
    let _res = bytes.pwrite_with::<&Header>(header, 0, LE).unwrap();
    let parsed = &Header::try_from_ctx(&bytes[..], LE).unwrap().0;
    assert_eq!(parsed, header);
  }
}
