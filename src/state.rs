// std
use std::sync::{Arc, RwLock};
// self
use crate::component::setting::Translation;

#[derive(Debug, Default)]
pub struct State {
	pub chat: Chat,
	pub setting: Setting,
}

#[derive(Debug, Default)]
pub struct Chat {
	pub quote: Arc<RwLock<String>>,
	pub input: Arc<RwLock<String>>,
	pub output: Arc<RwLock<String>>,
}

#[derive(Debug, Default)]
pub struct Setting {
	pub translation: Arc<RwLock<Translation>>,
}
