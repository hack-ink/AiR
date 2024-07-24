mod audio;
use audio::Audio;

mod chat;
use chat::Chat;

mod hotkey;
use hotkey::Hotkey;

mod keyboard;
use keyboard::Keyboard;

mod quoter;
use quoter::Quoter;

// crates.io
use eframe::egui::Context;
use tokio::runtime::Runtime;
// self
use crate::{component::Components, prelude::*, state::State, util::ArtBool};

#[derive(Debug)]
pub struct Services {
	pub keyboard: Keyboard,
	pub rt: Option<Runtime>,
	pub quoter: Quoter,
	pub is_chatting: ArtBool,
	pub chat: Chat,
	pub audio: Audio,
	pub hotkey: Hotkey,
}
impl Services {
	pub fn new(ctx: &Context, components: &Components, state: &State) -> Result<Self> {
		let keyboard = Keyboard::new();
		let rt = Runtime::new()?;
		let quoter = Quoter::new(&rt, state.chat.quote.clone(), state.chat.input.inner());
		let is_chatting = ArtBool::new(false);
		let chat = Chat::new(
			keyboard.clone(),
			&rt,
			is_chatting.clone(),
			&components.setting.ai,
			&components.setting.chat,
			&state.chat,
		);
		let audio = Audio::new()?;
		let hotkey = Hotkey::new(
			ctx,
			&components.setting.hotkeys,
			state,
			keyboard.clone(),
			audio.clone(),
			chat.tx.clone(),
		)?;

		Ok(Self { keyboard, rt: Some(rt), quoter, is_chatting, chat, audio, hotkey })
	}

	pub fn is_chatting(&self) -> bool {
		self.is_chatting.load()
	}

	pub fn abort(&mut self) {
		self.keyboard.abort();
		self.quoter.abort();
		self.chat.abort();
		self.audio.abort();
		self.hotkey.abort();

		if let Some(rt) = self.rt.take() {
			rt.shutdown_background();
		}
	}
}
