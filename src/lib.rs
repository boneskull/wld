#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate scroll_derive;
#[macro_use]
extern crate log;
use crate::model::World;
use log::LevelFilter;
use mowl;
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

pub fn init_logger() {
  mowl::init_with_level(LevelFilter::Debug).unwrap();
}
