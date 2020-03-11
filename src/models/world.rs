use crate::{
  constants::{
    BLOCKTYPE_COLOR_MAP,
    BLUE_WIRE_COLOR,
    CAVERN_COLOR,
    GREEN_WIRE_COLOR,
    HELL_COLOR,
    HONEY_COLOR,
    LAVA_COLOR,
    PAINT_COLOR_MAP,
    RED_WIRE_COLOR,
    SKY_COLOR,
    UNDERGROUND_COLOR,
    WALLTYPE_COLOR_MAP,
    WATER_COLOR,
    YELLOW_WIRE_COLOR,
  },
  enums::LiquidType,
  models::{
    Chests,
    Footer,
    Header,
    HouseVec,
    MobVec,
    NPCVec,
    PressurePlates,
    Properties,
    Signs,
    Status,
    TileEntities,
    TileMatrix,
  },
};
use image::{
  Rgba,
  RgbaImage,
};
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
  Endian,
  Error as ScrollError,
  Pread,
  Pwrite,
  LE,
};

/// Top-level representation of a Terraria world file (`.wld`).
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct World {
  pub header: Header,
  pub properties: Properties,
  pub status: Status,
  pub tiles: TileMatrix,
  pub chests: Chests,
  pub signs: Signs,
  pub npcs: NPCVec,
  pub mobs: MobVec,
  pub tile_entities: TileEntities,
  pub pressure_plates: PressurePlates,
  pub houses: HouseVec,
  pub town_manager: TownManager,
  pub footer: Footer,
}

impl SizeWith<World> for World {
  fn size_with(ctx: &Self) -> usize {
    Header::size_with(&LE)
      + Properties::size_with(&ctx.properties)
      + Status::size_with(&ctx.status)
      + TileMatrix::size_with(&ctx.tiles)
      + Chests::size_with(&ctx.chests)
      + Signs::size_with(&ctx.signs)
      + TileEntities::size_with(&ctx.tile_entities)
      + NPCVec::size_with(&ctx.npcs)
      + MobVec::size_with(&ctx.mobs)
      + PressurePlates::size_with(&ctx.pressure_plates)
      + HouseVec::size_with(&ctx.houses)
      + TownManager::size_with(&ctx.town_manager)
      + Footer::size_with(&ctx.properties.as_world_context())
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, AsRef, Index)]
#[repr(C)]
pub struct TownManager(Vec<u8>);

impl SizeWith<TownManager> for TownManager {
  fn size_with(ctx: &Self) -> usize {
    ctx.as_ref().len()
  }
}

impl<'a> TryFromCtx<'a, usize> for TownManager {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: usize,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let mut vec: Vec<u8> = Vec::with_capacity(ctx);
    for _ in 0..ctx {
      vec.push(buf.gread::<u8>(offset)?);
    }
    Ok((Self(vec), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &TownManager {
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
    Ok(*offset)
  }
}

impl World {
  /// Reads a [`World`] from a slice of bytes.
  ///
  /// # Errors
  /// May return all manner of [`scroll::Error`] instances.
  #[inline]
  pub fn read(bytes: &[u8]) -> Result<Self, ScrollError> {
    let offset = &mut 0;
    // order matters
    let header = bytes.gread::<Header>(offset)?;
    let properties = bytes.gread::<Properties>(offset)?;
    let status = bytes.gread::<Status>(offset)?;
    // need this context to associate various bits with other bits
    let world_ctx = properties.as_world_context();
    debug!("{:?}", properties);
    assert!(
      header.offsets.tiles as usize == *offset,
      "Tiles offset mismatch; expected {:?}, got {:?}",
      header.offsets.tiles,
      offset
    );
    let tiles = bytes.gread_with::<TileMatrix>(offset, world_ctx)?;
    assert!(
      header.offsets.chests as usize == *offset,
      "Chests offset mismatch"
    );
    let chests = bytes.gread::<Chests>(offset)?;
    assert!(
      header.offsets.signs as usize == *offset,
      "Signs offset mismatch"
    );
    let signs = bytes.gread::<Signs>(offset)?;
    assert!(
      header.offsets.npcs as usize == *offset,
      "NPCs offset mismatch"
    );
    let npcs = bytes.gread::<NPCVec>(offset)?;
    let mobs = bytes.gread::<MobVec>(offset)?;
    assert!(
      header.offsets.tile_entities as usize == *offset,
      "TileEntities offset mismatch"
    );
    let tile_entities = bytes.gread::<TileEntities>(offset)?;
    assert!(
      header.offsets.pressure_plates as usize == *offset,
      "PressurePlates offset mismatch"
    );
    let pressure_plates = bytes.gread::<PressurePlates>(offset)?;
    assert!(
      header.offsets.houses as usize == *offset,
      "RoomVec offset mismatch; expected {:?}, got {:?}",
      header.offsets.houses,
      offset
    );
    let houses = bytes.gread::<HouseVec>(offset)?;
    // likely junk data
    let town_manager = bytes.gread_with::<TownManager>(
      offset,
      header.offsets.footer as usize - *offset,
    )?;
    assert!(
      header.offsets.footer as usize == *offset,
      "Footer offset mismatch"
    );
    let footer = bytes.gread_with::<Footer>(offset, world_ctx)?;
    debug!("Read {:?} bytes", *offset);
    Ok(Self {
      header,
      properties,
      status,
      tiles,
      chests,
      signs,
      tile_entities,
      pressure_plates,
      npcs,
      mobs,
      houses,
      town_manager,
      footer,
    })
  }

  /// Given a [`World`] instance, serialize into a slice of bytes.
  ///
  /// # Errors
  /// Mainly emits [`scroll::Error`].
  pub fn write(&self) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
    let offset = &mut 0;
    let size = Self::size_with(self);
    debug!("Total size: {:?}", size);
    let mut v: Vec<u8> = Vec::with_capacity(size);
    unsafe {
      v.set_len(size);
    }
    v.gwrite(&self.header, offset)?;
    v.gwrite(&self.properties, offset)?;
    v.gwrite(&self.status, offset)?;
    assert!(
      self.header.offsets.tiles as usize == *offset,
      "Tiles offset mismatch; expected {:?}, got {:?}",
      self.header.offsets.tiles,
      offset
    );
    v.gwrite_with(&self.tiles, offset, *offset)?;
    assert!(
      self.header.offsets.chests as usize == *offset,
      "Chests offset mismatch; expected {:?}, got {:?}",
      self.header.offsets.chests,
      offset
    );
    v.gwrite(&self.chests, offset)?;
    assert!(
      self.header.offsets.signs as usize == *offset,
      "Signs offset mismatch; expected {:?}, got {:?}",
      self.header.offsets.signs,
      offset
    );
    v.gwrite(&self.signs, offset)?;
    assert!(
      self.header.offsets.npcs as usize == *offset,
      "NPCs offset mismatch"
    );

    v.gwrite(&self.npcs, offset)?;
    v.gwrite(&self.mobs, offset)?;
    assert!(
      self.header.offsets.tile_entities as usize == *offset,
      "TileEntities offset mismatch"
    );
    v.gwrite(&self.tile_entities, offset)?;
    assert!(
      self.header.offsets.pressure_plates as usize == *offset,
      "PressurePlates offset mismatch"
    );
    v.gwrite(&self.pressure_plates, offset)?;
    assert!(
      self.header.offsets.houses as usize == *offset,
      "RoomVec offset mismatch"
    );
    v.gwrite(&self.houses, offset)?;
    // v.gwrite(&self.town_manager, offset)?;
    assert!(
      self.header.offsets.footer as usize == *offset,
      "Footer offset mismatch"
    );
    v.gwrite_with(&self.footer, offset, self.properties.as_world_context())?;
    debug!("Size in bytes - actual: {}, expected: {}", offset, size);
    Ok(v.into_boxed_slice())
  }

  /// Given a [`World`] instance, render a PNG to file at `path`.
  ///
  /// # Errors
  /// Emits [`std::io::Error`] if unable to save to the path.
  pub fn render<P>(&self, path: P) -> Result<(), Box<dyn std::error::Error>>
  where
    P: AsRef<std::path::Path>,
  {
    let width = self.properties.width as u32;
    let height = self.properties.height as u32;
    let mut img = RgbaImage::new(width, height);
    self.draw_background(&mut img);
    info!("Rendering {:?} tiles...", width * height);
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
    let underground_level = self.properties.underground_level as u32;
    let cavern_level = self.properties.cavern_level as u32;
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
    if let Some(wall) = tile.wall {
      let color = match wall.wall_paint {
        Some(paint) => PAINT_COLOR_MAP.get(&paint).unwrap(),
        None => WALLTYPE_COLOR_MAP.get(&wall.wall_type).unwrap(),
      };
      img.put_pixel(x, y, *color);
    };
  }

  fn draw_liquid(&self, x: u32, y: u32, img: &mut RgbaImage) {
    let tile = &self.tiles[x as usize][y as usize];
    if let Some(liquid) = tile.liquid {
      let color: Option<Rgba<u8>> = match liquid.liquid_type {
        LiquidType::Water => Some(WATER_COLOR),
        LiquidType::Lava => Some(LAVA_COLOR),
        LiquidType::Honey => Some(HONEY_COLOR),
        _ => None,
      };
      if let Some(c) = color {
        img.put_pixel(x, y, c);
      }
    };
  }

  fn draw_block(&self, x: u32, y: u32, img: &mut RgbaImage) {
    let tile = &self.tiles[x as usize][y as usize];
    if let Some(block) = tile.block {
      let color = match block.block_paint {
        Some(paint) => PAINT_COLOR_MAP.get(&paint).unwrap(),
        None => BLOCKTYPE_COLOR_MAP.get(&block.block_type).unwrap(),
      };
      img.put_pixel(x, y, *color);
    };
  }

  fn draw_wire(&self, x: u32, y: u32, img: &mut RgbaImage) {
    let tile = &self.tiles[x as usize][y as usize];
    if let Some(wiring) = tile.wiring {
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
    };
  }
}
