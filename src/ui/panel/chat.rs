// crates.io
use eframe::egui::*;
use egui_commonmark::*;
// self
use super::super::UiT;
use crate::{air::AiRContext, component::util};

#[derive(Debug, Default)]
pub struct Chat {
	// TODO: use widgets instead.
	pub input: String,
	pub output: OutputWidget,
}
impl UiT for Chat {
	fn draw(&mut self, ui: &mut Ui, ctx: &mut AiRContext) {
		let size = ui.available_size();

		ScrollArea::vertical().id_source("Input").max_height((size.y - 50.) / 2.).show(ui, |ui| {
			ui.add_sized(
				(size.x, ui.available_height()),
				TextEdit::multiline({
					if ctx.services.hotkey.is_running() {
						if let Ok(i) = ctx.state.chat.input.try_read() {
							i.clone_into(&mut self.input);
						}
					}

					&mut self.input
				})
				.hint_text(ctx.components.quote.get()),
			);
		});

		// Indicators.
		ui.horizontal(|ui| {
			ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
				// TODO: when to show the spinner.
				ui.spinner();
				ui.vertical(|ui| {
					ui.add_space(4.5);
					ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
						let (ic, oc) =
							ctx.components.tokenizer.count_token(&self.input, &self.output.value);
						let (ip, op) = ctx.components.setting.ai.model.prices();

						ui.label(format!(
							"{} tokens (${:.6})",
							ic + oc,
							util::price_rounded(ic as f32 * ip + oc as f32 * op)
						));
					});
				});
			});
		});

		ui.separator();

		// Shortcuts.
		ui.horizontal(|ui| {
			ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
				if !self.output.widget.triggered {
					if ui.add(self.output.widget.copy.clone()).clicked() {
						self.output.widget.triggered = true;
					}
				} else {
					ui.add(self.output.widget.copied.clone());
				}
			});
		});

		// TODO?: use markdown.
		CommonMarkViewer::new("Output").show_scrollable(ui, &mut CommonMarkCache::default(), {
			if ctx.services.hotkey.is_running() {
				if let Ok(o) = ctx.state.chat.output.try_read() {
					o.clone_into(&mut self.output.value);
				}
			}

			&self.output.value
		});
	}
}

#[derive(Debug, Default)]
pub struct OutputWidget {
	value: String,
	widget: CopyWidget,
}
// TODO: https://github.com/emilk/egui/issues/3453.
#[derive(Debug)]
pub struct CopyWidget {
	copy: Image<'static>,
	copied: Image<'static>,
	triggered: bool,
}
impl Default for CopyWidget {
	fn default() -> Self {
		Self {
			copy: Image::new(include_image!("../../../asset/copy.svg"))
				.max_size((16., 16.).into())
				.sense(Sense::click()),
			copied: Image::new(include_image!("../../../asset/check.svg"))
				.max_size((16., 16.).into()),
			triggered: false,
		}
	}
}
