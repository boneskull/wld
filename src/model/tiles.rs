use super::{
  blocks::*,
  items::*,
  walls::*,
};
use crate::model::common::*;
use num_traits::FromPrimitive;
use scroll::{
  Pread,
  LE,
};
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum LiquidType {
  NoLiquid = 0,
  Water = 1,
  Lava = 2,
  Honey = 3,
}

impl From<&TBitVec> for LiquidType {
  fn from(flags: &TBitVec) -> Self {
    if flags[3] && flags[4] {
      LiquidType::Honey
    } else if flags[4] {
      LiquidType::Lava
    } else if flags[3] {
      LiquidType::Water
    } else {
      LiquidType::NoLiquid
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Liquid {
  pub liquid_type: LiquidType,
  pub volume: u8,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
  pub block: Option<Block>,
  pub wall: Option<Wall>,
  pub liquid: Option<Liquid>,
  pub wiring: Option<Wiring>,
  pub chest: Option<Chest>,
  pub sign: Option<Sign>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum RLEType {
  NoCompression = 0,
  SingleByte = 1,
  DoubleByte = 2,
}

impl From<&TBitVec> for RLEType {
  fn from(flags: &TBitVec) -> Self {
    let value = ((flags[7] as u8) << 1) + flags[6] as u8;
    Self::from_u8(value).unwrap()
  }
}

fn read_tile(
  buf: &[u8],
  tile_frame_importances: &VariableTBitVec,
  offset: &mut usize,
) -> std::result::Result<(Tile, u16), Box<dyn std::error::Error>> {
  let flags = buf.gread_with::<TBitVec>(offset, LE)?;
  // println!("{}: {:?}", offset, flags);
  let has_more_flags = flags[0];
  let has_block = flags[1];
  let has_wall = flags[2];
  let liquid_type = LiquidType::from(&flags);
  let has_extended_block_id = flags[5];
  let rle_type = RLEType::from(&flags);
  let mut shape = BlockShape::Normal;
  let mut is_block_active = true;
  let mut is_wall_painted = false;
  let mut is_block_painted = false;
  let mut wiring: Option<Wiring> = None;
  let mut block: Option<Block> = None;
  let mut wall: Option<Wall> = None;
  let mut liquid: Option<Liquid> = None;
  if has_more_flags {
    let more_flags = buf.gread_with::<TBitVec>(offset, LE)?;
    let has_even_more_flags = more_flags[0];
    shape = BlockShape::from(&more_flags);
    if has_even_more_flags {
      let even_more_flags = buf.gread_with::<TBitVec>(offset, LE)?;
      is_block_active = !even_more_flags[2];
      wiring = Some(Wiring::from((&more_flags, &even_more_flags)));
      is_block_painted = even_more_flags[3];
      is_wall_painted = even_more_flags[4];
    } else {
      wiring = Some(Wiring::from(&more_flags));
    }
  }

  if has_block {
    let mut frame_data: Option<(u16, u16)> = None;
    let mut block_paint: Option<u8> = None;

    let block_id = if has_extended_block_id {
      buf.gread_with::<u16>(offset, LE)? as i64
    } else {
      buf.gread::<u8>(offset)? as i64
    };
    let block_type = BlockType::from_i64(block_id).unwrap();
    if tile_frame_importances[block_type as usize] {
      let frame_data_x = buf.gread_with::<u16>(offset, LE)?;
      let frame_data_y = buf.gread_with::<u16>(offset, LE)?;
      frame_data = Some((frame_data_x, frame_data_y));
    }
    if is_block_painted {
      block_paint = Some(buf.gread::<u8>(offset)?);
    }
    block = Some(Block {
      block_type,
      shape,
      frame_data,
      block_paint,
      is_block_active,
    });
  }

  if has_wall {
    // println!("reading offset at {:?}", offset);
    let wall_id = buf.gread::<u8>(offset)? as i64;
    // println!(
    //   "{:?} is {:?}",
    //   wall_id,
    //   wall_id == WallType::RocksUnsafe2 as i64
    // );
    let res = WallType::from_i64(wall_id);
    let wall_type: WallType = res.unwrap();
    let wall_paint: Option<u8> = if is_wall_painted {
      Some(buf.gread::<u8>(offset)?)
    } else {
      None
    };
    wall = Some(Wall {
      wall_type,
      wall_paint,
    });
    // println!("{:?}", wall);
  }

  if liquid_type != LiquidType::NoLiquid {
    liquid = Some(Liquid {
      liquid_type,
      volume: buf.gread::<u8>(offset)?,
    });
    // println!("liquid: {:?}", liquid);
  }

  let multiply_by: u16 = match rle_type {
    RLEType::DoubleByte => buf.gread_with::<u16>(offset, LE)? + 1,
    RLEType::SingleByte => buf.gread::<u8>(offset)? as u16 + 1,
    _ => 1,
  };
  // println!("repeating {:?} times", multiply_by);
  let tile = Tile {
    block,
    wall,
    liquid,
    wiring,
    chest: None,
    sign: None,
  };
  Ok((tile, multiply_by))
}

#[derive(Clone, Debug, Default, PartialEq, Eq, AsMut, Index)]
pub struct Tiles(Vec<Vec<Tile>>);

impl Tiles {
  pub fn tile_at_point(&mut self, point: &Point) -> &mut Tile {
    &mut self.as_mut()[point.x as usize][point.y as usize]
  }
}

pub fn parse_tile_matrix(
  buf: &[u8],
  offset: &mut usize,
  world_width: &i32,
  world_height: &i32,
  tile_frame_importances: &VariableTBitVec,
) -> Tiles {
  let column_count: usize = (*world_height).try_into().unwrap();
  let row_count: usize = (*world_width).try_into().unwrap();
  let mut matrix: Vec<Vec<Tile>> = Vec::with_capacity(row_count);
  while matrix.len() < row_count {
    let mut column: Vec<Tile> = Vec::with_capacity(column_count);
    while column.len() < column_count {
      let (tile, repeat) =
        read_tile(buf, tile_frame_importances, offset).unwrap();
      for _ in 0..repeat {
        column.push(tile.clone());
      }
    }
    matrix.push(column);
  }
  Tiles(matrix)
}
