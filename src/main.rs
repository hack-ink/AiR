//! AI with Rust.

// TODO: check this.
// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(
	// clippy::all,
	missing_docs,
	unused_crate_dependencies,
)]

mod air;
mod component;
mod error;
mod os;
mod service;
mod ui;
mod state;

mod prelude {
	pub type Result<T> = std::result::Result<T, Error>;

	pub use crate::error::*;
}
use prelude::*;

// Only used for enable the svg support.
use egui_extras as _;

fn main() -> Result<()> {
	color_eyre::install().unwrap();
	tracing_subscriber::fmt::init();
	air::launch()?;

	Ok(())
}
