use crate::enums::WallType;
use num_traits::FromPrimitive;
use scroll::{
  ctx::{
    TryFromCtx,
    TryIntoCtx,
  },
  Endian,
  Error as ScrollError,
  Pread,
  Pwrite,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Wall {
  pub wall_type: WallType,
  pub wall_paint: Option<u8>,
}

impl<'a> TryFromCtx<'a, bool> for Wall {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    is_wall_painted: bool,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let wall_id = buf.gread::<u8>(offset)?;
    let wall_type = WallType::from_u8(wall_id).unwrap();
    let wall_paint: Option<u8> = if is_wall_painted {
      Some(buf.gread::<u8>(offset)?)
    } else {
      None
    };
    Ok((
      Self {
        wall_type,
        wall_paint,
      },
      *offset,
    ))
  }
}

impl TryIntoCtx<Endian> for Wall {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let Self {
      wall_type,
      wall_paint,
    } = self;
    let wall_id = wall_type as u8;
    buf.gwrite(wall_id, offset)?;
    if wall_paint.is_some() {
      buf.gwrite(wall_paint.unwrap(), offset)?;
    }
    Ok(*offset)
  }
}

#[cfg(test)]
mod test_wall {
  use super::*;

  #[test]
  fn test_wall_rw() {
    let wall = Wall {
      wall_paint: None,
      wall_type: WallType::Wood,
    };
    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(wall, 0).unwrap());
    assert_eq!(wall, buf.pread::<Wall>(0).unwrap());

    let wall = Wall {
      wall_paint: Some(1),
      wall_type: WallType::Wood,
    };
    let mut buf = [0; 2];
    assert_eq!(2, buf.pwrite(wall, 0).unwrap());
    assert_eq!(wall, buf.pread_with::<Wall>(0, true).unwrap());
  }
}
