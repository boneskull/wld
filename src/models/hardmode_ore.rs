use crate::enums::HardmodeOreType;
use num_traits::FromPrimitive;
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

/// Represents one of three types of ore available in hardmode.
///
/// If the world is not [in hardmode], or the player has not yet spawned the
/// corresponding tier of ore, [`HardmodeOre::ore_type`] will be
/// [`HardmodeOreType::UnknownOre`].
///
/// See [Terraria Wiki: Ores](https://terraria.gamepedia.com/Ores#Hardmode) for
/// more information.
///
/// [in hardmode]: crate::models::Status::is_hardmode
///
/// # Notes
///
/// - The integer representation of an unknown ore is either `-1` or seemingly
///   some random number. I have no idea. So that's why we save it in the
///   [`HardmodeOre::raw_value`] field.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct HardmodeOre {
  pub raw_value: i32,
  pub ore_type: HardmodeOreType,
}

impl SizeWith<Endian> for HardmodeOre {
  fn size_with(_: &Endian) -> usize {
    i32::size_with(&LE)
  }
}

impl<'a> TryFromCtx<'a, Endian> for HardmodeOre {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let raw_value = buf.gread_with::<i32>(offset, LE)?;
    let ore_type = match HardmodeOreType::from_i32(raw_value) {
      Some(o) => o,
      _ => HardmodeOreType::UnknownOre,
    };
    Ok((
      Self {
        ore_type,
        raw_value,
      },
      *offset,
    ))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a HardmodeOre {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    buf.gwrite_with(&self.raw_value, offset, LE)?;
    let expected_size = HardmodeOre::size_with(&LE);
    assert!(
      *offset == expected_size,
      "HardmodeOre size mismatch; expected {:?}, got {:?}",
      expected_size,
      offset
    );
    Ok(*offset)
  }
}
