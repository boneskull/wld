use super::{
  blocks::*,
  items::*,
  tile_entity::*,
  walls::*,
};
use crate::{
  enums::{
    BlockShape,
    LiquidType,
    RLEType,
  },
  model::common::*,
};
use scroll::{
  ctx::TryFromCtx,
  Error as ScrollError,
  Pread,
  LE,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Liquid {
  pub liquid_type: LiquidType,
  pub volume: u8,
}

impl<'a> TryFromCtx<'a, LiquidType> for Liquid {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    liquid_type: LiquidType,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    Ok((
      Self {
        liquid_type,
        volume: buf.gread::<u8>(offset)?,
      },
      *offset,
    ))
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Wiring {
  pub red: bool,
  pub green: bool,
  pub blue: bool,
  pub yellow: bool,
  pub actuator: bool,
}

impl From<(&TBitVec, &TBitVec)> for Wiring {
  fn from(flags: (&TBitVec, &TBitVec)) -> Self {
    let (flags, more_flags) = flags;
    Self {
      red: flags[1],
      green: flags[2],
      blue: flags[3],
      yellow: more_flags[5],
      actuator: more_flags[1],
    }
  }
}

impl From<&TBitVec> for Wiring {
  fn from(flags: &TBitVec) -> Self {
    Self {
      red: flags[1],
      green: flags[2],
      blue: flags[3],
      yellow: false,
      actuator: false,
    }
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Deref)]
pub struct RunLength(u16);

impl<'a> TryFromCtx<'a, RLEType> for RunLength {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    rle_type: RLEType,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let run_length = match rle_type {
      RLEType::DoubleByte => buf.gread_with::<u16>(offset, LE)? + 1,
      RLEType::SingleByte => buf.gread::<u8>(offset)? as u16 + 1,
      _ => 1,
    };
    Ok((Self(run_length), *offset))
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
  pub block: Option<Block>,
  pub wall: Option<Wall>,
  pub liquid: Option<Liquid>,
  pub wiring: Option<Wiring>,
  pub chest: Option<Chest>,
  pub sign: Option<Sign>,
  pub tile_entity: Option<TileEntity>,
  pub pressure_plate: Option<PressurePlate>,
  pub run_length: RunLength,
}

impl<'a> TryFromCtx<'a, WorldCtx<'a>> for Tile {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: WorldCtx,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let mut is_block_active = true;
    let mut is_wall_painted = false;
    let mut is_block_painted = false;
    let mut wiring: Option<Wiring> = None;
    let mut block: Option<Block> = None;
    let mut wall: Option<Wall> = None;
    let mut liquid: Option<Liquid> = None;
    let mut shape = BlockShape::Normal;

    let flags = buf.gread::<TBitVec>(offset)?;
    let has_more_flags = flags[0];
    let has_block = flags[1];
    let has_wall = flags[2];
    let liquid_type = LiquidType::from(&flags);
    let has_extended_block_id = flags[5];
    let rle_type = RLEType::from(&flags);

    if has_more_flags {
      let more_flags = buf.gread::<TBitVec>(offset)?;
      let has_even_more_flags = more_flags[0];
      shape = BlockShape::from(&more_flags);
      if has_even_more_flags {
        let even_more_flags = buf.gread::<TBitVec>(offset)?;
        is_block_active = !even_more_flags[2];
        wiring = Some(Wiring::from((&more_flags, &even_more_flags)));
        is_block_painted = even_more_flags[3];
        is_wall_painted = even_more_flags[4];
      } else {
        wiring = Some(Wiring::from(&more_flags));
      }
    }

    if has_block {
      block = Some(buf.gread_with::<Block>(
        offset,
        BlockCtx {
          has_extended_block_id,
          is_block_active,
          is_block_painted,
          tile_frame_importances: ctx.tile_frame_importances,
          shape: shape,
        },
      )?);
    }

    if has_wall {
      wall = Some(buf.gread_with::<Wall>(offset, is_wall_painted)?);
    }

    if liquid_type != LiquidType::NoLiquid {
      liquid = Some(buf.gread_with::<Liquid>(offset, liquid_type)?);
    }

    let run_length = buf.gread_with::<RunLength>(offset, rle_type)?;
    Ok((
      Tile {
        block,
        wall,
        liquid,
        wiring,
        chest: None,
        sign: None,
        run_length,
        tile_entity: None,
        pressure_plate: None,
      },
      *offset,
    ))
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Constructor)]
pub struct WorldCtx<'a> {
  pub world_width: &'a i32,
  pub world_height: &'a i32,
  pub tile_frame_importances: &'a VariableTBitVec,
  pub id: &'a i32,
  pub name: &'a TString,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, IndexMut, Index)]
pub struct TileVec(Vec<Tile>);

impl<'a> TryFromCtx<'a, WorldCtx<'a>> for TileVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: WorldCtx,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let size = *ctx.world_height as usize;
    let mut tiles: Vec<Tile> = Vec::with_capacity(size);
    while tiles.len() < size {
      let tile = buf.gread_with::<Tile>(offset, ctx)?;
      for _ in 0..*tile.run_length {
        tiles.push(tile.clone());
      }
    }
    Ok((TileVec(tiles), *offset))
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, AsMut, Index)]
pub struct TileMatrix(Vec<TileVec>);

impl TileMatrix {
  pub fn tile_at_point(&mut self, point: &Point) -> &mut Tile {
    &mut self.as_mut()[point.x as usize][point.y as usize]
  }
}

impl<'a> TryFromCtx<'a, WorldCtx<'a>> for TileMatrix {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: WorldCtx,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let row_count = *ctx.world_width as usize;
    let mut matrix: Vec<TileVec> = Vec::with_capacity(row_count);
    while matrix.len() < row_count {
      let column = buf.gread_with::<TileVec>(offset, ctx)?;
      matrix.push(column);
    }
    Ok((TileMatrix(matrix), *offset))
  }
}
