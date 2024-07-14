// std
use std::sync::atomic::Ordering;
// crates.io
use eframe::egui::*;
// self
use super::Chat;
use crate::{
	air::AiRContext,
	component::openai::Model,
	util,
	widget::{ShortcutWidget, SMALL_FONT_OFFSET},
};

#[derive(Debug, Default)]
pub struct Status {
	shortcut: ShortcutWidget,
}
impl Status {
	pub fn draw(&mut self, ctx: &mut AiRContext, ui: &mut Ui, y: f32, chat: &Chat) {
		let dark_mode = ui.visuals().dark_mode;

		ui.horizontal(|ui| {
			ui.set_height(y);
			ui.vertical(|ui| {
				ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
					let tcs = &ctx.state.chat.token_counts;
					let (itc, otc) = (tcs.0.load(Ordering::Relaxed), tcs.1.load(Ordering::Relaxed));
					let (ip, op) = ctx.components.setting.ai.model.prices();

					ui.hyperlink_to(
						RichText::new(format!(
							"↑{itc} ↓{otc} ${:.6}",
							util::price_rounded(itc as f32 * ip + otc as f32 * op)
						))
						.size(ctx.components.setting.general.font_size - SMALL_FONT_OFFSET),
						Model::PRICE_URI,
					)
					.on_hover_text(
						"The token indicator might not work if you are using a custom API provider.",
					);

					if ctx.services.is_chatting() {
						self.shortcut.copy.triggered = false;

						ui.spinner();
					}
				});
			});
			ui.vertical(|ui| {
				ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
					if ui.add(self.shortcut.send.icon(dark_mode)).clicked() {
						// TODO: the state will not be synced if previous action is triggered by
						// hotkey.
						if ui.add(self.shortcut.send.icon(dark_mode)).clicked() {
							ctx.services.chat.send((
								ctx.components.setting.general.active_func.basic(),
								chat.input.clone(),
								false,
							));
						}
					}
					if !self.shortcut.copy.triggered {
						if ui.add(self.shortcut.copy.copy_icon(dark_mode)).clicked() {
							self.shortcut.copy.triggered = true;
							ctx.components
								.clipboard
								.set_text(&chat.output)
								.expect("clipboard must be available");
						}
					} else {
						ui.add(self.shortcut.copy.copied_icon(dark_mode));
					}
				});
			});
		});
	}
}
