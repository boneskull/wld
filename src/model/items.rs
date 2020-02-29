use crate::{
  enums::ItemType,
  model::common::{
    Position,
    TString,
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
  fmt::{
    Debug,
    Formatter,
  },
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

impl TryIntoCtx<Endian> for &ItemStack {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let ItemStack {
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

    let expected_size = ItemStack::size_with(&self);
    assert!(
      *offset == expected_size,
      "ItemStack size mismatch; expected {:?}, got {:?}",
      expected_size,
      offset
    );
    Ok(*offset)
  }
}

#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Chest {
  pub position: Position,
  pub name: TString,
  pub contents: ItemStackVec,
}

impl<'a> TryFromCtx<'a, i16> for Chest {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    max_items: i16,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let position = buf.gread_with::<Position>(offset, LE)?;
    let name = buf.gread::<TString>(offset)?;
    let contents = buf.gread_with::<ItemStackVec>(offset, max_items)?;
    let chest = Self {
      position,
      name,
      contents,
    };
    Ok((chest, *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a Chest {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let Chest {
      position,
      name,
      contents,
    } = self;
    buf.gwrite_with(position, offset, LE)?;
    buf.gwrite(name, offset)?;
    buf.gwrite(contents, offset)?;
    let expected_size = Chest::size_with(&self);
    assert!(
      *offset == expected_size,
      "Chest size mismatch; expected {:?}, got {:?}",
      expected_size,
      offset
    );
    Ok(*offset)
  }
}
impl SizeWith<Chest> for Chest {
  fn size_with(ctx: &Chest) -> usize {
    let size = Position::size_with(&LE)
      + TString::size_with(&ctx.name)
      + ItemStackVec::size_with(&ctx.contents);

    trace!(
      "Chest size (position: {:?} + name: {:?} + contents: {:?} = {:?})",
      Position::size_with(&LE),
      TString::size_with(&ctx.name),
      ItemStackVec::size_with(&ctx.contents),
      size
    );
    size
  }
}

impl Debug for Chest {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl<'a> TryFromCtx<'a, i16> for ItemStackVec {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    size: i16,
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

impl<'a> TryIntoCtx<Endian> for &'a ItemStackVec {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    self.as_ref().iter().for_each(|stack| {
      buf.gwrite(stack, offset).unwrap();
    });

    let expected_size = ItemStackVec::size_with(&self);
    assert!(
      *offset == expected_size,
      "ItemStackVec size mismatch; expected {:?}, got {:?}",
      expected_size,
      offset
    );
    Ok(*offset)
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Chests {
  pub count: i16,
  pub stacks_per_chest: i16,
  pub chests: Vec<Chest>,
}

impl Chests {
  pub fn find_chest_at_position(&self, position: &Position) -> Option<&Chest> {
    let c = &self.chests;
    c.into_iter().find(|chest| &chest.position == position)
  }
}

impl<'a> TryFromCtx<'a, Endian> for Chests {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _ctx: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let count = buf.gread_with::<i16>(offset, LE)?;
    let stacks_per_chest = buf.gread_with::<i16>(offset, LE)?;
    let chests: Vec<_> = (0..count)
      .into_iter()
      .map(|_| buf.gread_with::<Chest>(offset, stacks_per_chest))
      .collect::<Result<Vec<_>, Self::Error>>()?;
    Ok((
      Chests {
        count,
        stacks_per_chest,
        chests,
      },
      *offset,
    ))
  }
}

impl SizeWith<Chests> for Chests {
  fn size_with(ctx: &Chests) -> usize {
    (i16::size_with(&LE) * 2)
      + ctx
        .chests
        .iter()
        .map(|chest| Chest::size_with(&chest))
        .fold(0, |acc, len| acc + len)
  }
}

impl TryIntoCtx<Endian> for &Chests {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let Chests {
      count,
      stacks_per_chest,
      chests,
    } = self;
    buf.gwrite_with(count, offset, LE)?;
    buf.gwrite_with(stacks_per_chest, offset, LE)?;
    chests
      .iter()
      .map(|chest| buf.gwrite(chest, offset))
      .collect::<Result<Vec<_>, Self::Error>>()?;
    let expected_size = Chests::size_with(&self);
    assert!(
      expected_size == *offset,
      "Chests offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );
    Ok(*offset)
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
    assert_eq!(7, buf.pwrite(&stack, 0).unwrap());
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
    assert_eq!(vec, buf.pread_with::<ItemStackVec>(0, 2i16).unwrap());
  }
}
