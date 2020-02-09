use insta::assert_debug_snapshot;
use lazy_static::lazy_static;
use std::fs::read;
use wld::{model::world::World, parse_world};

lazy_static! {
  // paths are relative to root, I guess?
  pub static ref WORLD: std::vec::Vec<u8> =
    read("tests/fixtures/Foon.wld").expect("Unable to read file");
  pub static ref PARSED_WORLD: World = parse_world(&WORLD).unwrap();
}

#[test]
fn test_parse_header() {
  assert_debug_snapshot!(PARSED_WORLD.header);
}

#[test]
fn test_parse_properties() {
  assert_debug_snapshot!(PARSED_WORLD.properties);
}

#[test]
fn test_parse_status() {
  assert_debug_snapshot!(PARSED_WORLD.status);
}
