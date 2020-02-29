use crate::{
  enums::EntityType,
  model::TBool,
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

#[derive(Clone, Debug, PartialEq, SizeWith, Pread, Pwrite)]
#[repr(C)]
pub struct Mob {
  pub entity_type: EntityType,
  pub position_x: f32,
  pub position_y: f32,
}

#[derive(Clone, Debug, Default, PartialEq, AsRef)]
#[repr(C)]
pub struct MobVec(Vec<Mob>);

impl SizeWith<MobVec> for MobVec {
  fn size_with(ctx: &MobVec) -> usize {
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
    for i in 0..len {
      buf.gwrite(&vec[i], offset)?;
      if i == len - 1 {
        buf.gwrite(&TBool::False, offset)?;
      } else {
        buf.gwrite(&TBool::True, offset)?;
      }
    }
    assert!(*offset == MobVec::size_with(&self), "MobVec size mismatch");

    Ok(*offset)
  }
}

#[cfg(test)]
mod test_mob {
  use super::*;

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
