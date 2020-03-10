use crate::models::{
  Position,
  TString,
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

#[derive(Clone, Debug, PartialEq, Eq, Pread, Pwrite)]
#[repr(C)]
pub struct Sign {
  pub text: TString,
  pub position: Position,
}

impl SizeWith<Sign> for Sign {
  fn size_with(ctx: &Self) -> usize {
    TString::size_with(&ctx.text) + Position::size_with(&LE)
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Signs {
  pub count: u16,
  pub signs: Vec<Sign>,
}

impl Signs {
  #[must_use]
  pub fn find_sign_at_position(&self, position: Position) -> Option<&Sign> {
    let s = &self.signs;
    s.iter().find(|sign| sign.position == position)
  }
}

impl<'a> TryFromCtx<'a, Endian> for Signs {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let count = buf.gread_with::<u16>(offset, LE)?;
    let signs = (0..count)
      .map(|_| buf.gread::<Sign>(offset))
      .collect::<Result<Vec<_>, Self::Error>>()?;

    Ok((Self { count, signs }, *offset))
  }
}

impl SizeWith<Signs> for Signs {
  fn size_with(ctx: &Self) -> usize {
    u16::size_with(&LE)
      + ctx
        .signs
        .iter()
        .map(|sign| Sign::size_with(sign))
        .sum::<usize>()
  }
}

impl TryIntoCtx<Endian> for &Signs {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    _: Endian,
  ) -> Result<usize, Self::Error> {
    let offset = &mut 0;
    let Signs { count, signs } = self;
    buf.gwrite_with(count, offset, LE)?;
    signs
      .iter()
      .map(|sign| buf.gwrite(sign, offset))
      .collect::<Result<Vec<_>, Self::Error>>()?;
    let expected_size = Signs::size_with(self);
    assert!(
      expected_size == *offset,
      "Signs offset mismatch on write; expected {:?}, got {:?}",
      expected_size,
      offset
    );
    Ok(*offset)
  }
}
