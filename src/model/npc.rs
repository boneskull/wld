use super::common::*;
use crate::enums::EntityType;
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

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct NPC {
  pub entity_type: EntityType,
  pub position: Point,
  pub name: TString,
  pub home_position: Point,
  pub is_homeless: TBool,
}

impl SizeWith<NPC> for NPC {
  fn size_with(ctx: &NPC) -> usize {
    EntityType::size_with(&LE)
      + TString::size_with(&ctx.name)
      + (Point::size_with(&LE) * 2)
      + TBool::size_with(&LE)
  }
}

impl<'a> TryFromCtx<'a, Endian> for NPC {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let entity_type = buf.gread::<EntityType>(offset)?;
    let name = buf.gread::<TString>(offset)?;
    let position = Point {
      x: buf.gread_with::<f32>(offset, LE)? as i32,
      y: buf.gread_with::<f32>(offset, LE)? as i32,
    };
    let is_homeless = buf.gread_with::<TBool>(offset, LE)?;
    let home_position = buf.gread_with::<Point>(offset, LE)?;
    Ok((
      Self {
        entity_type,
        position,
        name,
        home_position,
        is_homeless,
      },
      *offset,
    ))
  }
}

impl TryIntoCtx<Endian> for &NPC {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let NPC {
      entity_type,
      name,
      position,
      home_position,
      is_homeless,
    } = self;
    buf.gwrite(entity_type, offset)?;
    buf.gwrite(name, offset)?;
    let Point { x, y } = position;
    buf.gwrite_with(*x as f32, offset, LE)?;
    buf.gwrite_with(*y as f32, offset, LE)?;
    buf.gwrite(is_homeless, offset)?;
    buf.gwrite(home_position, offset)?;
    Ok(*offset)
  }
}

#[derive(Clone, Debug, Default, PartialEq, AsRef)]
#[repr(C)]
pub struct NPCVec(Vec<NPC>);

impl SizeWith<NPCVec> for NPCVec {
  fn size_with(ctx: &NPCVec) -> usize {
    let mut size: usize = ctx
      .as_ref()
      .iter()
      .map(|npc| u8::size_with(&LE) + NPC::size_with(&npc))
      .fold(0, |acc, len| acc + len);
    if size == 0 {
      size = u8::size_with(&LE);
    }
    eprintln!("NPCVec size: {}", size);
    size
  }
}

// todo: investigate a trait to dedupe this TryFromCtx
impl<'a> TryFromCtx<'a, Endian> for NPCVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let mut npcs: Vec<NPC> = vec![];
    let mut more_npcs = buf.gread_with::<TBool>(offset, LE)?;
    while more_npcs == TBool::True {
      let npc = buf.gread::<NPC>(offset)?;
      npcs.push(npc);
      more_npcs = buf.gread_with::<TBool>(offset, LE)?;
    }
    Ok((Self(npcs), *offset))
  }
}

impl TryIntoCtx<Endian> for &NPCVec {
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
    Ok(*offset)
  }
}

#[derive(Clone, Debug, PartialEq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct Mob {
  pub entity_type: EntityType,
  pub position: Point,
}

#[derive(Clone, Debug, Default, PartialEq, AsRef)]
#[repr(C)]
pub struct MobVec(Vec<Mob>);

impl SizeWith<MobVec> for MobVec {
  fn size_with(ctx: &MobVec) -> usize {
    let mut size =
      ctx.as_ref().len() * (Mob::size_with(&LE) + u8::size_with(&LE));
    if size == 0 {
      size = u8::size_with(&LE);
    }
    eprintln!("MobVec size: {}", size);
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
    let mut more_mobs = buf.gread_with::<TBool>(offset, LE)?;
    while more_mobs == TBool::True {
      let mob = buf.gread::<Mob>(offset)?;
      mobs.push(mob);
      more_mobs = buf.gread_with::<TBool>(offset, LE)?;
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
    Ok(*offset)
  }
}
#[cfg(test)]
mod test_npc {
  use super::*;

  #[test]
  fn test_npc_rw() {
    let npc = NPC {
      entity_type: EntityType::TruffleWorm,
      position: Point { x: 0, y: 0 },
      name: TString::from("Marvin K. Mooney"),
      home_position: Point { x: -100, y: -100 },
      is_homeless: TBool::False,
    };

    let mut buf = [0; 38];
    assert_eq!(38, buf.pwrite(&npc, 0).unwrap());
    assert_eq!(npc, buf.pread::<NPC>(0).unwrap());
  }

  #[test]
  fn test_mob_rw() {
    let mob = Mob {
      entity_type: EntityType::TruffleWorm,
      position: Point { x: 0, y: 0 },
    };

    let mut buf = [0; 12];
    assert_eq!(12, buf.pwrite(&mob, 0).unwrap());
    assert_eq!(mob, buf.pread::<Mob>(0).unwrap());
  }

  #[test]
  fn test_npc_vec_rw() {
    let npc_vec = NPCVec(vec![
      NPC {
        entity_type: EntityType::TruffleWorm,
        position: Point { x: 0, y: 0 },
        name: "Marvin K. Mooney".into(),
        home_position: Point { x: -100, y: -100 },
        is_homeless: TBool::False,
      },
      NPC {
        entity_type: EntityType::Duck,
        position: Point { x: 0, y: 0 },
        name: "Dave".into(),
        home_position: Point { x: -100, y: -100 },
        is_homeless: TBool::False,
      },
    ]);

    let mut buf = [0; 67];
    assert_eq!(67, buf.pwrite(&npc_vec, 0).unwrap());
    assert_eq!(npc_vec, buf.pread::<NPCVec>(0).unwrap());
  }

  #[test]
  fn test_mob_vec_rw() {
    let mob_vec = MobVec(vec![
      Mob {
        entity_type: EntityType::TruffleWorm,
        position: Point { x: 0, y: 0 },
      },
      Mob {
        entity_type: EntityType::Duck,
        position: Point { x: 0, y: 0 },
      },
    ]);

    let mut buf = [0; 27];
    assert_eq!(27, buf.pwrite(&mob_vec, 0).unwrap());
    assert_eq!(mob_vec, buf.pread::<MobVec>(0).unwrap());
  }
}
