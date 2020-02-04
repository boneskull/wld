use derive_new::new;
use super::common::Point;
use super::properties::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, new)]
pub struct Header {
  pub version: String,
  pub revision: u32,
  pub is_favorite: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
pub struct Properties {
  pub dungeon_point: Point,
  pub evil_type: EvilType
}

#[derive(Copy, Clone, Debug, Default, PartialEq, new)]
pub struct Status {
  pub time: f32, // may be f64
  pub is_daytime: bool,
  pub moon_phase: u32,
  pub is_blood_moon: bool,
  pub is_eclipse: bool
}

#[derive(Clone, Debug, PartialEq, new)]
pub struct World {
  pub header: Header,
  // pub properties: Properties,
  // pub status: Status
}
