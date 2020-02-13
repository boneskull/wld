use crate::model::common::*;
use num_traits::FromPrimitive;
use scroll::{
  ctx::{
    TryFromCtx,
    TryIntoCtx,
  },
  Endian,
  Pread,
  Pwrite,
  LE,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
pub struct BossesSlain2 {
  pub duke_fishron: TBool,
  pub martian_madness: TBool,
  pub lunatic_cultist: TBool,
  pub moon_lord: TBool,
  pub pumpking: TBool,
  pub mourning_wood: TBool,
  pub ice_queen: TBool,
  pub santa_nk1: TBool,
  pub everscream: TBool,
  pub solar_pillar: TBool,
  pub vortex_pillar: TBool,
  pub nebula_pillar: TBool,
  pub stardust_pillar: TBool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
pub struct PillarStatus {
  pub solar: TBool,
  pub vortex: TBool,
  pub nebula: TBool,
  pub stardust: TBool,
  pub is_active: TBool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
pub struct SavedNPCs {
  pub goblin_tinkerer: TBool,
  pub wizard: TBool,
  pub mechanic: TBool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
pub struct EventsCompleted {
  pub goblin_army: TBool,
  pub clown: TBool,
  pub frost_moon: TBool,
  pub pirates: TBool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
pub struct ShadowOrbStatus {
  pub smashed: TBool,
  pub meteorite_spawned: TBool,
  pub evil_boss_counter: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Pread, Pwrite)]
pub struct RainStatus {
  pub is_active: TBool,
  pub time_left: i32,
  pub max_rain: f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
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
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread_with::<i32>(offset, LE)?;
    let ore_opt = Self::from_i32(value);
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
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
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread_with::<i32>(offset, LE)?;
    let ore_opt = Self::from_i32(value);
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

#[derive(Copy, Clone, Debug, PartialEq, Pread, Pwrite)]
pub struct InvasionStatus {
  pub delay: i32,
  pub size: i32,
  pub invasion_type: InvasionType,
  pub position: f64,
  pub slime_rain_time_left: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
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

#[derive(Copy, Clone, Debug, PartialEq, Pread, Pwrite)]
pub struct Clouds {
  pub background: i32,
  pub count: i16,
  pub wind_speed: f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
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
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = buf.gread_with::<i32>(offset, LE)?;
    Ok((Self::from_i32(value).unwrap(), *offset))
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

#[derive(Clone, Debug, PartialEq)]
pub struct AnglerQuestStatus {
  pub completed_players: Vec<TString>,
  pub angler_saved: TBool,
  pub target: AnglerQuestFish,
}

impl<'a> TryFromCtx<'a, Endian> for AnglerQuestStatus {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let completed_players_count = buf.gread::<u8>(offset)?;
    let mut completed_players: Vec<TString> = vec![];
    if completed_players_count == 0 {
      completed_players.push(TString::from(""));
    } else {
      for _ in 0..completed_players_count {
        completed_players.push(buf.gread::<TString>(offset)?);
      }
    }
    let angler_saved = buf.gread::<TBool>(offset)?;
    let target = buf.gread::<AnglerQuestFish>(offset)?;
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

#[derive(Clone, Debug, Eq, PartialEq, AsRef)]
pub struct MobKills(Vec<i32>);

impl<'a> TryFromCtx<'a, Endian> for MobKills {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let mob_kills_count = buf.gread_with::<i16>(offset, LE)?;
    let mut mob_kills: Vec<i32> = vec![];
    for _ in 0..mob_kills_count {
      mob_kills.push(buf.gread_with::<i32>(offset, LE)?);
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
    let mob_kills = self.as_ref();
    let mob_kills_count = mob_kills.len();
    size += mob_kills_count.try_into_ctx(&mut buf[size..], ctx)?;
    mob_kills
      .iter()
      .for_each(|k| size += k.try_into_ctx(&mut buf[size..], ctx).unwrap());
    Ok(size)
  }
}

#[derive(Clone, Debug, Eq, PartialEq, AsRef)]
pub struct PartyingNPCs(Vec<i32>);

impl<'a> TryFromCtx<'a, Endian> for PartyingNPCs {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let partying_npcs_count = buf.gread_with::<i32>(offset, LE)?;
    let mut partying_npcs: Vec<i32> = vec![];
    for _ in 0..partying_npcs_count {
      partying_npcs.push(buf.gread_with::<i32>(offset, LE)?);
    }
    Ok((Self(partying_npcs), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a PartyingNPCs {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let mut size = 0;
    let partying_npcs = self.as_ref();
    let partying_npcs_count = partying_npcs.len();
    size += partying_npcs_count.try_into_ctx(&mut buf[size..], ctx)?;
    partying_npcs
      .iter()
      .for_each(|k| size += k.try_into_ctx(&mut buf[size..], ctx).unwrap());
    Ok(size)
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
pub struct PartyStatus {
  pub party_center_is_active: TBool,
  pub natural_party_is_active: TBool,
  pub party_cooldown: i32,
  pub partying_npcs: PartyingNPCs,
}

#[derive(Copy, Clone, Debug, PartialEq, Pread, Pwrite)]
pub struct SandstormStatus {
  pub is_active: TBool,
  pub time_left: i32,
  pub severity: f32,
  pub intended_severity: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
pub struct OldOnesArmyStatus {
  pub tier1: TBool,
  pub tier2: TBool,
  pub tier3: TBool,
}

#[derive(Clone, Debug, PartialEq, Pread, Pwrite)]
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
  pub fast_forward_time: TBool,
  pub bosses_slain_2: BossesSlain2,
  pub pillar_status: PillarStatus,
  pub party_status: PartyStatus,
  pub sandstorm_status: SandstormStatus,
  pub bartender_saved: TBool,
  pub old_ones_army_status: OldOnesArmyStatus,
}
