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
pub struct Liquid {
  pub liquid_type: LiquidType,
  pub volume: u8,
}

impl SizeWith<Liquid> for Liquid {
  fn size_with(ctx: &Liquid) -> usize {
    match ctx.liquid_type {
      LiquidType::NoLiquid => 0,
      _ => u8::size_with(&LE),
    }
  }
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

impl TryIntoCtx<Endian> for &Liquid {
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

  pub fn has_wires(&self) -> bool {
    self.red || self.blue || self.green || self.yellow || self.actuator
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Constructor)]
#[repr(C)]
pub struct RunLength {
  pub length: u16,
  pub rle_type: RLEType,
}

impl SizeWith<RunLength> for RunLength {
  fn size_with(ctx: &RunLength) -> usize {
    match ctx.rle_type {
      RLEType::DoubleByte => u16::size_with(&LE),
      RLEType::SingleByte => u8::size_with(&LE),
      _ => 0,
    }
  }
}

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
    Ok((Self::new(run_length, rle_type), *offset))
  }
}

impl TryIntoCtx<Endian> for &RunLength {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let value = self.length;
    match self.rle_type {
      RLEType::DoubleByte => {
        buf.gwrite_with(value - 1, offset, LE)?;
      }
      RLEType::SingleByte => {
        buf.gwrite((value - 1) as u8, offset)?;
      }
      _ => {}
    };
    Ok(*offset)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
struct TileHeader {
  has_block: bool,
  has_attributes: bool,
  has_wall: bool,
  liquid_type: Option<LiquidType>,
  has_extended_block_id: bool,
  has_extended_attributes: bool,
  rle_type: RLEType,
}

impl SizeWith<Endian> for TileHeader {
  fn size_with(_: &Endian) -> usize {
    u8::size_with(&LE)
  }
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
        has_extended_attributes: false,
      },
      *offset,
    ))
  }
}

impl TryIntoCtx<Endian> for &TileHeader {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let TileHeader {
      has_block,
      has_attributes,
      has_wall,
      liquid_type,
      has_extended_block_id,
      rle_type,
      has_extended_attributes: _,
    } = *self;
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
    if let Some(lt) = liquid_type {
      lt.assign_bits(&mut flags);
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
  pub is_block_inactive: bool,
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
    let mut is_block_inactive = false;
    let mut is_block_painted = false;
    let mut is_wall_painted = false;
    let wiring: Option<Wiring>;
    if has_extended_attributes {
      let extended_attrs =
        buf.gread_with::<ExtendedTileAttributes>(offset, &flags)?;
      is_block_inactive = extended_attrs.is_block_inactive;
      wiring = match extended_attrs.wiring.has_wires() {
        true => Some(extended_attrs.wiring),
        false => None,
      };
      is_block_painted = extended_attrs.is_block_painted;
      is_wall_painted = extended_attrs.is_wall_painted;
    } else {
      let w = Wiring::from(&flags);
      wiring = match w.has_wires() {
        true => Some(w),
        false => None,
      };
    }
    Ok((
      Self {
        shape,
        is_block_inactive,
        is_block_painted,
        has_extended_attributes,
        is_wall_painted,
        wiring,
      },
      *offset,
    ))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a TileAttributes {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let TileAttributes {
      shape,
      is_block_inactive,
      is_block_painted,
      is_wall_painted,
      has_extended_attributes,
      wiring,
    } = self;
    let mut attrs = TBitVec::from(vec![
      *has_extended_attributes,
      false,
      false,
      false,
      false,
      false,
      false,
      false,
    ]);
    shape.assign_bits(&mut attrs);
    if *has_extended_attributes {
      // this is dumb, but it works. BitVec doesn't allow you to just extend it
      // with e.g., a slice of bool's
      let bv = attrs.as_mut();
      bv.push(false);
      bv.push(false);
      bv.push(*is_block_inactive);
      bv.push(*is_block_painted);
      bv.push(*is_wall_painted);
      bv.push(false);
      bv.push(false);
      bv.push(false);
    }
    match wiring {
      Some(w) => w.assign_bits(&mut attrs),
      _ => {}
    }
    buf.gwrite(&attrs, offset)?;
    Ok(*offset)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
struct ExtendedTileAttributes {
  is_block_inactive: bool,
  is_block_painted: bool,
  is_wall_painted: bool,
  wiring: Wiring,
}

impl SizeWith<Endian> for ExtendedTileAttributes {
  fn size_with(_: &Endian) -> usize {
    u8::size_with(&LE)
  }
}

impl<'a> TryFromCtx<'a, &TBitVec> for ExtendedTileAttributes {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    attrs: &TBitVec,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let extended_attrs = buf.gread::<TBitVec>(offset)?;
    let is_block_inactive = extended_attrs[2];
    let wiring = Wiring::from((attrs, &extended_attrs));
    let is_block_painted = extended_attrs[3];
    let is_wall_painted = extended_attrs[4];
    Ok((
      Self {
        is_block_inactive,
        is_block_painted,
        is_wall_painted,
        wiring,
      },
      *offset,
    ))
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Tile {
  tile_header: TileHeader,
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

impl SizeWith<Tile> for Tile {
  fn size_with(ctx: &Tile) -> usize {
    let size = TileHeader::size_with(&LE)
      + match ctx.tile_header.has_attributes {
        true => {
          u8::size_with(&LE)
            + match ctx.tile_header.has_extended_attributes {
              true => u8::size_with(&LE),
              _ => 0,
            }
        }
        _ => 0,
      }
      + ctx.block.map_or(0, |block| Block::size_with(&block))
      + ctx.wall.map_or(0, |wall| Wall::size_with(&wall))
      + ctx.liquid.map_or(0, |liquid| Liquid::size_with(&liquid))
      + RunLength::size_with(&ctx.run_length);
    // trace!("Tile size: {}", size);
    size
  }
}

impl<'a> TryFromCtx<'a, WorldCtx<'a>> for Tile {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: WorldCtx,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let mut is_block_inactive = false; // is this correct?
    let mut is_wall_painted = false;
    let mut is_block_painted = false;
    let mut wiring: Option<Wiring> = None;
    let mut block: Option<Block> = None;
    let mut wall: Option<Wall> = None;
    let mut liquid: Option<Liquid> = None;
    let mut shape = BlockShape::Normal;

    let mut tile_header = buf.gread::<TileHeader>(offset)?;

    if tile_header.has_attributes {
      let tile_attrs = buf.gread::<TileAttributes>(offset)?;
      is_block_inactive = tile_attrs.is_block_inactive;
      is_block_painted = tile_attrs.is_block_painted;
      is_wall_painted = tile_attrs.is_wall_painted;
      wiring = tile_attrs.wiring;
      shape = tile_attrs.shape;
      tile_header.has_extended_attributes = tile_attrs.has_extended_attributes;
    }

    if tile_header.has_block {
      block = Some(buf.gread_with::<Block>(
        offset,
        BlockCtx {
          has_extended_block_id: tile_header.has_extended_block_id,
          is_block_inactive,
          is_block_painted,
          tile_frame_importances: ctx.tile_frame_importances,
          shape,
        },
      )?);
    }

    if tile_header.has_wall {
      wall = Some(buf.gread_with::<Wall>(offset, is_wall_painted)?);
    }

    match tile_header.liquid_type {
      Some(l) => liquid = Some(buf.gread_with::<Liquid>(offset, l)?),
      _ => {}
    }

    let run_length =
      buf.gread_with::<RunLength>(offset, tile_header.rle_type)?;
    Ok((
      Tile {
        tile_header,
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

impl TryIntoCtx<Endian> for &Tile {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let Tile {
      tile_header,
      block,
      wall,
      liquid,
      wiring,
      chest: _,
      sign: _,
      run_length,
      tile_entity: _,
      pressure_plate: _,
    } = self;

    match buf.gwrite(tile_header, offset) {
      Err(e) => {
        debug!("{}", e);
        return Err(e);
      }
      _ => {}
    }
    if tile_header.has_attributes {
      let (is_block_inactive, is_block_painted, shape) = match block {
        Some(block) => {
          (
            block.is_block_inactive,
            block.block_paint.is_some(),
            block.shape,
          )
        }
        _ => (false, false, BlockShape::Normal),
      };
      let is_wall_painted = match wall {
        Some(w) => w.wall_paint.is_some(),
        _ => false,
      };
      let attrs = TileAttributes {
        is_block_inactive,
        is_block_painted,
        shape,
        is_wall_painted,
        wiring: *wiring,
        has_extended_attributes: tile_header.has_extended_attributes,
      };

      match buf.gwrite(&attrs, offset) {
        Err(e) => {
          debug!("{}", e);
          return Err(e);
        }
        _ => {}
      }
    }

    match block {
      Some(b) => {
        match buf.gwrite(b, offset) {
          Err(e) => {
            debug!("{}", e);
            return Err(e);
          }
          _ => {}
        }
      }
      _ => {}
    };

    match wall {
      Some(w) => {
        match buf.gwrite(w, offset) {
          Err(e) => {
            debug!("{}", e);
            return Err(e);
          }
          _ => {}
        }
      }
      _ => {}
    };

    match liquid {
      Some(l) => {
        match buf.gwrite(l, offset) {
          Err(e) => {
            debug!("{}", e);
            return Err(e);
          }
          _ => {}
        }
      }
      _ => {}
    }

    match buf.gwrite(run_length, offset) {
      Err(e) => {
        debug!("{}", e);
        return Err(e);
      }
      _ => {}
    }

    Ok(*offset)
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

#[derive(Clone, Debug, Default, PartialEq, Eq, IndexMut, Index, AsRef)]
#[repr(C)]
pub struct TileVec(Vec<Tile>);

impl SizeWith<TileVec> for TileVec {
  fn size_with(ctx: &TileVec) -> usize {
    let len = ctx.as_ref().len();
    let mut i = 0;
    let mut size = 0;
    while i < len {
      let tile = &ctx[i];
      let run_length = tile.run_length.length as usize;
      size += Tile::size_with(tile);
      i += run_length;
    }
    size
  }
}

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
      for _ in 0..tile.run_length.length {
        tiles.push(tile.clone());
      }
    }
    Ok((TileVec(tiles), *offset))
  }
}

impl TryIntoCtx<Endian> for &TileVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let len = self.as_ref().len();
    let mut i = 0;
    while i < len {
      let tile: &Tile = &self[i];
      buf.gwrite(tile, offset)?;
      // this handles the RLE; the vector is bigger than the actual data
      // because of it.
      i += tile.run_length.length as usize;
    }
    Ok(*offset)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, AsMut, Index, AsRef)]
#[repr(C)]
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

impl TryIntoCtx<Endian> for &TileMatrix {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    for i in 0..self.as_ref().len() {
      buf.gwrite(&self[i], offset)?;
    }
    assert!(
      *offset == TileMatrix::size_with(self),
      "TileMatrix size mismatch"
    );
    Ok(*offset)
  }
}

impl SizeWith<TileMatrix> for TileMatrix {
  fn size_with(ctx: &TileMatrix) -> usize {
    let size = ctx
      .as_ref()
      .iter()
      .map(|tv| TileVec::size_with(&tv))
      .fold(0, |acc, len| acc + len);
    debug!("TileMatrix size: {}", size);
    size
  }
}

#[cfg(test)]
mod test_tiles {
  use super::*;
  use crate::enums::BlockType;

  #[test]
  fn test_liquid_rw() {
    let liquid = Liquid {
      liquid_type: LiquidType::Water,
      volume: 10,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(&liquid, 0).unwrap());
    assert_eq!(
      liquid,
      buf.pread_with::<Liquid>(0, LiquidType::Water).unwrap()
    );
  }

  #[test]
  fn test_tile_header_rw() {
    let th = TileHeader {
      has_extended_attributes: false,
      has_block: true,
      has_attributes: true,
      has_wall: false,
      liquid_type: Some(LiquidType::Water),
      has_extended_block_id: false,
      rle_type: RLEType::SingleByte,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(&th, 0).unwrap());
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());

    let th = TileHeader {
      has_extended_attributes: false,
      has_block: true,
      has_attributes: true,
      has_wall: false,
      liquid_type: None,
      has_extended_block_id: true,
      rle_type: RLEType::DoubleByte,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(&th, 0).unwrap());
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());

    let th = TileHeader {
      has_extended_attributes: false,
      has_block: true,
      has_attributes: true,
      has_wall: false,
      liquid_type: Some(LiquidType::Lava),
      has_extended_block_id: true,
      rle_type: RLEType::NoCompression,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(&th, 0).unwrap());
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());

    let th = TileHeader {
      has_extended_attributes: false,
      has_block: false,
      has_attributes: false,
      has_wall: true,
      liquid_type: Some(LiquidType::Honey),
      has_extended_block_id: false,
      rle_type: RLEType::SingleByte,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(&th, 0).unwrap());
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());
  }

  #[test]
  fn test_tile_attributes_rw() {
    let attrs = TileAttributes {
      shape: BlockShape::HalfTile,
      is_block_inactive: true,
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
    assert_eq!(2, buf.pwrite(&attrs, 0).unwrap());
    assert_eq!(attrs, buf.pread::<TileAttributes>(0).unwrap());

    let attrs = TileAttributes {
      shape: BlockShape::HalfTile,
      is_block_inactive: false,
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
    assert_eq!(1, buf.pwrite(&attrs, 0).unwrap());
    assert_eq!(attrs, buf.pread::<TileAttributes>(0).unwrap());
  }

  #[test]
  fn test_tile_rw() {
    let tile = Tile {
      tile_header: TileHeader {
        has_block: true,
        has_attributes: true,
        has_wall: false,
        liquid_type: None,
        has_extended_block_id: false,
        has_extended_attributes: true,
        rle_type: RLEType::SingleByte,
      },
      block: Some(Block {
        block_type: BlockType::Dirt,
        shape: BlockShape::Normal,
        frame_data: None,
        block_paint: None,
        is_block_inactive: true,
        has_extended_block_id: false,
      }),
      wall: None,
      liquid: None,
      wiring: None,
      chest: None,
      sign: None,
      tile_entity: None,
      pressure_plate: None,
      run_length: RunLength::new(2, RLEType::SingleByte),
    };
    let ctx = WorldCtx {
      world_width: &4200,
      world_height: &1200,
      tile_frame_importances: &VariableTBitVec::from(vec![
        false, false, false, false, false, false, false, false,
      ]),
      id: &123,
      name: &TString::from("Fat City"),
    };

    let mut buf = [0; 5];

    assert_eq!(5, buf.pwrite(&tile, 0).unwrap());
    assert_eq!(tile, buf.pread_with::<Tile>(0, ctx).unwrap());
  }

  #[test]
  fn test_tile_sizewith() {
    let tile = Tile {
      tile_header: TileHeader {
        has_block: true,
        has_attributes: true,
        has_wall: false,
        liquid_type: None,
        has_extended_block_id: false,
        has_extended_attributes: true,
        rle_type: RLEType::SingleByte,
      },
      block: Some(Block {
        block_type: BlockType::Dirt,
        shape: BlockShape::Normal,
        frame_data: None,
        block_paint: None,
        is_block_inactive: true,
        has_extended_block_id: false,
      }),
      wall: None,
      liquid: None,
      wiring: None,
      chest: None,
      sign: None,
      tile_entity: None,
      pressure_plate: None,
      run_length: RunLength::new(2, RLEType::SingleByte),
    };

    assert_eq!(7, Tile::size_with(&tile));
  }

  #[test]
  fn test_tilevec_sizewith() {
    let tile = Tile {
      tile_header: TileHeader {
        has_block: true,
        has_attributes: true,
        has_wall: false,
        liquid_type: None,
        has_extended_block_id: false,
        has_extended_attributes: true,
        rle_type: RLEType::SingleByte,
      },
      block: Some(Block {
        block_type: BlockType::Dirt,
        shape: BlockShape::Normal,
        frame_data: None,
        block_paint: None,
        is_block_inactive: true,
        has_extended_block_id: false,
      }),
      wall: None,
      liquid: None,
      wiring: None,
      chest: None,
      sign: None,
      tile_entity: None,
      pressure_plate: None,
      run_length: RunLength::new(2, RLEType::SingleByte),
    };
    let tv = TileVec(vec![tile.clone(), tile.clone()]);

    assert_eq!(7, TileVec::size_with(&tv));
  }
}
