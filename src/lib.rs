#![warn(clippy::pedantic, clippy::cargo, clippy::nursery)]
#![allow(
  clippy::pedantic::module_name_repetitions,
  clippy::pedantic::cast_possible_truncation,
  clippy::pedantic::cast_possible_wrap,
  clippy::pedantic::cast_sign_loss
)]
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
use crate::models::World;
use log::LevelFilter;
use std::{
  boxed::Box,
  error::Error,
};

pub mod constants;
pub mod enums;
pub mod models;
// use image, imageproc to render.

/// Instantiate a [`World`] from a slice of bytes.
///
/// # Errors
/// See [`scroll::Error`].
pub fn parse_world(bytes: &[u8]) -> Result<World, Box<dyn Error>> {
  Ok(World::read(bytes)?)
}

pub fn init_logger(level: LevelFilter) {
  mowl::init_with_level(level).unwrap();
}
