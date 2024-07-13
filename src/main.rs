//! AI with Rust.

// Hide console window on Windows in release.
#![cfg_attr(all(not(debug_assertions), not(feature = "dev")), windows_subsystem = "windows")]
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
mod state;
mod ui;
mod util;
mod widget;

mod prelude {
	pub type Result<T, E = Error> = std::result::Result<T, E>;

	pub use crate::error::*;
}

// std
#[cfg(not(feature = "dev"))] use std::panic;
// crates.io
use app_dirs2::{AppDataType, AppInfo};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
	filter::LevelFilter, fmt, layer::SubscriberExt, reload::Layer, util::SubscriberInitExt,
	EnvFilter,
};

const APP_INFO: AppInfo = AppInfo { name: "AiR", author: "hack.ink" };

fn main() {
	color_eyre::install().unwrap();

	let (non_blocking, _guard) = tracing_appender::non_blocking(
		RollingFileAppender::builder()
			.rotation(Rotation::DAILY)
			.filename_suffix("log")
			.build(app_dirs2::get_app_root(AppDataType::UserData, &APP_INFO).unwrap())
			.unwrap(),
	);
	let filter =
		EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();
	let (reloadable_filter, filter_handle) = Layer::new(filter);
	let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking);
	let subscriber = tracing_subscriber::registry().with(reloadable_filter).with(file_layer);
	#[cfg(feature = "dev")]
	let console_layer = fmt::layer();
	#[cfg(feature = "dev")]
	let subscriber = subscriber.with(console_layer);

	subscriber.init();

	#[cfg(not(feature = "dev"))]
	panic::set_hook(Box::new(|p| tracing::error!("{p}")));
	air::launch(filter_handle).unwrap();
}
