use crate::models::TBitVec;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum LiquidType {
  NoLiquid = 0,
  Water = 1,
  Lava = 2,
  Honey = 3,
}

impl LiquidType {
  pub fn assign_bits(self, tbv: &mut TBitVec) {
    match self {
      Self::Water => {
        tbv.set(3, true);
      }
      Self::Honey => {
        tbv.set(3, true);
        tbv.set(4, true);
      }
      Self::Lava => {
        tbv.set(4, true);
      }
      _ => {}
    }
  }
}

impl From<&TBitVec> for LiquidType {
  fn from(flags: &TBitVec) -> Self {
    if flags[3] && flags[4] {
      Self::Honey
    } else if flags[4] {
      Self::Lava
    } else if flags[3] {
      Self::Water
    } else {
      Self::NoLiquid
    }
  }
}
