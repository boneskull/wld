use crate::model::TBitVec;
use num_traits::FromPrimitive;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum BlockShape {
  Normal = 0,
  HalfTile = 1,
  TopRightSlope = 2,
  TopLeftSlope = 3,
  BottomRightSlope = 4,
  BottomLeftSlope = 5,
}

impl Default for BlockShape {
  fn default() -> Self {
    Self::Normal
  }
}

impl From<&TBitVec> for BlockShape {
  fn from(flags: &TBitVec) -> Self {
    let value =
      ((flags[6] as u8) << 2) + ((flags[5] as u8) << 1) + flags[4] as u8;
    Self::from_u8(value).unwrap()
  }
}

impl BlockShape {
  pub fn assign_bits(&self, attrs: &mut TBitVec) {
    match self {
      Self::HalfTile => {
        attrs.set(4, true);
      }
      Self::TopRightSlope => {
        attrs.set(5, true);
      }
      Self::TopLeftSlope => {
        attrs.set(4, true);
        attrs.set(5, true);
      }
      Self::BottomRightSlope => {
        attrs.set(6, true);
      }
      Self::BottomLeftSlope => {
        attrs.set(4, true);
        attrs.set(6, true);
      }
      _ => {}
    }
  }
}
