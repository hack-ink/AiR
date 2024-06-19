mod hotkey;
use hotkey::Hotkey;

mod quoter;
use quoter::Quoter;

// crates.io
use eframe::egui::Context;
use tokio::runtime::Runtime;
// self
use crate::{component::Components, prelude::*, state::State};

#[derive(Debug)]
pub struct Services {
	pub rt: Option<Runtime>,
	pub quoter: Quoter,
	pub hotkey: Hotkey,
}
impl Services {
	pub fn init(ctx: &Context, components: &Components, state: &State) -> Result<Self> {
		let rt = Runtime::new()?;
		let quoter = Quoter::init(&rt, state.chat.quote.clone());
		let hotkey = Hotkey::init(ctx, &rt, components, state)?;

		Ok(Self { rt: Some(rt), quoter, hotkey })
	}
}
