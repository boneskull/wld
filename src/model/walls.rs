use crate::enums::WallType;
use num_traits::FromPrimitive;
use scroll::{
  ctx::{
    SizeWith,
    TryFromCtx,
    TryIntoCtx,
  },
  Endian,
  Error as ScrollError,
  Pread,
  Pwrite,
  LE,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Wall {
  pub wall_type: WallType,
  pub wall_paint: Option<u8>,
}

impl SizeWith<Wall> for Wall {
  fn size_with(ctx: &Self) -> usize {
    WallType::size_with(&LE) + ctx.wall_paint.map_or(0, |_| u8::size_with(&LE))
  }
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

impl TryIntoCtx<Endian> for &Wall {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let Wall {
      wall_type,
      wall_paint,
    } = self;
    let wall_id = *wall_type as u8;
    buf.gwrite(wall_id, offset)?;
    if let Some(wp) = wall_paint {
      buf.gwrite(wp, offset)?;
    }
    let expected_size = Wall::size_with(self);
    assert!(
      expected_size == *offset,
      "Wall offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );

    Ok(*offset)
  }
}

#[cfg(test)]
mod test_wall {
  use super::{
    Pread,
    Pwrite,
    SizeWith,
    Wall,
    WallType,
  };

  #[test]
  fn test_wall_no_paint_rw() {
    let wall = Wall {
      wall_paint: None,
      wall_type: WallType::Wood,
    };
    let mut buf = [0; 1];
    assert_eq!(Wall::size_with(&wall), buf.pwrite(&wall, 0).unwrap());
    assert_eq!(wall, buf.pread::<Wall>(0).unwrap());
  }

  #[test]
  fn test_wall_with_paint_rw() {
    let wall = Wall {
      wall_paint: Some(1),
      wall_type: WallType::Wood,
    };
    let mut buf = [0; 2];
    assert_eq!(Wall::size_with(&wall), buf.pwrite(&wall, 0).unwrap());
    assert_eq!(wall, buf.pread_with::<Wall>(0, true).unwrap());
  }
}
