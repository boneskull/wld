use std::{
  boxed::Box,
  error::Error,
  fs::{
    read,
    write,
  },
};
use wld::{
  init_logger,
  parse_world,
};

fn main() -> Result<(), Box<dyn Error>> {
  init_logger(log::LevelFilter::Debug);
  let world_file: Vec<u8> =
    read("tests/fixtures/Foon.wld").expect("Unable to read file");
  let world = parse_world(&world_file)?;
  let rebuilt_world = world.write()?;
  write("newworld.wld", rebuilt_world)?;
  Ok(())
}
