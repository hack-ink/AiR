// std
use std::fmt::{Debug, Formatter, Result};
// crates.io
use clipboard::*;
use eframe::egui::*;
use egui_commonmark::*;
// air
use crate::component::{hotkey::*, quoter::*};

pub(super) struct Main {
	hotkey: Hotkey,
	clipboard: ClipboardContext,
	quoter: Quoter,
	input: String,
	output: String,
}
impl Main {
	pub(super) fn draw(&mut self, ui: &mut Ui) {
		if let Ok(hk) = self.hotkey.try_recv() {
			// TODO?: Move to hotkey.
			match hk {
				Function::Polish => self.input = self.clipboard.get_contents().unwrap_or_default(),
			}
		}

		let size = ui.available_size();

		ScrollArea::vertical().id_source("Input").max_height((size.y - 50.) / 2.).show(ui, |ui| {
			ui.add_sized(
				(size.x, ui.available_height()),
				TextEdit::multiline(&mut self.input).hint_text(
					self.quoter.quote.lock().map(|q| q.to_string()).unwrap_or("Thinking...".into()),
				),
			);
		});

		ui.add_space(20.);
		ui.separator();

		CommonMarkViewer::new("Output").show_scrollable(
			ui,
			&mut CommonMarkCache::default(),
			&self.output,
		);
	}

	pub(super) fn try_update(&mut self) {
		self.quoter.try_update();
	}
}
impl Debug for Main {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.debug_struct("Main").field("hotkey", &self.hotkey).field("input", &self.input).finish()
	}
}
impl Default for Main {
	fn default() -> Self {
		Self {
			hotkey: Hotkey::register().unwrap(),
			clipboard: ClipboardContext::new().unwrap(),
			quoter: Quoter::new(),
			input: Default::default(),
			output: Default::default(),
		}
	}
}
