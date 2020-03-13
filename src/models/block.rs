use crate::{
  enums::{
    BlockShape,
    BlockType,
  },
  models::VariableTBitVec,
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

/// Represents a block.  A [`Tile`] optionally "has" a [`Block`].
///
/// See [Terraria Wiki: Blocks] for more information.
///
/// [`Tile`]: crate::models::Tile
/// [Terraria Wiki: Blocks]: https://terraria.gamepedia.com/Blocks
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Block {
  /// The type of this block.  Dirt, brick, glass, etc.
  pub block_type: BlockType,

  /// The shape of this block. Will be [`BlockShape::Normal`] unless otherwise specified.
  pub shape: BlockShape,

  /// Optional frame data.  Set if
  /// [`Properties::tile_frame_importances`](crate::models::Properties::tile_frame_importances)
  /// has data at the [parent `Tile`'s position](crate::models::Tile::position).
  pub frame_data: Option<(u16, u16)>,

  /// Optional paint information, if painted.
  ///
  /// _TODO: This should probably have its own enum._
  pub block_paint: Option<u8>,

  /// From
  /// [`ExtendedTileAttributes::is_block_inactive`](crate::models::ExtendedTileAttributes::is_block_inactive)
  /// by way of [`BlockCtx`].
  pub is_block_inactive: bool,

  /// From
  /// [`TileHeader::has_extended_block_id`](crate::models::TileHeader::has_extended_block_id)
  /// by way of [`BlockCtx`].
  pub has_extended_block_id: bool,
}

impl SizeWith<Block> for Block {
  fn size_with(ctx: &Self) -> usize {
    // note that shape is ignored
    (if ctx.has_extended_block_id {
      u16::size_with(&LE)
    } else {
      u8::size_with(&LE)
    }) + ctx.frame_data.map_or(0, |_| (u16::size_with(&LE) * 2))
      + ctx.block_paint.map_or(0, |_| u8::size_with(&LE))
  }
}

/// Contains some metadata from the [`Tile`] instance to which a
/// [`Block`] belongs.
///
/// Only used when reading a [`Block`] from a slice of bytes via [`scroll::ctx::TryFromCtx`].
///
/// [`Tile`]: crate::models::Tile
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct BlockCtx<'a> {
  /// From [`TileHeader::has_extended_block_id`](crate::models::TileHeader::has_extended_block_id).
  pub has_extended_block_id: bool,
  /// From [`Properties::tile_frame_importances`](crate::models::Properties::tile_frame_importances).
  pub tile_frame_importances: &'a VariableTBitVec,
  /// From [`ExtendedTileAttributes::is_block_painted`](crate::models::ExtendedTileAttributes::is_block_painted).
  pub is_block_painted: bool,
  /// From [`ExtendedTileAttributes::is_block_inactive`](crate::models::ExtendedTileAttributes::is_block_inactive).
  pub is_block_inactive: bool,
  /// From [`TileAttributes::shape`](crate::models::TileAttributes::shape).
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
      buf.gread_with::<u16>(offset, LE)?
    } else {
      u16::from(buf.gread::<u8>(offset)?)
    };
    let block_type = BlockType::from_u16(block_id).unwrap();
    if ctx.tile_frame_importances[block_id as usize] {
      let x = buf.gread_with::<u16>(offset, LE)?;
      let y = buf.gread_with::<u16>(offset, LE)?;
      frame_data = Some((x, y));
    }
    if ctx.is_block_painted {
      block_paint = Some(buf.gread::<u8>(offset)?);
    }
    Ok((
      Self {
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
    if let Some(fd) = frame_data {
      buf.gwrite_with(fd.0, offset, LE)?;
      buf.gwrite_with(fd.1, offset, LE)?;
    };
    if let Some(bp) = block_paint {
      buf.gwrite(bp, offset)?;
    };
    assert!(*offset == Block::size_with(self), "Block size mismatch");
    Ok(*offset)
  }
}

#[cfg(test)]
mod test_blocks {
  use super::{
    Block,
    BlockCtx,
    BlockShape,
    BlockType,
    Pread,
    Pwrite,
    VariableTBitVec,
  };
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
      frame_data: Some((100, 100)),
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
