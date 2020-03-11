use crate::{
  enums::EntityType,
  models::Position,
};
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

/// Represents NPC housing.
///
/// See [Terraria Wiki: House] for more information.
///
/// [Terraria Wiki: House]: https://terraria.gamepedia.com/House
#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct House {
  /// The NPC who lives in the house.
  ///
  /// Practically, this must be a friendly NPC. What happens if you put [`Tim`] in there?
  ///
  /// [`Tim`]: crate::enums::EntityType::Tim
  pub entity_type: EntityType,

  /// The position of the house.
  ///
  /// Given this is just a _point_, I'm not sure what exactly it refers to.
  pub position: Position,
}

/// A variable-length list of [`House`]s.
///
/// # Notes
///
/// - The length is represented by an [`i32`], but cannot be negative.
/// - Upon write, the length is derived from the length of the underlying [`Vec`].
#[derive(Clone, Debug, Default, PartialEq, Eq, IntoIterator, AsRef)]
pub struct HouseVec(Vec<House>);

impl SizeWith<HouseVec> for HouseVec {
  fn size_with(ctx: &Self) -> usize {
    let size =
      i32::size_with(&LE) + (ctx.as_ref().len() * House::size_with(&LE));
    debug!("RoomVec size: {}", size);
    size
  }
}

impl<'a> TryFromCtx<'a, Endian> for HouseVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let house_count = buf.gread_with::<i32>(offset, LE)?;
    let mut hice: Vec<House> = Vec::with_capacity(house_count as usize);
    for _ in 0..house_count {
      let room = buf.gread::<House>(offset)?;
      hice.push(room);
    }
    Ok((Self(hice), *offset))
  }
}

impl TryIntoCtx<Endian> for &HouseVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let houses = self.as_ref();
    buf.gwrite_with(houses.len() as i32, offset, LE)?;
    // TODO: use iterator
    for house in houses {
      buf.gwrite(house, offset)?;
    }
    assert!(
      *offset == HouseVec::size_with(self),
      "HouseVec size mismatch"
    );

    Ok(*offset)
  }
}

#[cfg(test)]
mod test_house {
  use super::{
    EntityType,
    House,
    HouseVec,
    Position,
    Pread,
    Pwrite,
  };
  #[test]
  fn test_house_vec_rw() {
    let rv = HouseVec(vec![
      House {
        entity_type: EntityType::Derpling,
        position: Position { x: 0, y: 0 },
      },
      House {
        entity_type: EntityType::Herpling,
        position: Position { x: 2, y: 2 },
      },
    ]);
    let mut buf = [0; 28];
    assert_eq!(28, buf.pwrite(&rv, 0).unwrap());
    assert_eq!(rv, buf.pread::<HouseVec>(0).unwrap());
  }
}
