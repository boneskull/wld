use crate::{
  enums::TBool,
  models::{
    TString,
    WorldCtx,
  },
};
use scroll::{
  ctx::{
    SizeWith,
    TryFromCtx,
    TryIntoCtx,
  },
  Error as ScrollError,
  Pread,
  Pwrite,
  LE,
};

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Footer;

impl<'a> TryFromCtx<'a, WorldCtx<'a>> for Footer {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: WorldCtx<'a>,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let is_valid = buf.gread::<TBool>(offset)?;
    if is_valid == TBool::False {
      return Err(ScrollError::Custom("invalid footer".to_string()));
    }
    let name = buf.gread::<TString>(offset)?;
    if name != *ctx.name {
      return Err(ScrollError::Custom(format!(
        "invalid footer: name mismatch ({:?} vs. {:?})",
        name, *ctx.name
      )));
    }
    let id = buf.gread_with::<i32>(offset, LE)?;
    if id != *ctx.id {
      return Err(ScrollError::Custom(
        "invalid footer: id mismatch".to_string(),
      ));
    }
    Ok((Self, *offset))
  }
}

impl<'a> TryIntoCtx<WorldCtx<'a>> for &Footer {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: WorldCtx<'a>,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    buf.gwrite(&TBool::True, offset)?;
    buf.gwrite(ctx.name, offset)?;
    buf.gwrite_with(ctx.id, offset, LE)?;
    Ok(*offset)
  }
}

impl<'a> SizeWith<WorldCtx<'a>> for Footer {
  fn size_with(ctx: &WorldCtx) -> usize {
    let size = TBool::size_with(&LE)
      + TString::size_with(ctx.name)
      + i32::size_with(&LE);
    debug!("Footer size: {}", size);
    size
  }
}
