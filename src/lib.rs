//! `wld` is a library which will read and write Terraria world (`.wld`) files.
//!
//! # Layout
//!
//! Models (structs) live in the [`models`] module, and enums live in the
//! [`enums`] module.  Additionally, some constants (corresponding to colors
//! used by the [map-rendering functionality](World::render)) are available in
//! the [`constants`] module.
//!
//! You shouldn't need to instantiate _any_ struct or model directly.  Instead,
//! use [`parse_world`] to create a [`World`] instance, and use the methods
//! and properties on that instance.

#![warn(clippy::pedantic, clippy::cargo, clippy::nursery)]
#![allow(
  clippy::pedantic::module_name_repetitions,
  clippy::pedantic::cast_possible_truncation,
  clippy::pedantic::cast_possible_wrap,
  clippy::pedantic::cast_sign_loss,
  clippy::cargo::multiple_crate_versions
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

/// Instantiate a [`World`] from a slice of bytes.
///
/// # Errors
///
/// See [`scroll::Error`].
///
/// # Example
///
/// ```
/// let world: wld::models::World =
///   wld::parse_world(&std::fs::read("tests/fixtures/Foon.wld").unwrap())
///     .unwrap();
/// ```
pub fn parse_world(bytes: &[u8]) -> Result<World, Box<dyn Error>> {
  Ok(World::read(bytes)?)
}

/// Enable logging for this crate given [`log::LevelFilter`] `level`.
///
/// # Panics
///
/// It's possible to panic if [`mowl::init_with_level`] fails.
pub fn enable_logger(level: LevelFilter) {
  mowl::init_with_level(level).unwrap();
}
