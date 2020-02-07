use super::header::*;
use crate::model::properties::Properties;
use crate::model::variables::Variables;
use derive_new::new;
use scroll::{ctx::TryFromCtx, Pread, Pwrite, LE};

#[derive(Clone, Debug, PartialEq, new, Pwrite, Pread)]
pub struct World {
  pub header: Header,
  pub properties: Properties,
  pub variables: Variables, // pub status: Status
}

impl World {
  #[inline]
  /// Read a variable length u64 from `bytes` at `offset`
  pub fn read(bytes: &[u8]) -> Result<World, scroll::Error> {
    let (world, _) = World::try_from_ctx(bytes, LE)?;
    Ok(world)
  }
}
