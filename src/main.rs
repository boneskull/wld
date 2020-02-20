extern crate wld;
use wld::parse_world;

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
  let world_file: std::vec::Vec<u8> =
    std::fs::read("tests/fixtures/Foon.wld").expect("Unable to read file");
  let world = parse_world(&world_file)?;
  let rebuilt_world = world.write()?;
  std::fs::write("newworld.wld", rebuilt_world)?;
  Ok(())
}
