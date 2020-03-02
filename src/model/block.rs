use crate::{
  enums::{
    BlockShape,
    BlockType,
  },
  model::{
    Position,
    VariableTBitVec,
  },
};
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
pub struct Block {
  pub block_type: BlockType,
  pub shape: BlockShape,
  // actually a u16,u16
  pub frame_data: Option<Position>,
  pub block_paint: Option<u8>,
  pub is_block_inactive: bool,
  pub has_extended_block_id: bool,
}

impl SizeWith<Block> for Block {
  fn size_with(ctx: &Block) -> usize {
    // note that shape is ignored
    (match ctx.has_extended_block_id {
      true => u16::size_with(&LE),
      false => u8::size_with(&LE),
    }) + ctx.frame_data.map_or(0, |_| (u16::size_with(&LE) * 2))
      + ctx.block_paint.map_or(0, |_| u8::size_with(&LE))
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BlockCtx<'a> {
  pub has_extended_block_id: bool,
  pub tile_frame_importances: &'a VariableTBitVec,
  pub is_block_painted: bool,
  pub is_block_inactive: bool,
  pub shape: BlockShape,
}

impl<'a> TryFromCtx<'a, BlockCtx<'a>> for Block {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: BlockCtx,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let mut frame_data: Option<Position> = None;
    let mut block_paint: Option<u8> = None;

    let block_id = if ctx.has_extended_block_id {
      buf.gread_with::<u16>(offset, LE)?
    } else {
      buf.gread::<u8>(offset)? as u16
    };
    let block_type = BlockType::from_u16(block_id).unwrap();
    if ctx.tile_frame_importances[block_id as usize] {
      let x = buf.gread_with::<u16>(offset, LE)?;
      let y = buf.gread_with::<u16>(offset, LE)?;
      frame_data = Some(Position {
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
        is_block_inactive: ctx.is_block_inactive,
        has_extended_block_id: ctx.has_extended_block_id,
      },
      *offset,
    ))
  }
}

impl<'a> TryIntoCtx<Endian> for &Block {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let Block {
      block_type,
      shape: _,
      frame_data,
      block_paint,
      is_block_inactive: _,
      has_extended_block_id,
    } = self;
    let block_id = *block_type as u16;
    match has_extended_block_id {
      true => {
        buf.gwrite_with(block_id, offset, LE)?;
      }
      false => {
        buf.gwrite(block_id as u8, offset)?;
      }
    }
    match frame_data {
      Some(fd) => {
        buf.gwrite_with(fd.x as u16, offset, LE)?;
        buf.gwrite_with(fd.y as u16, offset, LE)?;
      }
      _ => {}
    };
    match block_paint {
      Some(bp) => {
        buf.gwrite(bp, offset)?;
      }
      _ => {}
    };
    assert!(*offset == Block::size_with(&self), "Block size mismatch");
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
      is_block_inactive: true,
      shape: BlockShape::Normal,
      tile_frame_importances: &VariableTBitVec::from(vec![true]),
    };

    let block = Block {
      block_type: BlockType::Dirt,
      shape: BlockShape::Normal,
      frame_data: Some(Position { x: 100, y: 100 }),
      block_paint: Some(1),
      is_block_inactive: true,
      has_extended_block_id: false,
    };

    let mut bytes = [0; 6];
    let size = bytes.pwrite::<&Block>(&block, 0).unwrap();
    assert_eq!(size, 6);

    let new_block = bytes.pread_with::<Block>(0, ctx).unwrap();
    assert_eq!(new_block, block);
  }
}
