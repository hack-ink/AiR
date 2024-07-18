// std
use std::sync::{atomic::AtomicU32, Arc};
// crates.io
use parking_lot::RwLock;
use tracing::Level;
use tracing_subscriber::{reload::Handle, EnvFilter, Registry};
// self
use crate::{
	component::{function::Function, setting::Setting},
	prelude::*,
	ui::panel::Panel,
	util::ArtBool,
};

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
		let chat = Chat::new(setting.chat.activated_function);
		let development = Development { log_filter_handle };

		development.reload_log_filter(setting.development.log_level.into())?;

		Ok(Self { general, chat, development, ui: Default::default() })
	}
}

#[derive(Debug, Default)]
pub struct General {
	pub notification_sound: ArtBool,
}

#[derive(Debug)]
pub struct Chat {
	pub quote: Arc<RwLock<String>>,
	pub activated_function: Arc<RwLock<Function>>,
	pub input: Arc<RwLock<String>>,
	pub output: Arc<RwLock<String>>,
	pub token_counts: Arc<(AtomicU32, AtomicU32)>,
	pub error: ArtBool,
}
impl Chat {
	pub fn new(activated_function: Function) -> Self {
		Self {
			quote: Default::default(),
			activated_function: Arc::new(RwLock::new(activated_function)),
			input: Default::default(),
			output: Default::default(),
			token_counts: Default::default(),
			error: Default::default(),
		}
	}
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
