// std
use std::sync::{atomic::AtomicBool, Arc};
// crates.io
use parking_lot::RwLock;
// self
use crate::component::setting::Setting;

#[derive(Debug)]
pub struct State {
	pub general: General,
	pub chat: Chat,
}
impl State {
	pub fn new(setting: &Setting) -> Self {
		Self {
			general: General {
				hide_on_lost_focus: Arc::new(AtomicBool::new(setting.general.hide_on_lost_focus)),
			},
			chat: Chat {
				quote: Arc::new(RwLock::new(String::new())),
				input: Arc::new(RwLock::new(String::new())),
				output: Arc::new(RwLock::new(String::new())),
			},
		}
	}
}

#[derive(Debug)]
pub struct General {
	pub hide_on_lost_focus: Arc<AtomicBool>,
}

#[derive(Debug)]
pub struct Chat {
	pub quote: Arc<RwLock<String>>,
	pub input: Arc<RwLock<String>>,
	pub output: Arc<RwLock<String>>,
}
