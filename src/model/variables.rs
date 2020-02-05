use derive_new::new;

#[derive(Copy, Clone, Debug, Default, PartialEq, new)]
pub struct Variables {
  pub time: f32, // may be f64
  pub is_daytime: bool,
  pub moon_phase: u32,
  pub is_blood_moon: bool,
  pub is_eclipse: bool,
}
