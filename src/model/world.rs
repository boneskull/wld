use super::header::*;
use crate::model::properties::Properties;
use crate::model::status::Status;
use derive_new::new;
use scroll::{Pread, Pwrite, LE};

#[derive(Clone, Debug, PartialEq, new, Pwrite, Pread)]
pub struct World {
  pub header: Header,
  pub properties: Properties,
  pub status: Status
}

impl World {
  #[inline]
  /// Read a variable length u64 from `bytes` at `offset`
  pub fn read(bytes: &[u8]) -> Result<World, scroll::Error> {
    let world = bytes.pread_with::<World>(0, LE)?;
    Ok(world)
  }
}

