use derive_new::new;
use scroll::Pwrite;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new, Pwrite)]
pub struct Rect {
  pub left: i32,
  pub right: i32,
  pub top: i32,
  pub bottom: i32,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, new, Pwrite)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}
