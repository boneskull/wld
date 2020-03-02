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
    Sign,
    Signs,
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
    let expected_size = Liquid::size_with(&self);
    assert!(
      expected_size == *offset,
      "Liquid offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );

    Ok(*offset)
  }
}

/// Represents wires on a [`Tile`].  A `Tile` can have all of these, none of
/// these, or any in between.
/// For more information, see [Wire](https://terraria.gamepedia.com/Wire) on
/// the [Official Terraria Wiki](https://terraria.gamepedia.com).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Constructor)]
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

impl Default for Wiring {
  /// Creates a [`Wiring`] instance with all flags disabled.
  fn default() -> Self {
    Self::new(false, false, false, false, false)
  }
}

impl From<(&TBitVec, &TBitVec)> for Wiring {
  /// Used when extended attributes are present.  Extended attributes are used
  /// if [`yellow`](Wiring::yellow) or [`actuator`](Wiring::actuator) should be
  /// set
  /// TODO: This should instead accept a [`Wiring`] instance and call
  /// [`Wiring::extend`] instead.
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
  /// Create a [`Wiring`] instance given the bits from [`TileAttributes`].
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
  /// Flips bits in a [`TBitVec`] for red, green, and blue wires.
  pub fn assign_bits(&self, attrs: &mut TBitVec) {
    if self.red {
      attrs.set(1, true);
    }
    if self.green {
      attrs.set(2, true);
    }
    if self.blue {
      attrs.set(3, true);
    }
  }

  // Flips bits in a [`TBitVec`] for yellow wires and actuators.
  pub fn assign_extended_bits(&self, ext_attrs: &mut TBitVec) {
    if self.yellow {
      ext_attrs.set(5, true);
    }
    if self.actuator {
      ext_attrs.set(1, true);
    }
  }

  /// Returns `true` if _any_ wire or actuator is present.
  pub fn has_wires(&self) -> bool {
    self.red || self.blue || self.green || self.yellow || self.actuator
  }

  /// Extend this [`Wiring`] with another.  Only enables flags; does not
  /// disable them.  Used to merge a `Wiring` instance from
  /// [`ExtendedTileAttributes`] into a `Wiring` instance from
  /// [`TileAttributes`].
  pub fn extend(&mut self, other: &Wiring) {
    if other.red {
      self.red = other.red;
    }
    if other.green {
      self.green = other.green;
    }
    if other.blue {
      self.blue = other.blue;
    }
    if other.yellow {
      self.yellow = other.yellow;
    }
    if other.actuator {
      self.actuator = other.actuator;
    }
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
    let expected_size = RunLength::size_with(&self);
    assert!(
      expected_size == *offset,
      "RunLength offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );

    Ok(*offset)
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct TileHeader {
  pub has_block: bool,
  pub has_attributes: bool,
  pub has_wall: bool,
  pub liquid_type: LiquidType,
  pub has_extended_block_id: bool,
  pub rle_type: RLEType,
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
    let bits = buf.gread::<TBitVec>(offset)?;
    let has_attributes = bits[0];
    let has_block = bits[1];
    let has_wall = bits[2];
    let liquid_type = LiquidType::from(&bits);
    let has_extended_block_id = bits[5];
    let rle_type = RLEType::from(&bits);
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

impl TryIntoCtx<Endian> for &TileHeader {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;

    buf.gwrite(&TBitVec::from(self), offset)?;
    let expected_size = TileHeader::size_with(&LE);
    assert!(
      expected_size == *offset,
      "TileHeader offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );

    Ok(*offset)
  }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct TileAttributes {
  pub shape: BlockShape,
  pub has_extended_attributes: bool,
  pub wiring: Wiring,
}

impl<'a> TryFromCtx<'a, Endian> for TileAttributes {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let bits = buf.gread::<TBitVec>(offset)?;
    let has_extended_attributes = bits[0];
    let shape = BlockShape::from(&bits);
    let wiring = Wiring::from(&bits);
    Ok((
      Self {
        shape,
        has_extended_attributes,
        wiring,
      },
      *offset,
    ))
  }
}

impl SizeWith<Endian> for TileAttributes {
  fn size_with(_: &Endian) -> usize {
    u8::size_with(&LE)
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
    buf.gwrite(&TBitVec::from(self), offset)?;
    Ok(*offset)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct ExtendedTileAttributes {
  pub is_block_inactive: bool,
  pub is_block_painted: bool,
  pub is_wall_painted: bool,
  pub wiring: Wiring,
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
    let bits = buf.gread::<TBitVec>(offset)?;
    let is_block_inactive = bits[2];
    let wiring = Wiring::from((attrs, &bits));
    let is_block_painted = bits[3];
    let is_wall_painted = bits[4];
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

impl TryIntoCtx<Endian> for &ExtendedTileAttributes {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    buf.gwrite(&TBitVec::from(self), offset)?;
    Ok(*offset)
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Tile {
  tile_header: TileHeader,
  attributes: Option<TileAttributes>,
  ext_attributes: Option<ExtendedTileAttributes>,
  pub block: Option<Block>,
  pub wall: Option<Wall>,
  pub liquid: Option<Liquid>,
  pub wiring: Option<Wiring>,
  pub run_length: RunLength,
  pub position: Position,
}

impl Tile {
  pub fn chest<'a>(&self, chests: &'a Chests) -> Option<&'a Chest> {
    chests.find_chest_at_position(&self.position)
  }

  pub fn sign<'a>(&self, signs: &'a Signs) -> Option<&'a Sign> {
    signs.find_sign_at_position(&self.position)
  }

  pub fn tile_entity<'a>(
    &self,
    tile_entities: &'a TileEntities,
  ) -> Option<&'a TileEntity> {
    tile_entities.find_tile_entity_at_position(&self.position)
  }
}

impl SizeWith<Tile> for Tile {
  fn size_with(ctx: &Tile) -> usize {
    let size = TileHeader::size_with(&LE)
      + match ctx.attributes {
        Some(_) => TileAttributes::size_with(&LE),
        _ => 0,
      }
      + match ctx.ext_attributes {
        Some(_) => ExtendedTileAttributes::size_with(&LE),
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

impl<'a> TryFromCtx<'a, TileCtx<'a>> for Tile {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: TileCtx,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let mut is_block_inactive = false; // is this correct?
    let mut is_wall_painted = false;
    let mut is_block_painted = false;
    let mut wiring: Option<Wiring> = None;
    let mut block: Option<Block> = None;
    let mut wall: Option<Wall> = None;
    let mut attributes: Option<TileAttributes> = None;
    let mut ext_attributes: Option<ExtendedTileAttributes> = None;
    let mut shape = BlockShape::Normal;

    let tile_header = buf.gread::<TileHeader>(offset)?;

    if tile_header.has_attributes {
      let attrs = buf.gread::<TileAttributes>(offset)?;
      shape = attrs.shape;
      wiring = Some(attrs.wiring);
      if attrs.has_extended_attributes {
        let ext_attrs = buf.gread_with::<ExtendedTileAttributes>(
          offset,
          &TBitVec::from(&attrs),
        )?;
        is_block_inactive = ext_attrs.is_block_inactive;
        is_block_painted = ext_attrs.is_block_painted;
        is_wall_painted = ext_attrs.is_wall_painted;
        if let Some(w) = &mut wiring {
          w.extend(&ext_attrs.wiring);
        }
        ext_attributes = Some(ext_attrs);
      }
      attributes = Some(attrs);
    }

    wiring = wiring.and_then(|w| if w.has_wires() { Some(w) } else { None });
    if tile_header.has_block {
      block = Some(buf.gread_with::<Block>(
        offset,
        BlockCtx {
          has_extended_block_id: tile_header.has_extended_block_id,
          is_block_inactive,
          is_block_painted,
          tile_frame_importances: ctx.world_ctx.tile_frame_importances,
          shape,
        },
      )?);
    }

    if tile_header.has_wall {
      wall = Some(buf.gread_with::<Wall>(offset, is_wall_painted)?);
    }

    let liquid = match tile_header.liquid_type {
      LiquidType::NoLiquid => None,
      _ => Some(buf.gread_with::<Liquid>(offset, tile_header.liquid_type)?),
    };

    let run_length =
      buf.gread_with::<RunLength>(offset, tile_header.rle_type)?;
    Ok((
      Tile {
        tile_header,
        attributes,
        ext_attributes,
        block,
        wall,
        liquid,
        wiring,
        run_length,
        position: ctx.position,
      },
      *offset,
    ))
  }
}

impl TryIntoCtx<usize> for &Tile {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    buffer_offset: usize,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let Tile {
      tile_header,
      attributes,
      ext_attributes,
      block,
      wall,
      liquid,
      wiring: _,
      run_length,
      position: _,
    } = self;

    buf.gwrite(tile_header, offset)?;
    if let Some(attrs) = attributes {
      buf.gwrite(attrs, offset)?;
    }

    if let Some(ext_attrs) = ext_attributes {
      buf.gwrite(ext_attrs, offset)?;
    }

    if let Some(b) = block {
      buf.gwrite(b, offset)?;
    }

    if let Some(w) = wall {
      buf.gwrite(w, offset)?;
    }

    if let Some(l) = liquid {
      buf.gwrite(l, offset)?;
    }

    buf.gwrite(run_length, offset)?;

    let expected_size = Tile::size_with(&self);
    assert!(
      expected_size == *offset,
      "Tile offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );

    trace!("Tile@{:X?}: {:X?}", buffer_offset, &buf[..*offset]);

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TileCtx<'a> {
  world_ctx: &'a WorldCtx<'a>,
  position: Position,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct TileVecCtx<'a> {
  world_ctx: &'a WorldCtx<'a>,
  x: i32,
}

impl<'a> TryFromCtx<'a, TileVecCtx<'a>> for TileVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: TileVecCtx,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let size = *ctx.world_ctx.world_height;
    let mut tiles: Vec<Tile> = Vec::with_capacity(size as usize);
    let mut i: i32 = 0;
    while i < size {
      let tile = buf.gread_with::<Tile>(
        offset,
        TileCtx {
          world_ctx: ctx.world_ctx,
          position: Position { x: ctx.x, y: i },
        },
      )?;
      for _ in 0..tile.run_length.length {
        tiles.push(tile.clone());
      }
      i += tile.run_length.length as i32;
    }
    Ok((TileVec(tiles), *offset))
  }
}

impl TryIntoCtx<usize> for &TileVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    buffer_offset: usize,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let len = self.as_ref().len();
    let mut i = 0;
    let mut buf_offset = buffer_offset;
    while i < len {
      let tile: &Tile = &self[i];
      buf.gwrite_with(tile, offset, buf_offset)?;
      // this handles the RLE; the vector is bigger than the actual data
      // because of it.
      i += tile.run_length.length as usize;
      buf_offset += Tile::size_with(&tile);
    }
    let expected_size = TileVec::size_with(&self);
    assert!(
      expected_size == *offset,
      "TileVec offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );

    Ok(*offset)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, AsMut, Index, AsRef)]
#[repr(C)]
pub struct TileMatrix(Vec<TileVec>);

impl TileMatrix {
  pub fn tile_at_position(&mut self, point: &Position) -> &mut Tile {
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
    let matrix = (0..row_count)
      .into_iter()
      .map(|x| {
        buf.gread_with::<TileVec>(
          offset,
          TileVecCtx {
            world_ctx: &ctx,
            x: x as i32,
          },
        )
      })
      .collect::<Result<Vec<TileVec>, Self::Error>>()?;
    Ok((TileMatrix(matrix), *offset))
  }
}

impl TryIntoCtx<usize> for &TileMatrix {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    buffer_offset: usize,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let mut buf_offset = buffer_offset;
    for i in 0..self.as_ref().len() {
      buf.gwrite_with(&self[i], offset, buf_offset)?;
      buf_offset += TileVec::size_with(&self[i]);
    }
    assert!(
      *offset == TileMatrix::size_with(&self),
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
      has_block: true,
      has_attributes: false,
      has_wall: false,
      liquid_type: LiquidType::Water,
      has_extended_block_id: false,
      rle_type: RLEType::SingleByte,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(&th, 0).unwrap());
    assert_eq!(
      TBitVec::from(vec![false, true, false, true, false, false, true, false,]),
      TBitVec::from(&buf[..])
    );
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());

    let th = TileHeader {
      has_block: true,
      has_attributes: true,
      has_wall: false,
      liquid_type: LiquidType::NoLiquid,
      has_extended_block_id: true,
      rle_type: RLEType::DoubleByte,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(&th, 0).unwrap());
    assert_eq!(
      TBitVec::from(vec![true, true, false, false, false, true, false, true]),
      TBitVec::from(&buf[..])
    );
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());

    let th = TileHeader {
      has_block: true,
      has_attributes: true,
      has_wall: false,
      liquid_type: LiquidType::Lava,
      has_extended_block_id: true,
      rle_type: RLEType::NoCompression,
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(&th, 0).unwrap());
    assert_eq!(
      TBitVec::from(vec![true, true, false, false, true, true, false, false]),
      TBitVec::from(&buf[..])
    );
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());

    let th = TileHeader {
      has_block: false,
      has_attributes: false,
      has_wall: true,
      liquid_type: LiquidType::Honey,
      has_extended_block_id: false,
      rle_type: RLEType::SingleByte,
    };

    let mut buf = [0u8; 1];
    assert_eq!(1, buf.pwrite(&th, 0).unwrap());
    assert_eq!(
      TBitVec::from(vec![false, false, true, true, true, false, true, false]),
      TBitVec::from(&buf[..])
    );
    assert_eq!(th, buf.pread::<TileHeader>(0).unwrap());
  }

  #[test]
  fn test_tile_attributes_rw() {
    let attrs = TileAttributes {
      has_extended_attributes: true,
      shape: BlockShape::HalfTile,
      wiring: Wiring::default(),
    };

    let mut buf = [0; 1];
    assert_eq!(1, buf.pwrite(&attrs, 0).unwrap());
    assert_eq!(attrs, buf.pread::<TileAttributes>(0).unwrap());

    let attrs = TileAttributes {
      has_extended_attributes: false,
      shape: BlockShape::TopRightSlope,
      wiring: Wiring {
        red: true,
        blue: false,
        green: false,
        yellow: false,
        actuator: false,
      },
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
        liquid_type: LiquidType::NoLiquid,
        has_extended_block_id: false,
        rle_type: RLEType::SingleByte,
      },
      attributes: Some(TileAttributes {
        has_extended_attributes: true,
        shape: BlockShape::HalfTile,
        wiring: Wiring::default(),
      }),
      ext_attributes: Some(ExtendedTileAttributes {
        is_block_inactive: false,
        is_block_painted: false,
        is_wall_painted: false,
        wiring: Wiring::new(false, false, false, false, true),
      }),
      block: Some(Block {
        block_type: BlockType::Dirt,
        shape: BlockShape::HalfTile,
        frame_data: None,
        block_paint: None,
        is_block_inactive: false,
        has_extended_block_id: false,
      }),
      wall: None,
      liquid: None,
      wiring: Some(Wiring::new(false, false, false, false, true)),
      run_length: RunLength::new(2, RLEType::SingleByte),
      position: Position { x: 0, y: 0 },
    };
    let world_ctx = WorldCtx {
      world_width: &4200,
      world_height: &1200,
      tile_frame_importances: &VariableTBitVec::from(vec![false; 257]),
      id: &123,
      name: &TString::from("Fat City"),
    };
    let ctx = TileCtx {
      world_ctx: &world_ctx,
      position: Position { x: 0, y: 0 },
    };

    let mut buf = [0; 5];
    assert_eq!(buf.pwrite(&tile, 0).unwrap(), Tile::size_with(&tile));
    assert_eq!(
      TBitVec::from(&buf[..2]),
      TBitVec::from(vec![
        true, true, false, false, false, false, true, false, true, false,
        false, false, true, false, false, false
      ])
    );
    assert_eq!(tile, buf.pread_with::<Tile>(0, ctx).unwrap());
  }

  #[test]
  fn test_tile_sizewith() {
    let tile = Tile {
      tile_header: TileHeader {
        has_block: true,
        has_attributes: true,
        has_wall: false,
        liquid_type: LiquidType::NoLiquid,
        has_extended_block_id: false,
        rle_type: RLEType::SingleByte,
      },
      attributes: Some(TileAttributes {
        has_extended_attributes: false,
        shape: BlockShape::HalfTile,
        wiring: Wiring::default(),
      }),
      ext_attributes: None,
      block: Some(Block {
        block_type: BlockType::Dirt,
        shape: BlockShape::HalfTile,
        frame_data: None,
        block_paint: None,
        is_block_inactive: false,
        has_extended_block_id: false,
      }),
      wall: None,
      liquid: None,
      wiring: Some(Wiring::default()),
      run_length: RunLength::new(2, RLEType::SingleByte),
      position: Position { x: 0, y: 0 },
    };

    assert_eq!(4, Tile::size_with(&tile));
  }

  #[test]
  fn test_tilevec_sizewith() {
    let tile = Tile {
      tile_header: TileHeader {
        has_block: true,
        has_attributes: true,
        has_wall: false,
        liquid_type: LiquidType::NoLiquid,
        has_extended_block_id: false,
        rle_type: RLEType::SingleByte,
      },
      attributes: Some(TileAttributes {
        has_extended_attributes: false,
        shape: BlockShape::HalfTile,
        wiring: Wiring::default(),
      }),
      ext_attributes: None,
      block: Some(Block {
        block_type: BlockType::Dirt,
        shape: BlockShape::HalfTile,
        frame_data: None,
        block_paint: None,
        is_block_inactive: false,
        has_extended_block_id: false,
      }),
      wall: None,
      liquid: None,
      wiring: Some(Wiring::default()),
      run_length: RunLength::new(2, RLEType::SingleByte),
      position: Position { x: 0, y: 0 },
    };
    let tv = TileVec(vec![tile.clone(), tile.clone(), tile.clone()]);
    // it's 8 because of the run_length of 2.
    assert_eq!(8, TileVec::size_with(&tv));
  }
}
