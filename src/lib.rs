#![allow(dead_code)]

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;

use crate::model::world::World;
use std::{
  boxed::Box,
  error::Error,
};

pub mod constants;
pub mod enums;
pub mod model;
// use image, imageproc to render.

pub fn parse_world<'a>(bytes: &'a [u8]) -> Result<World, Box<dyn Error>> {
  Ok(World::read(bytes)?)
}
