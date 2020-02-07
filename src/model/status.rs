use crate::model::common::*;
use derive_new::new;
use scroll::{Pread, Pwrite};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new, Pread, Pwrite)]
pub struct BossesSlain {
  pub eye_of_cthulhu: TBool,
  pub eater_of_world: TBool,
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new, Pread, Pwrite)]
pub struct SavedNPCs {
  pub goblin_tinkerer: TBool,
  pub wizard: TBool,
  pub mechanic: TBool
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new, Pread, Pwrite)]
pub struct EventsCompleted {
  pub goblin_army: TBool,
  pub clown: TBool,
  pub frost_moon: TBool,
  pub pirates: TBool
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new, Pread, Pwrite)]
pub struct ShadowOrbStatus {
  pub smashed: TBool,
  pub meteorite_spawned: TBool,
  pub evil_boss_counter: i32
}

#[derive(Copy, Clone, Debug, Default, PartialEq, new, Pread, Pwrite)]
pub struct RainStatus {
  pub is_active: TBool,
  pub time_left: i32,
  pub max_rain: f32
}


type HardmodeOre = i32;
type InvasionType = i32;

#[derive(Copy, Clone, Debug, Default, PartialEq, new, Pread, Pwrite)]
pub struct Status {
  pub bosses_slain: BossesSlain,
  pub saved_npcs: SavedNPCs,
  pub events_completed: EventsCompleted,
  pub shadow_orb_status: ShadowOrbStatus,
  pub smashed_altar_count: i32,
  pub is_hardmode: TBool,
  pub invasion_delay: i32,
  pub invasion_size: i32,
  pub invasion_type: InvasionType, // todo
  pub invasion_x: f64,
  pub slime_rain_time_remaining: f64,
  pub sundial_cooldown: u8,
  pub rain_status: RainStatus,
  pub hardmode_ore_1: HardmodeOre,
  pub hardmode_ore_2: HardmodeOre,
  pub hardmode_ore_3: HardmodeOre,
  // bg_forest ...
}
