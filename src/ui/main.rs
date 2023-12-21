// std
use std::{
	fmt::{Debug, Formatter, Result},
	sync::mpsc::Receiver,
};
// crates.io
use clipboard::*;
use eframe::egui::*;
use egui_commonmark::*;
// air
use crate::hotkey::{self, Function};

pub(super) struct Main {
	hotkey_rx: Receiver<Function>,
	clipboard: ClipboardContext,
	input: String,
	output: String,
}
impl Main {
	pub(super) fn draw(&mut self, ui: &mut Ui) {
		if let Ok(hk) = self.hotkey_rx.try_recv() {
			match hk {
				Function::Polish => self.input = self.clipboard.get_contents().unwrap_or_default(),
			}
		}

		let size = ui.available_size();

		ScrollArea::vertical().id_source("Input").max_height((size.y - 50.) / 2.).show(ui, |ui| {
			ui.add_sized((size.x, ui.available_height()), TextEdit::multiline(&mut self.input).hint_text("Type something..."));
		});

		ui.add_space(20.);
		ui.separator();

		CommonMarkViewer::new("Output").show_scrollable(
			ui,
			&mut CommonMarkCache::default(),
			&self.output,
		);
	}
}
impl Debug for Main {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.debug_struct("Main")
			.field("hotkey_rx", &self.hotkey_rx)
			.field("input", &self.input)
			.finish()
	}
}
impl Default for Main {
	fn default() -> Self {
		Self {
			hotkey_rx: hotkey::register().unwrap(),
			clipboard: ClipboardContext::new().unwrap(),
			input: Default::default(),
			output: "There is no expedient to which a man will not go to avoid the labor of thinking.\n\nThomas Edison".into(),
		}
	}
}
