use super::header::*;
use crate::{
  constants::*,
  enums::LiquidType,
  model::{
    common::{
      TBool,
      TString,
    },
    items::*,
    npc::*,
    properties::Properties,
    status::Status,
    tile_entity::*,
    tiles::*,
  },
};
use image::RgbaImage;
use imageproc::{
  drawing::draw_filled_rect_mut,
  rect::Rect,
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

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct World {
  pub status: WorldStatus,
  pub tiles: TileMatrix,
  pub chests_info: ChestsInfo,
  pub signs_info: SignsInfo,
  pub tile_entities_info: TileEntitiesInfo,
  pub npcs: NPCVec,
  pub mobs: MobVec,
  pub rooms: RoomVec,
  pub footer: Footer,
}

impl SizeWith<World> for World {
  fn size_with(ctx: &World) -> usize {
    WorldStatus::size_with(&ctx.status)
      + TileMatrix::size_with(&ctx.tiles)
      + ChestsInfo::size_with(&ctx.tiles)
      + SignsInfo::size_with(&ctx.tiles)
      + TileEntitiesInfo::size_with(&ctx.tiles)
      + NPCVec::size_with(&ctx.npcs)
      + MobVec::size_with(&ctx.mobs)
      + RoomVec::size_with(&ctx.rooms)
      + Footer::size_with(&ctx.status.properties.as_world_context())
  }
}

#[derive(Clone, Debug, PartialEq, Pwrite, Pread)]
#[repr(C)]
pub struct WorldStatus {
  pub header: Header,
  pub properties: Properties,
  pub status: Status,
}

impl SizeWith<WorldStatus> for WorldStatus {
  fn size_with(ctx: &WorldStatus) -> usize {
    let size = Header::size_with(&ctx.header)
      + Properties::size_with(&ctx.properties)
      + Status::size_with(&ctx.status);
    debug!("WorldStatus size: {}", size);
    size
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    buf.gwrite(ctx.id, offset)?;
    Ok(*offset)
  }
}

impl<'a> SizeWith<WorldCtx<'a>> for Footer {
  fn size_with(ctx: &WorldCtx) -> usize {
    let size = TBool::size_with(&LE)
      + TString::size_with(&ctx.name)
      + i32::size_with(&LE);
    debug!("Footer size: {}", size);
    size
  }
}

impl World {
  #[inline]
  pub fn read(bytes: &[u8]) -> Result<World, scroll::Error> {
    let offset = &mut 0;
    // order matters
    let status = bytes.gread::<WorldStatus>(offset)?;

    // need this context to associate various bits with other bits
    let world_ctx = status.properties.as_world_context();

    let mut tiles = bytes.gread_with::<TileMatrix>(offset, world_ctx)?;
    debug!(
      "Reading chests @ offset {}; expected offset {}",
      offset, status.header.offsets.chests
    );
    let chests = bytes.gread::<ChestVec>(offset)?;
    let chests_info = chests.chests_info();
    ChestVec::move_to_tile(chests, &mut tiles);
    let signs = bytes.gread::<SignVec>(offset)?;
    let signs_info = signs.signs_info();
    SignVec::move_to_tile(signs, &mut tiles);
    let npcs = bytes.gread::<NPCVec>(offset)?;
    let mobs = bytes.gread::<MobVec>(offset)?;
    let tile_entities = bytes.gread::<TileEntityVec>(offset)?;
    let tile_entities_info = tile_entities.tile_entities_info();
    TileEntityVec::move_to_tile(tile_entities, &mut tiles);
    let rooms = bytes.gread::<RoomVec>(offset)?;
    // footer is essentially useless, but needed on output and performs
    // assertions
    let mut offset = status.header.offsets.footer as usize;
    let footer = bytes.gread_with::<Footer>(&mut offset, world_ctx)?;
    Ok(World {
      status,
      tiles,
      chests_info,
      signs_info,
      tile_entities_info,
      npcs,
      mobs,
      rooms,
      footer,
    })
  }

  pub fn write(&self) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
    let offset = &mut 0;
    let size = World::size_with(self);
    let mut v: Vec<u8> = Vec::with_capacity(size);
    unsafe {
      v.set_len(size);
    }
    v.gwrite(&self.status, offset)?;
    v.gwrite(&self.tiles, offset)?;
    v.gwrite_with(&self.chests_info, offset, &self.tiles)?;
    v.gwrite_with(&self.signs_info, offset, &self.tiles)?;
    v.gwrite(&self.npcs, offset)?;
    v.gwrite(&self.mobs, offset)?;
    v.gwrite_with(&self.tile_entities_info, offset, &self.tiles)?;
    v.gwrite(&self.rooms, offset)?;
    let mut offset = self.status.header.offsets.footer as usize;
    v.gwrite_with(
      &self.footer,
      &mut offset,
      self.status.properties.as_world_context(),
    )?;
    debug!("Size in bytes - actual: {}, expected: {}", offset, size);
    Ok(v.into_boxed_slice())
  }

  pub fn render<P>(&self, path: P) -> Result<(), Box<dyn std::error::Error>>
  where
    P: AsRef<std::path::Path>,
  {
    let width = self.status.properties.width as u32;
    let height = self.status.properties.height as u32;
    let mut img = RgbaImage::new(width, height);
    self.draw_background(&mut img);
    println!("Rendering {:?} tiles...", width * height);
    for x in 0..width {
      for y in 0..height {
        self.draw_wall(x, y, &mut img);
        self.draw_liquid(x, y, &mut img);
        self.draw_block(x, y, &mut img);
        self.draw_wire(x, y, &mut img);
      }
    }
    img.save(path)?;
    Ok(())
  }

  fn draw_background(&self, img: &mut RgbaImage) {
    let mut y: i32 = 0;
    let underground_level = self.status.properties.underground_level as u32;
    let cavern_level = self.status.properties.cavern_level as u32;
    let world_width = img.width();
    let world_height = img.height();
    let sky_y = underground_level.min(world_height);
    let sky = Rect::at(0, y).of_size(world_width, sky_y);
    draw_filled_rect_mut(img, sky, SKY_COLOR);
    y = (sky_y + 1) as i32;
    let surface_y = cavern_level.min(world_height);
    let surface = Rect::at(0, y).of_size(world_width, surface_y);
    draw_filled_rect_mut(img, surface, UNDERGROUND_COLOR);
    y = (surface_y + 1) as i32;
    let cavern_y = (world_height - 192).min(world_height);
    let cavern = Rect::at(0, y).of_size(world_width, cavern_y);
    draw_filled_rect_mut(img, cavern, CAVERN_COLOR);
    y = (cavern_y + 1) as i32;
    let hell = Rect::at(0, y).of_size(world_width, world_height);
    draw_filled_rect_mut(img, hell, HELL_COLOR);
  }

  fn draw_wall(&self, x: u32, y: u32, img: &mut RgbaImage) {
    let tile = &self.tiles[x as usize][y as usize];
    match tile.wall {
      Some(wall) => {
        let color = match wall.wall_paint {
          Some(paint) => PAINT_COLOR_MAP.get(&paint).unwrap(),
          None => WALLTYPE_COLOR_MAP.get(&wall.wall_type).unwrap(),
        };
        img.put_pixel(x, y, *color);
      }
      _ => {}
    };
  }

  fn draw_liquid(&self, x: u32, y: u32, img: &mut RgbaImage) {
    let tile = &self.tiles[x as usize][y as usize];
    match tile.liquid {
      Some(liquid) => {
        let color = match liquid.liquid_type {
          LiquidType::Water => WATER_COLOR,
          LiquidType::Lava => LAVA_COLOR,
          LiquidType::Honey => HONEY_COLOR,
          _ => WATER_COLOR, // TODO: ERROR
        };
        img.put_pixel(x, y, color);
      }
      _ => {}
    };
  }

  fn draw_block(&self, x: u32, y: u32, img: &mut RgbaImage) {
    let tile = &self.tiles[x as usize][y as usize];
    match tile.block {
      Some(block) => {
        let color = match block.block_paint {
          Some(paint) => PAINT_COLOR_MAP.get(&paint).unwrap(),
          None => BLOCKTYPE_COLOR_MAP.get(&block.block_type).unwrap(),
        };
        img.put_pixel(x, y, *color);
      }
      _ => {}
    };
  }

  fn draw_wire(&self, x: u32, y: u32, img: &mut RgbaImage) {
    let tile = &self.tiles[x as usize][y as usize];
    match tile.wiring {
      Some(wiring) => {
        let color = if wiring.blue {
          BLUE_WIRE_COLOR
        } else if wiring.green {
          GREEN_WIRE_COLOR
        } else if wiring.yellow {
          YELLOW_WIRE_COLOR
        } else {
          RED_WIRE_COLOR
        };
        img.put_pixel(x, y, color);
      }
      _ => {}
    };
  }
}
