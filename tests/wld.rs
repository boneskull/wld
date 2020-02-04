use lazy_static::lazy_static;
use nom::IResult;
use std::fs::read;
use wld::{
  model::world::{Header, World},
  parse_world,
};

pub fn unwrap<T>(res: IResult<&[u8], T>) -> T {
  res.unwrap().1
}

lazy_static! {
  // paths are relative to root, I guess?
  pub static ref WORLD: std::vec::Vec<u8> =
    read("tests/fixtures/Foon.wld").expect("Unable to read file");
}

#[test]
fn test_parse() {
  assert_eq!(
    parse_world(&WORLD).unwrap(),
    World {
      header: Header::new(String::from("1.3.5.3"), 160, false)
    }
  );
}
