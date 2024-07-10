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
use crate::{component::setting::Setting, prelude::*};

#[derive(Debug)]
pub struct State {
	pub general: General,
	pub chat: Chat,
	pub development: Development,
}
impl State {
	pub fn new(log_filter_handle: Handle<EnvFilter, Registry>, setting: &Setting) -> Result<Self> {
		let development = Development { log_filter_handle };

		development.reload_log_filter(setting.development.log_level.clone().into())?;

		Ok(Self {
			general: General {
				hide_on_lost_focus: Arc::new(AtomicBool::new(setting.general.hide_on_lost_focus)),
			},
			chat: Default::default(),
			development,
		})
	}
}

#[derive(Debug)]
pub struct General {
	pub hide_on_lost_focus: Arc<AtomicBool>,
}

#[derive(Debug, Default)]
pub struct Chat {
	pub quote: Arc<RwLock<String>>,
	pub input: Arc<RwLock<String>>,
	pub output: Arc<RwLock<String>>,
	pub token_counts: Arc<(AtomicU32, AtomicU32)>,
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
