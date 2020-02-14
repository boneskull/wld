use crate::model::common::TBitVec;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum LiquidType {
  NoLiquid = 0,
  Water = 1,
  Lava = 2,
  Honey = 3,
}

impl From<&TBitVec> for LiquidType {
  fn from(flags: &TBitVec) -> Self {
    if flags[3] && flags[4] {
      LiquidType::Honey
    } else if flags[4] {
      LiquidType::Lava
    } else if flags[3] {
      LiquidType::Water
    } else {
      LiquidType::NoLiquid
    }
  }
}
