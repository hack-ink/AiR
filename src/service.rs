mod hotkey;
use hotkey::Hotkey;

// crates.io
use eframe::egui::Context;
use tokio::runtime::Runtime;
// self
use crate::{component::Components, prelude::*, state::State};

#[derive(Debug)]
pub struct Services {
	pub hotkey: Hotkey,
}
impl Services {
	pub fn init(
		ctx: &Context,
		runtime: &Runtime,
		components: &mut Components,
		state: &State,
	) -> Result<Self> {
		let hotkey = Hotkey::init(ctx, runtime, components, state)?;

		Ok(Self { hotkey })
	}
}
