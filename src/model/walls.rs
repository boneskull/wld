use crate::enums::WallType;
use num_traits::FromPrimitive;
use scroll::{
  ctx::TryFromCtx,
  Error as ScrollError,
  Pread,
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
