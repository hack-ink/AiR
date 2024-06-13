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
	pub output: String,
}
impl UiT for Chat {
	fn draw(&mut self, ui: &mut Ui, ctx: &mut AiRContext) {
		if let Some((func, input)) = ctx.services.hotkey.try_recv() {
			// TODO: focus on the chat panel.

			self.input = input;

			ctx.components
				.openai
				.chat(ctx.runtime, func.prompt(), &self.input)
				.expect("chat must succeed");
		}
		if let Ok(output) = ctx.components.openai.output.try_lock() {
			output.clone_into(&mut self.output);
		}

		let size = ui.available_size();

		// Input.
		ScrollArea::vertical().id_source("Input").max_height((size.y - 50.) / 2.).show(ui, |ui| {
			ui.add_sized(
				(size.x, ui.available_height()),
				TextEdit::multiline(&mut self.input).hint_text(ctx.components.quote.get()),
			);
		});

		// Separator.
		ui.add_space(20.);
		ui.separator();

		// Usage.
		ui.horizontal(|ui| {
			ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
				// TODO: when to show the spinner.
				ui.spinner();
				ui.vertical(|ui| {
					ui.add_space(4.5);
					ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
						// let (i, o) = self.token_count.lock().unwrap().to_owned();
						let (i, o) = (0, 0);
						let (ip, op) = ctx.components.setting.ai.model.prices();

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
			&self.output,
		);
	}
}
