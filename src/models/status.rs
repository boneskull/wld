use crate::{
  enums::{
    AnglerQuestFish,
    InvasionType,
    TBool,
  },
  models::{
    HardmodeOre,
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
  Pread,
  Pwrite,
  LE,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
#[repr(C)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
#[repr(C)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct PillarStatus {
  pub solar: TBool,
  pub vortex: TBool,
  pub nebula: TBool,
  pub stardust: TBool,
  pub is_active: TBool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct SavedNPCs {
  pub goblin_tinkerer: TBool,
  pub wizard: TBool,
  pub mechanic: TBool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
pub struct EventsCompleted {
  pub goblin_army: TBool,
  pub clown: TBool,
  pub frost_moon: TBool,
  pub pirates: TBool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct ShadowOrbStatus {
  pub smashed: TBool,
  pub meteorite_spawned: TBool,
  pub evil_boss_counter: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct RainStatus {
  pub is_active: TBool,
  pub time_left: i32,
  pub max_rain: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct InvasionStatus {
  pub delay: i32,
  pub size: i32,
  pub invasion_type: InvasionType,
  pub position: f64,
  pub slime_rain_time_left: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
#[repr(C)]
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

#[derive(Copy, Clone, Debug, PartialEq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct Clouds {
  pub background: i32,
  pub count: i16,
  pub wind_speed: f32,
}

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct AnglerQuestStatus {
  pub completed_players: Vec<TString>,
  pub angler_saved: TBool,
  pub target: AnglerQuestFish,
}

impl SizeWith<AnglerQuestStatus> for AnglerQuestStatus {
  fn size_with(ctx: &Self) -> usize {
    u8::size_with(&LE)
      + ctx
        .completed_players
        .iter()
        .map(|tstr| TString::size_with(tstr))
        .sum::<usize>()
      + TBool::size_with(&LE)
      + AnglerQuestFish::size_with(&LE)
  }
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
    for _ in 0..completed_players_count {
      completed_players.push(buf.gread::<TString>(offset)?);
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
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let AnglerQuestStatus {
      completed_players,
      angler_saved,
      target,
    } = self;
    let completed_players_count = completed_players.len();
    buf.gwrite(completed_players_count as u8, offset)?;

    for completed_player in completed_players {
      buf.gwrite(completed_player, offset)?;
    }
    buf.gwrite(angler_saved, offset)?;
    buf.gwrite(target, offset)?;
    assert!(
      *offset == AnglerQuestStatus::size_with(self),
      "Size mismatch for AnglerQuestStatus"
    );
    Ok(*offset)
  }
}

#[derive(Clone, Debug, Eq, PartialEq, AsRef)]
#[repr(C)]
pub struct MobKillVec(Vec<i32>);

impl SizeWith<MobKillVec> for MobKillVec {
  fn size_with(ctx: &Self) -> usize {
    i16::size_with(&LE) + (ctx.as_ref().len() * i32::size_with(&LE))
  }
}

impl<'a> TryFromCtx<'a, Endian> for MobKillVec {
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

impl<'a> TryIntoCtx<Endian> for &'a MobKillVec {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let mob_kills = self.as_ref();
    buf.gwrite_with(mob_kills.len() as i16, offset, LE)?;
    for mob_kill in mob_kills {
      buf.gwrite_with(mob_kill, offset, LE)?;
    }
    assert!(
      *offset == MobKillVec::size_with(self),
      "Size mismatch for MobKillVec"
    );
    Ok(*offset)
  }
}

#[derive(Clone, Debug, Eq, PartialEq, AsRef)]
#[repr(C)]
pub struct PartyingNPCVec(Vec<i32>);

impl SizeWith<PartyingNPCVec> for PartyingNPCVec {
  fn size_with(ctx: &Self) -> usize {
    i32::size_with(&LE) + (ctx.as_ref().len() * i32::size_with(&LE))
  }
}

impl<'a> TryFromCtx<'a, Endian> for PartyingNPCVec {
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

impl<'a> TryIntoCtx<Endian> for &'a PartyingNPCVec {
  type Error = scroll::Error;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let partying_npcs = self.as_ref();
    buf.gwrite_with(partying_npcs.len() as i32, offset, LE)?;
    for partying_npc in partying_npcs {
      buf.gwrite_with(partying_npc, offset, LE)?;
    }
    assert!(
      *offset == PartyingNPCVec::size_with(self),
      "Size mismatch for PartyingNPCVec"
    );
    Ok(*offset)
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
#[repr(C)]
pub struct PartyStatus {
  pub party_center_is_active: TBool,
  pub natural_party_is_active: TBool,
  pub party_cooldown: i32,
  pub partying_npcs: PartyingNPCVec,
}

impl SizeWith<PartyStatus> for PartyStatus {
  fn size_with(ctx: &Self) -> usize {
    TBool::size_with(&LE)
      + TBool::size_with(&LE)
      + i32::size_with(&LE)
      + PartyingNPCVec::size_with(&ctx.partying_npcs)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct SandstormStatus {
  pub is_active: TBool,
  pub time_left: i32,
  pub severity: f32,
  pub intended_severity: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct OldOnesArmyStatus {
  pub tier1: TBool,
  pub tier2: TBool,
  pub tier3: TBool,
}

#[derive(Clone, Debug, PartialEq, Pread, Pwrite)]
#[repr(C)]
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
  pub mob_kills: MobKillVec,
  pub fast_forward_time: TBool,
  pub bosses_slain_2: BossesSlain2,
  pub pillar_status: PillarStatus,
  pub party_status: PartyStatus,
  pub sandstorm_status: SandstormStatus,
  pub bartender_saved: TBool,
  pub old_ones_army_status: OldOnesArmyStatus,
}

impl SizeWith<Status> for Status {
  fn size_with(ctx: &Self) -> usize {
    let size = BossesSlain::size_with(&LE)
      + SavedNPCs::size_with(&LE)
      + EventsCompleted::size_with(&LE)
      + ShadowOrbStatus::size_with(&LE)
      + (i32::size_with(&LE) * 3)
      + InvasionStatus::size_with(&LE)
      + u8::size_with(&LE)
      + RainStatus::size_with(&LE)
      + (HardmodeOre::size_with(&LE) * 3)
      + Backgrounds::size_with(&LE)
      + Clouds::size_with(&LE)
      + AnglerQuestStatus::size_with(&ctx.angler_quest_status)
      + (TBool::size_with(&LE) * 5)
      + MobKillVec::size_with(&ctx.mob_kills)
      + BossesSlain2::size_with(&LE)
      + PillarStatus::size_with(&LE)
      + SandstormStatus::size_with(&LE)
      + OldOnesArmyStatus::size_with(&LE)
      + PartyStatus::size_with(&ctx.party_status);
    debug!("Status size: {}", size);
    size
  }
}

#[cfg(test)]
mod test_status {
  use super::{
    AnglerQuestFish,
    AnglerQuestStatus,
    MobKillVec,
    PartyingNPCVec,
    Pread,
    Pwrite,
    SizeWith,
    TBool,
    TString,
  };

  #[test]
  fn test_angler_quest_status_rw() {
    //   pub completed_players: Vec<TString>,
    // pub angler_saved: TBool,
    // pub target: AnglerQuestFish,
    let aqs = AnglerQuestStatus {
      completed_players: vec![TString::from("foo"), TString::from("bar")],
      angler_saved: TBool::True,
      target: AnglerQuestFish::Bonefish,
    };

    let mut buf = [0; 14];
    assert_eq!(14, buf.pwrite(&aqs, 0).unwrap());
    assert_eq!(aqs, buf.pread::<AnglerQuestStatus>(0).unwrap());
  }

  #[test]
  fn test_angler_quest_status_sizewith() {
    // each tstring will be 8 + 3, tbool is 1, and target is 4.
    let aqs = AnglerQuestStatus {
      completed_players: vec![TString::from("foo"), TString::from("bar")],
      angler_saved: TBool::True,
      target: AnglerQuestFish::Bonefish,
    };
    assert_eq!(14, AnglerQuestStatus::size_with(&aqs));
  }

  #[test]
  fn test_mob_kill_vec_rw() {
    let mkv = MobKillVec(vec![2_i32, 4_i32, 6_i32, 8_i32]);

    let mut buf = [0; 18];
    assert_eq!(18, buf.pwrite(&mkv, 0).unwrap());
    assert_eq!(mkv, buf.pread::<MobKillVec>(0).unwrap());
  }

  #[test]
  fn test_partying_npc_vec_rw() {
    let pnpcv = PartyingNPCVec(vec![2_i32, 4_i32, 6_i32, 8_i32]);

    let mut buf = [0; 20];
    assert_eq!(20, buf.pwrite(&pnpcv, 0).unwrap());
    assert_eq!(pnpcv, buf.pread::<PartyingNPCVec>(0).unwrap());
  }
}
