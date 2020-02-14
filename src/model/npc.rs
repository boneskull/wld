use super::common::*;
use crate::enums::EntityType;
use scroll::{
  ctx::TryFromCtx,
  Error as ScrollError,
  Pread,
  LE,
};

#[derive(Clone, Debug, PartialEq)]
pub struct NPC {
  pub entity: EntityType,
  pub position: (f32, f32),
  pub name: TString,
  pub home_position: Option<Point>,
}

impl<'a> TryFromCtx<'a, ()> for NPC {
  type Error = ScrollError;

  fn try_from_ctx(buf: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let entity = buf.gread::<EntityType>(offset)?;
    let name = buf.gread::<TString>(offset)?;
    let position = (
      buf.gread_with::<f32>(offset, LE)?,
      buf.gread_with::<f32>(offset, LE)?,
    );
    let is_homeless = buf.gread_with::<TBool>(offset, LE)?;
    let mut home_position = Some(buf.gread_with::<Point>(offset, LE)?);
    if is_homeless == TBool::True {
      home_position = None;
    }
    Ok((
      Self {
        entity,
        position,
        name,
        home_position,
      },
      *offset,
    ))
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mob {
  pub entity: EntityType,
  pub position: (f32, f32),
}

impl<'a> TryFromCtx<'a, ()> for Mob {
  type Error = ScrollError;

  fn try_from_ctx(buf: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let entity = buf.gread::<EntityType>(offset)?;
    let position = (
      buf.gread_with::<f32>(offset, LE)?,
      buf.gread_with::<f32>(offset, LE)?,
    );
    Ok((Self { entity, position }, *offset))
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NPCVec(Vec<NPC>);

// todo: investigate a trait to dedupe this TryFromCtx

impl<'a> TryFromCtx<'a, ()> for NPCVec {
  type Error = ScrollError;

  fn try_from_ctx(buf: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MobVec(Vec<Mob>);

impl<'a> TryFromCtx<'a, ()> for MobVec {
  type Error = ScrollError;

  fn try_from_ctx(buf: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
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
