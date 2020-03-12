use crate::enums::{
  EntityType,
  TBool,
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

/// Represents a "mob"; a non-friendly NPC entity.
///
/// # Notes
///
/// - I'm not sure when this is actually used, as mobs don't typically
///   persist.  Maybe it's for events?
#[derive(Clone, Debug, PartialEq, SizeWith, Pread, Pwrite)]
#[repr(C)]
pub struct Mob {
  /// The type of mob.
  pub entity_type: EntityType,

  /// X-coordinate.  I don't know why it's an [`f32`].
  pub position_x: f32,

  /// Y-coordinate.  I don't know why it's an [`f32`], either.
  pub position_y: f32,
}

/// List of [`Mob`]s; fancy wrapper around a [`Vec`].
///
/// # Notes
///
/// - When reading at the offset (see [`Offsets`]), we start by reading a byte flag (a [`u8`]).
/// - Instead of the list being preceded by a numeric count of mobs, each chunk of mob data is
///   followed by a byte flag.  If the byte flag is true (nonzero), then we read another mob.
///   If it's false (zero), then we're done.
///
/// [`Offsets`]: crate::models::Offsets
#[derive(Clone, Debug, Default, PartialEq, AsRef)]
#[repr(C)]
pub struct MobVec(Vec<Mob>);

impl SizeWith<MobVec> for MobVec {
  fn size_with(ctx: &Self) -> usize {
    let size = TBool::size_with(&LE)
      + (ctx.as_ref().len() * (Mob::size_with(&LE) + TBool::size_with(&LE)));
    debug!("MobVec size: {}", size);
    size
  }
}

impl<'a> TryFromCtx<'a, Endian> for MobVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let mut mobs: Vec<Mob> = vec![];
    while buf.gread_with::<TBool>(offset, LE)? == TBool::True {
      let mob = buf.gread::<Mob>(offset)?;
      mobs.push(mob);
    }
    Ok((Self(mobs), *offset))
  }
}

impl TryIntoCtx<Endian> for &MobVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let vec = self.as_ref();
    let len = vec.len();
    if len > 0 {
      buf.gwrite(&TBool::True, offset)?;
    } else {
      buf.gwrite(&TBool::False, offset)?;
    }
    for (i, mob) in vec.iter().enumerate() {
      buf.gwrite(mob, offset)?;
      if i == len - 1 {
        buf.gwrite(&TBool::False, offset)?;
      } else {
        buf.gwrite(&TBool::True, offset)?;
      }
    }
    assert!(*offset == MobVec::size_with(self), "MobVec size mismatch");

    Ok(*offset)
  }
}

#[cfg(test)]
mod test_mob {
  use super::{
    EntityType,
    Mob,
    MobVec,
    Pread,
    Pwrite,
  };

  #[test]
  fn test_mob_rw() {
    let mob = Mob {
      entity_type: EntityType::TruffleWorm,
      position_x: 0.0,
      position_y: 0.0,
    };

    let mut buf = [0; 12];
    assert_eq!(12, buf.pwrite(&mob, 0).unwrap());
    assert_eq!(mob, buf.pread::<Mob>(0).unwrap());
  }

  #[test]
  fn test_mob_vec_rw() {
    let mob_vec = MobVec(vec![
      Mob {
        entity_type: EntityType::TruffleWorm,
        position_x: 0.0,
        position_y: 0.0,
      },
      Mob {
        entity_type: EntityType::Duck,
        position_x: 0.0,
        position_y: 0.0,
      },
    ]);

    let mut buf = [0; 27];
    assert_eq!(27, buf.pwrite(&mob_vec, 0).unwrap());
    assert_eq!(mob_vec, buf.pread::<MobVec>(0).unwrap());
  }
}
