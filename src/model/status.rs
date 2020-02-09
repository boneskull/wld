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
  MartianMadness = 4,
}

impl<'a> TryFromCtx<'a, Endian> for InvasionType {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread_with::<i32>(offset, ctx)?;
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
  pub slime_rain_time_left: f64,
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
  pub ocean: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, new, Pread, Pwrite)]
pub struct Clouds {
  pub background: i32,
  pub count: i16,
  pub wind_speed: f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
pub enum AnglerQuestFish {
  Batfish = 0,
  BumblebeeTuna = 1,
  Catfish = 2,
  Cloudfish = 3,
  Cursedfish = 4,
  Dirtfish = 5,
  DynamiteFish = 6,
  EaterOfPlankton = 7,
  FallenStarfish = 8,
  TheFishOfCthulhu = 9,
  Fishotron = 10,
  Harpyfish = 11,
  Hungerfish = 12,
  Ichorfish = 13,
  Jewelfish = 14,
  MirageFish = 15,
  MutantFlinxfin = 16,
  Pengfish = 17,
  Pixiefish = 18,
  Spiderfish = 19,
  TundraTrout = 20,
  UnicornFish = 21,
  GuideVoodooFish = 22,
  Wyverntail = 23,
  ZombieFish = 24,
  AmanitiaFungifin = 25,
  Angelfish = 26,
  BloodyManowar = 27,
  Bonefish = 28,
  Bunnyfish = 29,
  CapnTunabeard = 30,
  Clownfish = 31,
  DemonicHellfish = 32,
  Derpfish = 33,
  Fishron = 34,
  InfectedScabbardfish = 35,
  Mudfish = 36,
  Slimefish = 37,
  TropicalBarracuda = 38,
}

impl<'a> TryFromCtx<'a, Endian> for AnglerQuestFish {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread_with::<i32>(offset, ctx)?;
    Ok((Self::try_from(value).unwrap(), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a AnglerQuestFish {
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

#[derive(Clone, Debug, PartialEq, new)]
pub struct AnglerQuestStatus {
  pub completed_players: Vec<TString>,
  pub angler_saved: TBool,
  pub target: AnglerQuestFish,
}

impl<'a> TryFromCtx<'a, Endian> for AnglerQuestStatus {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let completed_players_count = buf.gread::<u8>(offset)?;
    let mut completed_players: Vec<TString> = vec![];
    if completed_players_count == 0 {
      completed_players.push(TString::from(""));
    } else {
      for _ in 0..completed_players_count {
        completed_players.push(buf.gread_with::<TString>(offset, ctx)?);
      }
    }
    let angler_saved = buf.gread_with::<TBool>(offset, ctx)?;
    let target = buf.gread_with::<AnglerQuestFish>(offset, ctx)?;
    Ok((
      Self {
        completed_players,
        angler_saved,
        target,
      },
      *offset,
    ))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a AnglerQuestStatus {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let mut size = 0;
    let completed_players_count = self.completed_players.len();
    size += completed_players_count.try_into_ctx(&mut buf[size..], ctx)?;
    self
      .completed_players
      .iter()
      .for_each(|s| size += s.try_into_ctx(&mut buf[size..], ctx).unwrap());
    size += self.target.try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}

#[derive(Clone, Debug, Eq, PartialEq, new)]
pub struct MobKills(Vec<i32>);

impl<'a> TryFromCtx<'a, Endian> for MobKills {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    ctx: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let mob_kills_count = buf.gread_with::<i16>(offset, ctx)?;
    let mut mob_kills: Vec<i32> = vec![];
    for _ in 0..mob_kills_count {
      mob_kills.push(buf.gread_with::<i32>(offset, ctx)?);
    }
    Ok((Self(mob_kills), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a MobKills {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let mut size = 0;
    let mob_kills_count = self.0.len();
    size += mob_kills_count.try_into_ctx(&mut buf[size..], ctx)?;
    self
      .0
      .iter()
      .for_each(|k| size += k.try_into_ctx(&mut buf[size..], ctx).unwrap());
    Ok(size)
  }
}

#[derive(Clone, Debug, PartialEq, new, Pread, Pwrite)]
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
  pub backgrounds: Backgrounds,
  pub clouds: Clouds,
  pub angler_quest_status: AnglerQuestStatus,
  pub stylist_saved: TBool,
  pub tax_collector_saved: TBool,
  pub invasion_size_start: i32,
  pub cultist_delay: i32,
  pub mob_kills: MobKills,
}
