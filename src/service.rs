mod chat;
use chat::Chat;

mod hotkey;
use hotkey::Hotkey;

mod keyboard;
use keyboard::Keyboard;

mod quoter;
use quoter::Quoter;

// std
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};
// crates.io
use eframe::egui::Context;
use tokio::runtime::Runtime;
// self
use crate::{component::Components, prelude::*, state::State};

#[derive(Debug)]
pub struct Services {
	pub keyboard: Keyboard,
	pub rt: Option<Runtime>,
	pub quoter: Quoter,
	pub is_chatting: Arc<AtomicBool>,
	pub chat: Chat,
	pub hotkey: Hotkey,
}
impl Services {
	pub fn new(ctx: &Context, components: &Components, state: &State) -> Result<Self> {
		let keyboard = Keyboard::new();
		let rt = Runtime::new()?;
		let quoter = Quoter::new(&rt, state.chat.quote.clone());
		let is_chatting = Arc::new(AtomicBool::new(false));
		let chat =
			Chat::new(keyboard.clone(), &rt, is_chatting.clone(), &components.setting, &state.chat);
		let hotkey =
			Hotkey::new(ctx, keyboard.clone(), &components.setting.hotkeys, chat.tx.clone())?;

		Ok(Self { keyboard, rt: Some(rt), quoter, is_chatting, chat, hotkey })
	}

	pub fn is_chatting(&self) -> bool {
		self.is_chatting.load(Ordering::SeqCst)
	}

	pub fn abort(&mut self) {
		self.keyboard.abort();
		self.quoter.abort();
		self.chat.abort();
		self.hotkey.abort();

		if let Some(rt) = self.rt.take() {
			rt.shutdown_background();
		}
	}
}
