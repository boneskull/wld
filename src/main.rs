extern crate wld;
use wld::parse_world;

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
  let world: std::vec::Vec<u8> =
    std::fs::read("tests/fixtures/Foon.wld").expect("Unable to read file");

  match parse_world(&world) {
    Ok(world) => world.render("output.png"),
    Err(e) => Err(e),
  }
}
