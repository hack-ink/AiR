//! AI Rust.

// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(missing_docs, unused_crate_dependencies)]

mod air;
mod component;
mod hotkey;
mod os;
mod preference;
mod ui;
mod widget;

fn main() {
	color_eyre::install().unwrap();
	tracing_subscriber::fmt::init();
	air::launch();
}
