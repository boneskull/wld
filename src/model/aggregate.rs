use crate::model::common::TBool;

pub struct TimeStatus<'a> {
  pub current_time: &'a f64,
  pub is_daytime: &'a TBool,
  pub moon_phase: &'a u32,
  pub sundial_cooldown: &'a u8,
  pub fast_forward_time: &'a TBool,
}
