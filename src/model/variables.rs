use derive_new::new;
use scroll::{
  ctx::{TryFromCtx, TryIntoCtx},
  Endian, Pread, Pwrite
};
use super::common::{Point, TBool};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EvilType {
  Corruption,
  Crimson,
}

impl<'a> TryFromCtx<'a, Endian> for EvilType {
  type Error = scroll::Error;

  fn try_from_ctx(buf: &'a [u8], _ctx: Endian) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let raw_value = buf.gread::<u8>(offset)?;
    let value = if raw_value != 0 {
      Self::Crimson
    } else {
      Self::Corruption
    };
    Ok((value, *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a EvilType {
  type Error = scroll::Error;

  fn try_into_ctx(self, buf: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
    let mut size = 0;
    let value = *self as u8;
    size += value.try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, new, Pread, Pwrite)]
pub struct Variables {
  pub time: f32, // may be f64?
  pub is_daytime: TBool,
  pub moon_phase: u32,
  pub is_blood_moon: TBool,
  pub is_eclipse: TBool,
  pub dungeon_point: Point,
  pub evil_type: EvilType
}

#[cfg(test)]
mod test_variables {
  use super::*;
  use scroll::LE;

  #[test]
  fn test_variables_rw() {
    let vars = Variables {
      time: 0f32,
      is_daytime: TBool::from(true),
      moon_phase: 0u32,
      is_blood_moon: TBool::from(false),
      is_eclipse: TBool::from(true),
      dungeon_point: Point::new(0, 0),
      evil_type: EvilType::Crimson
    };
    let mut bytes = [0; 20];
    let _res = bytes.pwrite_with::<Variables>(vars, 0, LE).unwrap();
    assert_eq!(Variables::try_from_ctx(&bytes[..], LE).unwrap(), (vars, 20));
  }
}
