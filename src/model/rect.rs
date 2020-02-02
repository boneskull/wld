
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Rect {
  pub left: i32,
  pub right: i32,
  pub top: i32,
  pub bottom: i32,
}

impl Rect {
  pub fn new(left: i32, right: i32, top: i32, bottom: i32) -> Self {
    Rect {
      left,
      right,
      top,
      bottom,
    }
  }
}
