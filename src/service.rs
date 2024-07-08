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
use crate::{component::Components, prelude::*, state::State};

#[derive(Debug)]
pub struct Services {
	pub keyboard: Keyboard,
	pub rt: Option<Runtime>,
	pub quoter: Quoter,
	pub hotkey: Hotkey,
}
impl Services {
	pub fn init(ctx: &Context, components: &Components, state: &State) -> Result<Self> {
		let keyboard = Keyboard::init();
		let rt = Runtime::new()?;
		let quoter = Quoter::init(&rt, state.chat.quote.clone());
		let hotkey = Hotkey::init(ctx, keyboard.clone(), &rt, components, state)?;

		Ok(Self { keyboard, rt: Some(rt), quoter, hotkey })
	}

	pub fn abort(&mut self) {
		self.keyboard.abort();
		self.quoter.abort();
		self.hotkey.abort();

		if let Some(rt) = self.rt.take() {
			rt.shutdown_background();
		}
	}
}
