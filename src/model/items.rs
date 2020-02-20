use super::tiles::TileMatrix;
use crate::{
  enums::ItemType,
  model::{
    common::{
      Point,
      TString,
    },
    tiles::{
      Tile,
      TileVec,
    },
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
    match quantity {
      0 => {}
      _ => {
        match item_type {
          Some(it) => {
            buf.gwrite(it, offset)?;
          }
          _ => {}
        };
        match modifier {
          Some(m) => {
            buf.gwrite(m, offset)?;
          }
          _ => {}
        }
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Pread)]
pub struct ChestsInfo {
  pub count: u16,
  pub max_items: u16,
}

impl TryIntoCtx<&TileMatrix> for &ChestsInfo {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: &TileMatrix,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    buf.gwrite(self.count, offset)?;
    buf.gwrite(self.max_items, offset)?;
    let len = ctx.as_ref().len();
    for i in 0..len {
      let mut j = 0;
      let tv: &TileVec = &ctx[i];
      let tv_len = tv.as_ref().len();
      while j < tv_len {
        let tile: &Tile = &tv[j];
        match &tile.chest {
          Some(chest) => {
            buf.gwrite_with(chest, offset, LE)?;
          }
          _ => {}
        };
        j += *tile.run_length.as_ref() as usize;
      }
    }
    Ok(*offset)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChestVec(Vec<Chest>, ChestsInfo);

impl<'a> TryFromCtx<'a, Endian> for ChestVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let chests_info = buf.gread_with::<ChestsInfo>(offset, LE)?;
    let mut chests: Vec<Chest> = vec![];
    for _ in 0..chests_info.count {
      let chest = buf.gread_with::<Chest>(offset, chests_info.max_items)?;
      chests.push(chest);
    }
    Ok((Self(chests, chests_info), *offset))
  }
}

impl ChestVec {
  pub fn chests_info(&self) -> ChestsInfo {
    self.1
  }

  #[inline]
  pub fn move_to_tile(chests: Self, tiles: &mut TileMatrix) {
    chests.0.into_iter().for_each(|chest| {
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Pread)]
pub struct SignsInfo {
  pub count: u16,
}

impl TryIntoCtx<&TileMatrix> for &SignsInfo {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: &TileMatrix,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    buf.gwrite(self.count, offset)?;
    let len = ctx.as_ref().len();
    for i in 0..len {
      let mut j = 0;
      let tv: &TileVec = &ctx[i];
      let tv_len = tv.as_ref().len();
      while j < tv_len {
        let tile: &Tile = &tv[j];
        match &tile.sign {
          Some(sign) => {
            buf.gwrite_with(sign, offset, LE)?;
          }
          _ => {}
        };
        j += *tile.run_length.as_ref() as usize;
      }
    }
    Ok(*offset)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SignVec(Vec<Sign>, SignsInfo);

impl SignVec {
  pub fn signs_info(&self) -> SignsInfo {
    self.1
  }
}

impl<'a> TryFromCtx<'a, Endian> for SignVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let signs_info = buf.gread_with::<SignsInfo>(offset, LE)?;
    let signs = Vec::from_iter(
      (0..signs_info.count)
        .into_iter()
        .map(|_| buf.gread::<Sign>(offset).unwrap()),
    );
    Ok((Self(signs, signs_info), *offset))
  }
}

impl SignVec {
  #[inline]
  pub fn move_to_tile(signs: Self, tiles: &mut TileMatrix) {
    signs.0.into_iter().for_each(|sign| {
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
}
