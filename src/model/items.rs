use super::tiles::TileMatrix;
use crate::{
  enums::ItemType,
  model::common::{
    Point,
    TString,
  },
};
use scroll::{
  ctx::{
    TryFromCtx,
    TryIntoCtx,
  },
  Endian,
  Error as ScrollError,
  Pread,
  Pwrite,
  LE,
};
use std::iter::FromIterator;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ItemStack {
  pub quantity: i16,
  pub item_type: Option<ItemType>,
  pub modifier: Option<u8>,
}

impl<'a> TryFromCtx<'a, Endian> for ItemStack {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let quantity = buf.gread_with::<i16>(offset, LE)?;
    let mut item_type: Option<ItemType> = None;
    let mut modifier: Option<u8> = None;
    if quantity > 0 {
      item_type = Some(buf.gread::<ItemType>(offset)?);
      modifier = Some(buf.gread::<u8>(offset)?);
    }
    Ok((
      Self {
        quantity,
        item_type,
        modifier,
      },
      *offset,
    ))
  }
}

impl TryIntoCtx<Endian> for ItemStack {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let Self {
      quantity,
      item_type,
      modifier,
    } = self;
    let offset = &mut 0;
    buf.gwrite_with(quantity, offset, LE)?;
    if quantity > 0 {
      if item_type.is_some() {
        buf.gwrite(item_type.unwrap(), offset)?;
      }
      if modifier.is_some() {
        buf.gwrite(modifier.unwrap(), offset)?;
      }
    }
    Ok(*offset)
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Pwrite)]
pub struct Chest {
  pub position: Point,
  pub name: TString,
  pub contents: ItemStackVec,
}

impl<'a> TryFromCtx<'a, u16> for Chest {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    max_items: u16,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let position = buf.gread_with::<Point>(offset, LE)?;
    let name = buf.gread::<TString>(offset)?;
    let contents = buf.gread_with::<ItemStackVec>(offset, max_items)?;
    Ok((
      Self {
        position,
        name,
        contents,
      },
      *offset,
    ))
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, IntoIterator, AsRef)]
pub struct ItemStackVec(Vec<ItemStack>);

impl<'a> TryFromCtx<'a, u16> for ItemStackVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    size: u16,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let stack = Vec::from_iter(
      (0..size)
        .into_iter()
        .map(|_| buf.gread::<ItemStack>(offset).unwrap()),
    );
    Ok((Self(stack), *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a ItemStackVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    self.as_ref().iter().for_each(|stack| {
      buf.gwrite(*stack, offset).unwrap();
    });
    Ok(*offset)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, IntoIterator)]
pub struct ChestVec(Vec<Chest>);

impl<'a> TryFromCtx<'a, Endian> for ChestVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let chests_count = buf.gread_with::<u16>(offset, LE)?;
    let chests_max_items = buf.gread_with::<u16>(offset, LE)?;
    let mut chests: Vec<Chest> = vec![];
    for _ in 0..chests_count {
      let chest = buf.gread_with::<Chest>(offset, chests_max_items)?;
      chests.push(chest);
    }
    Ok((Self(chests), *offset))
  }
}

impl TryIntoCtx<Endian> for ChestVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    self.into_iter().for_each(|stack| {
      buf.gwrite(stack, offset).unwrap();
    });
    Ok(*offset)
  }
}

impl ChestVec {
  #[inline]
  pub fn assign_to_tile(chests: Self, tiles: &mut TileMatrix) {
    chests.into_iter().for_each(|chest| {
      let mut tile = tiles.tile_at_point(&chest.position);
      tile.chest = Some(chest);
    });
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
pub struct Sign {
  pub text: TString,
  pub position: Point,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, IntoIterator, AsRef)]
pub struct SignVec(Vec<Sign>);

impl<'a> TryFromCtx<'a, Endian> for SignVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let signs_count = buf.gread_with::<u16>(offset, LE)?;
    let signs = Vec::from_iter(
      (0..signs_count)
        .into_iter()
        .map(|_| buf.gread::<Sign>(offset).unwrap()),
    );
    Ok((Self(signs), *offset))
  }
}

impl TryIntoCtx<Endian> for SignVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    buf.gwrite(self.as_ref().len() as u16, offset)?;
    self.into_iter().for_each(|stack| {
      buf.gwrite(stack, offset).unwrap();
    });
    Ok(*offset)
  }
}

impl SignVec {
  #[inline]
  pub fn assign_to_tile(signs: Self, tiles: &mut TileMatrix) {
    signs.into_iter().for_each(|sign| {
      let mut tile = tiles.tile_at_point(&sign.position);
      tile.sign = Some(sign);
    });
  }
}

#[cfg(test)]
mod test_items {
  use super::*;

  #[test]
  fn test_item_stack_rw() {
    let stack = ItemStack {
      quantity: 5,
      item_type: Some(ItemType::Duck),
      modifier: Some(1),
    };
    let mut buf = [0; 7];
    assert_eq!(7, buf.pwrite(stack, 0).unwrap());
    assert_eq!(stack, buf.pread::<ItemStack>(0).unwrap());
  }

  #[test]
  fn test_item_stack_vec_rw() {
    let vec = ItemStackVec(vec![
      ItemStack {
        quantity: 5,
        item_type: Some(ItemType::Duck),
        modifier: Some(0),
      },
      ItemStack {
        quantity: 2,
        item_type: Some(ItemType::Bunny),
        modifier: Some(0),
      },
    ]);

    let mut buf = [0; 14];
    assert_eq!(14, buf.pwrite(&vec, 0).unwrap());
    assert_eq!(vec, buf.pread_with::<ItemStackVec>(0, 2u16).unwrap());
  }

  #[test]
  fn test_sign_vec_rw() {
    let signs = SignVec(vec![
      Sign {
        text: TString::from("foo"),
        position: Point { x: 0, y: 0 },
      },
      Sign {
        text: TString::from("bar"),
        position: Point { x: 10, y: 10 },
      },
    ]);

    let mut buf = [0; 26];
    assert_eq!(26, buf.pwrite(signs.clone(), 0).unwrap());
    assert_eq!(signs, buf.pread::<SignVec>(0).unwrap());
  }
}
