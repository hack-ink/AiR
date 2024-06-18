// std
use std::sync::{atomic::AtomicBool, Arc, RwLock};
// self
use crate::component::setting::Translation;

#[derive(Debug, Default)]
pub struct State {
	// TODO: https://github.com/emilk/egui/issues/4468.
	pub to_hide: Arc<AtomicBool>,
	pub chat: Chat,
	pub setting: Setting,
}

#[derive(Debug, Default)]
pub struct Chat {
	pub input: Arc<RwLock<String>>,
	pub output: Arc<RwLock<String>>,
}

#[derive(Debug, Default)]
pub struct Setting {
	pub translation: Arc<RwLock<Translation>>,
}
