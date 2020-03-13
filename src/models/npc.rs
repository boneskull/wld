use crate::{
  enums::{
    EntityType,
    TBool,
  },
  models::{
    Position,
    TString,
  },
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

/// Represents a friendly NPC.
///
/// See [Terraria Wiki: NPCs](https://terraria.gamepedia.com/NPCs) for more information.
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct NPC {
  /// The type of NPC.
  pub entity_type: EntityType,

  /// X-coordinate.  I don't know why it's an [`f32`].
  pub position_x: f32,

  /// Y-coordinate.  I don't know why it's an [`f32`], either.
  pub position_y: f32,

  /// The name of this NPC.
  pub name: TString,

  /// This _may_ correspond to a [`House`] at the same position.
  ///
  /// It will be [`None`] if [`is_homeless`] is [`True`].
  ///
  /// [`House`]: crate::models::House
  /// [`is_homeless`]: NPC::is_homeless
  /// [`True`]: TBool::True
  pub home_position: Option<Position>,

  /// If [`True`], this NPC currently does not have living arrangements.
  ///
  /// [`True`]: TBool::True
  pub is_homeless: TBool,
}

impl SizeWith<NPC> for NPC {
  fn size_with(ctx: &Self) -> usize {
    EntityType::size_with(&LE)
      + TString::size_with(&ctx.name)
      + TBool::size_with(&LE)
      + (f32::size_with(&LE) * 2)
      + match ctx.home_position {
        Some(_) => Position::size_with(&LE),
        None => 0,
      }
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

    let position_x = buf.gread_with::<f32>(offset, LE)?;
    let position_y = buf.gread_with::<f32>(offset, LE)?;
    let is_homeless = buf.gread_with::<TBool>(offset, LE)?;
    let home_position = if is_homeless == TBool::False {
      Some(buf.gread_with::<Position>(offset, LE)?)
    } else {
      None
    };
    Ok((
      Self {
        entity_type,
        position_x,
        position_y,
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
      position_x,
      position_y,
      home_position,
      is_homeless,
    } = self;
    buf.gwrite(entity_type, offset)?;
    buf.gwrite(name, offset)?;
    buf.gwrite_with(position_x, offset, LE)?;
    buf.gwrite_with(position_y, offset, LE)?;
    buf.gwrite(is_homeless, offset)?;
    if let Some(hp) = home_position {
      buf.gwrite(hp, offset)?;
    }

    assert!(*offset == NPC::size_with(self), "NPC size mismatch");
    Ok(*offset)
  }
}

#[derive(Clone, Debug, Default, PartialEq, AsRef)]
#[repr(C)]
pub struct NPCVec(Vec<NPC>);

impl SizeWith<NPCVec> for NPCVec {
  fn size_with(ctx: &Self) -> usize {
    let size: usize = TBool::size_with(&LE)
      + ctx
        .as_ref()
        .iter()
        .map(|npc| TBool::size_with(&LE) + NPC::size_with(npc))
        .sum::<usize>();
    debug!("NPCVec size: {}", size);
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
    while buf.gread_with::<TBool>(offset, LE)? == TBool::True {
      let npc = buf.gread::<NPC>(offset)?;
      npcs.push(npc);
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
    for (i, npc) in vec.iter().enumerate() {
      buf.gwrite(npc, offset)?;
      if i == len - 1 {
        buf.gwrite(&TBool::False, offset)?;
      } else {
        buf.gwrite(&TBool::True, offset)?;
      }
    }
    assert!(*offset == NPCVec::size_with(self), "NPCVec size mismatch");

    Ok(*offset)
  }
}

#[cfg(test)]
mod test_npc {
  use super::{
    EntityType,
    NPCVec,
    Position,
    Pread,
    Pwrite,
    TBool,
    TString,
    NPC,
  };

  #[test]
  fn test_npc_vec_rw() {
    let npc_vec = NPCVec(vec![
      NPC {
        entity_type: EntityType::TruffleWorm,
        position_x: 0.0,
        position_y: 0.0,
        name: "Marvin K. Mooney".into(),
        home_position: Some(Position { x: -100, y: -100 }),
        is_homeless: TBool::False,
      },
      NPC {
        entity_type: EntityType::Duck,
        position_x: 0.0,
        position_y: 0.0,
        name: "Dave".into(),
        home_position: Some(Position { x: -100, y: -100 }),
        is_homeless: TBool::False,
      },
    ]);

    let mut buf = [0; 67];
    assert_eq!(67, buf.pwrite(&npc_vec, 0).unwrap());
    assert_eq!(npc_vec, buf.pread::<NPCVec>(0).unwrap());
  }
  #[test]
  fn test_npc_rw() {
    let npc = NPC {
      entity_type: EntityType::TruffleWorm,
      position_x: 0.0,
      position_y: 0.0,
      name: TString::from("Marvin K. Mooney"),
      home_position: Some(Position { x: -100, y: -100 }),
      is_homeless: TBool::False,
    };

    let mut buf = [0; 38];
    assert_eq!(38, buf.pwrite(&npc, 0).unwrap());
    assert_eq!(npc, buf.pread::<NPC>(0).unwrap());
  }
}
