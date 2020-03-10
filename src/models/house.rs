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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
#[repr(C)]
pub struct House {
  pub entity_type: EntityType,
  pub position: Position,
}

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
    let room_count = buf.gread_with::<i32>(offset, LE)?;
    let mut rooms: Vec<House> = Vec::with_capacity(room_count as usize);
    for _ in 0..room_count {
      let room = buf.gread::<House>(offset)?;
      rooms.push(room);
    }
    Ok((Self(rooms), *offset))
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
    for house in houses {
      buf.gwrite(house, offset)?;
    }
    assert!(
      *offset == HouseVec::size_with(self),
      "RoomVec size mismatch"
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
