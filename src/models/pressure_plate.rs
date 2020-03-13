use crate::models::Position;
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

/// Represents a pressure plate.
///
/// See [Terraria Wiki: Pressure Plates](https://terraria.gamepedia.com/Pressure_Plates) for more information.
///
/// # Notes
///
/// - There's no further information in the data format about these _other_ than a position, so this is just an alias.
pub type PressurePlate = Position;

/// Represents all [`PressurePlate`]s in the game world.
///
/// A fancy wrapper around a `Vec`.
///
/// # Notes
///
/// - The length is dynamic and is represented by an [`i32`], but cannot be negative, practically speaking.
#[derive(Clone, Debug, Default, PartialEq, Eq, AsRef)]
#[repr(C)]
pub struct PressurePlates(Vec<PressurePlate>);

impl<'a> TryFromCtx<'a, Endian> for PressurePlates {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let count = buf.gread_with::<i32>(offset, LE)?;
    Ok((
      Self(
        (0..count)
          .map(|_| buf.gread::<PressurePlate>(offset))
          .collect::<Result<Vec<_>, Self::Error>>()?,
      ),
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
    let pressure_plates = self.as_ref();
    buf.gwrite_with(pressure_plates.len() as i32, offset, LE)?;
    pressure_plates
      .iter()
      .map(|sign| buf.gwrite(sign, offset))
      .collect::<Result<Vec<_>, Self::Error>>()?;
    let expected_size = PressurePlates::size_with(self);
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
  fn size_with(ctx: &Self) -> usize {
    i32::size_with(&LE) + (ctx.as_ref().len() * Position::size_with(&LE))
  }
}

#[cfg(test)]
mod test_pressure_plate {
  use super::{
    Pread,
    PressurePlate,
    Pwrite,
  };

  #[test]
  fn test_pressure_plate_rw() {
    let pp = PressurePlate { x: 0, y: 0 };
    let mut buf = [0; 8];
    assert_eq!(8, buf.pwrite(pp, 0).unwrap());
    assert_eq!(&pp, &buf.pread::<PressurePlate>(0).unwrap());
  }
}
