// std
use std::sync::{atomic::AtomicU32, Arc};
// crates.io
use parking_lot::RwLock;
use tracing::Level;
use tracing_subscriber::{reload::Handle, EnvFilter, Registry};
// self
use crate::{component::setting::Setting, prelude::*, ui::panel::Panel, util::ArtBool};

#[derive(Debug)]
pub struct State {
	pub general: General,
	pub chat: Chat,
	pub development: Development,
	pub ui: Ui,
}
impl State {
	pub fn new(log_filter_handle: Handle<EnvFilter, Registry>, setting: &Setting) -> Result<Self> {
		let general =
			General { notification_sound: ArtBool::new(setting.general.notification_sound) };
		let development = Development { log_filter_handle };

		development.reload_log_filter(setting.development.log_level.into())?;

		Ok(Self { general, chat: Default::default(), development, ui: Default::default() })
	}
}

#[derive(Debug, Default)]
pub struct General {
	pub notification_sound: ArtBool,
}

#[derive(Debug, Default)]
pub struct Chat {
	pub quote: Arc<RwLock<String>>,
	pub input: Arc<RwLock<String>>,
	pub output: Arc<RwLock<String>>,
	pub token_counts: Arc<(AtomicU32, AtomicU32)>,
	pub error: ArtBool,
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
