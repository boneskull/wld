use crate::model::Position;
use scroll::{
  ctx::{
    SizeWith,
    TryFromCtx,
    TryIntoCtx,
  },
  Endian,
  Error as ScrollError,
  Pread,
  Pwrite,
  LE,
};

pub type PressurePlate = Position;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct PressurePlates {
  count: i32,
  pub pressure_plates: Vec<PressurePlate>,
}

impl<'a> TryFromCtx<'a, Endian> for PressurePlates {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let count = buf.gread_with::<i32>(offset, LE)?;
    Ok((
      Self {
        count,
        pressure_plates: (0..count)
          .into_iter()
          .map(|_| buf.gread::<PressurePlate>(offset))
          .collect::<Result<Vec<_>, Self::Error>>()?,
      },
      *offset,
    ))
  }
}

impl TryIntoCtx<Endian> for &PressurePlates {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let PressurePlates {
      count,
      pressure_plates,
    } = self;
    buf.gwrite_with(count, offset, LE)?;
    pressure_plates
      .iter()
      .map(|sign| buf.gwrite(sign, offset))
      .collect::<Result<Vec<_>, Self::Error>>()?;
    let expected_size = PressurePlates::size_with(&self);
    assert!(
      expected_size == *offset,
      "PressurePlates offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );
    Ok(*offset)
  }
}

impl SizeWith<PressurePlates> for PressurePlates {
  fn size_with(ctx: &PressurePlates) -> usize {
    i32::size_with(&LE) + (ctx.pressure_plates.len() * Position::size_with(&LE))
  }
}

#[cfg(test)]
mod test_pressure_plate {
  use super::*;

  #[test]
  fn test_pressure_plate_rw() {
    let pp = PressurePlate { x: 0, y: 0 };
    let mut buf = [0; 8];
    assert_eq!(8, buf.pwrite(pp, 0).unwrap());
    assert_eq!(&pp, &buf.pread::<PressurePlate>(0).unwrap());
  }
}
