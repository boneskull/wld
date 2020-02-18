use crate::model::common::TBitVec;
use num_traits::FromPrimitive;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum RLEType {
  NoCompression = 0,
  SingleByte = 1,
  DoubleByte = 2,
}

impl From<&TBitVec> for RLEType {
  fn from(flags: &TBitVec) -> Self {
    let value = ((flags[7] as u8) << 1) + flags[6] as u8;
    Self::from_u8(value).unwrap()
  }
}

impl RLEType {
  pub fn assign_bits(&self, tbv: &mut TBitVec) {
    let bv = tbv.as_mut();
    match self {
      Self::SingleByte => {
        bv.set(6, true);
      }
      Self::DoubleByte => {
        bv.set(7, true);
      }
      _ => {}
    }
  }
}
