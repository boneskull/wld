use insta::assert_debug_snapshot;
use lazy_static::lazy_static;
use scroll::ctx::SizeWith;
use std::fs::read;
use wld::{
  model::World,
  parse_world,
};

lazy_static! {
  // paths are relative to root, I guess?
  pub static ref WORLD: std::vec::Vec<u8> =
    read("tests/fixtures/Foon.wld").expect("Unable to read file");
  pub static ref PARSED_WORLD: World = parse_world(&WORLD).unwrap();
}

#[test]
fn test_parse_status_header() {
  assert_debug_snapshot!(PARSED_WORLD.status.header);
}

#[test]
fn test_parse_status_properties() {
  assert_debug_snapshot!(PARSED_WORLD.status.properties);
}

#[test]
fn test_parse_status_status() {
  assert_debug_snapshot!(PARSED_WORLD.status.status);
}

#[test]
fn test_parse_tiles() {
  // if we did the whole thing, the snapshot would be about 2GB
  assert_debug_snapshot!(&PARSED_WORLD.tiles[0][0..50]);
}

#[test]
fn test_parse_npcs() {
  assert_debug_snapshot!(PARSED_WORLD.npcs);
}

#[test]
fn test_parse_mobs() {
  assert_debug_snapshot!(PARSED_WORLD.mobs);
}

#[test]
fn test_parse_houses() {
  assert_debug_snapshot!(PARSED_WORLD.houses);
}

#[test]
fn test_parse_footer() {
  assert_debug_snapshot!(PARSED_WORLD.footer);
}

#[test]
fn test_write() {
  // wld::init_logger();
  let res = PARSED_WORLD.write();
  assert_eq!(res.unwrap().len(), World::size_with(&PARSED_WORLD));
}

// #[test]
// fn test_render() {
//   assert_eq!(PARSED_WORLD.render().dimensions(), (4200, 1200));
// }
