use crate::{
  enums::ItemType,
  models::common::{
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

/// Represents a stack of items in a [`Chest`].
///
/// Each `Chest` will contain a number of these (probably 40).  If the slot is unused,
/// the [`quantity`] field will be `0`; otherwise it will be a positive integer and the [`item_type`]
/// and [`modifier`] fields will have a [`Some`] value.
///
/// [`quantity`]: ItemStack::quantity
/// [`item_type`]: ItemStack::item_type
/// [`modifier`]: ItemStack::modifier
///
/// # Notes
///
/// - Despite being a signed integer, `quantity` should not be negative.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct ItemStack {
  /// The number of items in this stack.  Defaults to `0`.
  pub quantity: i16,

  /// The type of item in the stack.
  pub item_type: Option<ItemType>,

  /// This _may_ correspond to a prefix ID.
  ///
  /// See [Terraria Wiki: Prefix IDs](https://terraria.gamepedia.com/Prefix_IDs) for more information.
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
  fn size_with(ctx: &Self) -> usize {
    i16::size_with(&LE)
      + ctx.item_type.map_or(0, |_| ItemType::size_with(&LE))
      + ctx.modifier.map_or(0, |_| u8::size_with(&LE))
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
    let (item_type, modifier) = if quantity > 0 {
      (
        Some(buf.gread::<ItemType>(offset)?),
        Some(buf.gread::<u8>(offset)?),
      )
    } else {
      (None, None)
    };
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
    if let 0 = quantity {
    } else {
      if let Some(it) = item_type {
        buf.gwrite(it, offset)?;
      }
      if let Some(m) = modifier {
        buf.gwrite(m, offset)?;
      }
    }

    let expected_size = ItemStack::size_with(self);
    assert!(
      *offset == expected_size,
      "ItemStack size mismatch; expected {:?}, got {:?}",
      expected_size,
      offset
    );
    Ok(*offset)
  }
}

/// Represents a chest.
///
/// See [Terraria Wiki: Chest](https://terraria.gamepedia.com/Chests) for more information.
#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Chest {
  /// The position of this chest on the map.
  pub position: Position,

  /// The custom name of this chest, if any.  Defaults to an empty string.
  pub name: TString,

  /// Chest contents.
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
    let expected_size = Chest::size_with(self);
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
  fn size_with(ctx: &Self) -> usize {
    Position::size_with(&LE)
      + TString::size_with(&ctx.name)
      + ItemStackVec::size_with(&ctx.contents)
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

/// A list of [`ItemStack`]s, which make up the contents of a [`Chest`].
#[derive(Clone, Default, PartialEq, Eq, IntoIterator, AsRef)]
#[repr(C)]
pub struct ItemStackVec(Vec<ItemStack>);

impl ItemStackVec {
  /// Gets the total count of items in this [`ItemStackVec`], and by association, a [`Chest`].
  ///
  /// While [`ItemStack::quantity`] is an [`i16`], an unmodded game has a maximum stack size of
  /// `999`, and the number of chest slots is `40`, we will use an [`i32`] here out of an abundance
  /// of caution.
  ///
  /// This is only used for debugging purposes, internally.
  #[must_use]
  pub fn total_quantity(&self) -> i32 {
    self.as_ref().iter().map(|is| i32::from(is.quantity)).sum()
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
      (0..size).map(|_| buf.gread::<ItemStack>(offset).unwrap()),
    );
    Ok((Self(stack), *offset))
  }
}

impl SizeWith<ItemStackVec> for ItemStackVec {
  fn size_with(ctx: &Self) -> usize {
    ctx.as_ref().iter().map(|is| ItemStack::size_with(is)).sum()
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

    let expected_size = ItemStackVec::size_with(self);
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
    let total_quantity: i16 = self.as_ref().iter().map(|is| is.quantity).sum();

    if total_quantity > 0 {
      write!(f, "ItemStackVec {{ {:?} }}", self.as_ref())
    } else {
      write!(f, "ItemStackVec []")
    }
  }
}

/// Represents all [`Chest`]s in a Terraria world.
///
/// # Notes
///
/// - [`Chests::stacks_per_chest`] may always be `40`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Chests {
  /// The number of [`Chest`]s on the map.
  ///
  /// Despite being a signed integer, this cannot be negative.
  pub count: i16,

  /// The number of [`ItemStack`]s (or slots) per [`Chest`].
  pub stacks_per_chest: i16,

  /// All the chests.
  pub chests: Vec<Chest>,
}

impl Chests {
  /// Try to find a [`Chest`] at a particular [`Position`].
  ///
  /// Used to associate a [`Tile`] with a [`Chest`].
  ///
  /// [`Tile`]: crate::model::Tile
  /// [`Position`]: crate::model::Position
  ///
  /// # Notes
  ///
  /// - This runs in linear time, bounded by [`Chests::count`].
  #[must_use]
  pub fn find_chest_at_position(&self, position: Position) -> Option<&Chest> {
    let c = &self.chests;
    c.iter().find(|chest| chest.position == position)
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
      .map(|_| buf.gread_with::<Chest>(offset, stacks_per_chest))
      .collect::<Result<Vec<_>, Self::Error>>()?;
    Ok((
      Self {
        count,
        stacks_per_chest,
        chests,
      },
      *offset,
    ))
  }
}

impl SizeWith<Chests> for Chests {
  fn size_with(ctx: &Self) -> usize {
    (i16::size_with(&LE) * 2)
      + ctx
        .chests
        .iter()
        .map(|chest| Chest::size_with(chest))
        .sum::<usize>()
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
    let expected_size = Chests::size_with(self);
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
  use super::{
    ItemStack,
    ItemStackVec,
    ItemType,
    Pread,
    Pwrite,
  };

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
    assert_eq!(vec, buf.pread_with::<ItemStackVec>(0, 2_i16).unwrap());
  }
}
