use crate::parser::header::parse_header;
use crate::model::world::World;
use std::boxed::Box;
use std::error::Error;

pub mod model;
mod parser;

#[cfg(test)]
pub(crate) mod test_helpers {
  use lazy_static::lazy_static;
  use nom::IResult;
  use std::fs::read;
  use std::convert::TryInto;
  use std::vec::Vec;
  pub use nom::{error::ErrorKind, Err::Error as NomError};

  pub fn unwrap<T>(res: IResult<&[u8], T>) -> T {
    res.unwrap().1
  }

  pub fn p_string<S>(s: S) -> Vec<u8> where S: Into<String> {
    let s = s.into();
    let len: u8 = s.len().try_into().unwrap();
    [&len.to_le_bytes(), s.as_bytes()].concat()
  }

  lazy_static! {
    // paths are relative to root, I guess?
    pub static ref WORLD: Vec<u8> =
      read("tests/fixtures/Foon.wld").expect("Unable to read file");
  }
}

pub fn parse_world(world: &'static [u8]) -> Result<World, Box<dyn Error>> {
  let (buf, header) = parse_header(world)?;
  Ok(World { header })
}
