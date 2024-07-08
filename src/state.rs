// std
use std::sync::Arc;
// crates.io
use parking_lot::RwLock;

#[derive(Debug, Default)]
pub struct State {
	pub chat: Chat,
}

#[derive(Debug, Default)]
pub struct Chat {
	pub quote: Arc<RwLock<String>>,
	pub input: Arc<RwLock<String>>,
	pub output: Arc<RwLock<String>>,
}
