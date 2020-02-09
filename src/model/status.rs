use crate::model::common::*;
use derive_new::new;
use derive_try_from_primitive::*;
use scroll::{
  ctx::{TryFromCtx, TryIntoCtx},
  Endian, Pread, Pwrite,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, new, Pread, Pwrite)]
pub struct BossesSlain {
  pub eye_of_cthulhu: TBool,
  pub eater_of_worlds: TBool,
  pub skeletron: TBool,
  pub queen_bee: TBool,
  pub the_twins: TBool,
  pub the_destroyer: TBool,
  pub skeletron_prime: TBool,
  pub mechanical_boss: TBool,
  pub plantera: TBool,
  pub golem: TBool,
  pub king_slime: TBool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new, Pread, Pwrite)]
pub struct SavedNPCs {
  pub goblin_tinkerer: TBool,
  pub wizard: TBool,
  pub mechanic: TBool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new, Pread, Pwrite)]
pub struct EventsCompleted {
  pub goblin_army: TBool,
  pub clown: TBool,
  pub frost_moon: TBool,
  pub pirates: TBool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new, Pread, Pwrite)]
pub struct ShadowOrbStatus {
  pub smashed: TBool,
  pub meteorite_spawned: TBool,
  pub evil_boss_counter: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, new, Pread, Pwrite)]
pub struct RainStatus {
  pub is_active: TBool,
  pub time_left: i32,
  pub max_rain: f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
pub enum HardmodeOre {
  UnknownOre = -1,
  CobaltOre = 107,
  MythrilOre = 108,
  AdamantiteOre = 111,
  PalladiumOre = 221,
  OrichalcumOre = 222,
  TitaniumOre = 223,
}

impl<'a> TryFromCtx<'a, Endian> for HardmodeOre {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    _ctx: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread::<i32>(offset)?;
    let ore_opt = Self::try_from(value);
    Ok((
      if ore_opt.is_none() {
        Self::UnknownOre
      } else {
        ore_opt.unwrap()
      },
      *offset,
    ))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a HardmodeOre {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let mut size = 0;
    let value = *self as i32;
    size += value.try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
pub enum InvasionType {
  NoInvasion = 0,
  GoblinInvasion = 1,
  FrostLegion = 2,
  PirateInvasion = 3,
  MartianMadness = 4
}

impl<'a> TryFromCtx<'a, Endian> for InvasionType {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    _ctx: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread::<i32>(offset)?;
    let ore_opt = Self::try_from(value);
    Ok((
      if ore_opt.is_none() {
        Self::NoInvasion
      } else {
        ore_opt.unwrap()
      },
      *offset,
    ))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a InvasionType {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let mut size = 0;
    let value = *self as i32;
    size += value.try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, new, Pread, Pwrite)]
pub struct InvasionStatus {
  pub delay: i32,
  pub size: i32,
  pub invasion_type: InvasionType,
  pub position: f64,
  pub slime_rain_time_left: f64
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, new, Pread, Pwrite)]
pub struct Backgrounds {
  pub forest: u8,
  pub corruption: u8,
  pub jungle: u8,
  pub snow: u8,
  pub hallow: u8,
  pub crimson: u8,
  pub desert: u8,
  pub ocean: u8
}

#[derive(Copy, Clone, Debug, PartialEq, new, Pread, Pwrite)]
pub struct Status {
  pub bosses_slain: BossesSlain,
  pub saved_npcs: SavedNPCs,
  pub events_completed: EventsCompleted,
  pub shadow_orb_status: ShadowOrbStatus,
  pub smashed_altar_count: i32,
  pub is_hardmode: TBool,
  pub invasion_status: InvasionStatus,
  pub sundial_cooldown: u8,
  pub rain_status: RainStatus,
  pub hardmode_ore_1: HardmodeOre,
  pub hardmode_ore_2: HardmodeOre,
  pub hardmode_ore_3: HardmodeOre,
  pub backgrounds: Backgrounds
}
