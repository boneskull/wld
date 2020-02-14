use super::common::*;
use crate::enums::{
  BlockShape,
  BlockType,
};
use num_traits::FromPrimitive;
use scroll::{
  ctx::TryFromCtx,
  Error as ScrollError,
  Pread,
  LE,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Block {
  pub block_type: BlockType,
  pub shape: BlockShape,
  pub frame_data: Option<(u16, u16)>,
  pub block_paint: Option<u8>,
  pub is_block_active: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BlockCtx<'a> {
  pub has_extended_block_id: bool,
  pub tile_frame_importances: &'a VariableTBitVec,
  pub is_block_painted: bool,
  pub is_block_active: bool,
  pub shape: BlockShape,
}

impl<'a> TryFromCtx<'a, BlockCtx<'a>> for Block {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: BlockCtx,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let mut frame_data: Option<(u16, u16)> = None;
    let mut block_paint: Option<u8> = None;

    let block_id = if ctx.has_extended_block_id {
      buf.gread_with::<u16>(offset, LE)? as i64
    } else {
      buf.gread::<u8>(offset)? as i64
    };
    let block_type = BlockType::from_i64(block_id).unwrap();
    if ctx.tile_frame_importances[block_type as usize] {
      let frame_data_x = buf.gread_with::<u16>(offset, LE)?;
      let frame_data_y = buf.gread_with::<u16>(offset, LE)?;
      frame_data = Some((frame_data_x, frame_data_y));
    }
    if ctx.is_block_painted {
      block_paint = Some(buf.gread::<u8>(offset)?);
    }
    Ok((
      Block {
        block_type,
        shape: ctx.shape,
        frame_data,
        block_paint,
        is_block_active: ctx.is_block_active,
      },
      *offset,
    ))
  }
}
