//! AI Rust.

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
mod data;
mod error;
mod os;
mod service;
mod widget;

mod prelude {
	pub type Result<T> = std::result::Result<T, Error>;

	pub use crate::error::*;
}
use prelude::*;

fn main() -> Result<()> {
	color_eyre::install().unwrap();
	tracing_subscriber::fmt::init();
	air::launch()?;

	Ok(())
}
