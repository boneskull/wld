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
use std::{
  fmt::Debug,
  iter::FromIterator,
};

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct ItemStack {
  pub quantity: i16,
  pub item_type: Option<ItemType>,
  pub modifier: Option<u8>,
}

impl Debug for ItemStack {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.quantity > 0 {
      write!(
        f,
        "ItemStack {{ {:?}, {:?}, {:?} }}",
        self.quantity, self.item_type, self.modifier
      )
    } else {
      write!(f, "ItemStack {{}}")
    }
  }
}

impl SizeWith<ItemStack> for ItemStack {
  fn size_with(ctx: &ItemStack) -> usize {
    let size = i16::size_with(&LE)
      + ctx.item_type.map_or(0, |_| ItemType::size_with(&LE))
      + ctx.modifier.map_or(0, |_| u8::size_with(&LE));
    trace!("ItemStack size: {}", size);
    size
  }
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
        if let Some(it) = item_type {
          buf.gwrite(it, offset)?;
        }
        if let Some(m) = modifier {
          buf.gwrite(m, offset)?;
        }
      }
    }
    Ok(*offset)
  }
}

#[derive(Clone, PartialEq, Eq, Pwrite)]
#[repr(C)]
pub struct Chest {
  pub position: Point,
  pub name: TString,
  pub contents: ItemStackVec,
}

impl SizeWith<Chest> for Chest {
  fn size_with(ctx: &Chest) -> usize {
    let size = Point::size_with(&LE)
      + TString::size_with(&ctx.name)
      + ItemStackVec::size_with(&ctx.contents);

    trace!(
      "Chest size (position: {:?} + name: {:?} + contents: {:?} = {:?})",
      Point::size_with(&LE),
      TString::size_with(&ctx.name),
      ItemStackVec::size_with(&ctx.contents),
      size
    );
    size
  }
}

impl Debug for Chest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.contents.total_quantity() > 0 {
      write!(
        f,
        "Chest {{ {:?}, {:?}, {:?} }}",
        self.position, self.name, self.contents
      )
    } else {
      write!(
        f,
        "Chest {{ {:?}, {:?}, (empty) }}",
        self.position, self.name
      )
    }
  }
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
    let chest = Self {
      position,
      name,
      contents,
    };
    trace!(
      "Found Chest with size {:?}: {:?}",
      Chest::size_with(&chest),
      chest
    );
    Ok((chest, *offset))
  }
}

#[derive(Clone, Default, PartialEq, Eq, IntoIterator, AsRef)]
#[repr(C)]
pub struct ItemStackVec(Vec<ItemStack>);

impl ItemStackVec {
  pub fn total_quantity(&self) -> i32 {
    self
      .as_ref()
      .iter()
      .map(|is| is.quantity)
      .fold(0, |acc, len| acc + (len as i32))
  }
}

impl SizeWith<ItemStackVec> for ItemStackVec {
  fn size_with(ctx: &ItemStackVec) -> usize {
    let size = ctx
      .as_ref()
      .iter()
      .map(|is| ItemStack::size_with(&is))
      .fold(0, |acc, len| acc + len);
    trace!("ItemStackVec size: {}", size);
    size
  }
}

impl Debug for ItemStackVec {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let total_quantity = self
      .as_ref()
      .iter()
      .map(|is| is.quantity)
      .fold(0, |acc, len| acc + len);

    if total_quantity > 0 {
      write!(f, "ItemStackVec {{ {:?} }}", self.as_ref())
    } else {
      write!(f, "ItemStackVec []")
    }
  }
}

impl<'a> TryFromCtx<'a, u16> for ItemStackVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    size: u16,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    // XXX this should be a loop
    let stack: Vec<ItemStack> = Vec::from_iter(
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
#[repr(C)]
pub struct ChestsInfo {
  pub count: u16,
  pub max_items: u16,
}

impl SizeWith<TileMatrix> for ChestsInfo {
  fn size_with(ctx: &TileMatrix) -> usize {
    let size = (u16::size_with(&LE) * 2)
      + ctx
        .as_ref()
        .iter()
        .map(|tv| {
          tv.as_ref()
            .iter()
            .map(|tile| {
              tile
                .chest
                .as_ref()
                .map_or(0, |chest| Chest::size_with(&chest))
            })
            .fold(0usize, |acc, len| acc + len)
        })
        .fold(0, |acc, len| acc + len);
    debug!("ChestsInfo size: {}", size);
    size
  }
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
        if let Some(chest) = &tile.chest {
          buf.gwrite(chest, offset)?;
        }
        j += tile.run_length.length as usize;
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
    let mut chests: Vec<Chest> = Vec::with_capacity(chests_info.count as usize);
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
#[repr(C)]
pub struct Sign {
  pub text: TString,
  pub position: Point,
}

impl SizeWith<Sign> for Sign {
  fn size_with(ctx: &Sign) -> usize {
    TString::size_with(&ctx.text) + Point::size_with(&LE)
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Pread)]
#[repr(C)]
pub struct SignsInfo {
  pub count: u16,
}

impl SizeWith<TileMatrix> for SignsInfo {
  fn size_with(ctx: &TileMatrix) -> usize {
    let size = u16::size_with(&LE)
      + ctx
        .as_ref()
        .iter()
        .map(|tv| {
          tv.as_ref()
            .iter()
            .filter(|tile| tile.sign.is_some())
            .map(|tile| Sign::size_with(&tile.sign.as_ref().unwrap()))
            .fold(0usize, |acc, len| acc + len)
        })
        .fold(0, |acc, len| acc + len);
    debug!("SignsInfo size: {}", size);
    size
  }
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
        j += tile.run_length.length as usize;
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
