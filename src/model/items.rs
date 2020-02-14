use super::tiles::TileMatrix;
use crate::model::common::{
  Point,
  TString,
};
use crate::enums::ItemType;
use scroll::{
  ctx::TryFromCtx,
  Endian,
  Pread,
  LE,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ItemStack {
  pub quantity: i16,
  pub item_type: Option<ItemType>,
  pub modifier: u8,
}

impl<'a> TryFromCtx<'a, Endian> for ItemStack {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let quantity = buf.gread_with::<i16>(offset, LE)?;
    let mut item_type: Option<ItemType> = None;
    let mut modifier: u8 = 0;
    if quantity > 0 {
      item_type = Some(buf.gread::<ItemType>(offset)?);
      modifier = buf.gread::<u8>(offset)?;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chest {
  pub position: Point,
  pub name: TString,
  pub contents: Vec<ItemStack>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, IntoIterator)]
pub struct ChestVec(Vec<Chest>);

impl<'a> TryFromCtx<'a, Endian> for ChestVec {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let chests_count = buf.gread_with::<u16>(offset, LE)?;
    let chests_max_items = buf.gread_with::<u16>(offset, LE)?;
    let mut chests: Vec<Chest> = vec![];
    for _ in 0..chests_count {
      let position = buf.gread_with::<Point>(offset, LE)?;
      let name = buf.gread::<TString>(offset)?;
      let mut contents: Vec<ItemStack> = vec![];
      for _ in 0..chests_max_items {
        let item_stack = buf.gread::<ItemStack>(offset)?;
        if item_stack.item_type.is_some() {
          contents.push(item_stack);
        }
      }
      let chest = Chest {
        position,
        name,
        contents,
      };
      chests.push(chest);
    }
    Ok((Self(chests), *offset))
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sign {
  pub text: TString,
  pub position: Point,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, IntoIterator)]
pub struct SignVec(Vec<Sign>);

impl<'a> TryFromCtx<'a, Endian> for SignVec {
  type Error = scroll::Error;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let signs_count = buf.gread_with::<u16>(offset, LE)?;
    let mut signs: Vec<Sign> = vec![];
    for _ in 0..signs_count {
      let text = buf.gread::<TString>(offset)?;
      let position = buf.gread_with::<Point>(offset, LE)?;
      let chest = Sign { text, position };
      signs.push(chest);
    }
    Ok((Self(signs), *offset))
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
