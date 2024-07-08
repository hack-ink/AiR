// std
use std::sync::Arc;
// crates.io
use parking_lot::RwLock;
// self
use crate::component::setting::Chat as ChatSetting;

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
	pub chat: Arc<RwLock<ChatSetting>>,
}
