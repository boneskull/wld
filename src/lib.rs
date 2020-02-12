#![allow(dead_code)]

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate derive_more;

use crate::model::world::World;
use std::{
  boxed::Box,
  error::Error,
};

pub mod model;

pub fn parse_world<'a>(bytes: &'a [u8]) -> Result<World, Box<dyn Error>> {
  Ok(World::read(bytes)?)
}
