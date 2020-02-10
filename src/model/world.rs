use super::header::*;
use crate::model::properties::Properties;
use crate::model::status::Status;
use crate::model::tiles::*;
use derive_new::new;
use scroll::{Pread, Pwrite, LE};

type Tiles = Vec<Vec<Tile>>;

#[derive(Clone, Debug, PartialEq, new)]
pub struct World {
  pub status: WorldStatus,
  pub tiles: Tiles,
}

#[derive(Clone, Debug, PartialEq, new, Pwrite, Pread)]
pub struct WorldStatus {
  pub header: Header,
  pub properties: Properties,
  pub status: Status,
}

impl World {
  #[inline]
  pub fn read(bytes: &[u8]) -> Result<World, scroll::Error> {
    let mut offset = 0;
    let status = bytes.gread_with::<WorldStatus>(&mut offset, LE)?;
    let tiles = parse_tile_matrix(
      bytes,
      &mut offset,
      &status.properties.width,
      &status.properties.height,
      &status.properties.tile_frame_importances,
    );
    Ok(World { status, tiles })
  }
}
