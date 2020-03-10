use crate::models::TBitVec;
use num_traits::FromPrimitive;
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum RLEType {
  NoCompression = 0,
  SingleByte = 1,
  DoubleByte = 2,
}

impl TryFrom<&TBitVec> for RLEType {
  type Error = String;

  fn try_from(flags: &TBitVec) -> Result<Self, Self::Error> {
    let value = ((flags[7] as u8) << 1) + flags[6] as u8;
    Self::from_u8(value)
      .ok_or_else(|| format!("unknown RLEType with value {:?}", value))
  }
}

impl RLEType {
  pub fn assign_bits(self, tbv: &mut TBitVec) {
    match self {
      Self::SingleByte => {
        tbv.set(6, true);
      }
      Self::DoubleByte => {
        tbv.set(7, true);
      }
      _ => {}
    }
  }
}
