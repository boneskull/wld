use crate::{
  enums::{
    BlockShape,
    LiquidType,
    RLEType,
  },
  model::{
    block::*,
    common::*,
    items::*,
    tile_entity::*,
    walls::*,
  },
};
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

impl TryIntoCtx<Endian> for Liquid {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    buf.gwrite(self.volume, offset)?;
    Ok(*offset)
  }
}

/**
Represents wires on a [`Tile`].  A `Tile` can have all of these, none of these, or any in between.

For more information, see [Wire](https://terraria.gamepedia.com/Wire) on the [Official Terraria Wiki](https://terraria.gamepedia.com).
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Wiring {
  /// If `true`, a red wire is present
  pub red: bool,
  /// If `true`, a green wire is present
  pub green: bool,
  /// If `true`, a blue wire is present
  pub blue: bool,
  /// If `true`, a yellow wire is present
  pub yellow: bool,
  /// If `true`, an actuator is present
  pub actuator: bool,
}

impl From<(&TBitVec, &TBitVec)> for Wiring {
  /// Used when extended attributes are present.  Extended attributes are used
  /// if [`yellow`](Wiring::yellow) or [`actuator`](Wiring::actuator) should be
  /// set.
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

impl Wiring {
  /// Flips the bit flags in a [`TBitVec`] depending on our internal flags.
  /// If either [`Wiring::yellow`] or [`Wiring::actuator`] is `true`, the
  /// `TBitVec` should be two (2) bytes in size (length 16).
  pub fn assign_bits(&self, attrs: &mut TBitVec) {
    let attrs = attrs.as_mut();
    if self.red {
      attrs.set(1, true);
    }
    if self.green {
      attrs.set(2, true);
    }
    if self.blue {
      attrs.set(3, true);
    }
    if self.yellow {
      attrs.set(13, true);
    }
    if self.actuator {
      attrs.set(8, true);
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

impl TryIntoCtx<Endian> for RunLength {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let value = *self;
    // this might be wrong, and we might need an RLEType
    match u8::try_from(value) {
      Ok(value_u8) => buf.gwrite(value_u8, offset),
      Err(_) => buf.gwrite_with(value, offset, LE),
    }?;
    Ok(*offset)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TileHeader {
  pub has_block: bool,
  pub has_attributes: bool,
  pub has_wall: bool,
  pub liquid_type: Option<LiquidType>,
  pub has_extended_block_id: bool,
  pub rle_type: RLEType,
}

impl<'a> TryFromCtx<'a, Endian> for TileHeader {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let flags = buf.gread::<TBitVec>(offset)?;
    let has_attributes = flags[0];
    let has_block = flags[1];
    let has_wall = flags[2];
    let liquid_type = LiquidType::from(&flags);
    let has_extended_block_id = flags[5];
    let rle_type = RLEType::from(&flags);
    let liquid_type = if liquid_type == LiquidType::NoLiquid {
      None
    } else {
      Some(liquid_type)
    };
    Ok((
      Self {
        has_block,
        has_attributes,
        has_wall,
        liquid_type,
        has_extended_block_id,
        rle_type,
      },
      *offset,
    ))
  }
}

impl TryIntoCtx<Endian> for TileHeader {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let Self {
      has_block,
      has_attributes,
      has_wall,
      liquid_type,
      has_extended_block_id,
      rle_type,
    } = self;
    let mut flags = TBitVec::from(vec![
      has_attributes,
      has_block,
      has_wall,
      false,
      false,
      false,
      false,
      false,
    ]);
    if liquid_type.is_some() {
      liquid_type.unwrap().assign_bits(&mut flags);
    }
    flags.as_mut().set(5, has_extended_block_id);
    rle_type.assign_bits(&mut flags);
    buf.gwrite(&flags, offset)?;
    Ok(*offset)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TileAttributes {
  pub shape: BlockShape,
  pub is_block_active: bool,
  pub is_block_painted: bool,
  pub is_wall_painted: bool,
  pub has_extended_attributes: bool,
  pub wiring: Option<Wiring>,
}

impl<'a> TryFromCtx<'a, Endian> for TileAttributes {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let flags = buf.gread::<TBitVec>(offset)?;
    let has_extended_attributes = flags[0];
    let shape = BlockShape::from(&flags);
    let mut is_block_active = false;
    let mut is_block_painted = false;
    let mut is_wall_painted = false;
    let wiring: Option<Wiring>;
    if has_extended_attributes {
      let extended_attrs =
        buf.gread_with::<ExtendedTileAttributes>(offset, &flags)?;
      is_block_active = extended_attrs.is_block_active;
      wiring = Some(extended_attrs.wiring);
      is_block_painted = extended_attrs.is_block_painted;
      is_wall_painted = extended_attrs.is_wall_painted;
    } else {
      wiring = Some(Wiring::from(&flags));
    }
    Ok((
      Self {
        shape,
        is_block_active,
        is_block_painted,
        has_extended_attributes,
        is_wall_painted,
        wiring,
      },
      *offset,
    ))
  }
}

impl TryIntoCtx<Endian> for TileAttributes {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let Self {
      shape,
      is_block_active,
      is_block_painted,
      is_wall_painted,
      has_extended_attributes,
      wiring,
    } = self;
    let mut attrs = TBitVec::from(vec![
      has_extended_attributes,
      false,
      false,
      false,
      false,
      false,
      false,
      false,
    ]);
    shape.assign_bits(&mut attrs);
    if has_extended_attributes {
      // this is dumb, but it works.
      let bv = attrs.as_mut();
      bv.push(false);
      bv.push(false);
      bv.push(!is_block_active);
      bv.push(is_block_painted);
      bv.push(is_wall_painted);
      bv.push(false);
      bv.push(false);
      bv.push(false);
    }
    if wiring.is_some() {
      wiring.unwrap().assign_bits(&mut attrs);
    }
    buf.gwrite(&attrs, offset)?;
    Ok(*offset)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct ExtendedTileAttributes {
  is_block_active: bool,
  is_block_painted: bool,
  is_wall_painted: bool,
  wiring: Wiring,
}

impl<'a> TryFromCtx<'a, &TBitVec> for ExtendedTileAttributes {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    attrs: &TBitVec,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let extended_attrs = buf.gread::<TBitVec>(offset)?;
    let is_block_active = !extended_attrs[2];
    let wiring = Wiring::from((attrs, &extended_attrs));
    let is_block_painted = extended_attrs[3];
    let is_wall_painted = extended_attrs[4];
    Ok((
      Self {
        is_block_active,
        is_block_painted,
        is_wall_painted,
        wiring,
      },
      *offset,
    ))
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

    let tile_header = buf.gread::<TileHeader>(offset)?;

    if tile_header.has_attributes {
      let tile_attrs = buf.gread::<TileAttributes>(offset)?;
      is_block_active = tile_attrs.is_block_active;
      is_block_painted = tile_attrs.is_block_painted;
      is_wall_painted = tile_attrs.is_wall_painted;
      wiring = tile_attrs.wiring;
      shape = tile_attrs.shape;
    }

    if tile_header.has_block {
      block = Some(buf.gread_with::<Block>(
        offset,
        BlockCtx {
          has_extended_block_id: tile_header.has_extended_block_id,
          is_block_active,
          is_block_painted,
          tile_frame_importances: ctx.tile_frame_importances,
          shape,
        },
      )?);
    }

    if tile_header.has_wall {
      wall = Some(buf.gread_with::<Wall>(offset, is_wall_painted)?);
    }

    if tile_header.liquid_type.is_some() {
      liquid = Some(
        buf.gread_with::<Liquid>(offset, tile_header.liquid_type.unwrap())?,
      );
    }

    let run_length =
      buf.gread_with::<RunLength>(offset, tile_header.rle_type)?;
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

#[cfg(test)]
mod test_tiles {
  use super::*;

  #[test]
  fn test_liquid_rw() {
    let liquid = Liquid {
      liquid_type: LiquidType::Water,
      volume: 10,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(liquid.clone(), 0).unwrap());
    assert_eq!(
      liquid,
      buf.pread_with::<Liquid>(0, LiquidType::Water).unwrap()
    );
  }

  #[test]
  fn test_tile_header_rw() {
    let th = TileHeader {
      has_block: true,
      has_attributes: true,
      has_wall: false,
      liquid_type: Some(LiquidType::Water),
      has_extended_block_id: false,
      rle_type: RLEType::SingleByte,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(th.clone(), 0).unwrap());
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());

    let th = TileHeader {
      has_block: true,
      has_attributes: true,
      has_wall: false,
      liquid_type: None,
      has_extended_block_id: true,
      rle_type: RLEType::DoubleByte,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(th.clone(), 0).unwrap());
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());

    let th = TileHeader {
      has_block: true,
      has_attributes: true,
      has_wall: false,
      liquid_type: Some(LiquidType::Lava),
      has_extended_block_id: true,
      rle_type: RLEType::NoCompression,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(th.clone(), 0).unwrap());
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());

    let th = TileHeader {
      has_block: false,
      has_attributes: false,
      has_wall: true,
      liquid_type: Some(LiquidType::Honey),
      has_extended_block_id: false,
      rle_type: RLEType::SingleByte,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(th.clone(), 0).unwrap());
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());
  }

  #[test]
  fn test_tile_attributes_rw() {
    let attrs = TileAttributes {
      shape: BlockShape::HalfTile,
      is_block_active: true,
      is_block_painted: true,
      is_wall_painted: false,
      has_extended_attributes: true,
      wiring: Some(Wiring {
        red: true,
        blue: false,
        green: false,
        yellow: true,
        actuator: false,
      }),
    };

    let mut buf = [0; 2];
    assert_eq!(2, buf.pwrite(attrs, 0).unwrap());
    assert_eq!(attrs, buf.pread::<TileAttributes>(0).unwrap());

    let attrs = TileAttributes {
      shape: BlockShape::HalfTile,
      is_block_active: false,
      is_block_painted: false,
      is_wall_painted: false,
      has_extended_attributes: false,
      wiring: Some(Wiring {
        red: true,
        blue: false,
        green: false,
        yellow: false,
        actuator: false,
      }),
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(attrs, 0).unwrap());
    assert_eq!(attrs, buf.pread::<TileAttributes>(0).unwrap());
  }
}
