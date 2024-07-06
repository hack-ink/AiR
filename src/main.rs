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
mod state;
mod ui;

mod prelude {
	pub type Result<T> = std::result::Result<T, Error>;

	pub use crate::error::*;
}

// std
use std::panic;
// crates.io
use app_dirs2::{AppDataType, AppInfo};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
	filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

const APP_INFO: AppInfo = AppInfo { name: "AiR", author: "xavier@inv.cafe" };

fn main() {
	color_eyre::install().unwrap();

	let (non_blocking, _guard) = tracing_appender::non_blocking(
		RollingFileAppender::builder()
			.rotation(Rotation::DAILY)
			.filename_suffix("log")
			.build(app_dirs2::get_app_root(AppDataType::UserData, &APP_INFO).unwrap())
			.unwrap(),
	);
	let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking);
	#[cfg(feature = "dev")]
	let console_layer = fmt::layer();
	let subscriber = tracing_subscriber::registry()
		.with(
			EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy(),
		)
		.with(file_layer);
	#[cfg(feature = "dev")]
	let subscriber = subscriber.with(console_layer);

	subscriber.init();

	panic::set_hook(Box::new(|p| {
		if let Some(p) = p.payload().downcast_ref::<&str>() {
			tracing::error!("{p}");
		} else {
			tracing::error!("panic occurred");
		}
	}));
	air::launch().unwrap();
}
