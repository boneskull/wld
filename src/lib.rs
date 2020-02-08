#![allow(dead_code)]

use crate::model::world::World;
use std::boxed::Box;
use std::error::Error;

pub mod model;

pub fn parse_world<'a>(bytes: &'a [u8]) -> Result<World, Box<dyn Error>> {
  Ok(World::read(bytes)?)
}
