use super::common::*;
use crate::enums::{
  BlockShape,
  BlockType,
};
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
  LE,
};
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Block {
  pub block_type: BlockType,
  pub shape: BlockShape,
  pub frame_data: Option<Point>,
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
    let mut frame_data: Option<Point> = None;
    let mut block_paint: Option<u8> = None;

    let block_id = if ctx.has_extended_block_id {
      buf.gread_with::<u16>(offset, LE)?
    } else {
      buf.gread::<u8>(offset)? as u16
    };
    let block_type = BlockType::from_u16(block_id).unwrap();
    if ctx.tile_frame_importances[block_type as usize] {
      let x = buf.gread_with::<u16>(offset, LE)?;
      let y = buf.gread_with::<u16>(offset, LE)?;
      frame_data = Some(Point {
        x: x as i32,
        y: y as i32,
      });
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

impl<'a> TryIntoCtx<Endian> for Block {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let Self {
      block_type,
      shape: _,
      frame_data,
      block_paint,
      is_block_active: _,
    } = self;
    let block_id = block_type as u16;
    match u8::try_from(block_id) {
      Ok(block_id_u8) => buf.gwrite(block_id_u8, offset),
      Err(_) => buf.gwrite_with(block_id, offset, LE),
    }?;
    if frame_data.is_some() {
      let frame_data = frame_data.unwrap();
      buf.gwrite_with(frame_data.x as u16, offset, LE)?;
      buf.gwrite_with(frame_data.y as u16, offset, LE)?;
    };
    if block_paint.is_some() {
      let block_paint = block_paint.unwrap();
      buf.gwrite(block_paint, offset)?;
    };
    Ok(*offset)
  }
}

#[cfg(test)]
mod test_blocks {
  use super::*;
  #[test]
  fn test_block_rw() {
    let ctx = BlockCtx {
      has_extended_block_id: false,
      is_block_painted: true,
      is_block_active: true,
      shape: BlockShape::Normal,
      tile_frame_importances: &VariableTBitVec::from(vec![true]),
    };

    let block = Block {
      block_type: BlockType::Dirt,
      shape: BlockShape::Normal,
      frame_data: Some(Point { x: 100, y: 100 }),
      block_paint: Some(1),
      is_block_active: true,
    };

    let mut bytes = [0; 6];
    let size = bytes.pwrite::<Block>(block, 0).unwrap();
    assert_eq!(size, 6);

    let new_block = bytes.pread_with::<Block>(0, ctx).unwrap();
    assert_eq!(new_block, block);
  }
}
