use super::header::*;
use crate::model::properties::Properties;
use crate::model::status::Status;
use derive_new::new;
use scroll::{Pread, Pwrite, LE};

#[derive(Clone, Debug, PartialEq, new, Pwrite, Pread)]
pub struct World {
  pub header: Header,
  pub properties: Properties,
  // pub status: Status
}

impl World {
  #[inline]
  /// Read a variable length u64 from `bytes` at `offset`
  pub fn read(bytes: &[u8]) -> Result<World, scroll::Error> {
    let offset = &mut 0;
    let header = bytes.gread_with::<Header>(offset, LE)?;
    let properties = bytes.gread_with::<Properties>(offset, LE)?;
    // let status = bytes.gread_with::<Status>(offset, LE)?;
    Ok(World {
      header,
      properties,
      // status,
    })
  }
}
