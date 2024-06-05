// std
use std::{
	fmt::{Debug, Formatter, Result},
	sync::{Arc, Mutex},
};
// crates.io
use clipboard::*;
use eframe::egui::*;
use egui_commonmark::*;
// self
use crate::{
	component::{function::*, hotkey::*, openai::*, quoter::*, util},
	data::Setting,
};

const HINT: &str = r#"  -----------
< Thinking... >
  -----------
         \   ^__^
          \  (oo)\_______
             (__)\       )\/\
                 ||----w |
                 ||     ||"#;

pub(super) struct Main {
	hotkey: Hotkey,
	clipboard: ClipboardContext,
	quoter: Quoter,
	openai: OpenAi,
	input: String,
	// TODO: Combine these things into one struct.
	output: Arc<Mutex<String>>,
	token_count: Arc<Mutex<(usize, usize)>>,
}
impl Main {
	pub(super) fn from_setting(setting: &Setting) -> Self {
		Self {
			hotkey: Hotkey::register().unwrap(),
			clipboard: ClipboardContext::new().unwrap(),
			quoter: Quoter::default(),
			openai: OpenAi::new(setting.ai.raw.clone()),
			input: Default::default(),
			output: Default::default(),
			token_count: Default::default(),
		}
	}

	pub(super) fn draw(&mut self, ui: &mut Ui) {
		if let Ok(f) = self.hotkey.try_recv() {
			// TODO?: Move to hotkey.
			match f {
				Function::Polish => {
					self.input = self.clipboard.get_contents().unwrap_or_default();

					self.output.lock().unwrap().clear();
					// self.openai.chat(
					// 	f.prompt(),
					// 	&self.input,
					// 	self.output.clone(),
					// 	self.token_count.clone(),
					// );
				},
			}
		}

		let size = ui.available_size();

		// Input.
		ScrollArea::vertical().id_source("Input").max_height((size.y - 50.) / 2.).show(ui, |ui| {
			ui.add_sized(
				(size.x, ui.available_height()),
				TextEdit::multiline(&mut self.input).hint_text(
					self.quoter
						.quote
						.try_lock()
						.ok()
						.and_then(|q: std::sync::MutexGuard<'_, Option<String>>| q.clone())
						.unwrap_or(HINT.into()),
				),
			);
		});

		// Separator.
		ui.add_space(20.);
		ui.separator();

		// Usage.
		ui.horizontal(|ui| {
			ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
				ui.spinner();
				ui.vertical(|ui| {
					ui.add_space(4.5);
					ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
						let (i, o) = self.token_count.lock().unwrap().to_owned();
						let (ip, op) = Model::price_of(&self.openai.setting.model);

						ui.label(format!(
							"{} tokens (${:.6})",
							i + o,
							util::price_rounded(i as f32 * ip + o as f32 * op)
						));
					});
				});
			});
		});

		// Output.
		CommonMarkViewer::new("Output").show_scrollable(
			ui,
			&mut CommonMarkCache::default(),
			&self.output.lock().unwrap(),
		);
	}

	pub(super) fn refresh(&mut self) {
		self.quoter.refresh();
	}
}
impl Debug for Main {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.debug_struct("Main").field("hotkey", &self.hotkey).field("input", &self.input).finish()
	}
}
