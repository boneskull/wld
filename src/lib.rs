mod model;
mod parser;

#[cfg(test)]
pub(crate) mod test_helpers {
  use nom::IResult;
  use lazy_static::lazy_static;
  use std::fs::read;

  pub fn unwrap<T>(res: IResult<&[u8], T>) -> T {
    res.unwrap().1
  }

  lazy_static! {
    // paths are relative to root, I guess?
    pub static ref WORLD: std::vec::Vec<u8> =
      read("tests/fixtures/Foon.wld").expect("Unable to read file");
  }
}
