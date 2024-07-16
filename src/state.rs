// std
use std::sync::{
	atomic::{AtomicBool, AtomicU32},
	Arc,
};
// crates.io
use parking_lot::RwLock;
use tracing::Level;
use tracing_subscriber::{reload::Handle, EnvFilter, Registry};
// self
use crate::{component::setting::Setting, prelude::*, ui::panel::Panel};

#[derive(Debug)]
pub struct State {
	pub chat: Chat,
	pub development: Development,
	pub ui: Ui,
}
impl State {
	pub fn new(log_filter_handle: Handle<EnvFilter, Registry>, setting: &Setting) -> Result<Self> {
		let development = Development { log_filter_handle };

		development.reload_log_filter(setting.development.log_level.into())?;

		Ok(Self { chat: Default::default(), development, ui: Default::default() })
	}
}

#[derive(Debug, Default)]
pub struct Chat {
	pub quote: Arc<RwLock<String>>,
	pub input: Arc<RwLock<String>>,
	pub output: Arc<RwLock<String>>,
	pub token_counts: Arc<(AtomicU32, AtomicU32)>,
	pub error: Arc<AtomicBool>,
}

#[derive(Debug)]
pub struct Development {
	pub log_filter_handle: Handle<EnvFilter, Registry>,
}
impl Development {
	pub fn reload_log_filter(&self, level: Level) -> Result<()> {
		self.log_filter_handle
			.reload(EnvFilter::builder().with_default_directive(level.into()).from_env_lossy())?;

		Ok(())
	}
}

#[derive(Debug, Default)]
pub struct Ui {
	pub focused_panel: Arc<RwLock<Panel>>,
}
